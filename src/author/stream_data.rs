use crate::author::author::Stream;
use crate::settings;

extern crate base64;

use anyhow::{Result};
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDateTime};


lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    certificate_number  : u32,
    partner_code : String,
    data_keys : String,
    date_time: NaiveDateTime,
}

impl Payload {
    pub fn new(
        certificate_number: u32,
        partner_code: String,
        data_keys: String
    ) -> Self{
        Self{
            certificate_number,
            partner_code,
            data_keys,
            date_time: Local::now().naive_utc(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamsData {
    state : String, // user's state
    payload : Payload // content of the message to be sent/read
}

impl StreamsData {
    pub async fn new(
    ) -> Result<Self> {

        let mut author = Stream::new(String::from(&CONFIG.node),None);
        let announce_result = author.author.send_announce()?;
        let announcement_tag = announce_result.msgid.to_string();
        let bytes = author.author.export(&CONFIG.password)?;
        let state =  base64::encode(&bytes);

        let payload = Payload::new(
            0,
            "".to_string(),
            " ".to_string(),
        );

        let serialized_payload = serde_json::to_string(&payload)?;

        author.send_tagged_packet( &announcement_tag,&serialized_payload)?;

        Ok (Self {
            state,
            payload
        } )
    }

    pub async fn send_message(
        data : StreamsData
    ) -> Result<String>  {

        let byte = base64::decode(&data.state)?;
        let mut author = Stream::open(
            String::from(&CONFIG.node),
            &byte,
            &CONFIG.password
        )?;

        // get identifier of the last message
        let tag_list = author.get_next_message()?;

        let msgid = tag_list.last().unwrap();

        let serialized_payload = serde_json::to_string(&data.payload)?;

        author.send_tagged_packet( msgid,&serialized_payload)?;

        let bytes = author.author.export(&CONFIG.password)?;
        let state =  base64::encode(&bytes);

        Ok(state)

    }

    pub async fn read_message(
        state : String
    ) -> Result<Payload>  {
        let byte = base64::decode(&state)?;
        let mut author = Stream::open(
            String::from(&CONFIG.node),
            &byte,
            &CONFIG.password
        )?;

        // get identifier of the last message
        let tag_list = author.get_next_message()?;
        let last_msg_id = tag_list.last().unwrap();

        let message = author.read_tagged_packet(last_msg_id)?;

        let payload: Payload = serde_json::from_str(&message).unwrap();

        Ok(payload)

    }

    pub async fn read_all_messages(
        root : String
    ) -> Result<Vec<Payload>>  {
        let byte = base64::decode(&root)?;
        let mut author = Stream::open(
            String::from(&CONFIG.node),
            &byte,
            &CONFIG.password
        )?;

        let tag_list = author.get_next_message()?;

        let mut msg_list: Vec<Payload> = vec![];

        for message_tag in tag_list {
            let message = author.read_tagged_packet(&message_tag).unwrap();
            let payload: Payload = serde_json::from_str(&message).unwrap();

            msg_list.push(payload);
        }

        Ok(msg_list)

    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     #[should_panic]
//     fn create_author_doesnt_panic() {
//         const URL: &str = "https://chrysalis-nodes.iota.org";
//         Stream::new(String::from(node));
//     }
// }
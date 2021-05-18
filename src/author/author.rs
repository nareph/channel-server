use crate::author::random_seed;

use iota_streams::app::transport::tangle::{client::Client, PAYLOAD_BYTES};
use iota_streams::app_channels::api::tangle::{Address, Author};

use anyhow::{Result};
use iota_streams::ddml::types::Bytes;

use std::str;

///
/// Channel author
///
pub struct Stream {
    pub author: Author<Client>
}

impl Stream {
    ///
    /// Initialize the author
    ///
    pub fn new(
        node: String,
        seed_option: Option<String>,
    ) -> Stream {
        let seed = match seed_option {
            Some(seed) => seed,
            None => random_seed(),
        };
        let multi_branching_flag = false;
        let encoding = "utf-8";
        let client: Client = Client::new_from_url(&node);
        let author = Author::new(
            &seed,
            encoding,
            PAYLOAD_BYTES,
            multi_branching_flag,
            client,
        );

        Self {
            author,
        }
    }

    ///
    /// get the author
    ///
    pub fn open(
        node: String,
        state: &[u8],
        pwd : &str
    ) -> Result<Stream> {
        let client: Client = Client::new_from_url(&node);
        let author = Author::import(
            state,
            pwd,
            client,
        )?;

        Ok(Self {
            author,
        })
    }

    ///
    /// send signed packet
    ///
    // fn send_signed_packet(
    //     &mut self,
    //     announce_message_identifier: &String,
    //     public_payload: &String,
    // ) -> Result<Address> {
    //     // Convert the payloads to a Trytes type
    //     let public_payload = Bytes(public_payload.as_bytes().to_vec());
    //     let empty_masked_payload = Bytes("".as_bytes().to_vec());
    //
    //     let channel_address= self.author.channel_address().unwrap().to_string();
    //
    //     // Convert the channel address and message identifier to an Address link type
    //     let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier)
    //     {
    //         Ok(announcement_link) => announcement_link,
    //         Err(e) => bail!(
    //         "Failed to create Address from {}:{}. Reason: {}",
    //         &channel_address,
    //         &announce_message_identifier,
    //         e
    //     ),
    //     };
    //
    //     // Create a `SignedPacket` message and link it to the message identifier of the `Announce` message
    //     let (msg, seq) =
    //         self.author.send_signed_packet(&announcement_link, &public_payload, &empty_masked_payload)?;
    //
    //     Ok(seq.unwrap_or(msg))
    // }


    ///
    /// send tagged packet
    ///
    pub fn send_tagged_packet(
        &mut self,
        message_identifier: &String,
        public_payload: &String,
    ) -> Result<Address> {
        // Convert the payloads to a Trytes type
        let public_payload = Bytes(public_payload.as_bytes().to_vec());
        let empty_masked_payload = Bytes("".as_bytes().to_vec());

        let channel_address= self.author.channel_address().unwrap().to_string();

        // Convert the channel address and message identifier to an Address link type
        let message_link = Address::from_str(&channel_address, &message_identifier)?;
        // {
        //     Ok(message_link) => message_link,
        //     Err(e) => bail!(
        //     "Failed to create Address from {}:{}. Reason: {}",
        //     &channel_address,
        //     &message_identifier,
        //     e
        // ),
        // };

        // Create a `TaggedPacket` message and link it to the message identifier (message_identifier)
        let (msg, seq) =
            self.author.send_tagged_packet(&message_link, &public_payload, &empty_masked_payload)?;

        Ok(seq.unwrap_or(msg))
    }

    ///
    /// Read tagged packet
    ///
    pub fn read_tagged_packet(
        &mut self,
        tagged_packet_tag: &String
    ) -> Result<String> {
        let channel_address= self.author.channel_address().unwrap().to_string();

        let link = Address::from_str(&channel_address, tagged_packet_tag)?;

        let (public_payload, _) = self.author.receive_tagged_packet(&link)?;

        let msg = String::from_utf8(public_payload.0).unwrap();

        Ok(msg)
    }

    ///
    /// Generates the next message in the channels
    ///
    pub fn get_next_message(&mut self) -> Result<Vec<String>> {
        let mut ids: Vec<String> = vec![];

        let mut msgs = self.author.fetch_next_msgs();

        for msg in &msgs {
            ids.push(msg.link.msgid.to_string());
        }

        while !msgs.is_empty() {
            msgs = self.author.fetch_next_msgs();

            for msg in &msgs {
                ids.push(msg.link.msgid.to_string());
            }
        }

        Ok(ids)
    }
}




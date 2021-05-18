#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[macro_use]
extern crate lazy_static;

mod author;
use crate::author::stream_data::StreamsData;
use tide::{Request, Response, Body};
// use log4rs::append::file::FileAppender;
// use log4rs::encode::pattern::PatternEncoder;
// use log4rs::config::{Appender, Config, Root};

mod settings;



lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[async_std::main]
async fn main() -> tide::Result<()>{

    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(utc)} {l} {t} - {m}\n")))
    //     .build("log/output.log")?;
    //
    // let config = Config::builder()
    //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
    //     .build(Root::builder()
    //         .appender("logfile")
    //         .build(tide::log::LevelFilter::Info))?;
    //
    // log4rs::init_config(config)?;

    tide::log::start();
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Welcome in iotaOrigin server!") });

    // create author & channel
    app.at("/iotaOrigin/create")
        .post(|_req: Request<()>| async move {
            // let  data: StreamsData = req.body_json().await?;
            // println!("{:?}", info);
            let mut res = Response::new(201);
            match StreamsData::new().await{
                Ok(result) => {
                    res.set_body(Body::from_json(&result)?);
                },
                Err(error) => {
                    res.set_status(500);
                    res.set_body(error.to_string());
                },

            };

            Ok(res)
        });

    // send message to channel
    app.at("/iotaOrigin/update")
        .put(|mut req: Request<()>| async move {
            let mut res = Response::new(201);
            match req.body_json().await{
                Ok(data) => {
                    match StreamsData::send_message(data).await{
                        Ok(result) => {
                            res.set_body(Body::from_json(&result)?);
                        },
                        Err(error) => {
                            res.set_status(500);
                            res.set_body(error.to_string());
                        }
                    };
                },
                Err(error) => {
                    res.set_status(500);
                    res.set_body(error.to_string());
                }
            };

            Ok(res)
        });

    // get the last message from the channel
    app.at("/iotaOrigin/balance")
        .get(|mut req: Request<()>| async move {
            let mut res = Response::new(201);
            match req.body_json().await {
                Ok(state) =>{
                    match StreamsData::read_message(state).await{
                        Ok(message) => {
                            res.set_body(Body::from_json(&message)?);
                        },
                        Err(error) => {
                            res.set_status(500);
                            res.set_body(error.to_string());
                        }
                    };
                },
                Err(error)=>{
                    res.set_status(500);
                    res.set_body(error.to_string());
                }
            }

            Ok(res)
        });

    // get the history of the channel
    app.at("/iotaOrigin/history")
        .get(|mut req: Request<()>| async move {
            let mut res = Response::new(200);
            match req.body_json().await{
                Ok(root) =>{
                    match StreamsData::read_all_messages(root).await{
                        Ok(message) => {
                            res.set_body(Body::from_json(&message)?);
                        },
                        Err(error) => {
                            res.set_status(500);
                            res.set_body(error.to_string());
                        }
                    };
                },
                Err(error) => {
                    res.set_status(500);
                    res.set_body(error.to_string());
                }
            }

            Ok(res)
        });

    let server_address = format!("{}:{}", CONFIG.server_address, CONFIG.server_port);
    app.listen(server_address).await?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn create_channel() -> tide::Result<()> {
        use tide::http::{Url, Method, Request, Response};

        let app = tide::new();
        let path = "/iotaOrigin/create";
        let server_address = format!("http://{}:{}{}", CONFIG.server_address, CONFIG.server_port,path);

        let url = Url::parse(&server_address).unwrap();
        let req = Request::new(Method::Post, url);
        let res: Response = app.respond(req).await?;

        assert_eq!(201, res.status());
        Ok(())
    }

}
#[macro_use]
extern crate serde_derive;

use reqwest::header::{REFERER,COOKIE,CONTENT_TYPE};
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
struct Session {
    pub session_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let resp: Session = r.put("https://192.168.0.81/sys/log_in")
        .header(REFERER, "https://192.168.0.81")
        .body("{\"password\":\"00000000\"}")
        .send().await?.json().await?;

    println!("{:#?}", resp);

    let res = r.get("https://192.168.0.81/data/tags/")
    .header(REFERER, "https://192.168.0.81")
    .header(COOKIE, resp.session_id)
    .send().await?.text().await?;

    println!("result: {:#?}", res);
    Ok(())
}



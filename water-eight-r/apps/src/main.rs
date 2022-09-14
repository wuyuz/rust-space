use std::{
    self,
    net::{IpAddr, SocketAddr},
};

use anyhow::anyhow;
use common::ENV_VALUES;

fn server_socket_address() -> anyhow::Result<SocketAddr> {
    Ok(SocketAddr::new(
        IpAddr::V4(ENV_VALUES.web_server_address),
        ENV_VALUES.web_server_port,
    ))
}

fn init_logging() -> anyhow::Result<()> {
    match log4rs::init_file(&ENV_VALUES.log4rs_config, Default::default()) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(
            "日志加载配置{}, 错误:{:?}",
            ENV_VALUES.log4rs_config,
            err,
        )),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_logging().unwrap();
    let address = server_socket_address().unwrap();

    let result = services::run(&address).await;
    if let Err(err) = result {
        log::error!("{}", err);
        std::process::exit(1);
    }

    Ok(())
}

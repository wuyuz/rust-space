use std::{env, net::Ipv4Addr, str::FromStr};

use dotenv::dotenv;
use once_cell::sync::Lazy;

// 环境变量
#[derive(Debug)]
pub struct EnvValues {
     pub jwt_token_secret_key: String,
     pub access_token_seconds: i64,
     pub refresh_token_seconds: i64,
     pub web_server_address: Ipv4Addr,
     pub web_server_port: u16,
     pub log_level: String,
     pub log4rs_config: String,
     pub password_hash_func: String,
     pub password_sault_len: usize,
     pub password_pepper: String,
     pub password_hash_round: u32,
     pub database_url: String,
}

//解析
pub static ENV_VALUES: Lazy<EnvValues> = Lazy::new(|| {
    dotenv().ok();

    let web_server_address =
        env::var("WEB_SERVER_ADDRESS").expect("环境变量没有ip地址");
    let web_server_address = Ipv4Addr::from_str(&web_server_address)
        .expect("ip地址解析错误");

    EnvValues {
        jwt_token_secret_key: env::var("JWT_TOKEN_SECRET_KEY")
            .expect("環境変数にSECRET_KEYが設定されていません。"),
        access_token_seconds: env::var("ACCESS_TOKEN_SECONDS")
            .expect("環境変数にACCESS_TOKEN_SECONDSが設定されていません。")
            .parse::<i64>()
            .expect("環境変数に設定されているACCESS_TOKEN_SECONDSが不正です。"),
        refresh_token_seconds: env::var("REFRESH_TOKEN_SECONDS")
            .expect("環境変数にREFRESH_TOKEN_SECONDSが設定されていません。")
            .parse::<i64>()
            .expect("環境変数に設定されているREFRESH_TOKEN_SECONDSが不正です。"),
        web_server_address,
        web_server_port: env::var("WEB_SERVER_PORT")
            .expect("環境変数にWEB_SERVER_PORTが設定されていません。")
            .parse::<u16>()
            .expect("環境変数に設定されているWEB_SERVER_PORTが不正です。"),
        log_level: env::var("RUST_LOG").expect("環境変数にRUST_LOGが設定されていません。"),
        log4rs_config: env::var("LOG4RS_CONFIG")
            .expect("環境変数にLOG4RS_CONFIGが設定されていません。"),
        password_hash_func: env::var("PASSWORD_HASH_FUNC")
            .expect("環境変数にPASSWORD_HASH_FUNCが設定されていません。"),
        password_sault_len: env::var("PASSWORD_SAULT_LEN")
            .expect("環境変数にPASSWORD_SAULT_LENが設定されていません。")
            .parse::<usize>()
            .expect("環境変数に設定されているPASSWORD_SAULT_LENが不正です。"),
        password_pepper: env::var("PASSWORD_PEPPER")
            .expect("環境変数にPASSWORD_PEPPERが設定されていません。"),
        password_hash_round: env::var("PASSWORD_HASH_ROUND")
            .expect("環境変数にPASSWORD_HASH_ROUNDが設定されていません。")
            .parse::<u32>()
            .expect("環境変数に設定されているPASSWORD_HASH_ROUNDが不正です。"),
        database_url: env::var("DATABASE_URL")
            .expect("環境変数にDATABASE_URLが設定されていません。"),
    }

});




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleReq {
    pub role_name: String
}


use druid::{ Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    pub name: String,
}


#[derive(Clone, Data, Lens)]
pub struct AppStauts {
    pub email: String,
    pub token: String,
    
    pub data: ClientData,
    pub status: bool,
}

impl AppStauts {
    pub fn new() -> AppStauts {
        AppStauts {
            email: "".into(),
            token: "".into(),
            status: false,
            data : ClientData::new(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct ClientData {
    pub search_text: String,

    // pub user_list: Arc<Vec<String>>,
    pub item: String,
}

impl ClientData {
    fn new() -> ClientData {
        ClientData{
            search_text: "".into(),
            item: "".into()
        }
    }
}
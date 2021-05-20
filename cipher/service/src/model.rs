use serde_derive::Serialize;


#[derive(Serialize)]
// login PostModel
pub struct LoginModel{
    email : String
}
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub  name: String,
    pub email: String,
}

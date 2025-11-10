
#[derive(Clone, Debug)]
pub struct Jwt {
    pub access_token: String,
    pub refresh_token: String,
    
}


#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: String,
    pub username: String,
    pub chat_id: String,
    pub jwt: Option<Jwt>
}

impl Session {
    pub fn new(user_id: String, username: String, chat_id: String) -> Self {
        Self {
            user_id,
            username,
            chat_id,
            jwt: None,
        }
    }
}



use serde::{Deserialize, Serialize};

/// Запрос от Telegram бота
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTelegramBotDto {
    pub client_id: String,
    pub secret: String,
    pub telegram_id: String,
    pub username: String,
}

/// Ответ с access/refresh токенами
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JwtResponse {
    #[serde(rename = "token")]
    pub access_token: String,
    pub refresh_token: String,
}
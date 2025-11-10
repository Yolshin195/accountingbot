use async_trait::async_trait;
use crate::accounting_client_api::dto::category_dto::{CategoryDto, CreateCategoryDto};
use crate::accounting_client_api::dto::pagination_dto::{PagedResponse, Pagination};
use crate::accounting_client_api::dto::transaction_dto::{CategoryExpenseSummaryDto, CreateTransactionDto, MonthlyTransactionQuery, TransactionDto};
use crate::accounting_client_api::dto::user_dto::{JwtResponse, LoginTelegramBotDto};
use crate::dto::user::Session;

const LOGIN_TELEGRAM_PATH: &str = "/users/login/telegram";
const CREATE_CATEGORY_PATH: &str = "/categories";

#[async_trait]
pub trait AccountingClientApi: Send + Sync {
    // Users
    async fn login_telegram(&self, telegram_id: &String, username: &String) -> Result<JwtResponse, String>;
    
    // Categories
    async fn create_category(&self, data: CreateCategoryDto) -> Result<CategoryDto, String>;
    async fn list_categories(&self, params: Pagination) -> Result<PagedResponse<CategoryDto>, String>;
    
    // Transactions
    async fn create_expense_transaction(&self, data: CreateTransactionDto) -> Result<TransactionDto, String>;
    async fn sum_today_expenses_grouped_by_category(&self) -> Result<CategoryExpenseSummaryDto, String>;
    async fn find_all_transaction_by_month(&self, params: MonthlyTransactionQuery) -> Result<PagedResponse<TransactionDto>, String>;
}

#[derive(Clone)]
pub struct AccountingClientApiRequest {
    base_url: String,
    login_telegram_url: String,
    create_category_path: String,
    client_id: String,
    client_secret: String,
    user_session: Session,
}

impl AccountingClientApiRequest {
    pub fn from_env(user_session: Session) -> Self {
        let base_url = std::env::var("ACCOUNTING_CLIENT_API_URL").expect("ACCOUNTING_CLIENT_API_URL not found");
        let client_id = std::env::var("TELEGRAM_BOT_CLIENT_ID").expect("TELEGRAM_BOT_CLIENT_ID not found");
        let client_secret = std::env::var("TELEGRAM_BOT_SECRET").expect("TELEGRAM_BOT_SECRET not found");
        Self {
            base_url: base_url.clone(),
            login_telegram_url: format!("{}{}", base_url, LOGIN_TELEGRAM_PATH),
            create_category_path: format!("{}{}", base_url, CREATE_CATEGORY_PATH),
            client_id,
            client_secret,
            user_session,
        }
    }
}

#[async_trait]
impl AccountingClientApi for AccountingClientApiRequest {
    async fn login_telegram(&self, telegram_id: &String, username: &String) -> Result<JwtResponse, String> {
        let client = reqwest::Client::new();
        let data = LoginTelegramBotDto {
            telegram_id: telegram_id.clone(),
            username: username.clone(),
            client_id: self.client_id.clone(),
            secret: self.client_secret.clone()
        };
        let res = client
            .post(&self.login_telegram_url)
            .json(&data)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<JwtResponse>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(res)
    }

    async fn create_category(&self, data: CreateCategoryDto) -> Result<CategoryDto, String> {
        let client = reqwest::Client::new();

        let res = client
            .post(&self.create_category_path)
            .json(&data)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<CategoryDto>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(res)
    }

    async fn list_categories(&self, params: Pagination) -> Result<PagedResponse<CategoryDto>, String> {
        todo!()
    }

    async fn create_expense_transaction(&self, data: CreateTransactionDto) -> Result<TransactionDto, String> {
        todo!()
    }

    async fn sum_today_expenses_grouped_by_category(&self) -> Result<CategoryExpenseSummaryDto, String> {
        todo!()
    }

    async fn find_all_transaction_by_month(&self, params: MonthlyTransactionQuery) -> Result<PagedResponse<TransactionDto>, String> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use super::*;
    use crate::dto::user::Session;
    
    #[tokio::test]
    async fn test_login_telegram() {
        dotenv().ok();
        let session = Session {
            user_id: "123456789".to_string(),
            username: "test".to_string(),
            chat_id: "Faick".to_string(),
            jwt: None,
        };
        let api = AccountingClientApiRequest::from_env(session);
        let res = api.login_telegram(&"123".to_string(), &"test".to_string()).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_create_category() {
        dotenv().ok();
        let data = CreateCategoryDto {
            code: "Test".to_string(),
            name: "Test".to_string(),
            description: Option::<String>::None,
            category_type: "INCOME".to_string(),
        };
        let session = Session {
            user_id: "123456789".to_string(),
            username: "test".to_string(),
            chat_id: "Faick".to_string(),
            jwt: None,
        };
        let api = AccountingClientApiRequest::from_env(session);
        let res = api.create_category(data).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.code, "Test");
        assert_eq!(res.name, "Test");
        assert_eq!(res.category_type, "INCOME");
    }
}
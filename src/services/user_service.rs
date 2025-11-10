use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use crate::dto::user::Session;

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn get_or_create(&self, user_id: String, username: String, chat_id: String) -> Session;
    async fn get(&self, user_id: String) -> Option<Session>;
    async fn set(&self, user_id: String, jwt: Session);
    async fn remove(&self, user_id: &str);
}


#[derive(Clone, Default)]
pub struct SessionStoreInMemory {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionStoreInMemory {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl SessionStore for SessionStoreInMemory {
    async fn get_or_create(&self, user_id: String, username: String, chat_id: String) -> Session {
        if let Some(session) = self.get(user_id.clone()).await {
            session
        } else {
            let session = Session::new(user_id.clone(), username, chat_id);
            self.set(user_id, session.clone()).await;
            session
        }
    }

    async fn get(&self, user_id: String) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(user_id.as_str()).cloned()
    }

    async fn set(&self, user_id: String, session: Session) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(user_id, session);
    }

    async fn remove(&self, user_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(user_id);
    }
}

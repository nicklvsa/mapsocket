use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;
use warp::Rejection;
use serde::{Serialize, Deserialize};

pub type ResultT<T> = std::result::Result<T, Rejection>;
pub type ClientsT = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: String,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    pub topics: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub user_id: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub topic: String,
    pub user_id: Option<String>,
    pub message: String,
    pub sender_user_id: String,
}

#[derive(Serialize, Debug)]
pub struct DisconnectEvent {
    pub event_type: String,
    pub user_id: String,
}

#[derive(Serialize, Debug)]
pub struct UpdatedEvent {
    pub message: String,
    pub sender_user_id: String,
    pub topic: String,
}
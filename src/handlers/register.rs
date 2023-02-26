use warp::{Reply, reply::json, http::StatusCode};
use uuid::Uuid;

use crate::types::{RegisterRequest, ClientsT, ResultT, Client, RegisterResponse};

pub async fn register(body: RegisterRequest, clients: ClientsT) -> ResultT<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().simple().to_string();
    register_client(uuid.clone(), user_id, clients).await;
    Ok(json(&RegisterResponse{
        url: format!("ws://127.0.0.1:8080/ws/{}", uuid),
    }))
}

pub async fn unregister(id: String, clients: ClientsT) -> ResultT<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

async fn register_client(id: String, user_id: String, clients: ClientsT) {
    let found_count = clients
        .read()
        .await
        .iter()
        .filter(|(_, c)| match c.user_id == user_id {
            v => v
        })
        .count();

    if found_count > 0 {
        return;
    }

    clients.write().await.insert(
        id,
        Client {
            user_id,
            topics: vec![],
            sender: None
        }
    );
}
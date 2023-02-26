use warp::{Reply,http::StatusCode, ws::Message};
use crate::types::{ClientsT, ResultT, Event, UpdatedEvent};

pub async fn publish(body: Event, clients: ClientsT) -> ResultT<impl Reply> {
    if body.sender_user_id.is_empty() {
        return Ok(StatusCode::FORBIDDEN)
    }

    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match body.user_id.clone() {
            Some(v) => client.user_id == v,
            None => true,
        })
        .filter(|(_, client)| match body.sender_user_id.clone() {
            v => client.user_id != v,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let resp = serde_json::to_string(&UpdatedEvent{
                    sender_user_id: body.sender_user_id.clone(),
                    topic: body.topic.clone(),
                    message: body.message.clone()
                });

                if let Ok(msg) = resp {
                    let _ = sender.send(Ok(Message::text(msg)));
                }
            }
        });

    Ok(StatusCode::OK)
}
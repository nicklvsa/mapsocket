use tokio::sync::mpsc;
use serde_json::from_str;
use futures::{FutureExt, StreamExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Reply, ws::{Ws, Message, WebSocket}};
use crate::types::{ClientsT, ResultT, Client, TopicsRequest, DisconnectEvent};

pub async fn websocket(ws: Ws, id: String, clients: ClientsT) -> ResultT<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

async fn client_connection(ws: WebSocket, id: String, clients: ClientsT, mut client: Client) {
    let cloned_user_id = &client.user_id.clone();
    
    let (client_sender, mut client_recv) = ws.split();
    let (client_send_chan, client_recv_chan) = mpsc::unbounded_channel();

    let client_recv_chan = UnboundedReceiverStream::new(client_recv_chan);
    tokio::task::spawn(client_recv_chan.forward(client_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("Error sending ws message: {}", e);
        }
    }));

    client.sender = Some(client_send_chan);
    clients.write().await.insert(id.clone(), client);

    println!("{} connected.", id);

    while let Some(result) = client_recv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_) => {
                eprintln!("Error receiving websocket message for id: {}", id.clone());
                break;
            }
        };

        client_msg(&id, msg, &clients).await
    }

    clients.write().await.remove(&id);
    clients.read()
        .await
        .iter()
        .filter(|(_, c)| match c.user_id.clone() {
            v => v != cloned_user_id.to_string(),
        })
        .for_each(|(_, c)| {
            if let Some(sender) = &c.sender {
                let resp = serde_json::to_string(&DisconnectEvent{
                    event_type: String::from("disconnect"),
                    user_id: cloned_user_id.to_string(),
                });
    
                if let Ok(msg) = resp {
                    let _ = sender.send(Ok(Message::text(msg)));
                }
            }
        });

    println!("{} disconnected.", id);
}

async fn client_msg(id: &str, msg: Message, clients: &ClientsT) {
    println!("Received message from {} : {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        return;
    }

    let topics_request: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error while parsing websocket message to topic: {}", e);
            return;
        }
    };

    let mut locked = clients.write().await;
    if let Some(v) = locked.get_mut(id) {
        v.topics = topics_request.topics;
    }
}
use anyhow::anyhow;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use log::error;
use serde::{Deserialize, Serialize};
use warp::ws::{Message, WebSocket};

use crate::ws::generate_email::{generate_email, GenerateEmailError};
use crate::ws::get_code::{get_code, GetCodeError};

pub async fn websocket(socket: WebSocket) {
    let (mut socket, mut stream) = socket.split();

    while let Some(message) = stream.next().await {
        if let Err(e) = async {
            let message = message?;
            let message = message.to_str().map_err(|_| anyhow!(""))?;
            let message = serde_json::from_str(message)?;
            handle_msg(message, &mut socket).await?;
            Ok::<_, anyhow::Error>(())
        }
        .await
        {
            error!("websocket error: {}", e);
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Input {
    GenerateEmail { id: String, pw: String },
    GetCode { id: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum Output {
    GenerateEmail(Result<String, GenerateEmailError>),
    GetCode(Result<String, GetCodeError>),
}

impl From<Result<String, GenerateEmailError>> for Output {
    fn from(inner: Result<String, GenerateEmailError>) -> Self {
        Output::GenerateEmail(inner)
    }
}

impl From<Result<String, GetCodeError>> for Output {
    fn from(inner: Result<String, GetCodeError>) -> Self {
        Output::GetCode(inner)
    }
}

async fn handle_msg(msg: Input, socket: &mut SplitSink<WebSocket, Message>) -> anyhow::Result<()> {
    let msg = match msg {
        Input::GenerateEmail { id, pw } => {
            serde_json::to_string(&Output::from(generate_email(id, pw).await)).unwrap()
        }
        Input::GetCode { id } => serde_json::to_string(&Output::from(get_code(id).await)).unwrap(),
    };
    socket.send(Message::text(msg)).await?;
    Ok(())
}

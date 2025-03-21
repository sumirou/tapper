use diesel::Identifiable;
use futures_util::StreamExt;
use serde_json::{Value, from_str};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use crate::database;

enum State {
    Play,
    ResultScreen,
    Other,
}

impl State {
    fn from_str(s: &str) -> Self {
        match s {
            "play" => State::Play,
            "resultScreen" => State::ResultScreen,
            _ => State::Other,
        }
    }
}

fn is_finished(json: &Value) -> bool {
    // when failed: "data["play"]["healthBar"]["normal"] == 0
    // when success: {"name":"resultScreen","number":7}
    let state = State::from_str(
        json.get("state")
            .unwrap()
            .get("name")
            .unwrap()
            .to_string()
            .as_str(),
    );
    match state {
        State::Play => {
            let health = json
                .get("data")
                .unwrap()
                .get("play")
                .unwrap()
                .get("healthBar")
                .unwrap()
                .get("normal")
                .unwrap()
                .as_f64()
                .unwrap();
            health == 0f64
        }
        State::ResultScreen => true,
        State::Other => false,
    }
}

pub async fn get_from_tosu() {
    let url = Url::parse("ws://127.0.0.1:24050/websocket/v2").unwrap();
    tauri::async_runtime::spawn(async move {
        let (ws_stream, _) = connect_async(url.to_string()).await.unwrap();
        let (_, mut read) = ws_stream.split();
        while let Some(message) = read.next().await {
            match message.unwrap() {
                Message::Text(utf8_bytes) => {
                    match from_str::<Value>(&utf8_bytes) {
                        Ok(json) => {
                            if is_finished(&json) {
                                // TODO: 重複して同じスコアを追加してしまう問題の修正
                                let accuracy = json
                                    .get("data")
                                    .unwrap()
                                    .get("resultScreen")
                                    .unwrap()
                                    .get("accuracy")
                                    .unwrap()
                                    .as_f64()
                                    .unwrap();
                                let unstable_rate = json
                                    .get("play")
                                    .unwrap()
                                    .get("unstableRate")
                                    .unwrap()
                                    .as_f64()
                                    .unwrap();
                                let title = json.get("beatmap").unwrap().get("title").unwrap();
                                let version = json.get("beatmap").unwrap().get("version").unwrap();
                                let map_set =
                                    database::find_map(title.to_string(), version.to_string())
                                        .expect("failed to find map set");
                                database::create_score_set(accuracy, unstable_rate, *map_set.id())
                                    .expect("failed to create score");
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
                Message::Binary(bytes) => todo!(),
                Message::Ping(bytes) => todo!(),
                Message::Pong(bytes) => todo!(),
                Message::Close(close_frame) => todo!(),
                Message::Frame(frame) => todo!(),
            };
        }
    });
}

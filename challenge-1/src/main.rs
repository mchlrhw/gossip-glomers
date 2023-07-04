use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    io,
    sync::atomic::{AtomicU32, Ordering},
};
use tracing::{error, warn};
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Body {
    msg_id: Option<u32>,
    in_reply_to: Option<u32>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Error {
        code: ErrorCode,
        text: Option<String>,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
enum ErrorCode {
    Timeout = 0,
    NodeNotFound = 1,
    NotSupported = 10,
    TemporarilyUnavailable = 11,
    MalformedRequest = 12,
    Crash = 13,
    Abort = 14,
    KeyDoesNotExist = 20,
    KeyAlreadyExists = 21,
    PreconditionFailed = 22,
    TxnConflict = 30,
}

static MSG_ID: AtomicU32 = AtomicU32::new(0);

fn msg_id() -> u32 {
    MSG_ID.fetch_add(1, Ordering::Relaxed)
}

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let stdin = io::stdin().lock();
    let messages = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut our_node_id = String::new();
    for serde_res in messages {
        match serde_res {
            Ok(message) => match &message.body.payload {
                Payload::Error { code, text } => {
                    warn!(?code, text, "received error");
                }
                Payload::Init { node_id, .. } => {
                    our_node_id = node_id.to_owned();

                    let resp = Message {
                        src: our_node_id.clone(),
                        dest: message.src.clone(),
                        body: Body {
                            msg_id: Some(msg_id()),
                            in_reply_to: message.body.msg_id,
                            payload: Payload::InitOk,
                        },
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();

                    println!("{serialized}");
                }
                Payload::Echo { echo } => {
                    let resp = Message {
                        src: our_node_id.clone(),
                        dest: message.src.clone(),
                        body: Body {
                            msg_id: Some(msg_id()),
                            in_reply_to: message.body.msg_id,
                            payload: Payload::EchoOk {
                                echo: echo.to_owned(),
                            },
                        },
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();

                    println!("{serialized}");
                }
                _ => warn!(?message, "unhandled message"),
            },
            Err(error) => error!(%error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_serde() -> anyhow::Result<()> {
        let msg = r#"{
            "src": "c1",
            "dest": "n3",
            "body": {
                "type": "init",
                "msg_id": 1,
                "node_id": "n3",
                "node_ids": ["n1", "n2", "n3"]
            }
        }"#;
        let expected = Message {
            src: "c1".to_string(),
            dest: "n3".to_string(),
            body: Body {
                msg_id: Some(1),
                in_reply_to: None,
                payload: Payload::Init {
                    node_id: "n3".to_string(),
                    node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()],
                },
            },
        };

        let deserialized: Message = serde_json::from_str(msg)?;

        assert_eq!(deserialized, expected);

        Ok(())
    }

    #[test]
    fn error_serde() -> anyhow::Result<()> {
        let msg = r#"{
            "src": "c1",
            "dest": "n3",
            "body": {
                "type": "error",
                "in_reply_to": 5,
                "code": 11,
                "text": "Node n5 is waiting for quorum and cannot service requests yet"
            }
        }"#;
        let expected = Message {
            src: "c1".to_string(),
            dest: "n3".to_string(),
            body: Body {
                msg_id: None,
                in_reply_to: Some(5),
                payload: Payload::Error {
                    code: ErrorCode::TemporarilyUnavailable,
                    text: Some(
                        "Node n5 is waiting for quorum and cannot service requests yet".to_string(),
                    ),
                },
            },
        };

        let deserialized: Message = serde_json::from_str(msg)?;

        assert_eq!(deserialized, expected);

        Ok(())
    }
}

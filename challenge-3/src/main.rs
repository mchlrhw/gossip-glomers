use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Deserialize)]
struct Broadcast {
    message: u32,
}

#[derive(Deserialize)]
struct Read;

#[derive(Deserialize)]
struct Topology {
    // topology: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Request {
    Broadcast(Broadcast),
    Read(Read),
    Topology(Topology),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct ReadOk {
    messages: Vec<u32>,
}

#[derive(Clone, Default)]
struct Handler {
    seen: Arc<RwLock<Vec<u32>>>,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if let Ok(request) = req.body.as_obj::<Request>() {
            match request {
                Request::Broadcast(Broadcast { message }) => {
                    self.seen.write().unwrap().push(message);

                    return runtime.reply_ok(req).await;
                }

                Request::Read(_) => {
                    let resp = ReadOk {
                        messages: self.seen.read().unwrap().clone(),
                    };

                    return runtime.reply(req, resp).await;
                }

                Request::Topology(_) => {
                    return runtime.reply_ok(req).await;
                }
            }
        }

        done(runtime, req)
    }
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());

    Runtime::new().with_handler(handler).run().await
}

fn main() -> Result<()> {
    Runtime::init(try_main())
}

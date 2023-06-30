use async_trait::async_trait;
use maelstrom::{done, protocol::Message, Node, Result, Runtime};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone, Default)]
struct Handler {}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "generate" {
            let mut uniq = req.body.clone().with_type("generate_ok");
            uniq.extra
                .insert("id".to_string(), json!(uuid::Uuid::new_v4().to_string()));

            return runtime.reply(req, uniq).await;
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

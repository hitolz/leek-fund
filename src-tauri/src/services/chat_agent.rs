use crate::errors::AppResult;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use std::time::Duration;

pub async fn stream_reply(content: String) -> AppResult<impl Stream<Item = String>> {
    let (tx, rx) = mpsc::channel::<String>(16);

    tokio::spawn(async move {
        let reply = format!("已收到: {}", content.trim());
        for chunk in reply.chars().collect::<Vec<_>>().chunks(6) {
            if tx.send(chunk.iter().collect::<String>()).await.is_err() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(120)).await;
        }
    });

    Ok(ReceiverStream::new(rx))
}

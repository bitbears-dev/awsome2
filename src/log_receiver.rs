use iced::{
    futures::{channel::mpsc, SinkExt, Stream},
    stream,
};
use tracing::info;

use crate::message::Message;

pub fn start() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        // Create channel
        let (tx, mut rx) = mpsc::channel(100);

        // Send the sender back to application
        info!("sending the sender back to app: {:?}", tx);
        let _ = output.send(Message::LogReceiverReady(tx)).await;

        loop {
            use iced_futures::futures::StreamExt;

            info!("selecting next log");
            let log = rx.select_next_some().await;
            let _ = output.send(Message::LogReceived(log)).await;
        }
    })
}

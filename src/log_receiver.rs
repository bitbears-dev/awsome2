use iced::{
    futures::{channel::mpsc, SinkExt, Stream},
    stream,
};

use crate::message::Message;

pub fn start() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        // Create channel
        let (tx, mut rx) = mpsc::channel(100);

        // Send the sender back to application
        let _ = output.send(Message::LogReceiverReady(tx)).await;

        loop {
            use iced_futures::futures::StreamExt;

            let log = rx.select_next_some().await;
            let _ = output.send(Message::LogReceived(log)).await;
        }
    })
}

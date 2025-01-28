use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::channel::mpsc as fmpsc;
use futures::Stream;
use futures::StreamExt;
use smart_string::DisplayExt;
use smart_string::SmartString;
use url::Url;

use crate::spot::client::WebSocketClient;
use crate::spot::client::WebSocketConnectError;
use crate::spot::types::ws_stream_name::StreamName;

pub struct RawWebSocket<T> {
    stream_name: StreamName,
    client: WebSocketClient,
    stream: fmpsc::Receiver<Vec<u8>>,
    _phantom: PhantomData<T>,
}

#[derive(Debug, serde::Serialize)]
struct Query {
    streams: SmartString<62>,
}

impl<T> RawWebSocket<T>
where
    T: serde::de::DeserializeOwned,
{
    pub async fn connect(
        mut stream_base: Url,
        stream_name: StreamName,
    ) -> Result<Self, WebSocketConnectError> {
        let name: SmartString<62> = stream_name.to_fmt();
        let url = stream_base.join(&name)?;
        let (client, stream) = WebSocketClient::connect(url).await?;

        Ok(RawWebSocket {
            stream_name,
            client,
            stream,
            _phantom: PhantomData,
        })
    }
}

impl<T> Stream for RawWebSocket<T>
where
    T: serde::de::DeserializeOwned + Unpin,
{
    type Item = Result<T, serde_json::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let stream = &mut this.stream;
        stream
            .poll_next_unpin(cx)
            .map(|item| item.map(|item| serde_json::from_slice(&item)))
    }
}

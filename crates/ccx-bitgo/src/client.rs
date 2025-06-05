use std::sync::Arc;

use reqwest::{IntoUrl, RequestBuilder};

use crate::config::ConnectionConfig;

mod conversion;
pub mod credential;
pub mod meta;
pub mod public;
pub mod ready;
pub mod signer;
pub mod stamped;

#[derive(Clone)]
pub struct BitGoClient {
    inner: Arc<ClientInner>,
}

pub(crate) struct ClientInner {
    client: reqwest::Client,
    config: ConnectionConfig,
}

impl BitGoClient {
    pub fn new(client: reqwest::Client, config: ConnectionConfig) -> Self {
        let inner = ClientInner { client, config };
        let inner = Arc::new(inner);
        BitGoClient { inner }
    }

    pub fn config(&self) -> &ConnectionConfig {
        &self.inner.config
    }

    #[tracing::instrument(skip_all, fields(method = %method))]
    pub(crate) fn request(&self, method: http::Method, url: impl IntoUrl) -> RequestBuilder {
        self.inner.client.request(method, url)
    }
}

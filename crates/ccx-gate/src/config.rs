use url_macro::url;

use url::Url;

pub struct ConnectionConfig {
    pub(crate) api_base: Url,
    pub(crate) combined_stream_base: Url,
}

impl ConnectionConfig {
    pub fn new(api_base: Url, combined_stream_base: Url) -> Self {
        ConnectionConfig {
            api_base,
            combined_stream_base,
        }
    }
}

pub fn production() -> ConnectionConfig {
    ConnectionConfig::new(url!("https://api.gateio.ws"), url!("wss://api.gateio.ws"))
}

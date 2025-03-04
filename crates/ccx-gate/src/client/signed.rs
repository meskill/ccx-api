use std::marker::PhantomData;

use bon::Builder;

use crate::proto::GateSpotReadyToSend;
use crate::proto::GateSpotSigned;


#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Builder)]
pub struct GateSignedRequest<T> {
    #[builder(required)]
    query: Option<String>,
    #[builder(required)]
    body: Option<String>,
    sign: String,
    api_key: String,
    #[builder(skip)]
    request_spec: PhantomData<T>
}

impl<T> GateSpotReadyToSend<T> for GateSignedRequest<T>
where
    T: GateSpotSigned,
{
    async fn send(
        self,
        client: &GateSpotClient,
    ) -> Result<GateResponseMeta<T::Response>, GateSpotSendError> {
        let inner = &client.inner;

        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        url.set_query(Some(&self.query));

        let request = inner
            .client
            .request(T::HTTP_METHOD, url)
            .header("X-MBX-APIKEY", self.api_key.as_str());

        handle_response(request.send().await?).await
    }
}

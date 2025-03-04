use std::borrow::Cow;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client::GateClient;
use crate::client::meta::GateError;
use crate::client::ready::ReadyRequest;
use crate::client::signer::GateSigner;
use crate::client::stamped::Stamped;
use crate::error::GateResult;
use crate::types::timestamp::Timestamp;
// TODO: add rate limiter support
// use crate::rate_limiter::RateLimiter;
// use crate::rate_limiter::RateLimiterError;
// use crate::types::rate_limits::RateLimitType;

pub trait Request: Serialize + Send + Sync {
    type Response: Response;

    const HTTP_METHOD: http::Method;
    /// IMPORTANT: endpoint should define the whole path i.e. starting from /api/v4
    const ENDPOINT: &'static str;

    fn path(&self) -> Cow<'_, str> {
        Self::ENDPOINT.into()
    }

    // /// Rate limiter bucket type and score for this request.
    // const COSTS: &'static [(RateLimitType, u32)];

    // fn costs(&self) -> &'static [(RateLimitType, u32)] {
    //     Self::COSTS
    // }

    // fn throttle(
    //     self,
    //     rate_limiter: &RateLimiter,
    // ) -> impl Future<Output = Result<Self, RateLimiterError>> + Send
    // where
    //     Self: Sized + Send,
    // {
    //     let mut rate_limiter = rate_limiter.clone();
    //     async move {
    //         rate_limiter.enqueue(0, Self::COSTS).await?;
    //         Ok(self)
    //     }
    // }
}

pub trait Response: DeserializeOwned + Send + Sync {}

impl<T> Response for Vec<T> where T: Response {}
impl<T, const N: usize> Response for smallvec::SmallVec<[T; N]> where T: Response {}

impl<T> Request for &T
where
    T: Request,
{
    type Response = T::Response;

    const HTTP_METHOD: http::Method = T::HTTP_METHOD;
    const ENDPOINT: &'static str = T::ENDPOINT;
    // const COSTS: &'static [(RateLimitType, u32)] = T::COSTS;
}

pub trait PublicRequest: Request {}

pub trait SignedRequest: Request + Send {
    fn stamp(self, timestamp: Timestamp) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, timestamp)
    }

    fn now(self) -> Stamped<Self>
    where
        Self: Sized,
    {
        Stamped::new(self, Timestamp::now())
    }

    fn sign_now(
        self,
        signer: impl GateSigner,
    ) -> impl Future<Output = Result<ReadyRequest<Self>, GateError>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await }
    }

    fn sign_now_and_send(
        self,
        signer: impl GateSigner,
        client: &GateClient,
    ) -> impl Future<Output = GateResult<Self::Response>> + Send
    where
        Self: Send + Sync + Sized,
    {
        async move { self.now().sign(signer).await?.send(client).await }
    }
}

pub trait RequestReadyToSend<T>
where
    T: Request,
{
    fn send(self, client: &GateClient) -> impl Future<Output = GateResult<T::Response>> + Send
    where
        Self: Sized + Send + Sync;
}

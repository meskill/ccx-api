use http::StatusCode;
use uuid::Uuid;

use crate::api::error::GateApiError;

pub type GateError = ccx_lib::Error<GateApiError>;

// use crate::spot::types::rate_limits::RateLimitInterval;
// use crate::spot::types::rate_limits::RateLimitType;

#[derive(Debug)]
pub struct GateResponseWithMeta<T> {
    pub meta: GateResponseMeta,
    pub payload: T,
}

#[derive(Debug, derive_more::Error, derive_more::Display)]
#[display("{error}")]
pub struct GateErrorWithMeta {
    pub meta: Option<GateResponseMeta>,
    pub error: GateError,
}

#[derive(Debug)]
pub struct GateResponseMeta {
    pub http_status: StatusCode,
    // pub id: Option<Uuid>,
    // pub usage: Vec<(RateLimitType, RateLimitInterval, u32, u64)>,
}

impl<T> GateResponseWithMeta<T> {
    pub fn new(payload: T, meta: GateResponseMeta) -> Self {
        GateResponseWithMeta { meta, payload }
    }

    pub fn into_parts(self) -> (GateResponseMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> GateResponseMeta {
        self.meta
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl GateResponseMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();

        // TODO: fix meta
        // let id = resp
        //     .headers()
        //     .get("x-mbx-uuid")
        //     .and_then(|v| Uuid::parse_str(v.to_str().ok()?).ok());

        // println!("Response headers:");
        // let mut usage = Vec::new();
        // for (k, v) in resp.headers() {
        //     let k = k.as_str();
        //     // println!("  {}: {:?}", k, v);
        //     if let Some(value) = v.to_str().ok() {
        //         for (typ, prefix) in [
        //             (RateLimitType::RequestWeight, "x-mbx-used-weight-"),
        //             (RateLimitType::Orders, "x-mbx-order-count-"),
        //         ] {
        //             if let Some((interval, quantity, used)) = parse_usage(prefix, k, value) {
        //                 println!("    ::  {typ:?} {quantity} {interval:?} {used}");
        //                 usage.push((typ, interval, quantity, used));
        //             }
        //         }
        //     }
        // }
        GateResponseMeta {
            http_status,
            // id,
            // usage,
        }
    }

    pub fn error(self, error: impl Into<GateError>) -> GateErrorWithMeta {
        GateErrorWithMeta {
            error: error.into(),
            meta: Some(self),
        }
    }

    pub fn response<T>(self, payload: T) -> GateResponseWithMeta<T> {
        GateResponseWithMeta {
            payload,
            meta: self,
        }
    }
}

impl<T> From<T> for GateErrorWithMeta
where
    T: Into<GateError>,
{
    fn from(error: T) -> Self {
        Self {
            error: error.into(),
            meta: None,
        }
    }
}

// fn parse_usage(prefix: &str, name: &str, value: &str) -> Option<(RateLimitInterval, u32, u64)> {
//     if !name.starts_with(prefix) {
//         None?
//     }
//     // Safety: prefix is a valid UTF-8 string and name starts with prefix, so prefix.len() is at a
//     //  valid UTF-8 boundary.
//     let suffix = name.split_at(prefix.len()).1;
//     if suffix.len() < 2 {
//         None?
//     }
//     let interval = RateLimitInterval::from_letter(*suffix.as_bytes().last()?)?;
//     let interval_quantity = suffix[..suffix.len() - 1].parse().ok()?;
//     let used = value.parse().ok()?;
//     Some((interval, interval_quantity, used))
// }

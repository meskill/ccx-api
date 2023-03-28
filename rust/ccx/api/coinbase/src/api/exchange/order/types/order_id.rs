use core::fmt;

use uuid::Uuid;

pub enum EitherOrderId {
    Coinbase(Uuid),
    Client(Uuid),
}

impl fmt::Display for EitherOrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EitherOrderId::Coinbase(order_id) => write!(f, "{order_id}"),
            EitherOrderId::Client(order_id) => write!(f, "client:{order_id}"),
        }
    }
}

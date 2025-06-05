#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RateLimitType {
    // TODO: drop?
    Public,
    Authenticated,
}

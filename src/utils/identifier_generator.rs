use crate::utils::UserId;

pub trait IdentifierGenerator: Clone + Send + 'static + Sized + Sync {
    fn new() -> Self;
    fn new_id(&mut self) -> UserId;
}

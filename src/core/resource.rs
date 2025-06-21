use crate::core::{Calendar, Collection, Provider};

#[derive(Debug)]
pub enum Resource {
    Collection(Collection),
    Calendar(Calendar),
    Provider(Provider),
}

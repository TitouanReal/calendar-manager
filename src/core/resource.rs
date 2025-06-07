use crate::core::Calendar;
use crate::core::Provider;

#[derive(Debug)]
pub enum Resource {
    Calendar(Calendar),
    Provider(Provider),
}

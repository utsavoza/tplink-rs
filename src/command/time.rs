use crate::error::Result;

pub trait Time {
    fn time(&self) -> Result<String>;
    fn timezone(&self) -> Result<String>;
}

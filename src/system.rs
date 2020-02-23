use crate::error::Result;

pub trait System {
    type Info;

    fn sys_info(&self) -> Result<Self::Info>;
}

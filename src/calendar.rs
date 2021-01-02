use std::fmt::Debug;

use anyhow::Result;

pub trait Calendar: Debug {
    fn print_row(&self, is_first: bool) -> Result<()>;
}

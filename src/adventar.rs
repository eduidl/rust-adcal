use anyhow::Result;

use crate::calendar::Calendar;

#[derive(Debug, Clone)]
pub struct Adventar {
    year: u32,
    url: String,
}

impl Adventar {
    const PREFIX: &'static str = "https://adventar.org/calendars";

    pub fn new(year: u32, id: u32) -> Box<Self> {
        Box::new(Self {
            year,
            url: format!("{}/{}", Self::PREFIX, id),
        })
    }
}

impl Calendar for Adventar {
    fn print_row(&self, is_first: bool) -> Result<()> {
        println!(
            "|{}|{}|-|-",
            if is_first {
                self.year.to_string()
            } else {
                ":".to_string()
            },
            self.url,
        );

        Ok(())
    }
}

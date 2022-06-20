use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};

use crate::calendar::Calendar;

#[derive(Clone, Debug)]
pub struct Qiita {
    year: u32,
    url: String,
}

impl Qiita {
    const PREFIX: &'static str = "https://qiita.com/advent-calendar";

    pub fn new(year: u32, name: &str) -> Box<Self> {
        Box::new(Self {
            year,
            url: format!("{}/{}/{}", Self::PREFIX, year, name),
        })
    }

    fn get_post_num(&self, doc: &Html) -> usize {
        let selector = Selector::parse(
            "table.table > tbody > tr > td > div.adventCalendarCalendar_comment > a",
        )
        .unwrap();

        doc.select(&selector).count()
    }

    fn get_subscriber_num(&self, doc: &Html) -> Result<usize> {
        let selector = Selector::parse("div.adventCalendarJumbotron_stats").unwrap();

        for elem in doc.select(&selector).skip(2).take(1) {
            let re = Regex::new(r"(\d+)\z").unwrap();
            if let Some(m) = re.captures(&elem.inner_html()) {
                return Ok(m[0].parse::<usize>()?);
            }
        }
        unreachable!("{:?}", doc.select(&selector).skip(2).take(1));
    }
}

impl Calendar for Qiita {
    fn print_row(&self, is_first: bool) -> Result<()> {
        sleep(Duration::from_secs(1));
        let body = reqwest::blocking::get(&self.url)?.text()?;
        let doc = Html::parse_document(&body);

        println!(
            "|{}|{}|{}|{}",
            if is_first {
                self.year.to_string()
            } else {
                ":".to_string()
            },
            self.url,
            self.get_post_num(&doc),
            self.get_subscriber_num(&doc)?
        );

        Ok(())
    }
}

use std::collections::BTreeMap;

use anyhow::Result;

mod adventar;
mod calendar;
mod qiita;

use adventar::Adventar;
use calendar::Calendar;
use qiita::Qiita;

fn get_source() -> BTreeMap<u32, Vec<Box<dyn Calendar>>> {
    let mut map: BTreeMap<u32, Vec<Box<dyn Calendar>>> = BTreeMap::new();
    map.insert(2013, vec![Qiita::new(2013, "rust")]);
    map.insert(2014, vec![Adventar::new(2014, 462)]);
    map.insert(2015, vec![Qiita::new(2015, "rust-lang")]);
    map.insert(
        2016,
        vec![
            Qiita::new(2016, "rust-lang"),
            Qiita::new(2016, "rust-lang-2"),
        ],
    );
    map.insert(
        2017,
        vec![
            Qiita::new(2017, "rust-lang"),
            Qiita::new(2017, "rust-lang-2"),
        ],
    );
    map.insert(
        2018,
        vec![Qiita::new(2018, "rust"), Qiita::new(2018, "rust2")],
    );
    map.insert(
        2019,
        vec![
            Qiita::new(2019, "rust"),
            Qiita::new(2019, "rust2"),
            Qiita::new(2019, "rust3"),
        ],
    );
    map.insert(
        2020,
        vec![
            Qiita::new(2020, "rust"),
            Qiita::new(2020, "rust2"),
            Qiita::new(2020, "rust3"),
        ],
    );
    map
}

fn main() -> Result<()> {
    let map = get_source();

    println!("|年|URL|記事数|購読者数（Qiita）|");
    println!("|:-:|--|-----:|----------------:|");
    for (_, calendars) in map {
        for (i, calendar) in calendars.iter().enumerate() {
            calendar.print_row(i == 0)?;
        }
    }

    Ok(())
}

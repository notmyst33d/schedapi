use std::ops::Range;

use serde::de;
use serde::Deserialize;
use serde::Deserializer;

use crate::data::schedule::Range as ScheduleRange;
use crate::data::Schedule;

#[derive(Deserialize, Debug)]
pub struct PortableScheduleEntry {
    pub day: u8,
    pub num: u8,
    #[serde(deserialize_with = "from_week_range")]
    pub week_range: Range<u8>,
    pub name: String,
    pub lesson_type: Option<String>,
    pub teacher: Option<String>,
    pub auditorium: String,
    pub even: Option<bool>,
    pub odd: Option<bool>,
    pub empty: Option<bool>,
}

impl Into<Schedule> for PortableScheduleEntry {
    fn into(self) -> Schedule {
        Schedule {
            day: self.day as i32,
            num: self.num as i32,
            week_range: ScheduleRange {
                start: self.week_range.start as i32,
                end: self.week_range.end as i32,
            },
            name: self.name,
            lesson_type: self.lesson_type,
            teacher: self.teacher,
            auditorium: self.auditorium,
            even: self.even,
            odd: self.odd,
        }
    }
}

fn from_week_range<'a, D>(deserializer: D) -> Result<Range<u8>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    if data == "all" {
        return Ok(Range {
            start: 0,
            end: u8::MAX,
        });
    }

    let split: Vec<&str> = data.split("-").collect();
    let start: u8 = split
        .get(0)
        .ok_or(de::Error::custom("No week range start"))
        .map(|v| v.parse::<u8>())?
        .map_err(de::Error::custom)?;

    let mut end = start;
    if !split.get(1).is_none() {
        end = split
            .get(1)
            .ok_or(de::Error::custom("No week range start"))
            .map(|v| v.parse::<u8>())?
            .map_err(de::Error::custom)?;
    }

    Ok(Range { start, end })
}

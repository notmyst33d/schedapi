use serde::de;
use serde::Deserialize;
use serde::Deserializer;

use crate::data::{Schedule, Range, EvenOdd};

#[derive(Deserialize, Debug)]
pub struct PortableScheduleEntry {
    pub day: u8,
    pub num: u8,
    #[serde(deserialize_with = "from_week_ranges")]
    pub week_ranges: Vec<Range>,
    pub name: String,
    pub lesson_type: Option<String>,
    pub teacher: Option<String>,
    pub auditorium: String,
    #[serde(deserialize_with = "crate::serialization::deserialize_even_odd")]
    pub even_odd: EvenOdd,
}

impl Into<Schedule> for PortableScheduleEntry {
    fn into(self) -> Schedule {
        Schedule {
            day: self.day as i32,
            num: self.num as i32,
            week_ranges: self.week_ranges,
            name: self.name,
            lesson_type: self.lesson_type,
            teacher: self.teacher,
            auditorium: self.auditorium,
            even_odd: self.even_odd,
        }
    }
}

fn from_week_ranges<'a, D>(deserializer: D) -> Result<Vec<Range>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    let weeks: Vec<&str> = data.split(",").collect();
    let mut ranges: Vec<Range> = vec![];

    for week in weeks {
        if week == "all" {
            return Ok(vec![Range {
                start: 0,
                end: i32::MAX,
            }]);
        }

        let split: Vec<&str> = week.split("-").collect();
        let start: i32 = split
            .get(0)
            .ok_or(de::Error::custom("No week range start"))
            .map(|v| v.parse::<i32>())?
            .map_err(de::Error::custom)?;

        let mut end = start;
        if !split.get(1).is_none() {
            end = split
                .get(1)
                .ok_or(de::Error::custom("No week range start"))
                .map(|v| v.parse::<i32>())?
                .map_err(de::Error::custom)?;
        }

        ranges.push(Range { start, end })
    }

    Ok(ranges)
}

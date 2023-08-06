use chrono::{DateTime, NaiveDate, Utc};

pub mod mountain_project;
pub mod thecrag;

use mountain_project::MountainProjectRouteType;
use mountain_project::MountainProjectTick;
use thecrag::TheCragGearStyle;
pub use thecrag::TheCragTick;

/// A tick
///
/// This struct is non-exhaustive; it will likely gain more fields in future.
/// # Examples
#[non_exhaustive]
#[derive(Debug)]
pub struct OpenTick {
    /// Date the climbing happened
    ///
    /// May be extended in future to account for multi-day ascents and more precise times.
    pub date: Option<NaiveDate>,
    /// Name of the route
    pub route_name: Option<String>,
    /// Location of the route
    pub route_location: Option<String>,
    /// Type of route as most often climbed
    pub route_discipline: Option<Discipline>,
    /// Type of route as climbed in this ascent
    pub ascent_discipline: Option<Discipline>,
    /// Consensus grade of the route
    pub route_grade: Option<String>,
    /// Personal grade, for this ascent
    pub ascent_grade: Option<String>,
    /// Free-form comments
    pub comment: Option<String>,
}

/// Disciplines
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Discipline {
    Aid,
    Bouldering,
    DeepWaterSolo,
    Ice,
    Sport,
    Trad,
    Unknown,
}

impl From<MountainProjectRouteType> for Discipline {
    fn from(value: MountainProjectRouteType) -> Self {
        match value {
            MountainProjectRouteType::Unknown => Discipline::Unknown,
            _ => Discipline::Unknown,
        }
    }
}

impl From<TheCragGearStyle> for Discipline {
    fn from(value: TheCragGearStyle) -> Self {
        match value {
            TheCragGearStyle::Boulder => Discipline::Bouldering,
            _ => Discipline::Unknown,
        }
    }
}

impl TryFrom<MountainProjectTick> for OpenTick {
    type Error = ConversionError;

    fn try_from(value: MountainProjectTick) -> Result<Self, Self::Error> {
        let date = value.date;
        let route_name = Some(value.route);
        let route_location = Some(value.location);
        let route_discipline = Some(Discipline::from(MountainProjectRouteType::from(
            value.route_type,
        )));
        let ascent_discipline = None;
        let route_grade = Some(value.rating);
        let ascent_grade = Some(value.your_rating);
        let comment = Some(value.notes);

        Ok(OpenTick {
            date,
            route_name,
            route_location,
            route_discipline,
            ascent_discipline,
            route_grade,
            ascent_grade,
            comment,
        })
    }
}

impl TryFrom<TheCragTick> for OpenTick {
    type Error = ConversionError;

    fn try_from(value: TheCragTick) -> Result<Self, Self::Error> {
        let date = value.ascent_date.map(|d: DateTime<Utc>| d.date_naive());
        let route_name = Some(value.route_name);
        let route_location = Some(value.crag_path);
        let route_discipline = Some(Discipline::from(value.route_gear_style));
        let ascent_discipline = Some(Discipline::from(value.ascent_gear_style));
        let route_grade = Some(value.route_grade);
        let ascent_grade = Some(value.ascent_grade);
        let comment = Some(value.comment);

        Ok(OpenTick {
            date,
            route_name,
            route_location,
            route_discipline,
            ascent_discipline,
            route_grade,
            ascent_grade,
            comment,
        })
    }
}

/// Errors in conversion of ticks
#[non_exhaustive]
pub enum ConversionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tick() {
        let t = OpenTick {
            date: NaiveDate::from_ymd_opt(2020, 1, 1),
            route_name: Some("A Route Name".to_string()),
            route_location: Some("Crag Name".to_string()),
            route_discipline: Some(Discipline::Aid),
            ascent_discipline: Some(Discipline::Trad),
            route_grade: Some("C3".to_string()),
            ascent_grade: Some("5.11".to_string()),
            comment: Some("What a fun route".to_string()),
        };

        println!("{t:?}")
    }
}

use chrono::NaiveDate;

pub mod mountain_project;
use mountain_project::MountainProjectTick;

/// A tick
///
/// This struct is non-exhaustive; it will likely gain more fields in future.
/// # Examples
#[non_exhaustive]
#[derive(Debug)]
pub struct OpenTick<'a> {
    /// Date the climbing happened
    ///
    /// May be extended in future to account for multi-day ascents and more precise times.
    pub date: Option<NaiveDate>,
    /// Name of the route
    pub route_name: Option<&'a str>,
    /// Location of the route
    pub route_location: Option<&'a str>,
    /// Type of route as most often climbed
    pub route_discipline: Option<&'a str>,
    /// Type of route as climbed in this ascent
    pub ascent_discipline: Option<&'a str>,
    /// Consensus grade of the route
    pub route_grade: Option<&'a str>,
    /// Personal grade, for this ascent
    pub ascent_grade: Option<&'a str>,
    /// Free-form comments
    pub comment: Option<&'a str>,
}

impl<'a> TryFrom<MountainProjectTick<'a>> for OpenTick<'a> {
    type Error = ConversionError;

    fn try_from(value: MountainProjectTick<'a>) -> Result<Self, Self::Error> {
        let date = value.date;
        let route_name = Some(value.route);
        let route_location = Some(value.location);
        let route_discipline = Some(value.route_type);
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

/// Errors in conversion of ticks
#[non_exhaustive]
pub enum ConversionError {
    MountainProjectConversionError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tick() {
        let t = OpenTick {
            date: NaiveDate::from_ymd_opt(2020, 1, 1),
            route_name: Some("A Route Name"),
            route_location: Some("Crag Name"),
            route_discipline: Some("aid"),
            ascent_discipline: Some("trad"),
            route_grade: Some("C3"),
            ascent_grade: Some("5.11"),
            comment: Some("What a fun route"),
        };

        println!("{t:?}")
    }
}

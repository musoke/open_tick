use open_tick::{MountainProjectTick, OpenTick};
use std::error::Error;

#[test]
fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("test-data/open-tick-list/test-data/mountainproject/ticks-2023-06-01.csv")?;

    let mut ticks = vec![];

    for result in rdr.deserialize() {
        let record: MountainProjectTick = result?;
        println!("{:?}", record);

        let tick = OpenTick::try_from(record).expect("good CSV data");
        println!("{:?}", tick);
        ticks.push(tick);
    }

    assert!(ticks.len() > 0);

    Ok(())
}

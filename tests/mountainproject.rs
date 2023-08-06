use open_tick::MountainProjectTick;
use std::error::Error;

#[test]
fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("../open-tick-list/test-data/mountainproject/ticks-2023-06-01.csv")?;

    for result in rdr.deserialize() {
        let record: MountainProjectTick = result?;
        println!("{:?}", record);
    }
    Ok(())
}

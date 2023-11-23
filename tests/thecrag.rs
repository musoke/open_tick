use open_tick::{OpenTick, TheCragTick};
use std::error::Error;

#[test]
fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("test-data/open-tick-list/test-data/thecrag/thecrag-logbook-2023-06-01.csv")?;

    let mut ticks = vec![];

    for result in rdr.deserialize() {
        let record: TheCragTick = result?;
        println!("{:?}", record);

        let tick = OpenTick::try_from(record).expect("good CSV data");
        println!("{:?}", tick);
        ticks.push(tick);
    }

    assert!(ticks.len() > 0);

    Ok(())
}

use open_tick::TheCragTick;
use std::error::Error;

#[test]
fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("../open-tick-list/test-data/thecrag/thecrag-logbook-2023-06-01.csv")?;

    for result in rdr.deserialize() {
        let record: TheCragTick = result?;
        println!("{:?}", record);
    }
    Ok(())
}

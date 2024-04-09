use crate::models::Record;

pub fn parse_csv(data: String) -> Vec<Record> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let records = rdr
        .deserialize::<Record>()
        .collect::<Result<Vec<Record>, csv::Error>>()
        .unwrap_or(vec![]);
    records
}

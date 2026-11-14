use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "TYPE")]
    property_type: String,
    #[serde(rename = "PRICE")]
    price: Option<u64>,
}

pub fn parse_csv(file_path: &str) -> Result<HashMap<String, HashMap<String, u64>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);
    let mut graph: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let mut type_price_ranges: HashMap<String, (u64, u64)> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Property = result?;
        if let Some(price) = record.price {
            let entry = type_price_ranges.entry(record.property_type.clone()).or_insert((u64::MAX, 0));
            entry.0 = entry.0.min(price);
            entry.1 = entry.1.max(price);
        }
    }

    for (type1, (min1, max1)) in &type_price_ranges {
        for (type2, (min2, max2)) in &type_price_ranges {
            if type1 != type2 && max1 >= min2 && max2 >= min1 {
                let overlap = calculate_overlap(min1, max1, min2, max2);
                graph.entry(type1.clone()).or_default().insert(type2.clone(), overlap);
            }
        }
    }

    Ok(graph)
}

fn calculate_overlap(min1: &u64, max1: &u64, min2: &u64, max2: &u64) -> u64 {
    std::cmp::min(*max1, *max2) - std::cmp::max(*min1, *min2)
}

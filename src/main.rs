use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    #[serde(rename = "nome")]
    name: Option<String>,
    #[serde(rename = "preço visto")]
    price_seen: Option<String>,
    #[serde(rename = "Marca")]
    brand: Option<String>,
    #[serde(rename = "Loja")]
    store: Option<String>,
    #[serde(rename = "data")]
    date: Option<String>,
    #[serde(rename = "preço pago")]
    price_paid: Option<String>,
    #[serde(rename = "data de compra")]
    purchase_date: Option<String>,
    #[serde(rename = "Quantidade")]
    quantity: Option<String>,
    #[serde(rename = "valor por unidade")]
    unit_price: Option<String>,
    #[serde(rename = "Categoria")]
    category: Option<String>,
    #[serde(rename = "Ambiente")]
    environment: Option<String>,
    #[serde(rename = "Observação")]
    note: Option<String>,
    #[serde(rename = "Total")]
    total: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.csv")?;
    let mut rdr = Reader::from_reader(file);

    let records: Vec<Record> = rdr.deserialize().collect::<Result<_, _>>()?;

    let json = to_string_pretty(&records)?;

    let mut output_file = File::create("output.json")?;
    output_file.write_all(json.as_bytes())?;

    Ok(())
}

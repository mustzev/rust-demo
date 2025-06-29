use csv::{Reader, ReaderBuilder};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use linfa::DatasetBase;
use ndarray::{Array1, Array2};
use serde::Deserialize;
use std::{error::Error, fs::File};

fn read_csv(path: &str) -> Result<Reader<DecodeReaderBytes<File, Vec<u8>>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    Ok(ReaderBuilder::new()
        .has_headers(true)
        .from_reader(transcoded))
}

#[derive(Debug, Deserialize)]
struct Product {
    #[serde(rename = "Category")]
    category: String,
    name: String,
    mrp: u32,
    #[serde(rename = "discountPercent")]
    discount_percent: u8,
    #[serde(rename = "availableQuantity")]
    available_quantity: u32,
    #[serde(rename = "discountedSellingPrice")]
    discounted_selling_price: u32,
    #[serde(rename = "weightInGms")]
    weight_in_gms: u32,
    #[serde(rename = "outOfStock")]
    out_of_stock: String,
    quantity: u32,
}

fn make_dataset(
    mut reader: Reader<DecodeReaderBytes<File, Vec<u8>>>,
) -> Result<(), Box<dyn Error>> {
    let mut features = Vec::new();
    let mut targets = Vec::new();

    for result in reader.deserialize() {
        let product: Product = result?;
        features.push(vec![product.mrp]);
        targets.push(0.0);
    }

    let nrows = features.len();
    let ncols = features[0].len();
    let feature_array =
        Array2::from_shape_vec((nrows, ncols), features.into_iter().flatten().collect())?;
    let target_array = Array1::from(targets);

    // let dataset = DatasetBase::new(feature_array, target_array);
    Ok(())
}

pub fn run() {
    let reader = read_csv("datasets/zepto_v2.csv").unwrap();

    let dataset = make_dataset(reader).unwrap();
}

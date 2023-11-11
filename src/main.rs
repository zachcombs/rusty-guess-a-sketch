use rand::{seq::IteratorRandom, Rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "categories.txt";

fn find_category() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

#[tokio::main]
async fn main() {

    let ndjson_output: Vec<&str>;

    let result = reqwest::get(format!(
        "https://storage.googleapis.com/quickdraw_dataset/full/simplified/{}.ndjson",
        find_category()
    ))
    .await
    .unwrap()
    .text()
    .await
    .expect("There was an error fetching the data");

    ndjson_output = result.split("\n").collect();

    let mut rng = rand::thread_rng();

    let data: &str = &ndjson_output[rng.gen_range(0..ndjson_output.len())];

    println!("{:?}", data);
}

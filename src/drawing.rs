use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use actix_web::{get, web};
use rand::{seq::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use serde_json;

const FILENAME: &str = "categories.txt";

#[derive(Deserialize, Serialize, Debug)]
pub struct Drawing {
    word: String,
    countrycode: String,
    timestamp: String,
    recognized: bool,
    key_id: String,
    drawing: Vec<Vec<Vec<i32>>>,
}

fn find_category() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

async fn get_drawing_from_data() -> Drawing {
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
    println!("{}", &ndjson_output[0]);

    let mut rng = rand::thread_rng();

    let data: Drawing = serde_json::from_str(&ndjson_output[rng.gen_range(0..ndjson_output.len())])
        .expect("Couldn't deserialize");

    return data;
}

#[get("/drawing")]
pub async fn drawing() -> web::Json<Drawing> {
    println!("==========DRAWING!=========");
    let drawing_data: Drawing = get_drawing_from_data().await;
    println!("{}", drawing_data.word);
    web::Json(Drawing {
        word: drawing_data.word.clone(),
        countrycode: drawing_data.countrycode.clone(),
        timestamp: drawing_data.timestamp.clone(),
        recognized: drawing_data.recognized.clone(),
        key_id: drawing_data.key_id.clone(),
        drawing: drawing_data.drawing.clone(),
    })
}

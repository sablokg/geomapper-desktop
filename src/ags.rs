use crate::mapper::MapperOrt;
use crate::mapper::MapperPrint;
use dotenv::dotenv;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

#[tokio::main]
pub async fn ags_mapper(ags: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let file2 = std::env::var("file2").expect("file not present");
    let file2read = File::open(file2).expect("file not present");
    let file2read = BufReader::new(file2read);

    let mut mapper_ort: Vec<MapperOrt> = Vec::new();

    for i in file2read.lines() {
        let line = i.expect("file not found");
        let ort_mapper: Vec<_> = line.split(",").collect::<Vec<_>>();
        mapper_ort.push(MapperOrt {
            osmid: ort_mapper[0].to_string(),
            ags: ort_mapper[1].to_string(),
            ord: ort_mapper[2..3]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            plz: ort_mapper[3].to_string(),
            landkries: ort_mapper[4..5]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            bundesland: ort_mapper[ort_mapper.len() - 1..ort_mapper.len()]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        });
    }

    let mut searched_plz2: Vec<MapperPrint> = Vec::new();
    for i in mapper_ort.iter() {
        if i.ags == ags.to_string() {
            searched_plz2.push(MapperPrint {
                plz: i.plz.clone(),
                osmid: i.osmid.clone(),
                ags: i.ags.clone(),
                ord: i.ord.clone().join("").to_string(),
                landkries: i.landkries.clone().join("").to_string(),
                bundesland: i.bundesland.clone().join("").to_string(),
            });
        }
    }

    println!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        "plz", "osmid", "ags", "ord", "landkries", "bundesland"
    );
    for i in searched_plz2.iter() {
        println!(
            "{}\t{}\t{}\t{:?}\t{:?}\t{:?}",
            i.plz, i.osmid, i.ags, i.ord, i.landkries, i.bundesland
        );
    }

    Ok("The searched results are as follows".to_string())
}

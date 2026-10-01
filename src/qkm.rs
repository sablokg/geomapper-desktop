use crate::mapper::MapperDE;
use dotenv::dotenv;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn qkm_mapper(qkm: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let file1 = std::env::var("file1").expect("file not present");
    let file1read = File::open(file1).expect("file not present");
    let file1read = BufReader::new(file1read);

    let mut mapper_de: Vec<MapperDE> = Vec::new();
    let mut searched_plz1: Vec<MapperDE> = Vec::new();

    for i in file1read.lines() {
        let line = i.expect("file not found");
        if line.starts_with("plz") {
            continue;
        } else if !line.starts_with("plz") {
            let mapper: Vec<_> = line.split(",").collect::<Vec<_>>();
            if mapper.len() == 6usize {
                mapper_de.push(MapperDE {
                    plz: mapper[0].to_string(),
                    note: mapper[1].to_string(),
                    einwohner: mapper[2].replace("/", "-").to_string(),
                    qkm: mapper[3].to_string(),
                    lat: mapper[4].to_string(),
                    lon: mapper[5].to_string(),
                });
            }
        }
    }

    for i in mapper_de.iter() {
        if i.qkm == qkm.to_string() {
            searched_plz1.push(MapperDE {
                plz: i.plz.clone(),
                note: i.note.clone(),
                einwohner: i.einwohner.clone(),
                qkm: i.qkm.clone(),
                lat: i.lat.clone(),
                lon: i.lon.clone(),
            })
        }
    }
    for i in searched_plz1.iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            i.plz, i.note, i.einwohner, i.qkm, i.lat, i.lon
        );
    }
    Ok("The searched note are as follows:".to_string())
}

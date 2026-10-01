use crate::mapper::MapperDE;
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

pub fn plz_mapper(plz: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let file1 = std::env::var("file1").expect("file not present");
    let file2 = std::env::var("file2").expect("file not present");
    let file1read = File::open(file1).expect("file not present");
    let file2read = File::open(file2).expect("file not present");
    let file1read = BufReader::new(file1read);
    let file2read = BufReader::new(file2read);

    let mut mapper_de: Vec<MapperDE> = Vec::new();
    let mut mapper_ort: Vec<MapperOrt> = Vec::new();

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

    for j in file2read.lines() {
        let line = j.expect("file not found");
        if line.starts_with("osm") {
            continue;
        } else if !line.starts_with("osm") {
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
    }

    let mut searched_plz1: Vec<MapperDE> = Vec::new();
    let mut searched_plz2: Vec<MapperPrint> = Vec::new();
    for i in mapper_de.iter() {
        if i.plz == plz.to_string() {
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
    for j in mapper_ort.iter() {
        if j.plz == plz.to_string() {
            searched_plz2.push(MapperPrint {
                plz: j.plz.clone(),
                osmid: j.osmid.clone(),
                ags: j.ags.clone(),
                ord: j.ord.clone().join("").to_string(),
                landkries: j.landkries.clone().join("").to_string(),
                bundesland: j.bundesland.clone().join("").to_string(),
            });
        }
    }

    println!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        "plz", "note", "einwohner", "qkm", "latitude", "longitude"
    );
    for i in searched_plz1.iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            i.plz, i.note, i.einwohner, i.qkm, i.lat, i.lon
        );
    }

    println!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        "plz", "osmid", "ags", "ord", "landkries", "bundesland"
    );
    for j in searched_plz2.iter() {
        println!(
            "{}\t{}\t{}\t{:?}\t{:?}\t{:?}",
            j.plz, j.osmid, j.ags, j.ord, j.landkries, j.bundesland
        );
    }

    Ok("The searched results are as follows".to_string())
}

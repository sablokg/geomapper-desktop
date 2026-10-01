use dotenv::dotenv;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn generalpattern_mapper(path: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let file1 = std::env::var("file1").expect("file not present");
    let file2 = std::env::var("file2").expect("file not found");
    let file1read = File::open(file1).expect("path not found");
    let file2read = File::open(file2).expect("path not found");
    let bufferfile1 = BufReader::new(file1read);
    let bufferfile2 = BufReader::new(file2read);

    let mut bufferfile1_part: Vec<Vec<String>> = Vec::new();
    let mut bufferfile2_part: Vec<Vec<String>> = Vec::new();

    for i in bufferfile1.lines() {
        let line = i.expect("line not found");
        if line.starts_with("plz") {
            continue;
        } else if !line.starts_with("plz") {
            let mutableline: Vec<_> = line.split(",").map(|x| x.to_string()).collect::<Vec<_>>();
            bufferfile1_part.push(mutableline);
        }
    }

    for i in bufferfile2.lines() {
        let line = i.expect("line not found");
        let mutline = line.split(",").map(|x| x.to_string()).collect::<Vec<_>>();
        bufferfile2_part.push(mutline);
    }

    for i in bufferfile1_part.iter() {
        if i.contains(&path.to_string()) {
            println!("{:?}", i.join(" ").to_string());
        }
    }

    for j in bufferfile2_part.iter() {
        if j.contains(&path.to_string()) {
            println!("{:?}", j.join(" ").to_string());
        }
    }

    Ok("The search results have been written".to_string())
}

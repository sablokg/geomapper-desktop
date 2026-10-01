mod ags;
mod args;
mod bundesland;
mod einwohner;
mod generalpattern;
mod landkries;
mod lat;
mod lon;
mod mapper;
mod note;
mod ord;
mod osm;
mod plz;
mod qkm;
use crate::ags::ags_mapper;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::bundesland::bundesland_mapper;
use crate::einwohner::einwohner_mapper;
use crate::generalpattern::generalpattern_mapper;
use crate::landkries::landkries_mapper;
use crate::lat::latitude_mapper;
use crate::lon::longitude_mapper;
use crate::note::note_mapper;
use crate::ord::ord_mapper;
use crate::osm::osm_mapper;
use crate::plz::plz_mapper;
use crate::qkm::qkm_mapper;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Gaurav Sablok
gsablok@proton.me
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("geoMapper");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Plz { plz, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = plz_mapper(plz).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Note { note, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = note_mapper(note).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Einwohner { einwohner, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = einwohner_mapper(einwohner).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Qkm { qkm, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = qkm_mapper(qkm).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Latitude { lat, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = latitude_mapper(lat).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Longitude { lon, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = longitude_mapper(lon).unwrap();
                println!("The results are: {:?}", command);
            })
        }
        Commands::Osm { osm_id, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = osm_mapper(osm_id).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Ags { ags, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = ags_mapper(ags).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Ord { ord, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = ord_mapper(ord).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Landkreis { landkries, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = landkries_mapper(landkries).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::Bundesland { bundesland, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = bundesland_mapper(bundesland).unwrap();
                println!("The results are: {:?}", command);
            });
        }
        Commands::GeneralPattern {
            generalpattern,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = generalpattern_mapper(generalpattern).unwrap();
                println!(" The results are: {:?}", command);
            });
        }
    }
}

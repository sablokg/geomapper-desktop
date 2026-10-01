use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "geomapper",
    version = "1.0",
    about = "geomapper, a geomapper for geman geographical data
   ************************************************
   Author Gaurav Sablok,
   Email: gsablok@proton.me
   ************************************************ "
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// search according to the plz
    Plz { plz: String, thread: String },
    /// search according to the note
    Note { note: String, thread: String },
    /// search according to the einwohner
    Einwohner { einwohner: String, thread: String },
    /// search according to the qkm
    Qkm { qkm: String, thread: String },
    /// search according to the latitude
    Latitude { lat: String, thread: String },
    /// search according to the longitude
    Longitude { lon: String, thread: String },
    /// search according to the osm
    Osm { osm_id: String, thread: String },
    /// search according to the ags
    Ags { ags: String, thread: String },
    ///search according to the ort
    Ord { ord: String, thread: String },
    /// search according to the landkries
    Landkreis { landkries: String, thread: String },
    /// search according to the bundesland
    Bundesland { bundesland: String, thread: String },
    /// general pattern search
    GeneralPattern {
        generalpattern: String,
        thread: String,
    },
}

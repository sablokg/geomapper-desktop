#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MapperDE {
    pub plz: String,
    pub note: String,
    pub einwohner: String,
    pub qkm: String,
    pub lat: String,
    pub lon: String,
}

pub struct MapperOrt {
    pub osmid: String,
    pub ags: String,
    pub ord: Vec<String>,
    pub plz: String,
    pub landkries: Vec<String>,
    pub bundesland: Vec<String>,
}

pub struct MapperPrint {
    pub osmid: String,
    pub ags: String,
    pub ord: String,
    pub plz: String,
    pub landkries: String,
    pub bundesland: String,
}

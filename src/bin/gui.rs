use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
 */

/// Overall color scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppTheme {
    Dark,
    Light,
}

/// UI text scale, selectable so the app stays readable at any display size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl FontSize {
    const ALL: [FontSize; 4] = [
        FontSize::Small,
        FontSize::Medium,
        FontSize::Large,
        FontSize::ExtraLarge,
    ];

    fn label(&self) -> &'static str {
        match self {
            FontSize::Small => "Small",
            FontSize::Medium => "Medium",
            FontSize::Large => "Large",
            FontSize::ExtraLarge => "Extra large",
        }
    }

    /// Multiplier applied to the base point sizes below.
    fn scale(&self) -> f32 {
        match self {
            FontSize::Small => 0.9,
            FontSize::Medium => 1.0,
            FontSize::Large => 1.25,
            FontSize::ExtraLarge => 1.55,
        }
    }
}

/// Applies the chosen theme (colors, rounding, spacing) and font scale to
/// the whole app. Called once per frame before any panels are drawn.
fn apply_style(theme: AppTheme, font_size: FontSize, ctx: &egui::Context) {
    let accent = egui::Color32::from_rgb(66, 150, 250);

    let mut visuals = match theme {
        AppTheme::Dark => egui::Visuals::dark(),
        AppTheme::Light => egui::Visuals::light(),
    };

    visuals.hyperlink_color = accent;
    visuals.selection.bg_fill = accent.linear_multiply(0.55);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, accent);

    match theme {
        AppTheme::Dark => {
            visuals.panel_fill = egui::Color32::from_rgb(23, 25, 31);
            visuals.window_fill = egui::Color32::from_rgb(29, 31, 38);
            visuals.extreme_bg_color = egui::Color32::from_rgb(17, 18, 23);
            visuals.faint_bg_color = egui::Color32::from_rgb(33, 35, 43);
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(23, 25, 31);
            visuals.widgets.noninteractive.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(48, 51, 61));
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(41, 44, 54);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(56, 60, 74);
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, accent);
            visuals.widgets.active.bg_fill = accent.linear_multiply(0.4);
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, accent);
        }
        AppTheme::Light => {
            visuals.panel_fill = egui::Color32::from_rgb(247, 248, 251);
            visuals.window_fill = egui::Color32::WHITE;
            visuals.extreme_bg_color = egui::Color32::WHITE;
            visuals.faint_bg_color = egui::Color32::from_rgb(237, 239, 245);
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(247, 248, 251);
            visuals.widgets.noninteractive.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(222, 225, 232));
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(230, 233, 239);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(213, 222, 240);
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, accent);
            visuals.widgets.active.bg_fill = accent.linear_multiply(0.75);
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, accent);
        }
    }

    let rounding = egui::Rounding::same(7.0);
    visuals.window_rounding = rounding;
    visuals.menu_rounding = rounding;
    visuals.widgets.noninteractive.rounding = rounding;
    visuals.widgets.inactive.rounding = rounding;
    visuals.widgets.hovered.rounding = rounding;
    visuals.widgets.active.rounding = rounding;
    visuals.widgets.open.rounding = rounding;

    ctx.set_visuals(visuals);

    let scale = font_size.scale();
    ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 10.0);
        style.spacing.button_padding = egui::vec2(12.0, 7.0);
        style.spacing.window_margin = egui::Margin::same(12.0);
        style.spacing.indent = 20.0;
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::proportional(25.0 * scale),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::proportional(15.5 * scale),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::proportional(15.5 * scale),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::monospace(14.5 * scale),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::proportional(12.5 * scale),
            ),
        ]
        .into();
    });
}

/// A row from einwohner.csv: plz,note,einwohner,qkm,lat,lon
#[derive(Debug, Clone, Default)]
struct Record1 {
    plz: String,
    note: String,
    einwohner: String,
    qkm: String,
    lat: String,
    lon: String,
}

/// A row from zuordnung_plz_ort.csv: osm_id,ags,ort,plz,landkreis,bundesland
#[derive(Debug, Clone, Default)]
struct Record2 {
    osmid: String,
    ags: String,
    ort: String,
    plz: String,
    landkreis: String,
    bundesland: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchKind {
    Plz,
    Note,
    Einwohner,
    Qkm,
    Latitude,
    Longitude,
    Osm,
    Ags,
    Ort,
    Landkreis,
    Bundesland,
    GeneralPattern,
}

impl SearchKind {
    const ALL: [SearchKind; 12] = [
        SearchKind::Plz,
        SearchKind::Note,
        SearchKind::Einwohner,
        SearchKind::Qkm,
        SearchKind::Latitude,
        SearchKind::Longitude,
        SearchKind::Osm,
        SearchKind::Ags,
        SearchKind::Ort,
        SearchKind::Landkreis,
        SearchKind::Bundesland,
        SearchKind::GeneralPattern,
    ];

    fn label(&self) -> &'static str {
        match self {
            SearchKind::Plz => "PLZ (postal code)",
            SearchKind::Note => "Note (place name)",
            SearchKind::Einwohner => "Einwohner (population)",
            SearchKind::Qkm => "Qkm (area)",
            SearchKind::Latitude => "Latitude",
            SearchKind::Longitude => "Longitude",
            SearchKind::Osm => "OSM ID",
            SearchKind::Ags => "AGS (official code)",
            SearchKind::Ort => "Ort (town)",
            SearchKind::Landkreis => "Landkreis (district)",
            SearchKind::Bundesland => "Bundesland (state)",
            SearchKind::GeneralPattern => "General pattern (any field)",
        }
    }

    /// Numeric/code searches are matched exactly; free-text searches are
    /// matched case-insensitively as a substring, which is friendlier in a
    /// GUI than the CLI's exact-match behaviour.
    fn is_exact(&self) -> bool {
        matches!(
            self,
            SearchKind::Plz
                | SearchKind::Einwohner
                | SearchKind::Qkm
                | SearchKind::Latitude
                | SearchKind::Longitude
                | SearchKind::Osm
                | SearchKind::Ags
        )
    }
}

enum SearchResults {
    None,
    De(Vec<Record1>),
    Ort(Vec<Record2>),
    Both(Vec<Record1>, Vec<Record2>),
    Pattern(Vec<String>),
}

struct GeoMapperApp {
    file1_path: String,
    file2_path: String,
    status: String,
    error: Option<String>,
    records1: Vec<Record1>,
    records2: Vec<Record2>,
    loaded: bool,
    search_kind: SearchKind,
    query: String,
    results: SearchResults,
    theme: AppTheme,
    font_size: FontSize,
}

impl Default for GeoMapperApp {
    fn default() -> Self {
        Self {
            file1_path: "deutschmapper/einwohner.csv".to_string(),
            file2_path: "deutschmapper/zuordnung_plz_ort.csv".to_string(),
            status: "Not loaded yet. Set the two CSV paths and click \"Load CSVs\".".to_string(),
            error: None,
            records1: Vec::new(),
            records2: Vec::new(),
            loaded: false,
            search_kind: SearchKind::Plz,
            query: String::new(),
            results: SearchResults::None,
            theme: AppTheme::Dark,
            font_size: FontSize::Medium,
        }
    }
}

fn matches_field(kind_is_exact: bool, field: &str, query: &str) -> bool {
    if kind_is_exact {
        field.trim() == query.trim()
    } else {
        field.to_lowercase().contains(&query.to_lowercase())
    }
}

impl GeoMapperApp {
    fn load_csvs(&mut self) {
        self.error = None;
        match load_file1(&self.file1_path) {
            Ok(rows) => self.records1 = rows,
            Err(e) => {
                self.error = Some(format!("Failed to load '{}': {e}", self.file1_path));
                self.loaded = false;
                return;
            }
        }
        match load_file2(&self.file2_path) {
            Ok(rows) => self.records2 = rows,
            Err(e) => {
                self.error = Some(format!("Failed to load '{}': {e}", self.file2_path));
                self.loaded = false;
                return;
            }
        }
        self.loaded = true;
        self.status = format!(
            "Loaded {} postal records and {} location records.",
            self.records1.len(),
            self.records2.len()
        );
        self.results = SearchResults::None;
    }

    fn run_search(&mut self) {
        if !self.loaded {
            self.error = Some("Load the CSV files first.".to_string());
            return;
        }
        self.error = None;
        let q = self.query.clone();
        let exact = self.search_kind.is_exact();

        self.results = match self.search_kind {
            SearchKind::Plz => {
                let de: Vec<Record1> = self
                    .records1
                    .iter()
                    .filter(|r| matches_field(true, &r.plz, &q))
                    .cloned()
                    .collect();
                let ort: Vec<Record2> = self
                    .records2
                    .iter()
                    .filter(|r| matches_field(true, &r.plz, &q))
                    .cloned()
                    .collect();
                SearchResults::Both(de, ort)
            }
            SearchKind::Note => SearchResults::De(
                self.records1
                    .iter()
                    .filter(|r| matches_field(exact, &r.note, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Einwohner => SearchResults::De(
                self.records1
                    .iter()
                    .filter(|r| matches_field(exact, &r.einwohner, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Qkm => SearchResults::De(
                self.records1
                    .iter()
                    .filter(|r| matches_field(exact, &r.qkm, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Latitude => SearchResults::De(
                self.records1
                    .iter()
                    .filter(|r| matches_field(exact, &r.lat, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Longitude => SearchResults::De(
                self.records1
                    .iter()
                    .filter(|r| matches_field(exact, &r.lon, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Osm => SearchResults::Ort(
                self.records2
                    .iter()
                    .filter(|r| matches_field(exact, &r.osmid, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Ags => SearchResults::Ort(
                self.records2
                    .iter()
                    .filter(|r| matches_field(exact, &r.ags, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Ort => SearchResults::Ort(
                self.records2
                    .iter()
                    .filter(|r| matches_field(exact, &r.ort, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Landkreis => SearchResults::Ort(
                self.records2
                    .iter()
                    .filter(|r| matches_field(exact, &r.landkreis, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::Bundesland => SearchResults::Ort(
                self.records2
                    .iter()
                    .filter(|r| matches_field(exact, &r.bundesland, &q))
                    .cloned()
                    .collect(),
            ),
            SearchKind::GeneralPattern => {
                let ql = q.to_lowercase();
                let mut lines = Vec::new();
                for r in &self.records1 {
                    if r.plz.to_lowercase().contains(&ql)
                        || r.note.to_lowercase().contains(&ql)
                        || r.einwohner.to_lowercase().contains(&ql)
                        || r.qkm.to_lowercase().contains(&ql)
                        || r.lat.to_lowercase().contains(&ql)
                        || r.lon.to_lowercase().contains(&ql)
                    {
                        lines.push(format!(
                            "[postal] {} | {} | {} | {} | {} | {}",
                            r.plz, r.note, r.einwohner, r.qkm, r.lat, r.lon
                        ));
                    }
                }
                for r in &self.records2 {
                    if r.osmid.to_lowercase().contains(&ql)
                        || r.ags.to_lowercase().contains(&ql)
                        || r.ort.to_lowercase().contains(&ql)
                        || r.plz.to_lowercase().contains(&ql)
                        || r.landkreis.to_lowercase().contains(&ql)
                        || r.bundesland.to_lowercase().contains(&ql)
                    {
                        lines.push(format!(
                            "[location] {} | {} | {} | {} | {} | {}",
                            r.osmid, r.ags, r.ort, r.plz, r.landkreis, r.bundesland
                        ));
                    }
                }
                SearchResults::Pattern(lines)
            }
        };
    }
}

fn load_file1(path: &str) -> Result<Vec<Record1>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.starts_with("plz") {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() == 6 {
            out.push(Record1 {
                plz: cols[0].to_string(),
                note: cols[1].to_string(),
                einwohner: cols[2].replace('/', "-"),
                qkm: cols[3].to_string(),
                lat: cols[4].to_string(),
                lon: cols[5].to_string(),
            });
        }
    }
    Ok(out)
}

fn load_file2(path: &str) -> Result<Vec<Record2>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.starts_with("osm_id") {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() == 6 {
            out.push(Record2 {
                osmid: cols[0].to_string(),
                ags: cols[1].to_string(),
                ort: cols[2].to_string(),
                plz: cols[3].to_string(),
                landkreis: cols[4].to_string(),
                bundesland: cols[5].to_string(),
            });
        }
    }
    Ok(out)
}

impl eframe::App for GeoMapperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_style(self.theme, self.font_size, ctx);

        egui::TopBottomPanel::top("top_bar")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.heading("🗺 geoMapper");
                    ui.label(
                        egui::RichText::new("German postal & geospatial lookup")
                            .weak()
                            .italics(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_source("font_size_combo")
                            .selected_text(format!("🔤 {}", self.font_size.label()))
                            .show_ui(ui, |ui| {
                                for fs in FontSize::ALL {
                                    ui.selectable_value(&mut self.font_size, fs, fs.label());
                                }
                            });

                        ui.add_space(10.0);

                        if ui
                            .selectable_label(self.theme == AppTheme::Light, "☀ Light")
                            .clicked()
                        {
                            self.theme = AppTheme::Light;
                        }
                        if ui
                            .selectable_label(self.theme == AppTheme::Dark, "🌙 Dark")
                            .clicked()
                        {
                            self.theme = AppTheme::Dark;
                        }
                    });
                });
                ui.add_space(6.0);
            });

        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(320.0)
            .width_range(260.0..=480.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.strong("📁 Data files");
                            ui.add_space(4.0);
                            ui.label("Postal data CSV");
                            ui.label(
                                egui::RichText::new("plz,note,einwohner,qkm,lat,lon")
                                    .small()
                                    .weak(),
                            );
                            ui.text_edit_singleline(&mut self.file1_path);
                            ui.add_space(6.0);
                            ui.label("Location data CSV");
                            ui.label(
                                egui::RichText::new("osm_id,ags,ort,plz,landkreis,bundesland")
                                    .small()
                                    .weak(),
                            );
                            ui.text_edit_singleline(&mut self.file2_path);
                            ui.add_space(8.0);
                            if ui
                                .add_sized(
                                    [ui.available_width(), 0.0],
                                    egui::Button::new("⬆ Load CSVs"),
                                )
                                .clicked()
                            {
                                self.load_csvs();
                            }
                            ui.add_space(4.0);
                            let status_color = if self.loaded {
                                if ctx.style().visuals.dark_mode {
                                    egui::Color32::from_rgb(120, 220, 150)
                                } else {
                                    egui::Color32::from_rgb(30, 140, 60)
                                }
                            } else {
                                ui.style().visuals.weak_text_color()
                            };
                            ui.colored_label(status_color, &self.status);
                        });
                    });

                ui.add_space(10.0);

                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.strong("🔍 Search");
                            ui.add_space(4.0);
                            ui.label("Field");
                            egui::ComboBox::from_id_source("field_combo")
                                .width(ui.available_width())
                                .selected_text(self.search_kind.label())
                                .show_ui(ui, |ui| {
                                    for kind in SearchKind::ALL {
                                        ui.selectable_value(
                                            &mut self.search_kind,
                                            kind,
                                            kind.label(),
                                        );
                                    }
                                });
                            ui.add_space(6.0);
                            ui.label("Query");
                            let response = ui.add_sized(
                                [ui.available_width(), 0.0],
                                egui::TextEdit::singleline(&mut self.query),
                            );
                            let enter_pressed = response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter));
                            ui.add_space(8.0);
                            if ui
                                .add_sized(
                                    [ui.available_width(), 0.0],
                                    egui::Button::new("🔎 Search"),
                                )
                                .clicked()
                                || enter_pressed
                            {
                                self.run_search();
                            }
                        });
                    });

                if let Some(err) = &self.error {
                    ui.add_space(10.0);
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(120, 30, 30).linear_multiply(0.5))
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            egui::Color32::from_rgb(220, 90, 90),
                        ))
                        .rounding(egui::Rounding::same(7.0))
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.colored_label(egui::Color32::from_rgb(255, 180, 180), err);
                        });
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("Results");
            ui.separator();
            ui.add_space(4.0);

            match &self.results {
                SearchResults::None => {
                    ui.label(
                        egui::RichText::new("No search run yet. Load the CSVs and run a search.")
                            .weak(),
                    );
                }
                SearchResults::De(rows) => {
                    ui.label(format!("{} postal record(s) found.", rows.len()));
                    ui.add_space(4.0);
                    render_de_table(ui, rows);
                }
                SearchResults::Ort(rows) => {
                    ui.label(format!("{} location record(s) found.", rows.len()));
                    ui.add_space(4.0);
                    render_ort_table(ui, rows);
                }
                SearchResults::Both(de, ort) => {
                    ui.label(format!(
                        "{} postal record(s), {} location record(s) found.",
                        de.len(),
                        ort.len()
                    ));
                    ui.add_space(6.0);
                    ui.strong("Postal data");
                    ui.add_space(4.0);
                    render_de_table(ui, de);
                    ui.add_space(10.0);
                    ui.strong("Location data");
                    ui.add_space(4.0);
                    render_ort_table(ui, ort);
                }
                SearchResults::Pattern(lines) => {
                    ui.label(format!("{} matching row(s) found.", lines.len()));
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in lines {
                            ui.monospace(line);
                        }
                    });
                }
            }
        });
    }
}

fn render_de_table(ui: &mut egui::Ui, rows: &[Record1]) {
    if rows.is_empty() {
        return;
    }
    let row_h = ui.text_style_height(&egui::TextStyle::Body) + 6.0;
    let header_h = ui.text_style_height(&egui::TextStyle::Body) + 8.0;
    egui::ScrollArea::vertical()
        .id_source("de_scroll")
        .max_height(320.0)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .columns(Column::auto().at_least(60.0), 6)
                .header(header_h, |mut header| {
                    for h in ["PLZ", "Note", "Einwohner", "Qkm", "Lat", "Lon"] {
                        header.col(|ui| {
                            ui.strong(h);
                        });
                    }
                })
                .body(|mut body| {
                    for r in rows {
                        body.row(row_h, |mut row| {
                            row.col(|ui| {
                                ui.label(&r.plz);
                            });
                            row.col(|ui| {
                                ui.label(&r.note);
                            });
                            row.col(|ui| {
                                ui.label(&r.einwohner);
                            });
                            row.col(|ui| {
                                ui.label(&r.qkm);
                            });
                            row.col(|ui| {
                                ui.label(&r.lat);
                            });
                            row.col(|ui| {
                                ui.label(&r.lon);
                            });
                        });
                    }
                });
        });
}

fn render_ort_table(ui: &mut egui::Ui, rows: &[Record2]) {
    if rows.is_empty() {
        return;
    }
    let row_h = ui.text_style_height(&egui::TextStyle::Body) + 6.0;
    let header_h = ui.text_style_height(&egui::TextStyle::Body) + 8.0;
    egui::ScrollArea::vertical()
        .id_source("ort_scroll")
        .max_height(320.0)
        .show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .columns(Column::auto().at_least(60.0), 6)
                .header(header_h, |mut header| {
                    for h in ["OSM ID", "AGS", "Ort", "PLZ", "Landkreis", "Bundesland"] {
                        header.col(|ui| {
                            ui.strong(h);
                        });
                    }
                })
                .body(|mut body| {
                    for r in rows {
                        body.row(row_h, |mut row| {
                            row.col(|ui| {
                                ui.label(&r.osmid);
                            });
                            row.col(|ui| {
                                ui.label(&r.ags);
                            });
                            row.col(|ui| {
                                ui.label(&r.ort);
                            });
                            row.col(|ui| {
                                ui.label(&r.plz);
                            });
                            row.col(|ui| {
                                ui.label(&r.landkreis);
                            });
                            row.col(|ui| {
                                ui.label(&r.bundesland);
                            });
                        });
                    }
                });
        });
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "geoMapper GUI",
        options,
        Box::new(|_cc| Box::new(GeoMapperApp::default())),
    )
}

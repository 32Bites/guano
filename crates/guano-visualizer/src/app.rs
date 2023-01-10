use std::{fmt::Write, ops::Range};

use egui::{
    Color32, FontData, FontDefinitions, FontFamily, Frame, Rounding, ScrollArea, TextEdit, Vec2,
};
use egui_extras::{Size, StripBuilder};
use guano_files::{file::Files, module::Module};
use guano_parser::ast::parse;
use tracing::info;

use crate::tree::Tree;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VisualizerApp {
    code: String,
    // #[serde(skip)]
    tree: Tree<Range<usize>>,
    #[serde(skip)]
    modified: bool,
    #[serde(skip)]
    logs: String,
}

impl Default for VisualizerApp {
    fn default() -> Self {
        let code = r#""Hello, World!""#;
        Self {
            code: code.to_owned(),
            tree: (&parse(&*code).1.unwrap()).into(),
            modified: false,
            logs: "".into(),
        }
    }
}

impl VisualizerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for VisualizerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            code,
            tree,
            modified,
            logs,
        } = self;

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Tree View");

            ScrollArea::both().show(ui, |ui| {
                if *modified {
                    let mut files = Files::new();

                    let _module = Module::open("./".into(), &mut files).unwrap();

                    let file = files.get(1.into()).unwrap();

                    let (remaining, expression, errors) = parse(file);
                    if let Some(expression) = expression.as_ref() {
                        *tree = expression.into();
                    } else {
                        writeln!(logs, "Failed parsing the expression").unwrap();
                    }

                    writeln!(logs, "Remaining: {:?}", remaining.display()).unwrap();

                    for error in errors {
                        writeln!(logs, "Error: {error}").unwrap();
                    }

                    *modified = false;
                }

                ui.add(&*tree);
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.5))
                .size(Size::remainder())
                .vertical(|mut s| {
                    s.cell(|ui| {
                        ui.heading("Code");
                        Frame::none().outer_margin(10.0).show(ui, |ui| {
                            ScrollArea::both()
                                .max_height(ui.available_height())
                                .max_width(ui.available_width())
                                .id_source("editor")
                                .show(ui, |ui| {
                                    let editor = ui.add_sized(
                                        ui.available_size(),
                                        TextEdit::multiline(code).code_editor(),
                                    );

                                    if editor.changed() {
                                        *modified = true;
                                    }
                                });
                        });
                    });

                    s.cell(|ui| {
                        ui.heading("Console");
                        Frame::none().outer_margin(10.0).show(ui, |ui| {
                            ScrollArea::both()
                                .max_height(ui.available_height())
                                .max_width(ui.available_width())
                                .stick_to_bottom(true)
                                .id_source("console")
                                .show(ui, |ui| {
                                    let mut buffer = if logs.is_empty() {
                                        "No console output"
                                    } else {
                                        &**logs
                                    };
                                    ui.add_sized(
                                        ui.available_size(),
                                        TextEdit::multiline(&mut buffer).code_editor(), // .interactive(false)
                                    );
                                });
                        });
                    });
                })

            // ui.separator();
        });
    }
}

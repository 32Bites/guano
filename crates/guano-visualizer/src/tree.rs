use std::borrow::Cow;

use egui::{
    collapsing_header::CollapsingState, CollapsingHeader, Color32, CursorIcon, Label, Response,
    RichText, Sense, TextEdit, Ui, Widget,
};
// use egui_node_graph::NodeTemplateTrait;
use guano_parser::ast::{expression::Expr, span::NodeSpan};

use serde::{Deserialize, Serialize, __private::de};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tree<E> {
    Leaf(Element, E),
    List {
        name: Element,
        items: Vec<Tree<E>>,
        extra: E,
    },
    Struct {
        name: Element,
        items: Vec<(String, Tree<E>)>,
        extra: E,
    },
}

fn depth_to_color(depth: usize) -> Color32 {
    let colors = [
        Color32::GOLD,
        // Color32::GREEN,
        Color32::KHAKI,
        Color32::LIGHT_BLUE,
        Color32::LIGHT_GREEN,
        Color32::LIGHT_RED,
        Color32::LIGHT_YELLOW,
        // Color32::RED,
        // Color32::YELLOW,
    ];
    colors[depth % colors.len()]
}

impl<E> Tree<E> {
    pub fn render_ui(&self, ui: &mut Ui, depth: usize) -> Response {
        match self {
            Tree::Leaf(text, _) => self.leaf_ui(text, ui, depth),
            Tree::List {
                name,
                items,
                extra: _,
            } => self.list_ui(name, &items, ui, depth),
            Tree::Struct {
                name,
                items,
                extra: _,
            } => self.struct_ui(name, &items, ui, depth),
        }
    }

    fn leaf_ui(&self, text: &Element, ui: &mut Ui, depth: usize) -> Response {
        text.render_clickable(depth_to_color(depth), ui)
    }

    fn list_ui(&self, name: &Element, items: &[Tree<E>], ui: &mut Ui, depth: usize) -> Response {
        CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(self as *const Tree<E>),
            true,
        )
        .show_header(ui, |ui| name.render(Color32::WHITE, ui))
        .body(|ui| {
            for item in items {
                item.render_ui(ui, depth + 1);
            }
        })
        .0
    }

    fn struct_ui(
        &self,
        name: &Element,
        items: &[(String, Tree<E>)],
        ui: &mut Ui,
        depth: usize,
    ) -> Response {
        CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(self as *const Tree<E>),
            true,
        )
        .show_header(ui, |ui| name.render(Color32::WHITE, ui))
        .body(|ui| {
            for item in items {
                CollapsingState::load_with_default_open(
                    ui.ctx(),
                    ui.make_persistent_id(item as *const (_, _)),
                    true,
                )
                .show_header(ui, |ui| {
                    ui.label(
                        RichText::new(&item.0)
                            .color(Color32::WHITE)
                            .underline()
                            .size(15.0),
                    )
                })
                .body(|ui| {
                    item.1.render_ui(ui, depth + 1);
                });
            }
        })
        .0
    }
}

impl<E> Widget for Tree<E> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.render_ui(ui, 0)
    }
}

impl<E> Widget for &Tree<E> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.render_ui(ui, 0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    icon: char,
    text: String,
    hover_text: Option<String>,
}

impl From<String> for Element {
    fn from(s: String) -> Self {
        Self {
            text: s,
            icon: '🗋',
            hover_text: None,
        }
    }
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        Self {
            text: s.into(),
            icon: '🗋',
            hover_text: None,
        }
    }
}

impl Element {
    pub fn new(icon: char, text: String, hover_text: Option<String>) -> Self {
        Element {
            icon,
            text,
            hover_text,
        }
    }

    fn render(&self, color: Color32, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label(RichText::new(self.icon.to_string()).color(color).size(20.0));
            let label = ui.add(Label::new(
                RichText::new(&self.text).color(color).size(15.0),
            ));

            if let Some(hover_text) = &self.hover_text {
                label.on_hover_text(&*hover_text);
            }
        })
        .response
    }

    fn render_clickable(&self, color: Color32, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label(RichText::new(self.icon.to_string()).color(color).size(20.0));
            // ui.colored_label(Color32::WHITE, self.icon.to_string());

            let mut label = ui.add(
                Label::new(RichText::new(&self.text).color(color).size(15.0)).sense(Sense::click()),
            );

            if let Some(hover_text) = &self.hover_text {
                label = label.on_hover_text(&*hover_text);
            }

            label = label.on_hover_cursor(CursorIcon::PointingHand);

            if label.clicked() {
                ui.output().copied_text = self.text.clone();
            }
        })
        .response
    }
}

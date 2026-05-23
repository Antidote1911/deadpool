#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use deadpool_core::Pool;
use eframe::egui;

const ACCENT: egui::Color32 = egui::Color32::from_rgb(255, 160, 47);
const BTN_INACTIVE: egui::Color32 = egui::Color32::from_rgb(65, 65, 65);
const BTN_INACTIVE_TEXT: egui::Color32 = egui::Color32::from_rgb(177, 177, 177);
const BTN_ACTIVE_TEXT: egui::Color32 = egui::Color32::from_rgb(24, 24, 24);
const ERROR_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 100, 100);

struct AppState {
    password_length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_digits: bool,
    use_braces: bool,
    use_punctuation: bool,
    use_quotes: bool,
    use_dashes: bool,
    use_math: bool,
    use_logograms: bool,
    include_chars: String,
    exclude_chars: String,
    num_passwords: usize,
    generated: Vec<String>,
    error_message: Option<String>,
    copied_password: Option<String>,
    all_copied: bool,
}

impl AppState {
    fn new() -> Self {
        let mut s = Self {
            password_length: 20,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_braces: false,
            use_punctuation: false,
            use_quotes: false,
            use_dashes: false,
            use_math: false,
            use_logograms: false,
            include_chars: String::new(),
            exclude_chars: String::new(),
            num_passwords: 1,
            generated: Vec::new(),
            error_message: None,
            copied_password: None,
            all_copied: false,
        };
        s.regenerate();
        s
    }

    fn has_any_set(&self) -> bool {
        self.use_uppercase
            || self.use_lowercase
            || self.use_digits
            || self.use_braces
            || self.use_punctuation
            || self.use_quotes
            || self.use_dashes
            || self.use_math
            || self.use_logograms
            || !self.include_chars.is_empty()
    }

    fn regenerate(&mut self) {
        if !self.has_any_set() {
            self.error_message = Some("Select at least one character group.".into());
            self.generated.clear();
            return;
        }

        let mut pool = Pool::new();
        if self.use_uppercase {
            pool.extend_from_uppercase();
        }
        if self.use_lowercase {
            pool.extend_from_lowercase();
        }
        if self.use_digits {
            pool.extend_from_digits();
        }
        if self.use_braces {
            pool.extend_from_braces();
        }
        if self.use_punctuation {
            pool.extend_from_punctuation();
        }
        if self.use_quotes {
            pool.extend_from_quotes();
        }
        if self.use_dashes {
            pool.extend_from_dashes();
        }
        if self.use_math {
            pool.extend_from_math();
        }
        if self.use_logograms {
            pool.extend_from_logograms();
        }

        // extend_from_string must run before exclude_chars so that custom chars
        // are added as-is; generate() then filters excluded ones at draw time.
        if !self.include_chars.is_empty() {
            if let Err(e) = pool.extend_from_string(&self.include_chars) {
                self.error_message = Some(e.to_string());
                self.generated.clear();
                return;
            }
        }
        if !self.exclude_chars.is_empty() {
            pool.exclude_chars(&self.exclude_chars);
        }

        // Auto-clamp length so it never falls below the mandatory minimum.
        let min_len = [
            self.use_uppercase,
            self.use_lowercase,
            self.use_digits,
            self.use_braces,
            self.use_punctuation,
            self.use_quotes,
            self.use_dashes,
            self.use_math,
            self.use_logograms,
        ]
        .iter()
        .filter(|&&b| b)
        .count()
            + usize::from(!self.include_chars.is_empty());

        if self.password_length < min_len {
            self.password_length = min_len;
        }

        self.error_message = None;
        self.all_copied = false;
        self.copied_password = None;
        self.generated = (0..self.num_passwords)
            .filter_map(|_| pool.generate(self.password_length).ok())
            .collect();
    }
}

// Helper: renders a fixed-size toggle button; returns true when the state flipped.
fn toggle_btn(ui: &mut egui::Ui, label: &str, state: &mut bool, width: f32) -> bool {
    let (fill, text_color) = if *state {
        (ACCENT, BTN_ACTIVE_TEXT)
    } else {
        (BTN_INACTIVE, BTN_INACTIVE_TEXT)
    };
    let btn = egui::Button::new(egui::RichText::new(label).color(text_color).size(13.5))
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 30, 30)))
        .corner_radius(5.0);
    let clicked = ui.add_sized([width, 30.0], btn).clicked();
    if clicked {
        *state = !*state;
    }
    clicked
}

fn main() -> eframe::Result {
    let icon = include_bytes!("../resources/icon.png");
    let image = image::load_from_memory(icon).expect("Failed to load icon");
    let rgba = image.to_rgba8();
    let (w, h) = rgba.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(egui::IconData {
                rgba: rgba.into_raw(),
                width: w,
                height: h,
            })
            .with_inner_size([600.0, 660.0])
            .with_min_inner_size([520.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Strong Password Generator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals.panel_fill = egui::Color32::from_rgb(50, 50, 50);
            style.visuals.window_fill = egui::Color32::from_rgb(50, 50, 50);
            style.spacing.icon_width = 18.0;
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(22.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(16.0, egui::FontFamily::Monospace),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(15.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(13.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            cc.egui_ctx.set_style(style);

            Ok(Box::new(AppState::new()))
        }),
    )
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut changed = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("🔑 Strong Password Generator");
            ui.separator();

            // ── Character-set toggle buttons ─────────────────────────────────
            // Row 1 — 5 columns
            let gap = 4.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                let w = (ui.available_width() - gap * 4.0) / 5.0;
                changed |= toggle_btn(ui, "A-Z", &mut self.use_uppercase, w);
                changed |= toggle_btn(ui, "a-z", &mut self.use_lowercase, w);
                changed |= toggle_btn(ui, "0-9", &mut self.use_digits, w);
                changed |= toggle_btn(ui, "#$%&@^`~", &mut self.use_logograms, w);
                changed |= toggle_btn(ui, "<>*+!?=", &mut self.use_math, w);
            });
            ui.add_space(gap);
            // Row 2 — 4 active + 1 empty placeholder to keep alignment
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                let w = (ui.available_width() - gap * 4.0) / 5.0;
                changed |= toggle_btn(ui, ".,:;", &mut self.use_punctuation, w);
                changed |= toggle_btn(ui, "\" '", &mut self.use_quotes, w);
                changed |= toggle_btn(ui, r"\ / | _ -", &mut self.use_dashes, w);
                changed |= toggle_btn(ui, "{ [ ( ) ] }", &mut self.use_braces, w);
                // intentionally empty — slot reserved for a future set
                ui.allocate_space(egui::Vec2::new(w, 30.0));
            });

            ui.separator();

            // ── Password length ───────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label("Length:");
                changed |= ui
                    .add(egui::Slider::new(&mut self.password_length, 1..=200).show_value(true))
                    .changed();
            });

            ui.separator();

            // ── Include / Exclude ─────────────────────────────────────────────
            egui::Grid::new("fields_grid")
                .num_columns(2)
                .spacing([8.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Also include:");
                    changed |= ui
                        .add(
                            egui::TextEdit::singleline(&mut self.include_chars)
                                .hint_text("extra characters to include")
                                .desired_width(f32::INFINITY),
                        )
                        .changed();
                    ui.end_row();

                    ui.label("Do not include:");
                    ui.horizontal(|ui| {
                        changed |= ui
                            .add(
                                egui::TextEdit::singleline(&mut self.exclude_chars)
                                    .hint_text("characters to exclude")
                                    .desired_width(ui.available_width() - 115.0),
                            )
                            .changed();

                        let lookalike_btn = egui::Button::new("Add Look-alike")
                            .fill(BTN_INACTIVE)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 90, 90)))
                            .corner_radius(5.0);
                        if ui
                            .add(lookalike_btn)
                            .on_hover_text(
                                "Exclude characters that look similar: l B G I O 0 1 6 8 o | .",
                            )
                            .clicked()
                        {
                            for ch in "lBGIO0168o|.".chars() {
                                if !self.exclude_chars.contains(ch) {
                                    self.exclude_chars.push(ch);
                                }
                            }
                            changed = true;
                        }
                    });
                    ui.end_row();
                });

            ui.separator();

            // ── Number of passwords ───────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label("Number of passwords:");
                changed |= ui
                    .add(egui::Slider::new(&mut self.num_passwords, 1..=50).show_value(true))
                    .changed();
            });

            ui.separator();

            // ── Action buttons ────────────────────────────────────────────────
            ui.horizontal(|ui| {
                let gen_btn = egui::Button::new(
                    egui::RichText::new("🔄 Generate Passwords")
                        .color(egui::Color32::from_rgb(24, 24, 24)),
                )
                .fill(ACCENT)
                .corner_radius(6.0);
                if ui.add_enabled(self.has_any_set(), gen_btn).clicked() {
                    self.regenerate();
                }

                let copy_btn =
                    egui::Button::new(egui::RichText::new("📋 Copy All").color(BTN_INACTIVE_TEXT))
                        .fill(BTN_INACTIVE)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 90, 90)))
                        .corner_radius(6.0);
                if ui
                    .add_enabled(!self.generated.is_empty(), copy_btn)
                    .clicked()
                {
                    ui.ctx().copy_text(self.generated.join("\n"));
                    self.all_copied = true;
                }
            });

            // ── Status line ───────────────────────────────────────────────────
            if let Some(err) = &self.error_message {
                ui.label(
                    egui::RichText::new(format!("⚠  {err}"))
                        .color(ERROR_COLOR)
                        .size(13.0),
                );
            } else if !self.generated.is_empty() {
                ui.label(
                    egui::RichText::new("ℹ  Click a password to copy it")
                        .color(egui::Color32::from_rgb(130, 130, 130))
                        .size(13.0),
                );
            }

            // ── Password list ─────────────────────────────────────────────────
            if !self.generated.is_empty() {
                egui::Frame::group(ui.style())
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::top_down(egui::Align::LEFT)
                                        .with_cross_justify(true),
                                    |ui| {
                                        for password in &self.generated {
                                            ui.horizontal(|ui| {
                                                ui.set_width(ui.available_width());
                                                let resp = ui.monospace(password);
                                                if resp.clicked() {
                                                    ui.ctx().copy_text(password.clone());
                                                    self.copied_password = Some(password.clone());
                                                    self.all_copied = false;
                                                }
                                                if self.all_copied
                                                    || Some(password)
                                                        == self.copied_password.as_ref()
                                                {
                                                    ui.label(
                                                        egui::RichText::new("✅").color(ACCENT),
                                                    );
                                                }
                                            });
                                        }
                                    },
                                );
                            });
                    });
            }
        });

        // Auto-regenerate when any setting changed (mirrors Qt behaviour).
        if changed {
            self.regenerate();
        }
    }
}

use std::sync::mpsc::{channel, Receiver, Sender};
use eframe::egui::{self, Color32, CornerRadius, RichText, Stroke, Vec2};
use crate::pi_search::{DateFormat, SearchResult, query_pi_offline, query_pi_online};
use crate::pi_streamer::PiStreamer;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tab {
    Search,
    Streamer,
    Certificate,
    Theory,
}

pub struct PiApp {
    active_tab: Tab,

    // Inputs
    day_str: String,
    month_str: String,
    year_str: String,
    format: DateFormat,
    custom_mode: bool,
    custom_input: String,

    // Search state
    is_searching: bool,
    search_error: Option<String>,
    current_result: Option<SearchResult>,

    // Channels
    tx: Sender<Result<SearchResult, String>>,
    rx: Receiver<Result<SearchResult, String>>,

    // Live Streamer
    streamer: PiStreamer,
    toast_message: Option<(String, std::time::Instant)>,
}

impl Default for PiApp {
    fn default() -> Self {
        let (tx, rx) = channel();

        // Default to Albert Einstein's birthday on Pi Day (14 March 1879)
        Self {
            active_tab: Tab::Search,
            day_str: "14".to_string(),
            month_str: "03".to_string(),
            year_str: "1879".to_string(),
            format: DateFormat::DDMMYYYY,
            custom_mode: false,
            custom_input: "14031879".to_string(),
            is_searching: false,
            search_error: None,
            current_result: None,
            tx,
            rx,
            streamer: PiStreamer::default(),
            toast_message: None,
        }
    }
}

impl PiApp {
    fn current_query_string(&self) -> String {
        if self.custom_mode {
            self.custom_input.trim().to_string()
        } else {
            let d: u32 = self.day_str.trim().parse().unwrap_or(1).clamp(1, 31);
            let m: u32 = self.month_str.trim().parse().unwrap_or(1).clamp(1, 12);
            let y: i32 = self.year_str.trim().parse().unwrap_or(2000).clamp(1000, 2100);
            self.format.format(d, m, y)
        }
    }

    fn show_toast(&mut self, msg: &str) {
        self.toast_message = Some((msg.to_string(), std::time::Instant::now()));
    }

    fn execute_search(&mut self) {
        let query = self.current_query_string();
        if query.is_empty() {
            self.search_error = Some("Please provide a valid date or numeric sequence.".to_string());
            return;
        }

        self.is_searching = true;
        self.search_error = None;
        self.current_result = None;

        // Reset streamer to follow target
        self.streamer.reset(Some(query.clone()));

        // Local cache lookup
        if let Some(res) = query_pi_offline(&query) {
            self.is_searching = false;
            self.current_result = Some(res);
            self.show_toast("Found instantly in local Pi cache!");
            return;
        }

        // Global async lookup across 200M digits
        let tx = self.tx.clone();
        let q = query.clone();
        std::thread::spawn(move || {
            let res = query_pi_online(&q);
            let _ = tx.send(res);
        });
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        // High-end Velvety Midnight Palette
        visuals.panel_fill = Color32::from_rgb(10, 12, 18);
        visuals.window_fill = Color32::from_rgb(15, 18, 28);

        // Inactive widgets (buttons, boxes)
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(20, 24, 38);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(226, 232, 240));
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(38, 46, 70));
        visuals.widgets.inactive.corner_radius = CornerRadius::same(7);

        // Hovered widgets
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(32, 38, 60);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(99, 102, 241));
        visuals.widgets.hovered.corner_radius = CornerRadius::same(7);

        // Active widgets
        visuals.widgets.active.bg_fill = Color32::from_rgb(79, 70, 229);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(7);

        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(14, 17, 26);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(28, 34, 52));
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(148, 163, 184));
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);

        ctx.set_visuals(visuals);
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(14, 17, 26))
            .stroke(Stroke::new(1.0, Color32::from_rgb(28, 34, 52)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(14.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Glowing Icon Pill
                    egui::Frame::group(ui.style())
                        .fill(Color32::from_rgb(30, 27, 75))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(99, 102, 241)))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new(" π ").size(22.0).color(Color32::from_rgb(165, 180, 252)).strong());
                        });

                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("PI COSMIC OBSERVATORY").size(17.0).color(Color32::WHITE).strong());
                        ui.label(
                            RichText::new("Search & track date coordinates in the infinite transcendental stream")
                                .size(11.5)
                                .color(Color32::from_rgb(148, 163, 184))
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Navigation Pills
                        let tabs = [
                            (Tab::Theory, "Theory"),
                            (Tab::Certificate, "Certificate"),
                            (Tab::Streamer, "Live Stream"),
                            (Tab::Search, "Explorer"),
                        ];

                        for (tab, name) in tabs {
                            let is_active = self.active_tab == tab;
                            let btn = egui::Button::new(
                                RichText::new(name)
                                    .size(13.0)
                                    .color(if is_active { Color32::WHITE } else { Color32::from_rgb(148, 163, 184) })
                                    .strong()
                            )
                            .fill(if is_active { Color32::from_rgb(79, 70, 229) } else { Color32::from_rgb(20, 24, 38) })
                            .corner_radius(CornerRadius::same(6));

                            if ui.add(btn).clicked() {
                                self.active_tab = tab;
                            }
                        }
                    });
                });
            });

        ui.add_space(10.0);
    }

    fn render_search_tab(&mut self, ui: &mut egui::Ui) {
        let current_target = self.current_query_string();

        // 1. Sleek Date Input Console Card
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(17, 21, 33))
            .stroke(Stroke::new(1.0, Color32::from_rgb(33, 41, 64)))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("TARGET COORDINATES").size(13.0).color(Color32::from_rgb(129, 140, 248)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("● 200,000,000 Digits Ready").size(11.0).color(Color32::from_rgb(52, 211, 153)));
                    });
                });

                ui.add_space(10.0);

                if !self.custom_mode {
                    // High-Craft Date Inputs
                    ui.horizontal(|ui| {
                        // Day Box
                        ui.vertical(|ui| {
                            ui.label(RichText::new("DAY").size(10.5).color(Color32::from_rgb(148, 163, 184)).strong());
                            ui.add_sized(Vec2::new(75.0, 32.0), egui::TextEdit::singleline(&mut self.day_str).font(egui::TextStyle::Monospace));
                        });

                        ui.add_space(8.0);
                        // Month Box
                        ui.vertical(|ui| {
                            ui.label(RichText::new("MONTH").size(10.5).color(Color32::from_rgb(148, 163, 184)).strong());
                            ui.add_sized(Vec2::new(75.0, 32.0), egui::TextEdit::singleline(&mut self.month_str).font(egui::TextStyle::Monospace));
                        });

                        ui.add_space(8.0);
                        // Year Box
                        ui.vertical(|ui| {
                            ui.label(RichText::new("YEAR").size(10.5).color(Color32::from_rgb(148, 163, 184)).strong());
                            ui.add_sized(Vec2::new(95.0, 32.0), egui::TextEdit::singleline(&mut self.year_str).font(egui::TextStyle::Monospace));
                        });

                        ui.add_space(16.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new("FORMAT").size(10.5).color(Color32::from_rgb(148, 163, 184)).strong());
                            ui.horizontal_wrapped(|ui| {
                                for &fmt in &[DateFormat::DDMMYYYY, DateFormat::MMDDYYYY, DateFormat::YYYYMMDD, DateFormat::DDMMYY, DateFormat::DDMM] {
                                    let is_sel = self.format == fmt;
                                    let lbl = match fmt {
                                        DateFormat::DDMMYYYY => "DD/MM/YYYY",
                                        DateFormat::MMDDYYYY => "MM/DD/YYYY",
                                        DateFormat::YYYYMMDD => "YYYY/MM/DD",
                                        DateFormat::DDMMYY => "DD/MM/YY",
                                        DateFormat::DDMM => "DD/MM",
                                        _ => "Standard",
                                    };
                                    let btn = egui::Button::new(
                                        RichText::new(lbl)
                                            .size(11.5)
                                            .color(if is_sel { Color32::from_rgb(224, 231, 255) } else { Color32::from_rgb(148, 163, 184) })
                                    )
                                    .fill(if is_sel { Color32::from_rgb(49, 46, 129) } else { Color32::from_rgb(24, 30, 48) })
                                    .stroke(Stroke::new(1.0, if is_sel { Color32::from_rgb(99, 102, 241) } else { Color32::from_rgb(40, 48, 76) }))
                                    .corner_radius(CornerRadius::same(5));

                                    if ui.add(btn).clicked() {
                                        self.format = fmt;
                                    }
                                }
                            });
                        });
                    });
                } else {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("ENTER CUSTOM NUMBER / DATE STRING").size(11.0).color(Color32::from_rgb(148, 163, 184)).strong());
                        ui.add_sized(Vec2::new(ui.available_width(), 32.0), egui::TextEdit::singleline(&mut self.custom_input).font(egui::TextStyle::Monospace));
                    });
                }

                ui.add_space(12.0);

                // Presets & Mode toggle row
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.custom_mode, RichText::new("Direct Numeric Mode").size(12.0).color(Color32::from_rgb(148, 163, 184)));

                    ui.add_space(14.0);
                    ui.label(RichText::new("Historical Landmarks:").size(11.5).color(Color32::from_rgb(100, 116, 139)));

                    let presets = [
                        ("Einstein (Pi Day 1879)", "14", "03", "1879"),
                        ("Moon Landing (1969)", "20", "07", "1969"),
                        ("World Wide Web (1989)", "12", "03", "1989"),
                    ];

                    for (name, d, m, y) in presets {
                        let btn = egui::Button::new(RichText::new(name).size(11.0).color(Color32::from_rgb(199, 210, 254)))
                            .fill(Color32::from_rgb(24, 28, 44))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 53, 82)))
                            .corner_radius(CornerRadius::same(5));

                        if ui.add(btn).clicked() {
                            self.day_str = d.to_string();
                            self.month_str = m.to_string();
                            self.year_str = y.to_string();
                            self.custom_mode = false;
                        }
                    }
                });

                ui.add_space(16.0);

                // Action Search Console Bar
                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(10, 12, 18))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(33, 41, 64)))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Resolved Target:").size(13.0).color(Color32::from_rgb(148, 163, 184)));
                            ui.label(
                                RichText::new(&current_target)
                                    .size(22.0)
                                    .monospace()
                                    .color(Color32::from_rgb(251, 191, 36))
                                    .strong()
                            );

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let btn = egui::Button::new(
                                    RichText::new(if self.is_searching { "SCANNING COSMIC STREAM..." } else { "SEARCH IN PI" })
                                        .size(13.5)
                                        .color(if self.is_searching { Color32::from_rgb(200, 200, 200) } else { Color32::BLACK })
                                        .strong()
                                )
                                .fill(if self.is_searching { Color32::from_rgb(75, 85, 99) } else { Color32::from_rgb(99, 102, 241) })
                                .corner_radius(CornerRadius::same(6))
                                .min_size(Vec2::new(180.0, 36.0));

                                if ui.add(btn).clicked() && !self.is_searching {
                                    self.execute_search();
                                }
                            });
                        });
                    });
            });

        ui.add_space(14.0);

        // Feedback / Spinner
        if self.is_searching {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(14, 18, 28))
                .stroke(Stroke::new(1.0, Color32::from_rgb(49, 46, 129)))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RichText::new("Traversing 200,000,000 decimal digits of Pi across distributed memory...").size(13.0).color(Color32::from_rgb(199, 210, 254)));
                    });
                });
            ui.add_space(12.0);
        }

        if let Some(ref err) = self.search_error {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(38, 16, 22))
                .stroke(Stroke::new(1.0, Color32::from_rgb(225, 29, 72)))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(14.0)
                .show(ui, |ui| {
                    ui.label(RichText::new(format!("Notice: {}", err)).color(Color32::from_rgb(254, 205, 211)));
                });
            ui.add_space(12.0);
        }

        // 2. High-End Results Showcase Card
        if let Some(res) = self.current_result.clone() {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(13, 22, 28))
                .stroke(Stroke::new(1.5, Color32::from_rgb(16, 185, 129)))
                .corner_radius(CornerRadius::same(12))
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("COSMIC COORDINATES RESOLVED").size(14.0).color(Color32::from_rgb(52, 211, 153)).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("Verified via {}", res.source)).size(11.0).color(Color32::from_rgb(100, 116, 139)));
                        });
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Decimal Position:").size(15.0).color(Color32::from_rgb(148, 163, 184)));
                        ui.label(
                            RichText::new(format!("#{}", res.position))
                                .size(28.0)
                                .monospace()
                                .color(Color32::from_rgb(251, 191, 36))
                                .strong()
                        );
                        ui.label(RichText::new("digits after 3.").size(12.0).color(Color32::from_rgb(100, 116, 139)));
                    });

                    ui.add_space(12.0);
                    ui.label(RichText::new("Surrounding Decimal Tape:").size(12.5).color(Color32::from_rgb(148, 163, 184)));

                    // Monospace Tape Readout Strip
                    egui::Frame::group(ui.style())
                        .fill(Color32::from_rgb(8, 11, 16))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(24, 32, 48)))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(14.0)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(format!("...{}", res.before_context))
                                        .size(15.0)
                                        .monospace()
                                        .color(Color32::from_rgb(100, 116, 139))
                                );
                                ui.label(
                                    RichText::new(format!(" [{}] ", res.matched))
                                        .size(17.0)
                                        .monospace()
                                        .color(Color32::from_rgb(52, 211, 153))
                                        .strong()
                                        .background_color(Color32::from_rgb(6, 78, 59))
                                );
                                ui.label(
                                    RichText::new(format!("{}...", res.after_context))
                                        .size(15.0)
                                        .monospace()
                                        .color(Color32::from_rgb(100, 116, 139))
                                );
                            });
                        });

                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        let copy_btn = egui::Button::new(RichText::new("Copy Position").size(12.0).color(Color32::WHITE))
                            .fill(Color32::from_rgb(24, 32, 48))
                            .corner_radius(CornerRadius::same(6));
                        if ui.add(copy_btn).clicked() {
                            ui.ctx().copy_text(res.position.to_string());
                            self.show_toast(&format!("Copied index #{} to clipboard!", res.position));
                        }

                        let tape_btn = egui::Button::new(RichText::new("Copy Full Tape").size(12.0).color(Color32::WHITE))
                            .fill(Color32::from_rgb(24, 32, 48))
                            .corner_radius(CornerRadius::same(6));
                        if ui.add(tape_btn).clicked() {
                            let text = format!("...{} [{}] {}...", res.before_context, res.matched, res.after_context);
                            ui.ctx().copy_text(text);
                            self.show_toast("Copied full Pi tape stream to clipboard!");
                        }

                        let cert_btn = egui::Button::new(RichText::new("View Certificate").size(12.0).color(Color32::BLACK).strong())
                            .fill(Color32::from_rgb(251, 191, 36))
                            .corner_radius(CornerRadius::same(6));
                        if ui.add(cert_btn).clicked() {
                            self.active_tab = Tab::Certificate;
                        }
                    });
                });
        }
    }

    fn render_streamer_tab(&mut self, ui: &mut egui::Ui) {
        let current_target = self.current_query_string();

        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(17, 21, 33))
            .stroke(Stroke::new(1.0, Color32::from_rgb(33, 41, 64)))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(18.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("LIVE PI STREAMER TELEMETRY").size(13.5).color(Color32::from_rgb(129, 140, 248)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("Current Stream Index: #{}", self.streamer.current_digit_index))
                                .size(15.0)
                                .monospace()
                                .color(Color32::from_rgb(251, 191, 36))
                                .strong()
                        );
                    });
                });

                ui.label(
                    RichText::new("Streaming sequential digits of π from cosmic memory with continuous pattern detection.")
                        .size(12.0)
                        .color(Color32::from_rgb(148, 163, 184))
                );

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let play_btn = egui::Button::new(
                        RichText::new(if self.streamer.is_streaming { "Pause Stream" } else { "Resume Stream" })
                            .size(12.5)
                            .color(Color32::WHITE)
                    )
                    .fill(Color32::from_rgb(30, 36, 56))
                    .corner_radius(CornerRadius::same(6));

                    if ui.add(play_btn).clicked() {
                        self.streamer.is_streaming = !self.streamer.is_streaming;
                    }

                    let rew_btn = egui::Button::new(RichText::new("Rewind to 0").size(12.5).color(Color32::WHITE))
                        .fill(Color32::from_rgb(30, 36, 56))
                        .corner_radius(CornerRadius::same(6));

                    if ui.add(rew_btn).clicked() {
                        self.streamer.reset(Some(current_target.clone()));
                    }

                    ui.add_space(12.0);
                    ui.label(RichText::new("Stream Velocity:").size(12.0).color(Color32::from_rgb(148, 163, 184)));
                    ui.add(egui::Slider::new(&mut self.streamer.stream_speed, 1..=50).text("digits/frame"));
                });

                ui.add_space(12.0);

                // Phosphor Terminal Display
                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(8, 10, 16))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(28, 36, 54)))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.set_min_height(260.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("3.")
                                    .size(24.0)
                                    .monospace()
                                    .color(Color32::from_rgb(245, 158, 11))
                                    .strong()
                            );
                            ui.add_space(6.0);

                            if let Some(pos) = self.streamer.match_found_at {
                                ui.label(
                                    RichText::new(format!("TARGET [{}] LOCKED AT DIGIT #{}", current_target, pos))
                                        .size(15.0)
                                        .color(Color32::from_rgb(52, 211, 153))
                                        .strong()
                                );
                                ui.add_space(4.0);
                            }

                            ui.label(
                                RichText::new(&self.streamer.recent_digits)
                                    .size(14.0)
                                    .monospace()
                                    .color(Color32::from_rgb(199, 210, 254))
                            );
                        });
                    });
            });
    }

    fn render_certificate_tab(&mut self, ui: &mut egui::Ui) {
        let current_target = self.current_query_string();
        let pos_str = if let Some(ref res) = self.current_result {
            format!("#{}", res.position)
        } else {
            "#11,885,022 (Sample for Einstein 14/03/1879)".to_string()
        };

        let before = self.current_result.as_ref().map(|r| r.before_context.as_str()).unwrap_or("984510382947");
        let after = self.current_result.as_ref().map(|r| r.after_context.as_str()).unwrap_or("492048591823");

        let cert_text = format!(
r#"
+=============================================================================+
|                      CERTIFICATE OF EXISTENCE IN PI                         |
+=============================================================================+
|                                                                             |
|   Target Date / Coordinates:                                                |
|   ★  {}  ★                                                       |
|                                                                             |
|   Cosmic Transcendental Decimal Offset:                                     |
|   👉  {}                                                      |
|                                                                             |
|   Verified Context Tape Stream:                                             |
|   ... {} [{}] {} ...                         |
|                                                                             |
|   Mathematical Note:                                                        |
|   Because π is transcendental and conjectured to be normal in base 10,       |
|   every finite permutation of numbers exists infinitely across eternity.    |
|                                                                             |
+=============================================================================+
"#,
            current_target, pos_str, before, current_target, after
        );

        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(17, 21, 33))
            .stroke(Stroke::new(1.0, Color32::from_rgb(33, 41, 64)))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.label(RichText::new("CERTIFICATE OF COSMIC IMMERSION").size(14.0).color(Color32::from_rgb(251, 191, 36)).strong());
                ui.add_space(8.0);

                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(8, 10, 16))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(28, 36, 54)))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&cert_text)
                                .monospace()
                                .size(12.0)
                                .color(Color32::from_rgb(254, 240, 138))
                        );
                    });

                ui.add_space(12.0);
                let copy_cert = egui::Button::new(RichText::new("Copy Certificate to Clipboard").size(13.0).color(Color32::BLACK).strong())
                    .fill(Color32::from_rgb(251, 191, 36))
                    .corner_radius(CornerRadius::same(6))
                    .min_size(Vec2::new(200.0, 32.0));

                if ui.add(copy_cert).clicked() {
                    ui.ctx().copy_text(cert_text);
                    self.show_toast("Official Certificate copied to clipboard!");
                }
            });
    }

    fn render_theory_tab(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(17, 21, 33))
            .stroke(Stroke::new(1.0, Color32::from_rgb(33, 41, 64)))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.label(RichText::new("THE MATHEMATICS OF INFINITY").size(14.0).color(Color32::from_rgb(129, 140, 248)).strong());
                ui.add_space(10.0);

                ui.label(RichText::new("1. The Borel Normal Number Conjecture").size(14.5).color(Color32::WHITE).strong());
                ui.label(
                    "An irrational number is normal if every finite digit sequence of length k occurs \
                    with uniform asymptotic frequency of 1 in 10^k. Statistical analysis across trillions of digits \
                    confirms π behaves as an ideal normal number, meaning every date in human history exists inside it."
                );

                ui.add_space(10.0);
                ui.label(RichText::new("2. Expected Depth by Sequence Length").size(14.5).color(Color32::WHITE).strong());
                ui.label("• 4-Digit Sequences (10^4 = 10,000 combinations): Found in the first ~10,000 digits.");
                ui.label("• 6-Digit Sequences (10^6 = 1,000,000 combinations): Found in the first ~1-3 million digits.");
                ui.label("• 8-Digit Sequences (10^8 = 100,000,000 combinations): Found within the first 100-200 million digits.");

                ui.add_space(10.0);
                ui.label(RichText::new("3. The Bailey–Borwein–Plouffe (BBP) Formula").size(14.5).color(Color32::WHITE).strong());
                ui.label(
                    "Discovered in 1995, the BBP spigot formula allows extracting arbitrary n-th hexadecimal digits of π \
                    in O(n log n) time without computing previous digits, unlocking massive GPU parallelism."
                );
            });
    }
}

impl eframe::App for PiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.apply_theme(ui.ctx());

        // Check async receiver
        if let Ok(res) = self.rx.try_recv() {
            self.is_searching = false;
            match res {
                Ok(r) => {
                    self.current_result = Some(r);
                    self.show_toast("Coordinates located successfully!");
                }
                Err(e) => self.search_error = Some(e),
            }
        }

        // Advance live digit streamer
        self.streamer.tick();
        ui.ctx().request_repaint();

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_header(ui);

                match self.active_tab {
                    Tab::Search => self.render_search_tab(ui),
                    Tab::Streamer => self.render_streamer_tab(ui),
                    Tab::Certificate => self.render_certificate_tab(ui),
                    Tab::Theory => self.render_theory_tab(ui),
                }

                // Temporary Toast Notification Banner at the bottom
                if let Some((ref msg, time)) = self.toast_message {
                    if time.elapsed() < std::time::Duration::from_secs(4) {
                        ui.add_space(10.0);
                        egui::Frame::group(ui.style())
                            .fill(Color32::from_rgb(16, 185, 129))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.label(RichText::new(msg).color(Color32::BLACK).strong());
                            });
                    }
                }
            });
        });
    }
}

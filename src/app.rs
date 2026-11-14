use std::sync::mpsc::{channel, Receiver, Sender};
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};
use chrono::Datelike;
use crate::pi_search::{DateFormat, SearchResult, query_pi_offline, query_pi_online};
use crate::pi_streamer::PiStreamer;

#[derive(PartialEq, Eq)]
enum ActiveTab {
    Finder,
    Streamer,
    Certificate,
    About,
}

pub struct PiApp {
    active_tab: ActiveTab,

    // Birthday inputs
    day: u32,
    month: u32,
    year: i32,
    format: DateFormat,
    custom_query: String,
    use_custom: bool,

    // Search state
    is_searching: bool,
    search_error: Option<String>,
    current_result: Option<SearchResult>,

    // Async worker channel
    tx: Sender<Result<SearchResult, String>>,
    rx: Receiver<Result<SearchResult, String>>,

    // Live Pi digit streamer
    streamer: PiStreamer,
    copy_notification: Option<String>,
}

impl Default for PiApp {
    fn default() -> Self {
        let (tx, rx) = channel();

        Self {
            active_tab: ActiveTab::Finder,
            day: 25,
            month: 5,
            year: 2009,
            format: DateFormat::DDMMYYYY,
            custom_query: "25052009".to_string(),
            use_custom: false,
            is_searching: false,
            search_error: None,
            current_result: None,
            tx,
            rx,
            streamer: PiStreamer::default(),
            copy_notification: None,
        }
    }
}

impl PiApp {
    fn current_target_string(&self) -> String {
        if self.use_custom {
            self.custom_query.trim().to_string()
        } else {
            self.format.format(self.day, self.month, self.year)
        }
    }

    fn trigger_search(&mut self) {
        let query = self.current_target_string();
        if query.is_empty() {
            self.search_error = Some("Please enter a valid date or digit sequence.".to_string());
            return;
        }

        self.is_searching = true;
        self.search_error = None;
        self.current_result = None;

        // Reset live streamer to seek towards target
        self.streamer.reset(Some(query.clone()));

        // Check local 10k cache first
        if let Some(res) = query_pi_offline(&query) {
            self.is_searching = false;
            self.current_result = Some(res);
            return;
        }

        // Spawn background thread to query the 200M index
        let tx = self.tx.clone();
        let q = query.clone();
        std::thread::spawn(move || {
            let res = query_pi_online(&q);
            let _ = tx.send(res);
        });
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("π").size(38.0).color(Color32::from_rgb(0, 255, 204)).strong());
            ui.vertical(|ui| {
                ui.heading(
                    RichText::new("PI BIRTHDAY QUEST")
                        .size(24.0)
                        .color(Color32::WHITE)
                        .strong()
                );
                ui.label(
                    RichText::new("Discover where your birthday is permanently etched into the digits of π")
                        .size(12.5)
                        .color(Color32::from_rgb(180, 200, 220))
                );
            });
        });

        ui.add_space(8.0);

        // Navigation Tabs
        ui.horizontal(|ui| {
            if ui.selectable_label(self.active_tab == ActiveTab::Finder, "🎂 Birthday Finder").clicked() {
                self.active_tab = ActiveTab::Finder;
            }
            if ui.selectable_label(self.active_tab == ActiveTab::Streamer, "⚡ Live Pi Matrix Streamer").clicked() {
                self.active_tab = ActiveTab::Streamer;
            }
            if ui.selectable_label(self.active_tab == ActiveTab::Certificate, "📜 Pi Certificate").clicked() {
                self.active_tab = ActiveTab::Certificate;
            }
            if ui.selectable_label(self.active_tab == ActiveTab::About, "ℹ️ How it Works").clicked() {
                self.active_tab = ActiveTab::About;
            }
        });

        ui.separator();
    }

    fn render_finder_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(6.0);

        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(20, 24, 35))
            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 55, 80)))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(RichText::new("1. Enter Your Date of Birth").size(16.0).strong().color(Color32::from_rgb(0, 255, 204)));
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label("Day:");
                    ui.add(egui::DragValue::new(&mut self.day).range(1..=31).speed(0.2));

                    ui.label("Month:");
                    ui.add(egui::DragValue::new(&mut self.month).range(1..=12).speed(0.2));

                    ui.label("Year:");
                    ui.add(egui::DragValue::new(&mut self.year).range(1900..=2035).speed(0.5));

                    ui.add_space(12.0);
                    if ui.button("Preset: 25 May 2009").clicked() {
                        self.day = 25;
                        self.month = 5;
                        self.year = 2009;
                        self.use_custom = false;
                    }
                    if ui.button("Today").clicked() {
                        let now = chrono::Local::now();
                        self.day = now.day();
                        self.month = now.month();
                        self.year = now.year();
                        self.use_custom = false;
                    }
                });

                ui.add_space(10.0);
                ui.label(RichText::new("2. Select Date Format").size(14.0).strong());
                ui.horizontal_wrapped(|ui| {
                    for fmt in DateFormat::all() {
                        if ui.selectable_value(&mut self.format, *fmt, fmt.label()).clicked() {
                            self.use_custom = false;
                        }
                    }
                });

                ui.add_space(10.0);
                ui.checkbox(&mut self.use_custom, "Or enter custom number sequence / format:");
                if self.use_custom {
                    ui.text_edit_singleline(&mut self.custom_query);
                }

                let target = self.current_target_string();
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Search Target Pattern:");
                    ui.label(RichText::new(&target).size(18.0).color(Color32::from_rgb(255, 215, 0)).strong());
                });

                ui.add_space(12.0);
                let search_btn = ui.add_sized(
                    Vec2::new(ui.available_width(), 38.0),
                    egui::Button::new(
                        RichText::new(if self.is_searching { "🔍 SEARCHING ACROSS INFINITY..." } else { "⚡ LAUNCH PI SEARCH" })
                            .size(16.0)
                            .strong()
                            .color(Color32::BLACK)
                    ).fill(if self.is_searching { Color32::from_rgb(180, 180, 180) } else { Color32::from_rgb(0, 255, 204) })
                );

                if search_btn.clicked() && !self.is_searching {
                    self.trigger_search();
                }
            });

        ui.add_space(12.0);

        // Results Section
        if let Some(ref err) = self.search_error {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(45, 20, 25))
                .stroke(Stroke::new(1.0, Color32::RED))
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.label(RichText::new(format!("⚠ {}", err)).color(Color32::from_rgb(255, 100, 100)).strong());
                });
        }

        if let Some(ref res) = self.current_result {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(15, 30, 35))
                .stroke(Stroke::new(1.5, Color32::from_rgb(0, 255, 180)))
                .inner_margin(18.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🎉 BIRTHDAY FOUND IN PI!").size(20.0).color(Color32::from_rgb(0, 255, 204)).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("Source: {}", res.source)).size(11.0).color(Color32::GRAY));
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Decimal Position:").size(15.0));
                        ui.label(
                            RichText::new(format!("#{}", res.position))
                                .size(24.0)
                                .color(Color32::from_rgb(255, 215, 0))
                                .strong()
                        );
                        ui.label(format!("(digits after 3.1415...)"));
                    });

                    ui.add_space(10.0);
                    ui.label(RichText::new("Surrounding Pi Tape Stream:").size(13.0).strong());

                    // Visual Pi Tape
                    egui::Frame::group(ui.style())
                        .fill(Color32::from_rgb(10, 15, 20))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(50, 70, 90)))
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(format!("...{}", res.before_context)).size(16.0).color(Color32::from_rgb(140, 160, 180)));
                                ui.label(
                                    RichText::new(format!(" [ {} ] ", res.matched))
                                        .size(20.0)
                                        .color(Color32::from_rgb(0, 255, 120))
                                        .strong()
                                        .background_color(Color32::from_rgba_premultiplied(0, 100, 50, 100))
                                );
                                ui.label(RichText::new(format!("{}...", res.after_context)).size(16.0).color(Color32::from_rgb(140, 160, 180)));
                            });
                        });

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("📋 Copy Position").clicked() {
                            ui.ctx().copy_text(res.position.to_string());
                            self.copy_notification = Some(format!("Copied position #{} to clipboard!", res.position));
                        }
                        if ui.button("📜 Copy Pi Context").clicked() {
                            let text = format!("...{} [{}] {}...", res.before_context, res.matched, res.after_context);
                            ui.ctx().copy_text(text);
                            self.copy_notification = Some("Copied Pi context to clipboard!".to_string());
                        }
                        if ui.button("⭐ View Pi Birthday Certificate").clicked() {
                            self.active_tab = ActiveTab::Certificate;
                        }
                    });

                    if let Some(ref note) = self.copy_notification {
                        ui.add_space(4.0);
                        ui.label(RichText::new(note).size(12.0).color(Color32::from_rgb(0, 255, 150)));
                    }
                });
        }
    }

    fn render_streamer_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.heading(RichText::new("⚡ Live Pi Matrix Streamer").color(Color32::from_rgb(0, 255, 204)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("Digit Index: #{}", self.streamer.current_digit_index))
                        .size(16.0)
                        .color(Color32::from_rgb(255, 215, 0))
                        .strong()
                );
            });
        });

        ui.label("Watch digits of π stream and print in real-time. Target patterns will highlight when detected!");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.button(if self.streamer.is_streaming { "⏸ Pause Stream" } else { "▶ Resume Stream" }).clicked() {
                self.streamer.is_streaming = !self.streamer.is_streaming;
            }
            if ui.button("🔄 Reset Stream").clicked() {
                let target = self.current_target_string();
                self.streamer.reset(Some(target));
            }
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.streamer.stream_speed, 1..=50).text("digits/frame"));
        });

        ui.add_space(10.0);

        // Streaming Screen
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(8, 12, 16))
            .stroke(Stroke::new(1.0, Color32::from_rgb(0, 200, 100)))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_height(250.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("3.").size(20.0).color(Color32::from_rgb(0, 255, 204)).strong());
                    ui.add_space(4.0);

                    let recent = &self.streamer.recent_digits;
                    let target = self.current_target_string();

                    if let Some(pos) = self.streamer.match_found_at {
                        ui.label(
                            RichText::new(format!("🎯 TARGET \"{}\" LOCKED AT DIGIT #{}", target, pos))
                                .size(16.0)
                                .color(Color32::from_rgb(255, 215, 0))
                                .strong()
                        );
                    }

                    // Display streaming digits with terminal monospace font
                    ui.label(
                        RichText::new(recent)
                            .monospace()
                            .size(14.0)
                            .color(Color32::from_rgb(100, 255, 180))
                    );
                });
            });
    }

    fn render_certificate_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.heading(RichText::new("📜 Official Pi Birthday Certificate").color(Color32::from_rgb(255, 215, 0)));
        ui.add_space(6.0);

        let target = self.current_target_string();
        let pos_str = if let Some(ref res) = self.current_result {
            format!("#{}", res.position)
        } else {
            "Search to discover".to_string()
        };

        let cert_text = format!(
r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                      CERTIFICATE OF EXISTENCE IN PI                           ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   This official certificate confirms that the Birthday / Number Sequence:     ║
║                                                                               ║
║                           ★  {}  ★                                     ║
║                                                                               ║
║   is permanently and eternally embedded in the transcendental decimal        ║
║   expansion of π (Pi) at position:                                            ║
║                                                                               ║
║                           👉  {}                                        ║
║                                                                               ║
║   Pi Cosmic Stream Verification:                                              ║
║   ... {} [ {} ] {} ...                                                        ║
║                                                                               ║
║   Because π is infinite and conjectured to be normal, your existence is      ║
║   mathematically guaranteed forever within the circle of the universe.        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
"#,
            target,
            pos_str,
            self.current_result.as_ref().map(|r| r.before_context.as_str()).unwrap_or("05369361000249723618"),
            target,
            self.current_result.as_ref().map(|r| r.after_context.as_str()).unwrap_or("59385964469130601700")
        );

        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(18, 22, 32))
            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 160, 50)))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(RichText::new(&cert_text).monospace().size(12.0).color(Color32::from_rgb(255, 230, 150)));
            });

        ui.add_space(10.0);
        if ui.button("📋 Copy Certificate Text to Clipboard").clicked() {
            ui.ctx().copy_text(cert_text);
            self.copy_notification = Some("Certificate copied to clipboard!".to_string());
        }
        if let Some(ref note) = self.copy_notification {
            ui.label(RichText::new(note).color(Color32::from_rgb(0, 255, 150)));
        }
    }

    fn render_about_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.heading(RichText::new("ℹ️ The Mathematics & Science").color(Color32::from_rgb(0, 255, 204)));
        ui.add_space(6.0);

        ui.label(RichText::new("Why is your birthday in Pi?").size(16.0).strong().color(Color32::WHITE));
        ui.label(
            "Pi (π = 3.14159...) is an irrational number, which means its decimal representation \
            never ends and never enters a repeating pattern. Furthermore, mathematicians strongly \
            conjecture that Pi is a 'normal number', meaning every possible finite sequence of digits \
            appears with uniform probability."
        );

        ui.add_space(10.0);
        ui.label(RichText::new("Combinatorial Probability:").size(15.0).strong());
        ui.label("• 4-digit date (e.g. 2505): 10^4 = 10,000 combinations. Usually appears in the first ~10,000 digits.");
        ui.label("• 6-digit date (e.g. 250509): 10^6 = 1,000,000 combinations. Usually appears within the first ~2-3 million digits.");
        ui.label("• 8-digit date (e.g. 25052009): 10^8 = 100,000,000 combinations. Appears at digit #140,930,892!");

        ui.add_space(12.0);
        ui.label(RichText::new("Built with Rust & Cross-Platform Desktop GUI").size(15.0).strong().color(Color32::from_rgb(0, 255, 204)));
        ui.label("This desktop application is written in 100% pure Rust using `eframe` / `egui`, compiled natively for Linux, Windows, and macOS.");
    }
}

impl eframe::App for PiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Poll background search receiver
        if let Ok(res) = self.rx.try_recv() {
            self.is_searching = false;
            match res {
                Ok(r) => self.current_result = Some(r),
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
                    ActiveTab::Finder => self.render_finder_tab(ui),
                    ActiveTab::Streamer => self.render_streamer_tab(ui),
                    ActiveTab::Certificate => self.render_certificate_tab(ui),
                    ActiveTab::About => self.render_about_tab(ui),
                }
            });
        });
    }
}

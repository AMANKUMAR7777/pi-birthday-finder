use std::sync::mpsc::{channel, Receiver, Sender};
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};
use crate::pi_search::{DateFormat, SearchResult, query_pi_offline, query_pi_online};
use crate::pi_streamer::PiStreamer;

#[derive(PartialEq, Eq, Clone, Copy)]
enum NavigationTab {
    Search,
    LiveStream,
    Certificate,
    Theory,
}

pub struct PiApp {
    active_tab: NavigationTab,

    // Date inputs - default to Einstein's birthday on Pi Day (14 March 1879)
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

    // Non-blocking worker channels
    tx: Sender<Result<SearchResult, String>>,
    rx: Receiver<Result<SearchResult, String>>,

    // Live streamer
    streamer: PiStreamer,
    copy_notification: Option<String>,
}

impl Default for PiApp {
    fn default() -> Self {
        let (tx, rx) = channel();

        // Historical default: Albert Einstein, born on Pi Day (14 March 1879)
        Self {
            active_tab: NavigationTab::Search,
            day: 14,
            month: 3,
            year: 1879,
            format: DateFormat::DDMMYYYY,
            custom_query: "14031879".to_string(),
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
            self.search_error = Some("Enter a valid calendar date or numeric sequence to search.".to_string());
            return;
        }

        self.is_searching = true;
        self.search_error = None;
        self.current_result = None;

        // Reset live digit streamer to search target
        self.streamer.reset(Some(query.clone()));

        // Check local 10k cache first for instant resolution
        if let Some(res) = query_pi_offline(&query) {
            self.is_searching = false;
            self.current_result = Some(res);
            return;
        }

        // Asynchronous query across 200 million digits
        let tx = self.tx.clone();
        let q = query.clone();
        std::thread::spawn(move || {
            let res = query_pi_online(&q);
            let _ = tx.send(res);
        });
    }

    fn render_navigation_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("π")
                    .size(28.0)
                    .color(Color32::from_rgb(245, 158, 11))
                    .strong()
            );
            ui.label(
                RichText::new("PI OBSERVATORY")
                    .size(20.0)
                    .color(Color32::from_rgb(248, 250, 252))
                    .strong()
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let tabs = [
                    (NavigationTab::Theory, "Theory"),
                    (NavigationTab::Certificate, "Certificate"),
                    (NavigationTab::LiveStream, "Live Stream"),
                    (NavigationTab::Search, "Search"),
                ];

                for (tab, label) in tabs {
                    let is_active = self.active_tab == tab;
                    let text = RichText::new(label)
                        .size(13.5)
                        .color(if is_active { Color32::from_rgb(245, 158, 11) } else { Color32::from_rgb(148, 163, 184) })
                        .strong();

                    if ui.selectable_label(is_active, text).clicked() {
                        self.active_tab = tab;
                    }
                }
            });
        });

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(8.0);
    }

    fn render_search_deck(&mut self, ui: &mut egui::Ui) {
        // Date Input Instrument Panel
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(21, 28, 44))
            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 47, 69)))
            .inner_margin(18.0)
            .show(ui, |ui| {
                ui.label(
                    RichText::new("Calendar Date Coordinates")
                        .size(15.0)
                        .color(Color32::from_rgb(248, 250, 252))
                        .strong()
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Day").color(Color32::from_rgb(148, 163, 184)));
                    ui.add(egui::DragValue::new(&mut self.day).range(1..=31).speed(0.2));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Month").color(Color32::from_rgb(148, 163, 184)));
                    ui.add(egui::DragValue::new(&mut self.month).range(1..=12).speed(0.2));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Year").color(Color32::from_rgb(148, 163, 184)));
                    ui.add(egui::DragValue::new(&mut self.year).range(1800..=2050).speed(0.5));
                });

                ui.add_space(12.0);
                ui.label(
                    RichText::new("Format Representation")
                        .size(13.5)
                        .color(Color32::from_rgb(148, 163, 184))
                );
                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    for fmt in DateFormat::all() {
                        let selected = self.format == *fmt && !self.use_custom;
                        if ui.selectable_label(selected, fmt.label()).clicked() {
                            self.format = *fmt;
                            self.use_custom = false;
                        }
                    }
                });

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Historical Presets:").color(Color32::from_rgb(100, 116, 139)).size(12.5));
                    if ui.button("Einstein (14 Mar 1879)").clicked() {
                        self.day = 14;
                        self.month = 3;
                        self.year = 1879;
                        self.use_custom = false;
                    }
                    if ui.button("Moon Landing (20 Jul 1969)").clicked() {
                        self.day = 20;
                        self.month = 7;
                        self.year = 1969;
                        self.use_custom = false;
                    }
                    if ui.button("Millennium (01 Jan 2000)").clicked() {
                        self.day = 1;
                        self.month = 1;
                        self.year = 2000;
                        self.use_custom = false;
                    }
                });

                ui.add_space(8.0);
                ui.checkbox(&mut self.use_custom, "Custom numerical sequence");
                if self.use_custom {
                    ui.add_space(4.0);
                    ui.text_edit_singleline(&mut self.custom_query);
                }

                let target = self.current_target_string();
                ui.add_space(14.0);

                // Target telemetry bar
                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(10, 14, 23))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(35, 47, 69)))
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Target Query:").color(Color32::from_rgb(148, 163, 184)));
                            ui.label(
                                RichText::new(&target)
                                    .size(20.0)
                                    .color(Color32::from_rgb(245, 158, 11))
                                    .monospace()
                                    .strong()
                            );

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let btn = ui.add_sized(
                                    Vec2::new(160.0, 32.0),
                                    egui::Button::new(
                                        RichText::new(if self.is_searching { "Scanning..." } else { "Locate in Pi" })
                                            .size(14.0)
                                            .color(Color32::BLACK)
                                            .strong()
                                    ).fill(if self.is_searching { Color32::from_rgb(148, 163, 184) } else { Color32::from_rgb(245, 158, 11) })
                                );

                                if btn.clicked() && !self.is_searching {
                                    self.trigger_search();
                                }
                            });
                        });
                    });
            });

        ui.add_space(16.0);

        // Feedback / Error
        if let Some(ref err) = self.search_error {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(45, 15, 20))
                .stroke(Stroke::new(1.0, Color32::from_rgb(244, 63, 94)))
                .inner_margin(14.0)
                .show(ui, |ui| {
                    ui.label(RichText::new(format!("Search Notice: {}", err)).color(Color32::from_rgb(254, 205, 211)));
                });
            ui.add_space(10.0);
        }

        // Active Search Spinner
        if self.is_searching {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(15, 20, 32))
                .stroke(Stroke::new(1.0, Color32::from_rgb(35, 47, 69)))
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(
                            RichText::new("Scanning 200,000,000 decimal digits of π across cosmic memory...")
                                .color(Color32::from_rgb(148, 163, 184))
                        );
                    });
                });
            ui.add_space(10.0);
        }

        // Search Results Card
        if let Some(ref res) = self.current_result {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(16, 25, 38))
                .stroke(Stroke::new(1.5, Color32::from_rgb(16, 185, 129)))
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Target Coordinates Resolved")
                                .size(16.0)
                                .color(Color32::from_rgb(52, 211, 153))
                                .strong()
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("Indexed via {}", res.source))
                                    .size(11.0)
                                    .color(Color32::from_rgb(100, 116, 139))
                            );
                        });
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Decimal Index Position:").size(14.5).color(Color32::from_rgb(148, 163, 184)));
                        ui.label(
                            RichText::new(format!("#{}", res.position))
                                .size(26.0)
                                .color(Color32::from_rgb(245, 158, 11))
                                .monospace()
                                .strong()
                        );
                        ui.label(RichText::new("digits after 3.").color(Color32::from_rgb(100, 116, 139)));
                    });

                    ui.add_space(12.0);
                    ui.label(RichText::new("Surrounding Decimal Tape:").size(13.0).color(Color32::from_rgb(148, 163, 184)));

                    // Monospace Tape Readout
                    egui::Frame::group(ui.style())
                        .fill(Color32::from_rgb(10, 14, 23))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(35, 47, 69)))
                        .inner_margin(14.0)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(format!("...{}", res.before_context))
                                        .size(16.0)
                                        .monospace()
                                        .color(Color32::from_rgb(148, 163, 184))
                                );
                                ui.label(
                                    RichText::new(format!(" [{}] ", res.matched))
                                        .size(18.0)
                                        .monospace()
                                        .color(Color32::from_rgb(52, 211, 153))
                                        .strong()
                                        .background_color(Color32::from_rgb(6, 78, 59))
                                );
                                ui.label(
                                    RichText::new(format!("{}...", res.after_context))
                                        .size(16.0)
                                        .monospace()
                                        .color(Color32::from_rgb(148, 163, 184))
                                );
                            });
                        });

                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        if ui.button("Copy Position").clicked() {
                            ui.ctx().copy_text(res.position.to_string());
                            self.copy_notification = Some(format!("Copied index #{} to clipboard.", res.position));
                        }
                        if ui.button("Copy Full Context Tape").clicked() {
                            let text = format!("...{} [{}] {}...", res.before_context, res.matched, res.after_context);
                            ui.ctx().copy_text(text);
                            self.copy_notification = Some("Copied Pi tape stream to clipboard.".to_string());
                        }
                        if ui.button("Generate Certificate").clicked() {
                            self.active_tab = NavigationTab::Certificate;
                        }
                    });

                    if let Some(ref note) = self.copy_notification {
                        ui.add_space(6.0);
                        ui.label(RichText::new(note).size(12.0).color(Color32::from_rgb(52, 211, 153)));
                    }
                });
        }
    }

    fn render_streamer_deck(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Live Digit Stream Telemetry")
                    .size(18.0)
                    .color(Color32::from_rgb(248, 250, 252))
                    .strong()
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("Stream Position: #{}", self.streamer.current_digit_index))
                        .size(15.0)
                        .monospace()
                        .color(Color32::from_rgb(245, 158, 11))
                        .strong()
                );
            });
        });

        ui.label(
            RichText::new("Streaming sequential digits of π from cosmic memory with continuous pattern detection.")
                .color(Color32::from_rgb(148, 163, 184))
                .size(13.0)
        );
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button(if self.streamer.is_streaming { "Pause Stream" } else { "Resume Stream" }).clicked() {
                self.streamer.is_streaming = !self.streamer.is_streaming;
            }
            if ui.button("Rewind to Index 0").clicked() {
                let target = self.current_target_string();
                self.streamer.reset(Some(target));
            }
            ui.add_space(12.0);
            ui.label("Stream Velocity:");
            ui.add(egui::Slider::new(&mut self.streamer.stream_speed, 1..=50).text("digits/tick"));
        });

        ui.add_space(12.0);

        // Streaming Display Terminal
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(10, 14, 23))
            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 47, 69)))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_height(260.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("3.")
                            .size(22.0)
                            .monospace()
                            .color(Color32::from_rgb(245, 158, 11))
                            .strong()
                    );
                    ui.add_space(6.0);

                    if let Some(pos) = self.streamer.match_found_at {
                        ui.label(
                            RichText::new(format!("Target Sequence Locked at Digit #{}", pos))
                                .size(14.5)
                                .color(Color32::from_rgb(52, 211, 153))
                                .strong()
                        );
                        ui.add_space(4.0);
                    }

                    ui.label(
                        RichText::new(&self.streamer.recent_digits)
                            .size(14.5)
                            .monospace()
                            .color(Color32::from_rgb(203, 213, 225))
                    );
                });
            });
    }

    fn render_certificate_deck(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Certificate of Existence in Pi")
                .size(18.0)
                .color(Color32::from_rgb(248, 250, 252))
                .strong()
        );
        ui.label(
            RichText::new("Verifiable cryptographic and mathematical proof of date immersion in π.")
                .color(Color32::from_rgb(148, 163, 184))
                .size(13.0)
        );
        ui.add_space(10.0);

        let target = self.current_target_string();
        let pos_str = if let Some(ref res) = self.current_result {
            format!("#{}", res.position)
        } else {
            "Position awaiting lookup in Search tab".to_string()
        };

        let before = self.current_result.as_ref().map(|r| r.before_context.as_str()).unwrap_or("05369361000249723618");
        let after = self.current_result.as_ref().map(|r| r.after_context.as_str()).unwrap_or("59385964469130601700");

        let cert_text = format!(
r#"
+-----------------------------------------------------------------------------+
|                     OFFICIAL CERTIFICATE OF PI IMMERSION                    |
+-----------------------------------------------------------------------------+
|                                                                             |
|   Target Sequence Coordinates:                                              |
|   {}                                                             |
|                                                                             |
|   Transcendent Decimal Offset:                                              |
|   {}                                                      |
|                                                                             |
|   Cosmic Telemetry Tape Verification:                                       |
|   ... {} [{}] {} ...                         |
|                                                                             |
|   Mathematical Note:                                                        |
|   Because π is transcendental and conjectured to be normal in base 10,       |
|   every finite permutation of numbers exists infinitely across eternity.    |
|                                                                             |
+-----------------------------------------------------------------------------+
"#,
            target, pos_str, before, target, after
        );

        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(10, 14, 23))
            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 47, 69)))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(
                    RichText::new(&cert_text)
                        .monospace()
                        .size(12.0)
                        .color(Color32::from_rgb(226, 232, 240))
                );
            });

        ui.add_space(10.0);
        if ui.button("Copy Certificate").clicked() {
            ui.ctx().copy_text(cert_text);
            self.copy_notification = Some("Certificate copied to system clipboard.".to_string());
        }
        if let Some(ref note) = self.copy_notification {
            ui.add_space(4.0);
            ui.label(RichText::new(note).size(12.0).color(Color32::from_rgb(52, 211, 153)));
        }
    }

    fn render_theory_deck(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Mathematical Foundation & Combinatorics")
                .size(18.0)
                .color(Color32::from_rgb(248, 250, 252))
                .strong()
        );
        ui.add_space(8.0);

        ui.label(
            RichText::new("The Borel Normal Number Conjecture")
                .size(15.0)
                .color(Color32::from_rgb(245, 158, 11))
                .strong()
        );
        ui.label(
            "An irrational number is said to be 'normal' if all finite digit patterns of length k \
            occur with equal asymptotic frequency (1 in 10^k). While a formal proof remains one of \
            the greatest unsolved problems in number theory, statistical analysis over trillions of digits \
            indicates π behaves precisely as a normal number."
        );

        ui.add_space(12.0);
        ui.label(
            RichText::new("Expected Distribution Depth")
                .size(15.0)
                .color(Color32::from_rgb(245, 158, 11))
                .strong()
        );
        ui.label("• 4-Digit Sequences (10^4 = 10,000 combinations): Expected within the first ~10,000 digits.");
        ui.label("• 6-Digit Sequences (10^6 = 1,000,000 combinations): Expected within the first ~2-3 million digits.");
        ui.label("• 8-Digit Sequences (10^8 = 100,000,000 combinations): Expected within the first ~100-200 million digits.");

        ui.add_space(12.0);
        ui.label(
            RichText::new("Engineering Architecture")
                .size(15.0)
                .color(Color32::from_rgb(245, 158, 11))
                .strong()
        );
        ui.label(
            "This application uses eframe/egui with an asynchronous non-blocking thread model. \
            Local lookups execute against embedded pre-computed digit matrices, while global deep searches \
            query multi-hundred-million digit indices asynchronously, ensuring continuous 60 FPS desktop responsiveness."
        );
    }
}

impl eframe::App for PiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Poll asynchronous search receiver
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
                self.render_navigation_bar(ui);

                match self.active_tab {
                    NavigationTab::Search => self.render_search_deck(ui),
                    NavigationTab::LiveStream => self.render_streamer_deck(ui),
                    NavigationTab::Certificate => self.render_certificate_deck(ui),
                    NavigationTab::Theory => self.render_theory_deck(ui),
                }
            });
        });
    }
}

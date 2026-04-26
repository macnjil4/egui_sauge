//! Full showcase of the egui_sauge design system: theme tokens at the top,
//! every component below. Run with: `cargo run --example showcase`.

use eframe::egui;
use egui::{Color32, RichText, Sense, Stroke, TextStyle, Vec2, vec2};
use egui_sauge::components::{
    Accordion, Alert, Avatar, AvatarGroup, AvatarSize, Badge, BadgeTone, Breadcrumb, Button,
    ButtonSize, Card, Checkbox, CodeBlock, Column, ConfirmDialog, Dialog, Drawer, EmptyState,
    InputField, Kbd, KeyValue, Level, LogLevel, LogLine, MenuItem, NavItem, NumberField,
    PageHeader, Pagination, ProgressBar, RadioGroup, RadioOption, Section, SelectField, Skeleton,
    SortState, Spinner, Stat, StatusDot, StatusLevel, SubMenu, Switch, Table, Tabs, Tag, Toasts,
    TooltipExt, Trend,
};
use egui_sauge::{
    Density, Elevation, Icon, Locale, Palette, RADIUS, SPACING, apply_theme_with, install_fonts,
    set_locale, set_reduce_motion,
};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1120.0, 860.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui_sauge — showcase",
        options,
        Box::new(|cc| {
            install_fonts(&cc.egui_ctx);
            apply_theme_with(&cc.egui_ctx, &Palette::light(), Density::Comfortable);
            Ok(Box::new(Showcase::default()) as Box<dyn eframe::App>)
        }),
    )
}

struct Showcase {
    palette: Palette,
    density: Density,
    locale: Locale,
    dark: bool,
    compact: bool,
    spacious: bool,
    reduce_motion: bool,

    // Input demo state
    email: String,
    email_error: bool,
    password: String,
    search: String,
    region: Region,
    notifications: bool,
    tag_closed: bool,
    alert_dismissed: bool,
    dialog_open: bool,
    confirm_open: bool,
    toasts: Toasts,

    // Navigation demo state
    nav_route: NavRoute,
    detail_tab: DetailTab,

    // v1.2 demo state
    cb_accept: bool,
    cb_promo: bool,
    cb_indeterminate: bool,
    radio_plan: Plan,
    num_replicas: f64,
    num_threshold: f64,
    table_selection: std::collections::HashSet<usize>,
    table_sort: Option<SortState>,
    page: usize,
    page_size: usize,
    drawer_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Plan {
    #[default]
    Free,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NavRoute {
    Overview,
    Servers,
    Deployments,
    Logs,
    Users,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DetailTab {
    Details,
    Logs,
    Permissions,
    Activity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Region {
    #[default]
    Europe,
    NorthAmerica,
    AsiaPacific,
}

impl Default for Showcase {
    fn default() -> Self {
        Self {
            palette: Palette::light(),
            density: Density::Comfortable,
            locale: Locale::En,
            dark: false,
            compact: false,
            spacious: false,
            reduce_motion: false,
            email: "alice@example.com".into(),
            email_error: false,
            password: "••••••".into(),
            search: String::new(),
            region: Region::Europe,
            notifications: true,
            tag_closed: false,
            alert_dismissed: false,
            dialog_open: false,
            confirm_open: false,
            toasts: Toasts::new(),
            nav_route: NavRoute::Servers,
            detail_tab: DetailTab::Details,
            cb_accept: false,
            cb_promo: true,
            cb_indeterminate: true,
            radio_plan: Plan::Pro,
            num_replicas: 6.0,
            num_threshold: 75.0,
            table_selection: std::collections::HashSet::new(),
            table_sort: Some(SortState::asc(1)),
            page: 0,
            page_size: 10,
            drawer_open: false,
        }
    }
}

impl eframe::App for Showcase {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.topbar(ui);

        egui::ScrollArea::vertical().show(ui, |ui| {
            Section::new("Theme tokens")
                .description("Colors, type, spacing, radii and shadows.")
                .show(ui, |ui| self.token_card(ui));

            Section::new("Atoms").show(ui, |ui| {
                self.show_buttons(ui);
                self.show_status_atoms(ui);
                self.show_feedback(ui);
            });

            Section::new("Inputs").show(ui, |ui| self.show_inputs(ui));

            Section::new("Data & content").show(ui, |ui| self.show_data(ui));

            Section::new("Feedback surfaces").show(ui, |ui| self.show_alerts(ui));

            Section::new("Overlays")
                .description("Dialog, tooltips, toasts.")
                .show(ui, |ui| self.show_overlays(ui));

            Section::new("Navigation & layout")
                .description("Sidebar, tabs, breadcrumb, page header.")
                .show(ui, |ui| self.show_navigation(ui));

            Section::new("Logs & data")
                .description("LogLine, KeyValue, Skeleton.")
                .show(ui, |ui| self.show_logs_and_data(ui));

            Section::new("v1.2 — Forms, Data, Layout")
                .description("Checkbox / RadioGroup / NumberField / Avatar / Drawer / Accordion / Table / Pagination.")
                .show(ui, |ui| self.show_v12(ui));

            Section::new("Icons").show(ui, |ui| self.show_icons(ui));
        });

        // Render the toast stack last so it floats on top.
        self.toasts.show(ui.ctx());

        // Dialog — called from the Overlays section state but rendered here
        // so it overlays everything.
        self.render_dialog(ui.ctx());
    }
}

impl Showcase {
    fn topbar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("topbar").show_inside(ui, |ui| {
            ui.add_space(SPACING.s2);
            ui.horizontal(|ui| {
                Icon::Leaf.show(ui, 22.0, self.palette.brand_default);
                ui.add_space(SPACING.s2);
                ui.label(
                    RichText::new("egui_sauge")
                        .text_style(TextStyle::Name("h2".into()))
                        .color(self.palette.text_primary),
                );
                ui.label(
                    RichText::new("· design system showcase")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let dark_changed = ui.checkbox(&mut self.dark, "Dark").changed();
                    let prev_density = self.density;
                    egui::ComboBox::from_id_salt("density")
                        .selected_text(match self.density {
                            Density::Spacious => "Spacious",
                            Density::Comfortable => "Comfortable",
                            Density::Compact => "Compact",
                        })
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.density, Density::Spacious, "Spacious");
                            ui.selectable_value(
                                &mut self.density,
                                Density::Comfortable,
                                "Comfortable",
                            );
                            ui.selectable_value(&mut self.density, Density::Compact, "Compact");
                        });
                    let density_changed = self.density != prev_density;
                    if dark_changed || density_changed {
                        self.palette = if self.dark {
                            Palette::dark()
                        } else {
                            Palette::light()
                        };
                        // Keep the legacy bool flags coherent for any code below.
                        self.compact = matches!(self.density, Density::Compact);
                        self.spacious = matches!(self.density, Density::Spacious);
                        apply_theme_with(ui.ctx(), &self.palette, self.density);
                    }
                    ui.add_space(SPACING.s2);
                    let prev_locale = self.locale;
                    egui::ComboBox::from_id_salt("locale")
                        .selected_text(match self.locale {
                            Locale::En => "EN",
                            Locale::Fr => "FR",
                            Locale::De => "DE",
                            Locale::Es => "ES",
                        })
                        .width(56.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.locale, Locale::En, "EN — English");
                            ui.selectable_value(&mut self.locale, Locale::Fr, "FR — Français");
                            ui.selectable_value(&mut self.locale, Locale::De, "DE — Deutsch");
                            ui.selectable_value(&mut self.locale, Locale::Es, "ES — Español");
                        });
                    if prev_locale != self.locale {
                        set_locale(ui.ctx(), self.locale);
                    }
                    ui.add_space(SPACING.s2);
                    if ui
                        .checkbox(&mut self.reduce_motion, "Reduce motion")
                        .changed()
                    {
                        set_reduce_motion(ui.ctx(), self.reduce_motion);
                    }
                });
            });
            ui.add_space(SPACING.s2);
        });
    }

    fn token_card(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                // Palette swatches column.
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Palette")
                            .text_style(TextStyle::Name("h3".into()))
                            .color(self.palette.text_primary),
                    );
                    ui.add_space(SPACING.s2);
                    let p = &self.palette;
                    let swatches = [
                        ("bg_app", p.bg_app),
                        ("bg_surface", p.bg_surface),
                        ("bg_surface_alt", p.bg_surface_alt),
                        ("brand_default", p.brand_default),
                        ("brand_hover", p.brand_hover),
                        ("text_primary", p.text_primary),
                        ("text_secondary", p.text_secondary),
                        ("success", p.success),
                        ("warning", p.warning),
                        ("error", p.error),
                        ("info", p.info),
                    ];
                    for (name, color) in swatches {
                        swatch_row(ui, name, color, self.palette.border_subtle);
                    }
                });
                ui.add_space(SPACING.s5);
                // Typography column.
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Typography")
                            .text_style(TextStyle::Name("h3".into()))
                            .color(self.palette.text_primary),
                    );
                    ui.add_space(SPACING.s2);
                    for (sample, style) in [
                        ("Display 40", TextStyle::Name("display".into())),
                        ("Heading 28", TextStyle::Heading),
                        ("H2 20", TextStyle::Name("h2".into())),
                        ("H3 16", TextStyle::Name("h3".into())),
                        ("Body-lg 16", TextStyle::Name("body-lg".into())),
                        ("Body 14 — UI default", TextStyle::Body),
                        ("Small 12", TextStyle::Small),
                        ("fn sauge() { 13 }", TextStyle::Monospace),
                    ] {
                        ui.label(
                            RichText::new(sample)
                                .text_style(style)
                                .color(self.palette.text_primary),
                        );
                    }
                });
                ui.add_space(SPACING.s5);
                // Elevation column.
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Elevation")
                            .text_style(TextStyle::Name("h3".into()))
                            .color(self.palette.text_primary),
                    );
                    ui.add_space(SPACING.s2);
                    for (label, elev) in [
                        ("Flat", Elevation::Flat),
                        ("Card", Elevation::Card),
                        ("Popover", Elevation::Popover),
                        ("Modal", Elevation::Modal),
                    ] {
                        ui.add_space(SPACING.s2);
                        egui::Frame::default()
                            .fill(self.palette.bg_surface)
                            .stroke(Stroke::new(1.0, self.palette.border_subtle))
                            .corner_radius(corner(RADIUS.md))
                            .inner_margin(egui::Margin::same(SPACING.s3 as i8))
                            .shadow(elev.shadow(self.palette.dark_mode))
                            .show(ui, |ui| {
                                ui.set_min_size(vec2(200.0, 34.0));
                                ui.label(
                                    RichText::new(label)
                                        .text_style(TextStyle::Small)
                                        .color(self.palette.text_secondary),
                                );
                            });
                    }
                });
            });
        });
    }

    fn show_buttons(&mut self, ui: &mut egui::Ui) {
        Card::new().title("Buttons").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(Button::primary("Save changes").leading(Icon::Check));
                ui.add(Button::secondary("Cancel"));
                ui.add(Button::ghost("Reset"));
                ui.add(Button::danger("Delete").leading(Icon::Close));
                ui.add(Button::primary("Next").trailing(Icon::ChevronRight));
                ui.add(Button::primary("Disabled").disabled(true));
            });
            ui.add_space(SPACING.s3);
            ui.label(
                RichText::new("Sizes")
                    .text_style(TextStyle::Small)
                    .color(self.palette.text_secondary),
            );
            ui.horizontal(|ui| {
                ui.add(Button::primary("Small").size(ButtonSize::Sm));
                ui.add(Button::primary("Medium"));
                ui.add(Button::primary("Large").size(ButtonSize::Lg));
            });
            ui.add_space(SPACING.s3);
            ui.label(
                RichText::new("Icon buttons")
                    .text_style(TextStyle::Small)
                    .color(self.palette.text_secondary),
            );
            ui.horizontal(|ui| {
                use egui_sauge::components::IconButton;
                ui.add(IconButton::new(Icon::Search).tooltip("Search (⌘K)"));
                ui.add(IconButton::new(Icon::Settings).tooltip("Settings"));
                ui.add(IconButton::new(Icon::Heart).tooltip("Favorite"));
                ui.add(
                    IconButton::new(Icon::Close)
                        .variant(egui_sauge::components::ButtonVariant::Danger),
                );
            });
        });
    }

    fn show_status_atoms(&mut self, ui: &mut egui::Ui) {
        Card::new().title("Status & badges").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Draft"));
                ui.add(Badge::new("Active").tone(BadgeTone::Success));
                ui.add(Badge::new("Warning").tone(BadgeTone::Warning));
                ui.add(Badge::new("Failed").tone(BadgeTone::Error));
                ui.add(Badge::new("Info").tone(BadgeTone::Info));
                ui.add(Badge::new("Brand").tone(BadgeTone::Brand));
                ui.add(
                    Badge::new("3 pending")
                        .tone(BadgeTone::Info)
                        .leading(Icon::Info),
                );
            });
            ui.add_space(SPACING.s3);
            ui.horizontal_wrapped(|ui| {
                if !self.tag_closed {
                    let mut clicked = false;
                    ui.add(
                        Tag::new("rust")
                            .tone(BadgeTone::Brand)
                            .closable(&mut clicked),
                    );
                    if clicked {
                        self.tag_closed = true;
                    }
                }
                ui.add(Tag::new("production"));
                ui.add(Tag::new("on-call").tone(BadgeTone::Warning));
                ui.add(Tag::new("sev-1").tone(BadgeTone::Error));
            });
            ui.add_space(SPACING.s3);
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    StatusDot::new(StatusLevel::Online)
                        .label("api.eu.prod · online")
                        .pulse(),
                );
                ui.add(StatusDot::new(StatusLevel::Degraded).label("api.us.prod · degraded"));
                ui.add(StatusDot::new(StatusLevel::Offline).label("api.ap.prod · offline"));
                ui.add(StatusDot::new(StatusLevel::Idle).label("api.ap.stage · idle"));
            });
            ui.add_space(SPACING.s3);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Press")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.add(Kbd::new("⌘K"));
                ui.label(
                    RichText::new("or")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.add(Kbd::new("Ctrl+K"));
                ui.label(
                    RichText::new("to open the command palette")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
            });
        });
    }

    fn show_feedback(&mut self, ui: &mut egui::Ui) {
        Card::new().title("Progress & loading").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(Spinner::new());
                ui.add(Spinner::new().size(28.0));
                ui.add(Spinner::new().size(36.0).color(self.palette.info));
                ui.label(
                    RichText::new("Loading…")
                        .text_style(TextStyle::Body)
                        .color(self.palette.text_secondary),
                );
            });
            ui.add_space(SPACING.s3);
            ui.add(
                ProgressBar::new(0.42)
                    .label("Deploy pipeline · step 4/9")
                    .show_percent(),
            );
            ui.add_space(SPACING.s2);
            ui.add(
                ProgressBar::new(0.78)
                    .label("Disk usage")
                    .show_percent()
                    .color(self.palette.warning),
            );
            ui.add_space(SPACING.s2);
            ui.add(
                ProgressBar::new(0.95)
                    .label("Memory")
                    .show_percent()
                    .color(self.palette.error),
            );
        });
    }

    fn show_inputs(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    InputField::new(&mut self.email)
                        .label("Email")
                        .placeholder("you@company.com")
                        .helper(if self.email_error {
                            ""
                        } else {
                            "We won't share your email."
                        })
                        .error(if self.email_error {
                            "Use your work email (@company.com)."
                        } else {
                            ""
                        })
                        .leading(Icon::Info)
                        .show(ui);

                    ui.add_space(SPACING.s2);
                    if ui
                        .add(Button::secondary(if self.email_error {
                            "Clear error"
                        } else {
                            "Trigger error"
                        }))
                        .clicked()
                    {
                        self.email_error = !self.email_error;
                    }

                    ui.add_space(SPACING.s3);
                    InputField::new(&mut self.password)
                        .label("Password")
                        .password(true)
                        .leading(Icon::Settings)
                        .show(ui);

                    ui.add_space(SPACING.s3);
                    InputField::new(&mut self.search)
                        .label("Search")
                        .placeholder("Try typing…")
                        .leading(Icon::Search)
                        .trailing(Icon::Close)
                        .show(ui);
                });

                ui.add_space(SPACING.s5);

                ui.vertical(|ui| {
                    let region_label = match self.region {
                        Region::Europe => "Europe (Paris)",
                        Region::NorthAmerica => "North America (Virginia)",
                        Region::AsiaPacific => "Asia Pacific (Tokyo)",
                    };
                    SelectField::new("region")
                        .label("Region")
                        .helper("Where to host your data.")
                        .width(260.0)
                        .show(
                            ui,
                            &mut self.region,
                            region_label,
                            [
                                (Region::Europe, "Europe (Paris)"),
                                (Region::NorthAmerica, "North America (Virginia)"),
                                (Region::AsiaPacific, "Asia Pacific (Tokyo)"),
                            ],
                        );

                    ui.add_space(SPACING.s3);
                    ui.horizontal(|ui| {
                        ui.add(Switch::new(&mut self.notifications));
                        ui.label(
                            RichText::new("Email notifications")
                                .text_style(TextStyle::Body)
                                .color(self.palette.text_primary),
                        );
                    });
                });
            });
        });
    }

    fn show_data(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(
                Stat::new("Active sessions")
                    .value("1,284")
                    .delta("↑ 2.3% · 24 h", Trend::Up),
            );
            ui.add_space(SPACING.s3);
            ui.add(
                Stat::new("Error rate")
                    .value("0.42%")
                    .delta("↓ 0.18 pp", Trend::Down),
            );
            ui.add_space(SPACING.s3);
            ui.add(
                Stat::new("p99 latency")
                    .value("184 ms")
                    .delta("—", Trend::Flat),
            );
            ui.add_space(SPACING.s3);
            ui.add(Stat::new("Monthly cost").value("$4,210"));
        });

        ui.add_space(SPACING.s4);

        Card::new()
            .title("Recent output")
            .subtitle("Tail of today's deploy log.")
            .show(ui, |ui| {
                CodeBlock::new(
                    "2026-04-24T14:02:11Z  INFO  build: cargo build --release\n\
                     2026-04-24T14:02:41Z  INFO  test:  cargo test --all\n\
                     2026-04-24T14:03:09Z  INFO  image: docker build -t sauge:1.8.4 .\n\
                     2026-04-24T14:03:52Z  INFO  push:  pushed sauge:1.8.4 (142 MiB)\n\
                     2026-04-24T14:04:02Z  WARN   k8s:   3 pods taking > 30 s to become ready\n\
                     2026-04-24T14:04:21Z  INFO  k8s:   rollout complete (replicas: 6/6)",
                )
                .header("deploy.log")
                .show(ui);
            });

        ui.add_space(SPACING.s4);

        Card::new().show(ui, |ui| {
            ui.add(
                EmptyState::new(Icon::Leaf, "No alerts right now")
                    .body("Everything is quiet. You'll see open incidents here."),
            );
        });
    }

    fn show_alerts(&mut self, ui: &mut egui::Ui) {
        if !self.alert_dismissed {
            ui.add(
                Alert::new(
                    Level::Info,
                    "You're reading the showcase in light mode. Try the Dark checkbox in the header.",
                )
                .title("New to egui_sauge?")
                .dismiss(&mut self.alert_dismissed),
            );
            ui.add_space(SPACING.s2);
        }
        ui.add(Alert::new(Level::Success, "Build #1,284 finished in 2 m 31 s.").title("Deploy OK"));
        ui.add_space(SPACING.s2);
        ui.add(
            Alert::new(
                Level::Warning,
                "Disk usage on api-eu-3 is above 75%. Provision more or prune old images.",
            )
            .title("Capacity warning"),
        );
        ui.add_space(SPACING.s2);
        ui.add(
            Alert::new(
                Level::Error,
                "Connection refused on api-us-2. Last successful probe 47 s ago.",
            )
            .title("Probe failure"),
        );
    }

    fn show_overlays(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(Button::primary("Open dialog").leading(Icon::Settings))
                    .clicked()
                {
                    self.dialog_open = true;
                }
                if ui
                    .add(Button::secondary("Push info toast").leading(Icon::Info))
                    .clicked()
                {
                    self.toasts.info("Deploy queued for eu-west-1.");
                }
                if ui
                    .add(Button::secondary("Push success toast").leading(Icon::Success))
                    .clicked()
                {
                    self.toasts.success("Merged PR #1284 into main.");
                }
                if ui
                    .add(Button::secondary("Push warning toast").leading(Icon::Warning))
                    .clicked()
                {
                    self.toasts.warning("api-eu-3 running hot (85% CPU).");
                }
                if ui
                    .add(Button::danger("Push error toast").leading(Icon::Error))
                    .clicked()
                {
                    self.toasts
                        .error("Probe failed on api-us-2: connection refused.");
                }
            });

            ui.add_space(SPACING.s3);
            ui.label(
                RichText::new("Hover any button above for its tooltip. Dialog uses Modal shadow; toasts use Popover shadow.")
                    .text_style(TextStyle::Small)
                    .color(self.palette.text_secondary),
            );
        });
    }

    fn show_navigation(&mut self, ui: &mut egui::Ui) {
        Card::new().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                // Sidebar mock.
                ui.vertical(|ui| {
                    ui.set_min_width(220.0);
                    ui.set_max_width(220.0);
                    ui.label(
                        RichText::new("ROUTES")
                            .text_style(TextStyle::Small)
                            .color(self.palette.text_tertiary),
                    );
                    ui.add_space(SPACING.s1);
                    let routes = [
                        (NavRoute::Overview, Icon::Home, "Overview", None),
                        (NavRoute::Servers, Icon::Server, "Servers", Some("12")),
                        (
                            NavRoute::Deployments,
                            Icon::Rocket,
                            "Deployments",
                            Some("3"),
                        ),
                        (NavRoute::Logs, Icon::FileText, "Logs", None),
                        (NavRoute::Users, Icon::Users, "Users", None),
                        (NavRoute::Settings, Icon::Settings, "Settings", None),
                    ];
                    for (route, icon, label, badge) in routes {
                        let mut item = NavItem::new(label)
                            .icon(icon)
                            .selected(self.nav_route == route);
                        if let Some(b) = badge {
                            item = item.badge(b);
                        }
                        if ui.add(item).clicked() {
                            self.nav_route = route;
                        }
                    }
                });

                ui.add_space(SPACING.s4);
                ui.separator();
                ui.add_space(SPACING.s4);

                // Detail panel.
                ui.vertical(|ui| {
                    let crumbs = ["Acme Inc.", "Production", "EU-West", "api-eu-3"];
                    PageHeader::new("api-eu-3")
                        .breadcrumb(&crumbs)
                        .subtitle("Fargate · 2 vCPU · 4 GiB · running")
                        .show(ui, |ui| {
                            // Right-aligned actions; the right-most is primary.
                            if ui
                                .add(Button::primary("Deploy").leading(Icon::Rocket))
                                .clicked()
                            {
                                self.toasts.success("Deploy triggered for api-eu-3.");
                            }
                            ui.add_space(SPACING.s2);
                            let _ = ui
                                .add(Button::secondary("Logs").leading(Icon::FileText))
                                .clicked();
                            ui.add_space(SPACING.s2);
                            // Icon button with themed tooltip.
                            ui.add(egui_sauge::components::IconButton::new(
                                Icon::DotsHorizontal,
                            ))
                            .sauge_tooltip("More actions");
                        });

                    Tabs::new(&mut self.detail_tab)
                        .tab_with_icon(DetailTab::Details, Icon::FileCode, "Details")
                        .tab_with_icon(DetailTab::Logs, Icon::Terminal, "Logs")
                        .tab_with_icon(DetailTab::Permissions, Icon::Shield, "Permissions")
                        .tab_with_icon(DetailTab::Activity, Icon::Activity, "Activity")
                        .show(ui);

                    ui.add_space(SPACING.s3);

                    match self.detail_tab {
                        DetailTab::Details => {
                            KeyValue::new()
                                .item("ID", "srv_01HX2K3M4F8YW")
                                .item("Region", "eu-west-1")
                                .item("Cluster", "api-prod-eu")
                                .item("CPU / RAM", "2 vCPU · 4 GiB")
                                .item("Image", "registry/api:1.8.4")
                                .item("Last deploy", "2026-04-25 10:14")
                                .show(ui);
                            ui.add_space(SPACING.s2);
                            ui.horizontal(|ui| {
                                if ui
                                    .add(Button::danger("Delete server").leading(Icon::Trash))
                                    .clicked()
                                {
                                    self.confirm_open = true;
                                }
                                ui.add_space(SPACING.s2);
                                ui.add(egui_sauge::components::IconButton::new(Icon::Refresh))
                                    .sauge_tooltip("Refresh metrics");
                                ui.add_space(SPACING.s1);
                                ui.add(egui_sauge::components::IconButton::new(Icon::Share))
                                    .sauge_tooltip("Copy share link");
                            });
                        }
                        DetailTab::Logs => {
                            ui.label(
                                RichText::new(
                                    "Switch to the Logs section below for the full demo.",
                                )
                                .text_style(TextStyle::Small)
                                .color(self.palette.text_tertiary),
                            );
                        }
                        DetailTab::Permissions => {
                            ui.add(Alert::new(
                                Level::Info,
                                "Only Owner and Maintainer roles can deploy this server.",
                            ));
                        }
                        DetailTab::Activity => {
                            for (when, who, what) in [
                                ("10:14", "alice", "deployed v1.8.4"),
                                ("09:02", "bob", "updated env vars"),
                                ("yesterday", "ci-bot", "scaled to 6 replicas"),
                            ] {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(when)
                                            .text_style(TextStyle::Small)
                                            .color(self.palette.text_tertiary),
                                    );
                                    ui.label(
                                        RichText::new(who)
                                            .text_style(TextStyle::Body)
                                            .color(self.palette.text_primary),
                                    );
                                    ui.label(
                                        RichText::new(what)
                                            .text_style(TextStyle::Body)
                                            .color(self.palette.text_secondary),
                                    );
                                });
                            }
                        }
                    }
                });
            });
        });

        ui.add_space(SPACING.s3);

        // Standalone breadcrumb demo.
        Card::new().show(ui, |ui| {
            ui.label(
                RichText::new("Breadcrumb (clickable):")
                    .text_style(TextStyle::Small)
                    .color(self.palette.text_secondary),
            );
            ui.add_space(SPACING.s1);
            if let Some(i) = Breadcrumb::new(&["Org", "Project", "Environment", "Service"]).show(ui)
            {
                self.toasts.info(format!("Breadcrumb segment {i} clicked."));
            }
        });

        ui.add_space(SPACING.s3);

        // Menu demo with nested submenus, themed trigger via Button + Popup::menu.
        Card::new()
            .title("Menu & submenus")
            .subtitle(
                "Themed trigger (Button) + Popup::menu, MenuItem rows, and SubMenu \
                 for nested groups.",
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let trigger = ui.add(
                        Button::secondary("More actions")
                            .leading(Icon::DotsHorizontal)
                            .trailing(Icon::ChevronDown),
                    );
                    trigger.clone().sauge_tooltip("Per-row actions");
                    egui::Popup::menu(&trigger).show(|ui| {
                        ui.set_min_width(220.0);
                        if ui.add(MenuItem::with_icon(Icon::Edit, "Rename")).clicked() {
                            self.toasts.info("Rename selected.");
                            ui.close();
                        }
                        if ui
                            .add(MenuItem::with_icon(Icon::Copy, "Duplicate").shortcut("⌘D"))
                            .clicked()
                        {
                            self.toasts.info("Duplicate selected.");
                            ui.close();
                        }
                        if ui
                            .add(MenuItem::with_icon(Icon::Share, "Share link").shortcut("⌘⇧L"))
                            .clicked()
                        {
                            self.toasts.info("Share link copied.");
                            ui.close();
                        }

                        ui.separator();

                        // Nested submenu: Export → CSV / JSON / PDF.
                        SubMenu::with_icon(Icon::Download, "Export").show(ui, |ui| {
                            ui.set_min_width(160.0);
                            if ui.add(MenuItem::with_icon(Icon::FileText, "CSV")).clicked() {
                                self.toasts.success("Exported as CSV.");
                                ui.close();
                            }
                            if ui
                                .add(MenuItem::with_icon(Icon::FileCode, "JSON"))
                                .clicked()
                            {
                                self.toasts.success("Exported as JSON.");
                                ui.close();
                            }
                            if ui.add(MenuItem::with_icon(Icon::File, "PDF")).clicked() {
                                self.toasts.success("Exported as PDF.");
                                ui.close();
                            }
                        });

                        // Two-level nesting: Move to → Region → eu / us / ap.
                        SubMenu::with_icon(Icon::FolderOpen, "Move to").show(ui, |ui| {
                            ui.set_min_width(180.0);
                            SubMenu::with_icon(Icon::Globe, "Region").show(ui, |ui| {
                                ui.set_min_width(180.0);
                                if ui.add(MenuItem::new("eu-west-1 (Paris)")).clicked() {
                                    self.toasts.info("Moved to eu-west-1.");
                                    ui.close();
                                }
                                if ui.add(MenuItem::new("us-east-1 (Virginia)")).clicked() {
                                    self.toasts.info("Moved to us-east-1.");
                                    ui.close();
                                }
                                if ui.add(MenuItem::new("ap-northeast-1 (Tokyo)")).clicked() {
                                    self.toasts.info("Moved to ap-northeast-1.");
                                    ui.close();
                                }
                            });
                            if ui
                                .add(MenuItem::with_icon(Icon::Folder, "Archive"))
                                .clicked()
                            {
                                self.toasts.info("Archived.");
                                ui.close();
                            }
                        });

                        ui.separator();

                        if ui
                            .add(MenuItem::with_icon(Icon::Trash, "Delete").danger())
                            .clicked()
                        {
                            self.confirm_open = true;
                            ui.close();
                        }
                    });
                });
            });
    }

    fn show_logs_and_data(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            // Logs panel.
            ui.vertical(|ui| {
                ui.set_min_width(420.0);
                ui.set_max_width(540.0);
                Card::new().title("Live tail · deploy.log").show(ui, |ui| {
                    ui.add(
                        LogLine::new(LogLevel::Info, "build: cargo build --release")
                            .timestamp("14:02:11"),
                    );
                    ui.add(
                        LogLine::new(LogLevel::Info, "test:  cargo test --all")
                            .timestamp("14:02:41"),
                    );
                    ui.add(
                        LogLine::new(LogLevel::Debug, "image: docker build -t sauge:1.8.4 .")
                            .timestamp("14:03:09"),
                    );
                    ui.add(
                        LogLine::new(LogLevel::Info, "push:  pushed sauge:1.8.4 (142 MiB)")
                            .timestamp("14:03:52"),
                    );
                    ui.add(
                        LogLine::new(
                            LogLevel::Warn,
                            "k8s:   3 pods taking > 30 s to become ready",
                        )
                        .timestamp("14:04:02"),
                    );
                    ui.add(
                        LogLine::new(LogLevel::Error, "probe: api-us-2 connection refused")
                            .timestamp("14:04:12"),
                    );
                    ui.add(
                        LogLine::new(LogLevel::Info, "k8s:   rollout complete (6/6)")
                            .timestamp("14:04:21"),
                    );
                });
            });

            ui.add_space(SPACING.s3);

            // Right column: KeyValue + Skeleton.
            ui.vertical(|ui| {
                Card::new().title("Service details").show(ui, |ui| {
                    KeyValue::new()
                        .item("Status", "Running · 6/6 replicas")
                        .item("Image", "registry/api:1.8.4")
                        .item("Region", "eu-west-1")
                        .item("Cluster", "api-prod-eu")
                        .item("Created", "2025-09-12")
                        .item("Last deploy", "2026-04-25 14:04 UTC")
                        .show(ui);
                });
                ui.add_space(SPACING.s3);
                Card::new()
                    .title("Loading state")
                    .subtitle("Skeleton placeholders while data fetches.")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add(Skeleton::circle(40.0));
                            ui.add_space(SPACING.s2);
                            ui.vertical(|ui| {
                                ui.add(Skeleton::line(180.0));
                                ui.add_space(4.0);
                                ui.add(Skeleton::line(120.0));
                            });
                        });
                        ui.add_space(SPACING.s3);
                        ui.add(Skeleton::block(320.0, 80.0));
                    });
            });
        });
    }

    fn render_dialog(&mut self, ctx: &egui::Context) {
        // ConfirmDialog: turn-key destructive flow.
        if self.confirm_open {
            match ConfirmDialog::new(
                "Supprimer le serveur api-eu-3 ?",
                "Tous les déploiements en cours seront interrompus. Cette action est définitive.",
            )
            .danger()
            .confirm_label("Supprimer")
            .cancel_label("Annuler")
            .show(ctx)
            {
                Some(true) => {
                    self.confirm_open = false;
                    self.toasts.error("Serveur api-eu-3 supprimé.");
                }
                Some(false) => {
                    self.confirm_open = false;
                }
                None => {}
            }
        }

        if !self.dialog_open {
            return;
        }
        let mut dummy = String::from("sauge-prod");
        let mut deleted = false;
        let close = Dialog::new("Delete project?").show(
            ctx,
            |ui, _ctrl| {
                ui.label(
                    "This action cannot be undone. All associated deployments, secrets, \
                     and logs older than 30 days will be permanently removed.",
                );
                ui.add_space(SPACING.s2);
                ui.label(
                    RichText::new("Type the project name to confirm:").text_style(TextStyle::Small),
                );
                ui.add_space(SPACING.s1);
                InputField::new(&mut dummy).show(ui);
            },
            |ui, ctrl| {
                if ui.add(Button::danger("Delete")).clicked() {
                    deleted = true;
                    ctrl.close();
                }
                ui.add_space(SPACING.s2);
                if ui.add(Button::secondary("Cancel")).clicked() {
                    ctrl.close();
                }
            },
        );
        if close {
            self.dialog_open = false;
        }
        if deleted {
            self.toasts.error("Project deleted (demo).");
        }
    }

    fn show_v12(&mut self, ui: &mut egui::Ui) {
        // -- Forms row -----------------------------------------------------
        ui.horizontal_top(|ui| {
            // Checkboxes.
            Card::new().title("Checkboxes").show(ui, |ui| {
                ui.set_min_width(280.0);
                ui.add(Checkbox::with_label(&mut self.cb_accept, "Accept terms"));
                ui.add_space(SPACING.s2);
                ui.add(Checkbox::with_label(
                    &mut self.cb_promo,
                    "Email me about new features",
                ));
                ui.add_space(SPACING.s2);
                let mut master = false; // local "select all" master
                ui.add(
                    Checkbox::new(&mut master)
                        .label("Select all (indeterminate)")
                        .indeterminate(self.cb_indeterminate),
                );
                if master {
                    self.cb_indeterminate = false;
                }
                ui.add_space(SPACING.s2);
                let mut error_demo = true;
                ui.add(Checkbox::with_label(&mut error_demo, "Required (error state)").error(true));
            });

            ui.add_space(SPACING.s3);

            // Radio.
            Card::new().title("Radio group").show(ui, |ui| {
                ui.set_min_width(280.0);
                RadioGroup::new(&mut self.radio_plan)
                    .label("Subscription plan")
                    .helper("Switch any time from billing settings.")
                    .option(RadioOption::new(Plan::Free, "Free").helper("1 user, 100 deploys/mo"))
                    .option(
                        RadioOption::new(Plan::Pro, "Pro")
                            .helper("Unlimited deploys, custom domains"),
                    )
                    .option(
                        RadioOption::new(Plan::Enterprise, "Enterprise")
                            .helper("SSO, audit logs, dedicated support"),
                    )
                    .show(ui);
            });

            ui.add_space(SPACING.s3);

            // Numbers.
            Card::new().title("Number fields").show(ui, |ui| {
                ui.set_min_width(280.0);
                NumberField::new(&mut self.num_replicas)
                    .label("Replicas")
                    .helper("Number of pods to run.")
                    .min(1.0)
                    .max(20.0)
                    .desired_width(220.0)
                    .show(ui);
                ui.add_space(SPACING.s3);
                NumberField::new(&mut self.num_threshold)
                    .label("CPU alert threshold")
                    .suffix("%")
                    .min(0.0)
                    .max(100.0)
                    .step(5.0)
                    .desired_width(220.0)
                    .show(ui);
            });
        });

        ui.add_space(SPACING.s4);

        // -- Avatars row ---------------------------------------------------
        Card::new().title("Avatars").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Sizes:")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.add(Avatar::initials("Alice Martin").size(AvatarSize::Xs));
                ui.add(Avatar::initials("Alice Martin").size(AvatarSize::Sm));
                ui.add(Avatar::initials("Alice Martin").size(AvatarSize::Md));
                ui.add(Avatar::initials("Alice Martin").size(AvatarSize::Lg));
                ui.add(Avatar::initials("Alice Martin").size(AvatarSize::Xl));
            });
            ui.add_space(SPACING.s3);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Status dots:")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.add(
                    Avatar::initials("Alice")
                        .status(self.palette.success)
                        .tooltip("Alice — online"),
                );
                ui.add(
                    Avatar::initials("Bob Carter")
                        .status(self.palette.warning)
                        .tooltip("Bob — away"),
                );
                ui.add(Avatar::icon(Icon::Users).tooltip("Engineering team"));
                ui.add(Avatar::icon(Icon::UserGear).tooltip("System / automation"));
            });
            ui.add_space(SPACING.s3);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Group with overflow:")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                AvatarGroup::new()
                    .max_visible(4)
                    .push(Avatar::initials("Alice Martin"))
                    .push(Avatar::initials("Bob Carter"))
                    .push(Avatar::initials("Chen Liu"))
                    .push(Avatar::initials("Dora Eaton"))
                    .push(Avatar::initials("Erika Schmidt"))
                    .push(Avatar::initials("Felix Brown"))
                    .push(Avatar::initials("Gina Park"))
                    .show(ui);
            });
        });

        ui.add_space(SPACING.s4);

        // -- Accordion -----------------------------------------------------
        Card::new().title("Accordion").show(ui, |ui| {
            Accordion::new("Notification settings")
                .icon(Icon::Bell)
                .subtitle("Email, push, in-app")
                .open()
                .show(ui, |ui| {
                    let mut a = true;
                    let mut b = false;
                    ui.add(Checkbox::with_label(&mut a, "Email · weekly digest"));
                    ui.add(Checkbox::with_label(&mut b, "Email · per-deployment"));
                    ui.add(Checkbox::with_label(&mut b, "Push · only failures"));
                });
            ui.add_space(SPACING.s2);
            Accordion::new("Security")
                .icon(Icon::ShieldCheck)
                .subtitle("2FA, sessions, API keys")
                .show(ui, |ui| {
                    ui.label("(advanced security settings live here)");
                });
            ui.add_space(SPACING.s2);
            Accordion::new("Danger zone")
                .icon(Icon::Trash)
                .show(ui, |ui| {
                    let _ = ui.add(Button::danger("Delete workspace"));
                });
        });

        ui.add_space(SPACING.s4);

        // -- Table + Pagination -------------------------------------------
        Card::new()
            .title("Servers")
            .subtitle("Click headers to sort, checkboxes to multi-select.")
            .show(ui, |ui| {
                #[derive(Clone)]
                struct Row<'a> {
                    name: &'a str,
                    region: &'a str,
                    status: StatusLevel,
                    cpu: f32,
                    mem: f32,
                }
                let all_rows: Vec<Row<'static>> = vec![
                    Row {
                        name: "api-eu-1",
                        region: "eu-west-1",
                        status: StatusLevel::Online,
                        cpu: 42.0,
                        mem: 56.0,
                    },
                    Row {
                        name: "api-eu-2",
                        region: "eu-west-1",
                        status: StatusLevel::Online,
                        cpu: 51.0,
                        mem: 61.0,
                    },
                    Row {
                        name: "api-eu-3",
                        region: "eu-west-1",
                        status: StatusLevel::Degraded,
                        cpu: 86.0,
                        mem: 71.0,
                    },
                    Row {
                        name: "api-us-1",
                        region: "us-east-1",
                        status: StatusLevel::Online,
                        cpu: 38.0,
                        mem: 49.0,
                    },
                    Row {
                        name: "api-us-2",
                        region: "us-east-1",
                        status: StatusLevel::Offline,
                        cpu: 0.0,
                        mem: 0.0,
                    },
                    Row {
                        name: "api-us-3",
                        region: "us-east-1",
                        status: StatusLevel::Online,
                        cpu: 44.0,
                        mem: 52.0,
                    },
                    Row {
                        name: "api-ap-1",
                        region: "ap-northeast-1",
                        status: StatusLevel::Online,
                        cpu: 33.0,
                        mem: 47.0,
                    },
                    Row {
                        name: "api-ap-2",
                        region: "ap-northeast-1",
                        status: StatusLevel::Idle,
                        cpu: 4.0,
                        mem: 12.0,
                    },
                    Row {
                        name: "worker-eu-1",
                        region: "eu-west-1",
                        status: StatusLevel::Online,
                        cpu: 65.0,
                        mem: 70.0,
                    },
                    Row {
                        name: "worker-us-1",
                        region: "us-east-1",
                        status: StatusLevel::Online,
                        cpu: 71.0,
                        mem: 68.0,
                    },
                    Row {
                        name: "queue-eu-1",
                        region: "eu-west-1",
                        status: StatusLevel::Online,
                        cpu: 22.0,
                        mem: 30.0,
                    },
                    Row {
                        name: "cron-eu-1",
                        region: "eu-west-1",
                        status: StatusLevel::Idle,
                        cpu: 1.0,
                        mem: 8.0,
                    },
                ];
                // Apply sort to a local copy.
                let mut rows = all_rows.clone();
                if let Some(s) = self.table_sort {
                    rows.sort_by(|a, b| {
                        let ord = match s.column {
                            1 => a.name.cmp(b.name),
                            2 => a.region.cmp(b.region),
                            3 => a
                                .status
                                .partial_cmp(&b.status)
                                .unwrap_or(std::cmp::Ordering::Equal),
                            4 => a
                                .cpu
                                .partial_cmp(&b.cpu)
                                .unwrap_or(std::cmp::Ordering::Equal),
                            5 => a
                                .mem
                                .partial_cmp(&b.mem)
                                .unwrap_or(std::cmp::Ordering::Equal),
                            _ => std::cmp::Ordering::Equal,
                        };
                        if s.ascending { ord } else { ord.reverse() }
                    });
                }
                let total = rows.len();
                let from = self.page * self.page_size;
                let to = (from + self.page_size).min(total);
                let page_rows = &rows[from..to];

                Table::new(page_rows)
                    .selectable(&mut self.table_selection)
                    .sort(&mut self.table_sort)
                    .column("Name", |ui, r| {
                        let palette = egui_sauge::palette_of(ui.ctx());
                        ui.label(
                            RichText::new(r.name)
                                .text_style(TextStyle::Body)
                                .color(palette.text_primary),
                        );
                    })
                    .last(Column::sortable)
                    .column_text("Region", |r| r.region.to_string())
                    .last(Column::sortable)
                    .column("Status", |ui, r| {
                        ui.add(StatusDot::new(r.status));
                    })
                    .last(Column::sortable)
                    .column_text("CPU", |r| format!("{:.0}%", r.cpu))
                    .last(|c| c.sortable().align_right())
                    .column_text("Mem", |r| format!("{:.0}%", r.mem))
                    .last(|c| c.sortable().align_right())
                    .show(ui);

                ui.add_space(SPACING.s3);
                Pagination::new(&mut self.page, total, &mut self.page_size)
                    .page_sizes([5, 10, 25])
                    .show(ui);
            });

        ui.add_space(SPACING.s4);

        // -- Drawer trigger ------------------------------------------------
        Card::new()
            .title("Drawer (non-blocking side panel)")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(Button::primary("Open filters drawer").leading(Icon::Filter))
                        .clicked()
                    {
                        self.drawer_open = true;
                    }
                    ui.label(
                        RichText::new("Drawer slides in on the right. Underneath stays clickable.")
                            .text_style(TextStyle::Small)
                            .color(self.palette.text_secondary),
                    );
                });
            });

        // The drawer is rendered last (taking precedence over the central
        // area) — see `ui` callback in the main `App::ui`.
        if self.drawer_open
            && Drawer::new("Filters").width(320.0).show(ui, true, |ui| {
                ui.label(
                    RichText::new("Region")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                let mut eu = true;
                let mut us = true;
                let mut ap = false;
                ui.add(Checkbox::with_label(&mut eu, "EU West"));
                ui.add(Checkbox::with_label(&mut us, "US East"));
                ui.add(Checkbox::with_label(&mut ap, "AP Northeast"));
                ui.add_space(SPACING.s3);
                ui.label(
                    RichText::new("Status")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                let mut online = true;
                let mut degraded = false;
                let mut offline = false;
                ui.add(Checkbox::with_label(&mut online, "Online"));
                ui.add(Checkbox::with_label(&mut degraded, "Degraded"));
                ui.add(Checkbox::with_label(&mut offline, "Offline"));
                ui.add_space(SPACING.s4);
                ui.horizontal(|ui| {
                    let _ = ui.add(Button::primary("Apply filters").full_width());
                });
            })
        {
            self.drawer_open = false;
        }
    }

    fn show_icons(&mut self, ui: &mut egui::Ui) {
        Card::new()
            .title("Phosphor icon set")
            .subtitle("Backed by egui-phosphor. Icon::Glyph(...) and Icon::Custom(fn) are escape hatches.")
            .show(ui, |ui| {
                let groups: &[(&str, &[(&str, Icon)])] = &[
                    (
                        "Status",
                        &[
                            ("check", Icon::Check),
                            ("close", Icon::Close),
                            ("success", Icon::Success),
                            ("info", Icon::Info),
                            ("warning", Icon::Warning),
                            ("error", Icon::Error),
                            ("question", Icon::Question),
                        ],
                    ),
                    (
                        "Navigation",
                        &[
                            ("chev-down", Icon::ChevronDown),
                            ("chev-up", Icon::ChevronUp),
                            ("chev-left", Icon::ChevronLeft),
                            ("chev-right", Icon::ChevronRight),
                            ("arrow-up", Icon::ArrowUp),
                            ("arrow-down", Icon::ArrowDown),
                            ("home", Icon::Home),
                            ("menu", Icon::Menu),
                            ("dots-h", Icon::DotsHorizontal),
                            ("dots-v", Icon::DotsVertical),
                        ],
                    ),
                    (
                        "Actions",
                        &[
                            ("plus", Icon::Plus),
                            ("minus", Icon::Minus),
                            ("trash", Icon::Trash),
                            ("edit", Icon::Edit),
                            ("copy", Icon::Copy),
                            ("download", Icon::Download),
                            ("upload", Icon::Upload),
                            ("save", Icon::Save),
                            ("send", Icon::Send),
                            ("refresh", Icon::Refresh),
                            ("search", Icon::Search),
                            ("filter", Icon::Filter),
                            ("share", Icon::Share),
                        ],
                    ),
                    (
                        "Infrastructure",
                        &[
                            ("server", Icon::Server),
                            ("database", Icon::Database),
                            ("cpu", Icon::Cpu),
                            ("cloud", Icon::Cloud),
                            ("globe", Icon::Globe),
                            ("network", Icon::Network),
                            ("activity", Icon::Activity),
                            ("lightning", Icon::Lightning),
                            ("package", Icon::Package),
                            ("rocket", Icon::Rocket),
                            ("recycle", Icon::Recycle),
                        ],
                    ),
                    (
                        "Files & code",
                        &[
                            ("file", Icon::File),
                            ("file-code", Icon::FileCode),
                            ("file-text", Icon::FileText),
                            ("folder", Icon::Folder),
                            ("folder-open", Icon::FolderOpen),
                            ("code", Icon::Code),
                            ("terminal", Icon::Terminal),
                            ("bug", Icon::Bug),
                            ("brain", Icon::Brain),
                        ],
                    ),
                    (
                        "Git",
                        &[
                            ("branch", Icon::GitBranch),
                            ("commit", Icon::GitCommit),
                            ("pr", Icon::GitPullRequest),
                        ],
                    ),
                    (
                        "Security",
                        &[
                            ("lock", Icon::Lock),
                            ("unlock", Icon::Unlock),
                            ("key", Icon::Key),
                            ("shield", Icon::Shield),
                            ("shield-check", Icon::ShieldCheck),
                            ("eye", Icon::Eye),
                            ("eye-slash", Icon::EyeSlash),
                        ],
                    ),
                    (
                        "Comms & people",
                        &[
                            ("bell", Icon::Bell),
                            ("bell-off", Icon::BellOff),
                            ("envelope", Icon::Envelope),
                            ("chat", Icon::Chat),
                            ("user", Icon::User),
                            ("users", Icon::Users),
                            ("user-circle", Icon::UserCircle),
                            ("user-gear", Icon::UserGear),
                        ],
                    ),
                    (
                        "Time & misc",
                        &[
                            ("clock", Icon::Clock),
                            ("calendar", Icon::Calendar),
                            ("hourglass", Icon::Hourglass),
                            ("timer", Icon::Timer),
                            ("settings", Icon::Settings),
                            ("heart", Icon::Heart),
                            ("star", Icon::Star),
                            ("bookmark", Icon::Bookmark),
                            ("flag", Icon::Flag),
                            ("tag", Icon::Tag),
                            ("pin", Icon::Pin),
                            ("link", Icon::Link),
                            ("sun", Icon::Sun),
                            ("moon", Icon::Moon),
                            ("bullet", Icon::Bullet),
                            ("leaf", Icon::Leaf),
                        ],
                    ),
                    (
                        "Escape hatches",
                        &[
                            ("Glyph", Icon::Glyph(egui_phosphor::regular::ROCKET)),
                            ("Custom ★", Icon::Custom(paint_star)),
                        ],
                    ),
                ];
                for (group, items) in groups {
                    ui.add_space(SPACING.s2);
                    ui.label(
                        RichText::new(*group)
                            .text_style(TextStyle::Small)
                            .color(self.palette.text_tertiary),
                    );
                    ui.add_space(SPACING.s1);
                    egui::Grid::new(format!("icons_grid_{group}"))
                        .num_columns(10)
                        .spacing([SPACING.s4, SPACING.s3])
                        .show(ui, |ui| {
                            for (i, (name, icon)) in items.iter().enumerate() {
                                ui.vertical_centered(|ui| {
                                    icon.show(ui, 24.0, self.palette.text_primary);
                                    ui.label(
                                        RichText::new(*name)
                                            .text_style(TextStyle::Small)
                                            .color(self.palette.text_secondary),
                                    );
                                });
                                if (i + 1) % 10 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                }
            });
    }
}

// -- helpers ---------------------------------------------------------------

fn corner(px: f32) -> egui::CornerRadius {
    egui::CornerRadius::same(px.round().clamp(0.0, 255.0) as u8)
}

fn swatch_row(ui: &mut egui::Ui, name: &str, color: Color32, border: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(200.0, 22.0), Sense::hover());
    let chip = egui::Rect::from_min_size(rect.min, Vec2::splat(rect.height()));
    ui.painter().rect(
        chip,
        corner(RADIUS.sm),
        color,
        Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
    let font = egui::FontId::new(12.0, egui::FontFamily::Proportional);
    let [r, g, b, _] = color.to_array();
    let text = format!("{name}   #{r:02X}{g:02X}{b:02X}");
    ui.painter().text(
        egui::pos2(chip.right() + SPACING.s2, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        egui_sauge::palette_of(ui.ctx()).text_primary,
    );
}

/// A 5-point star to demonstrate `Icon::Custom`.
fn paint_star(p: &egui::Painter, rect: egui::Rect, color: Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let outer = rect.width() * 0.45;
    let inner = outer * 0.45;
    let mut pts = Vec::with_capacity(11);
    for i in 0..10 {
        let angle = -std::f32::consts::FRAC_PI_2 + (i as f32) * std::f32::consts::PI / 5.0;
        let r = if i % 2 == 0 { outer } else { inner };
        pts.push(egui::pos2(cx + angle.cos() * r, cy + angle.sin() * r));
    }
    p.add(egui::Shape::closed_line(
        pts,
        Stroke::new((rect.width() * 0.10).max(1.0), color),
    ));
}

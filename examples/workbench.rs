//! IDE-style workbench layout demo.
//!
//! Composes the v2.0 layout primitives: `Workbench` shell with topbar,
//! left activity rail, resizable file tree, editor tabs, central editor
//! area, bottom terminal, and a status bar. Run with:
//!
//! ```text
//! cargo run --example workbench
//! ```

use eframe::egui;
use egui::{Color32, FontId, RichText, Stroke, TextStyle};
use egui_sauge::components::{
    ActivityBar, ActivityItem, Button, ButtonSize, Card, Checkbox, CodeBlock, EditorTab,
    EditorTabAction, EditorTabs, IconButton, KeyValue, LogLevel, LogLine, StatusBar, StatusDot,
    StatusLevel, TooltipExt, TreeNode, TreeView, Workbench,
};
use egui_sauge::{
    Density, Icon, Locale, Palette, SPACING, apply_theme_with, install_fonts, set_locale,
};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui_sauge — workbench",
        options,
        Box::new(|cc| {
            install_fonts(&cc.egui_ctx);
            apply_theme_with(&cc.egui_ctx, &Palette::dark(), Density::Compact);
            set_locale(&cc.egui_ctx, Locale::En);
            Ok(Box::new(App::default()) as Box<dyn eframe::App>)
        }),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeftPanel {
    Project,
    Commit,
    Structure,
    Services,
    Problems,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RightPanel {
    Cargo,
    Build,
    AiAssist,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FileNode {
    Folder(&'static str),
    File(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DocId {
    Readme,
    Spec,
    Changelog,
    Cargo,
}

struct App {
    palette: Palette,
    density: Density,
    dark: bool,

    left: Option<LeftPanel>,
    right: Option<RightPanel>,
    bottom_open: bool,

    file_selected: Option<FileNode>,
    doc_open: Vec<DocId>,
    doc_active: DocId,
    doc_modified: std::collections::HashSet<DocId>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            palette: Palette::dark(),
            density: Density::Compact,
            dark: true,
            left: Some(LeftPanel::Project),
            right: Some(RightPanel::Cargo),
            bottom_open: true,
            file_selected: Some(FileNode::File("README.md")),
            doc_open: vec![DocId::Readme, DocId::Spec, DocId::Changelog, DocId::Cargo],
            doc_active: DocId::Changelog,
            doc_modified: [DocId::Changelog].into_iter().collect(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Workbench::begin(ui)
            .top(|ui| self.topbar(ui))
            .status(|ui| {
                StatusBar::show(
                    ui,
                    |ui| {
                        StatusBar::segment(ui, Some(Icon::GitBranch), "main");
                        StatusBar::segment(ui, None, "8 changed files");
                        ui.add(StatusDot::new(StatusLevel::Online).label("Cargo Check"));
                    },
                    |_ui| {},
                    |ui| {
                        StatusBar::segment(ui, None, "UTF-8");
                        StatusBar::segment(ui, None, "LF");
                        StatusBar::segment(ui, None, "Ln 142, Col 8");
                        StatusBar::segment(ui, Some(Icon::Code), "Rust");
                    },
                );
            })
            .left_activity(|ui| {
                ActivityBar::new(&mut self.left)
                    .item(
                        ActivityItem::new(LeftPanel::Project, Icon::Folder).tooltip("Project · ⌘1"),
                    )
                    .item(
                        ActivityItem::new(LeftPanel::Commit, Icon::GitCommit)
                            .tooltip("Commit · ⌘0")
                            .badge(true),
                    )
                    .item(
                        ActivityItem::new(LeftPanel::Structure, Icon::Code)
                            .tooltip("Structure · ⌘7"),
                    )
                    .item(
                        ActivityItem::new(LeftPanel::Services, Icon::Server)
                            .tooltip("Services · ⌥8"),
                    )
                    .item(
                        ActivityItem::new(LeftPanel::Problems, Icon::Warning)
                            .tooltip("Problems · ⌥6"),
                    )
                    .bottom_item(
                        ActivityItem::new(LeftPanel::Project, Icon::Settings).tooltip("Settings"),
                    )
                    .show(ui);
            })
            .right_activity(|ui| {
                ActivityBar::new(&mut self.right)
                    .item(ActivityItem::new(RightPanel::Cargo, Icon::Package).tooltip("Cargo · ⌘9"))
                    .item(ActivityItem::new(RightPanel::Build, Icon::Rocket).tooltip("Build · ⌘B"))
                    .item(
                        ActivityItem::new(RightPanel::AiAssist, Icon::Brain)
                            .tooltip("AI Assistant"),
                    )
                    .show(ui);
            })
            .left(left_panel_title(self.left), self.left.is_some(), |ui| {
                self.render_left_panel(ui);
            })
            .right(right_panel_title(self.right), self.right.is_some(), |ui| {
                self.render_right_panel(ui);
            })
            .bottom("Terminal", self.bottom_open, |ui| {
                self.render_terminal(ui);
            })
            .central(|ui| {
                self.render_editor_area(ui);
            });
    }
}

impl App {
    fn topbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            Icon::Leaf.show(ui, 18.0, self.palette.brand_default);
            ui.add_space(SPACING.s2);
            ui.label(
                RichText::new("egui_sauge")
                    .text_style(TextStyle::Body)
                    .color(self.palette.text_primary),
            );
            ui.label(
                RichText::new("·")
                    .text_style(TextStyle::Body)
                    .color(self.palette.text_tertiary),
            );
            ui.label(
                RichText::new("main")
                    .text_style(TextStyle::Body)
                    .color(self.palette.text_secondary),
            );
            ui.add_space(SPACING.s4);
            // Run config button.
            let _ = ui.add(
                Button::ghost("api-prod")
                    .size(ButtonSize::Sm)
                    .leading(Icon::Rocket)
                    .trailing(Icon::ChevronDown),
            );
            ui.add(IconButton::new(Icon::Activity).size(ButtonSize::Sm))
                .sauge_tooltip("Run");
            ui.add(IconButton::new(Icon::Bug).size(ButtonSize::Sm))
                .sauge_tooltip("Debug");
            ui.add(IconButton::new(Icon::Refresh).size(ButtonSize::Sm))
                .sauge_tooltip("Reload");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(IconButton::new(Icon::UserCircle).size(ButtonSize::Sm))
                    .sauge_tooltip("Profile");
                ui.add(IconButton::new(Icon::Settings).size(ButtonSize::Sm))
                    .sauge_tooltip("Settings");
                ui.add(IconButton::new(Icon::Search).size(ButtonSize::Sm))
                    .sauge_tooltip("Search · ⌘⇧F");
                ui.add(IconButton::new(Icon::Bell).size(ButtonSize::Sm))
                    .sauge_tooltip("Notifications");
                if ui
                    .add(
                        IconButton::new(if self.dark { Icon::Sun } else { Icon::Moon })
                            .size(ButtonSize::Sm),
                    )
                    .sauge_tooltip("Toggle theme")
                    .clicked()
                {
                    self.dark = !self.dark;
                    self.palette = if self.dark {
                        Palette::dark()
                    } else {
                        Palette::light()
                    };
                    apply_theme_with(ui.ctx(), &self.palette, self.density);
                }
            });
        });
    }

    fn render_left_panel(&mut self, ui: &mut egui::Ui) {
        match self.left {
            Some(LeftPanel::Project) => render_project_tree(ui, &mut self.file_selected),
            Some(LeftPanel::Commit) => {
                ui.add_space(SPACING.s3);
                ui.label(
                    RichText::new("Changes")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.add_space(SPACING.s2);
                let mut a = true;
                let mut b = true;
                let mut c = false;
                ui.add(Checkbox::with_label(&mut a, "M src/components/mod.rs"));
                ui.add(Checkbox::with_label(&mut b, "M Cargo.toml"));
                ui.add(Checkbox::with_label(&mut c, "?? cible.png"));
                ui.add_space(SPACING.s3);
                let _ = ui.add(Button::primary("Commit").full_width());
            }
            Some(LeftPanel::Structure) => {
                render_structure_tree(ui);
            }
            Some(LeftPanel::Services) => {
                ui.add_space(SPACING.s3);
                ui.label(
                    RichText::new("Run targets")
                        .text_style(TextStyle::Small)
                        .color(self.palette.text_secondary),
                );
                ui.add(StatusDot::new(StatusLevel::Online).label("api-prod"));
                ui.add(StatusDot::new(StatusLevel::Idle).label("worker-eu"));
                ui.add(StatusDot::new(StatusLevel::Offline).label("nightly-build"));
            }
            Some(LeftPanel::Problems) => {
                ui.add_space(SPACING.s3);
                ui.add(LogLine::new(LogLevel::Warn, "unused variable: `x`").timestamp("clippy"));
                ui.add(LogLine::new(LogLevel::Error, "cannot find type `Foo`").timestamp("rustc"));
            }
            None => {}
        }
    }

    fn render_right_panel(&mut self, ui: &mut egui::Ui) {
        match self.right {
            Some(RightPanel::Cargo) => {
                render_cargo_tree(ui);
            }
            Some(RightPanel::Build) => {
                ui.add_space(SPACING.s3);
                ui.add(LogLine::new(LogLevel::Info, "cargo build --release").timestamp("14:02:11"));
                ui.add(
                    LogLine::new(LogLevel::Info, "Compiling egui_sauge v2.0.0")
                        .timestamp("14:02:14"),
                );
                ui.add(
                    LogLine::new(
                        LogLevel::Info,
                        "Finished `release` profile [optimized] target(s) in 18.4s",
                    )
                    .timestamp("14:02:33"),
                );
            }
            Some(RightPanel::AiAssist) => {
                ui.add_space(SPACING.s3);
                ui.label(
                    RichText::new("Ask anything about this codebase…")
                        .text_style(TextStyle::Body)
                        .color(self.palette.text_secondary),
                );
            }
            None => {}
        }
    }

    fn render_editor_area(&mut self, ui: &mut egui::Ui) {
        let palette = self.palette;
        let action = EditorTabs::new(&mut self.doc_active)
            .tab(
                EditorTab::with_icon(DocId::Readme, Icon::FileText, "README.md")
                    .modified(self.doc_modified.contains(&DocId::Readme)),
            )
            .tab(
                EditorTab::with_icon(DocId::Spec, Icon::FileText, "egui_sauge-spec.md")
                    .modified(self.doc_modified.contains(&DocId::Spec)),
            )
            .tab(
                EditorTab::with_icon(DocId::Changelog, Icon::FileText, "CHANGELOG.md")
                    .modified(self.doc_modified.contains(&DocId::Changelog)),
            )
            .tab(
                EditorTab::with_icon(DocId::Cargo, Icon::Package, "Cargo.toml")
                    .modified(self.doc_modified.contains(&DocId::Cargo))
                    .pinned(),
            )
            .show(ui);
        if let Some(EditorTabAction::Closed(id)) = action {
            self.doc_open.retain(|d| d != &id);
            self.doc_modified.remove(&id);
            if self.doc_active == id
                && let Some(first) = self.doc_open.first()
            {
                self.doc_active = first.clone();
            }
        }

        // Editor body — placeholder text per file.
        egui::ScrollArea::vertical()
            .id_salt("editor_body")
            .show(ui, |ui| {
                ui.add_space(SPACING.s3);
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(SPACING.s4 as i8))
                    .show(ui, |ui| match self.doc_active {
                        DocId::Readme => render_readme(ui, &palette),
                        DocId::Spec => render_spec(ui, &palette),
                        DocId::Changelog => render_changelog(ui, &palette),
                        DocId::Cargo => render_cargo(ui, &palette),
                    });
            });
    }

    fn render_terminal(&mut self, ui: &mut egui::Ui) {
        let palette = self.palette;
        egui::Frame::default()
            .fill(palette.bg_app)
            .inner_margin(egui::Margin::same(SPACING.s3 as i8))
            .show(ui, |ui| {
                let lines = [
                    "$ cargo build --release",
                    "    Compiling egui_sauge v2.0.0 (./)",
                    "    Finished `release` profile [optimized] target(s) in 18.41s",
                    "",
                    "$ cargo run --example workbench",
                    "    Compiling egui_sauge v2.0.0 (./)",
                    "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.02s",
                    "     Running `target/debug/examples/workbench`",
                ];
                for line in lines {
                    ui.label(
                        RichText::new(line)
                            .font(FontId::new(12.0, egui::FontFamily::Monospace))
                            .color(palette.text_primary),
                    );
                }
                ui.label(
                    RichText::new("$ █")
                        .font(FontId::new(12.0, egui::FontFamily::Monospace))
                        .color(palette.brand_default),
                );
            });
    }
}

fn left_panel_title(p: Option<LeftPanel>) -> &'static str {
    match p {
        Some(LeftPanel::Project) => "Project",
        Some(LeftPanel::Commit) => "Commit",
        Some(LeftPanel::Structure) => "Structure",
        Some(LeftPanel::Services) => "Services",
        Some(LeftPanel::Problems) => "Problems",
        None => "",
    }
}

fn right_panel_title(p: Option<RightPanel>) -> &'static str {
    match p {
        Some(RightPanel::Cargo) => "Cargo",
        Some(RightPanel::Build) => "Build",
        Some(RightPanel::AiAssist) => "AI Assistant",
        None => "",
    }
}

fn render_project_tree(ui: &mut egui::Ui, selected: &mut Option<FileNode>) {
    use FileNode::{File, Folder};
    TreeView::new(selected, "project_tree")
        .node(
            TreeNode::folder(
                Folder("egui_sauge"),
                "egui_sauge",
                vec![
                    TreeNode::folder(
                        Folder("src"),
                        "src",
                        vec![
                            TreeNode::folder(
                                Folder("components"),
                                "components",
                                vec![
                                    TreeNode::leaf(File("activity_bar.rs"), "activity_bar.rs"),
                                    TreeNode::leaf(File("editor_tabs.rs"), "editor_tabs.rs"),
                                    TreeNode::leaf(File("status_bar.rs"), "status_bar.rs"),
                                    TreeNode::leaf(File("tree.rs"), "tree.rs"),
                                    TreeNode::leaf(File("workbench.rs"), "workbench.rs"),
                                ],
                            )
                            .open(),
                            TreeNode::folder(
                                Folder("theme"),
                                "theme",
                                vec![
                                    TreeNode::leaf(File("apply.rs"), "apply.rs"),
                                    TreeNode::leaf(File("palette.rs"), "palette.rs"),
                                    TreeNode::leaf(File("tokens.rs"), "tokens.rs"),
                                ],
                            ),
                            TreeNode::leaf(File("lib.rs"), "lib.rs"),
                            TreeNode::leaf(File("icons.rs"), "icons.rs"),
                        ],
                    )
                    .open(),
                    TreeNode::folder(
                        Folder("examples"),
                        "examples",
                        vec![
                            TreeNode::leaf(File("minimal.rs"), "minimal.rs"),
                            TreeNode::leaf(File("showcase.rs"), "showcase.rs"),
                            TreeNode::leaf(File("workbench.rs"), "workbench.rs"),
                        ],
                    ),
                    TreeNode::folder(
                        Folder("tests"),
                        "tests",
                        vec![TreeNode::leaf(File("contrast.rs"), "contrast.rs")],
                    ),
                    TreeNode::leaf(File("Cargo.toml"), "Cargo.toml"),
                    TreeNode::leaf(File("README.md"), "README.md"),
                    TreeNode::leaf(File("CHANGELOG.md"), "CHANGELOG.md"),
                    TreeNode::leaf(File("GUIDE.md"), "GUIDE.md"),
                ],
            )
            .open(),
        )
        .show(ui);
}

fn render_structure_tree(ui: &mut egui::Ui) {
    let mut sel: Option<&'static str> = None;
    TreeView::new(&mut sel, "structure")
        .node(
            TreeNode::folder(
                "trait Workbench",
                "trait Workbench",
                vec![
                    TreeNode::leaf("fn new()", "fn new()"),
                    TreeNode::leaf("fn show()", "fn show()"),
                ],
            )
            .open()
            .icon(Icon::Code),
        )
        .node(
            TreeNode::folder(
                "struct ActivityBar",
                "struct ActivityBar",
                vec![
                    TreeNode::leaf("fn new()", "fn new()"),
                    TreeNode::leaf("fn item()", "fn item()"),
                    TreeNode::leaf("fn show()", "fn show()"),
                ],
            )
            .icon(Icon::Code),
        )
        .show(ui);
}

fn render_cargo_tree(ui: &mut egui::Ui) {
    let mut sel: Option<&'static str> = None;
    TreeView::new(&mut sel, "cargo")
        .node(
            TreeNode::folder(
                "egui_sauge",
                "egui_sauge",
                vec![
                    TreeNode::folder(
                        "targets",
                        "targets",
                        vec![
                            TreeNode::leaf("egui_sauge (lib)", "egui_sauge").icon(Icon::Package),
                            TreeNode::leaf("minimal (example)", "minimal").icon(Icon::FileCode),
                            TreeNode::leaf("showcase (example)", "showcase").icon(Icon::FileCode),
                            TreeNode::leaf("workbench (example)", "workbench").icon(Icon::FileCode),
                            TreeNode::leaf("contrast (test)", "contrast").icon(Icon::Bug),
                        ],
                    )
                    .open(),
                    TreeNode::folder(
                        "dependencies",
                        "Dependencies",
                        vec![
                            TreeNode::leaf("egui 0.34", "egui 0.34").icon(Icon::Package),
                            TreeNode::leaf("egui-phosphor 0.12", "egui-phosphor 0.12")
                                .icon(Icon::Package),
                        ],
                    ),
                ],
            )
            .open()
            .icon(Icon::Folder),
        )
        .show(ui);
}

fn render_readme(ui: &mut egui::Ui, palette: &Palette) {
    ui.label(
        RichText::new("# egui_sauge")
            .font(FontId::new(20.0, egui::FontFamily::Proportional))
            .color(palette.text_primary),
    );
    ui.add_space(SPACING.s2);
    ui.label(
        RichText::new(
            "A fresh, natural design system for egui — sage palette, warm neutrals, WCAG AA contrast, and a ready-to-use component library aimed at IT applications.",
        )
        .text_style(TextStyle::Body)
        .color(palette.text_secondary),
    );
    ui.add_space(SPACING.s4);
    Card::new().title("Quickstart").show(ui, |ui| {
        CodeBlock::new(
            "use egui_sauge::{Palette, apply_theme, install_fonts};\n\n\
             fn setup(ctx: &egui::Context) {\n\
                 install_fonts(ctx);\n\
                 apply_theme(ctx, &Palette::light());\n\
             }",
        )
        .header("examples/minimal.rs")
        .show(ui);
    });
}

fn render_spec(ui: &mut egui::Ui, palette: &Palette) {
    ui.label(
        RichText::new("egui_sauge — spec")
            .font(FontId::new(20.0, egui::FontFamily::Proportional))
            .color(palette.text_primary),
    );
    ui.add_space(SPACING.s3);
    ui.label(
        RichText::new("Authoritative implementation document. See README.md for end-user docs.")
            .text_style(TextStyle::Body)
            .color(palette.text_secondary),
    );
}

fn render_changelog(ui: &mut egui::Ui, palette: &Palette) {
    ui.label(
        RichText::new("CHANGELOG")
            .font(FontId::new(20.0, egui::FontFamily::Proportional))
            .color(palette.text_primary),
    );
    ui.add_space(SPACING.s3);
    Card::new().title("[2.0.0] — 2026-04-26").show(ui, |ui| {
        ui.label(
            RichText::new("Added")
                .font(FontId::new(14.0, egui::FontFamily::Proportional))
                .color(palette.text_secondary),
        );
        for line in [
            "ActivityBar — vertical icon nav (left/right of workbench).",
            "StatusBar — bottom multi-slot bar with segments helper.",
            "Splitter — themed resizable side / bottom panel.",
            "TreeView — recursive expand/collapse list (file tree pattern).",
            "EditorTabs — file-style tabs with modified dot + close ×.",
            "Workbench — top-level IDE-shell orchestrator.",
        ] {
            ui.label(
                RichText::new(format!("• {line}"))
                    .text_style(TextStyle::Body)
                    .color(palette.text_primary),
            );
        }
    });
    let _ = Color32::default();
    let _ = Stroke::NONE;
}

fn render_cargo(ui: &mut egui::Ui, _palette: &Palette) {
    KeyValue::new()
        .item("name", "egui_sauge")
        .item("version", "2.0.0")
        .item("edition", "2024")
        .item("rust-version", "1.92")
        .item("license", "Apache-2.0")
        .show(ui);
}

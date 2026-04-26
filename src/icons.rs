//! Icon set, backed by [Phosphor Icons](https://phosphoricons.com/) via the
//! `egui-phosphor` crate. The font is registered automatically by
//! [`crate::install_fonts`]; if you skip that, register it yourself with
//! `egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular)`.
//!
//! The crate ships a curated [`Icon`] enum covering the icons most useful
//! for IT applications (status, navigation, infrastructure, code, security,
//! comms, time…). For anything not in the enum you can use:
//!
//! - [`Icon::Glyph`] — pass any `&'static str` codepoint, e.g.
//!   `Icon::Glyph(egui_phosphor::regular::ROCKET)`.
//! - [`Icon::Custom`] — pass a hand-rolled painter when you need a fully
//!   custom shape.

use egui::{Align2, Color32, FontFamily, FontId, Painter, Rect, Sense, Ui, Vec2, vec2};
use egui_phosphor::regular as ph;

/// Signature for custom icon painters. See [`Icon::Custom`].
pub type IconPainter = fn(&Painter, Rect, Color32);

/// Icon set. Most variants resolve to a Phosphor codepoint; [`Icon::Custom`]
/// lets you inject a painter, [`Icon::Glyph`] lets you reference any
/// Phosphor codepoint directly.
#[derive(Debug, Clone, Copy)]
pub enum Icon {
    // -- Status / feedback ---------------------------------------------------
    /// ✓ Check.
    Check,
    /// ✕ Close / dismiss.
    Close,
    /// ⓘ Info.
    Info,
    /// ▲ Warning triangle.
    Warning,
    /// ⊗ Error (circled X).
    Error,
    /// ◉ Success (circled check).
    Success,
    /// ? Question.
    Question,

    // -- Navigation ----------------------------------------------------------
    /// ˅ Chevron down.
    ChevronDown,
    /// ˄ Chevron up.
    ChevronUp,
    /// ‹ Chevron left.
    ChevronLeft,
    /// › Chevron right.
    ChevronRight,
    /// ↑ Arrow up.
    ArrowUp,
    /// ↓ Arrow down.
    ArrowDown,
    /// ← Arrow left.
    ArrowLeft,
    /// → Arrow right.
    ArrowRight,
    /// ⌂ House / home.
    Home,

    // -- Common actions ------------------------------------------------------
    /// + Plus.
    Plus,
    /// − Minus.
    Minus,
    /// 🗑 Trash.
    Trash,
    /// ✎ Edit (pencil).
    Edit,
    /// ⧉ Copy.
    Copy,
    /// ↓ Download.
    Download,
    /// ↑ Upload.
    Upload,
    /// 💾 Save (floppy).
    Save,
    /// ✉ Send (paper plane tilted).
    Send,
    /// ⟳ Refresh.
    Refresh,
    /// 🔍 Search.
    Search,
    /// ⏷ Filter (funnel).
    Filter,
    /// ↗ Share.
    Share,
    /// ⋯ Three dots horizontal — overflow menu.
    DotsHorizontal,
    /// ⋮ Three dots vertical — row context menu.
    DotsVertical,
    /// ☰ List / menu.
    Menu,

    // -- Infrastructure ------------------------------------------------------
    /// Server / hard drives.
    Server,
    /// Database cylinder.
    Database,
    /// CPU chip.
    Cpu,
    /// Cloud.
    Cloud,
    /// Globe / region.
    Globe,
    /// Network / mesh.
    Network,
    /// Activity / pulse line.
    Activity,
    /// Lightning bolt — fast / energy.
    Lightning,
    /// Package / artifact.
    Package,
    /// Rocket — deploy / launch.
    Rocket,
    /// Recycle / dispose.
    Recycle,

    // -- Files & code --------------------------------------------------------
    /// File.
    File,
    /// File with code glyph.
    FileCode,
    /// File with text lines.
    FileText,
    /// Folder closed.
    Folder,
    /// Folder open.
    FolderOpen,
    /// Code brackets.
    Code,
    /// Terminal window.
    Terminal,
    /// Bug.
    Bug,
    /// Brain (AI / inference).
    Brain,

    // -- Git -----------------------------------------------------------------
    /// Branch glyph.
    GitBranch,
    /// Commit glyph.
    GitCommit,
    /// Pull request glyph.
    GitPullRequest,

    // -- Security ------------------------------------------------------------
    /// Lock closed.
    Lock,
    /// Lock open.
    Unlock,
    /// Key.
    Key,
    /// Shield.
    Shield,
    /// Shield with check.
    ShieldCheck,
    /// Eye open.
    Eye,
    /// Eye crossed.
    EyeSlash,

    // -- Communications ------------------------------------------------------
    /// Bell.
    Bell,
    /// Bell crossed.
    BellOff,
    /// Envelope.
    Envelope,
    /// Chat bubble.
    Chat,

    // -- People --------------------------------------------------------------
    /// Single user.
    User,
    /// Multiple users.
    Users,
    /// User circle (avatar placeholder).
    UserCircle,
    /// User with gear (settings).
    UserGear,

    // -- Time ----------------------------------------------------------------
    /// Clock.
    Clock,
    /// Calendar.
    Calendar,
    /// Hourglass.
    Hourglass,
    /// Timer.
    Timer,

    // -- Misc ----------------------------------------------------------------
    /// Settings (gear).
    Settings,
    /// Heart outline.
    Heart,
    /// Star outline.
    Star,
    /// Bookmark.
    Bookmark,
    /// Flag.
    Flag,
    /// Tag.
    Tag,
    /// Pin / pushpin.
    Pin,
    /// Link / chain.
    Link,
    /// Sun (light theme).
    Sun,
    /// Moon (dark theme).
    Moon,
    /// Filled bullet dot.
    Bullet,
    /// Leaf — the project's signature mark.
    Leaf,

    // -- Escape hatches ------------------------------------------------------
    /// Any [Phosphor regular](https://phosphoricons.com/) codepoint as a
    /// `&'static str`. Pull from [`egui_phosphor::regular`].
    Glyph(&'static str),
    /// Hand-rolled painter, given the same rect any built-in icon uses.
    Custom(IconPainter),
}

impl Icon {
    /// Allocate a square `size × size` rect and paint the icon into it.
    /// Returns the rect so callers can compose further.
    pub fn show(self, ui: &mut Ui, size: f32, color: Color32) -> Rect {
        let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
        self.paint(ui.painter(), rect, color);
        rect
    }

    /// Paint the icon into `rect` using `painter`. The icon is centered in a
    /// square inscribed in `rect`.
    pub fn paint(self, painter: &Painter, rect: Rect, color: Color32) {
        let side = rect.width().min(rect.height());
        let r = Rect::from_center_size(rect.center(), Vec2::splat(side));

        match self {
            Self::Custom(f) => f(painter, r, color),
            other => paint_glyph(painter, r, color, other.codepoint()),
        }
    }

    /// Resolve this icon to its Phosphor codepoint. Returns `"?"` for
    /// [`Icon::Custom`] (which has no glyph). Use [`Self::glyph`] when
    /// you need to differentiate.
    pub(crate) fn codepoint(self) -> &'static str {
        match self {
            // Status
            Self::Check => ph::CHECK,
            Self::Close => ph::X,
            Self::Info => ph::INFO,
            Self::Warning => ph::WARNING,
            Self::Error => ph::WARNING_CIRCLE,
            Self::Success => ph::CHECK_CIRCLE,
            Self::Question => ph::QUESTION,

            // Navigation
            Self::ChevronDown => ph::CARET_DOWN,
            Self::ChevronUp => ph::CARET_UP,
            Self::ChevronLeft => ph::CARET_LEFT,
            Self::ChevronRight => ph::CARET_RIGHT,
            Self::ArrowUp => ph::ARROW_UP,
            Self::ArrowDown => ph::ARROW_DOWN,
            Self::ArrowLeft => ph::ARROW_LEFT,
            Self::ArrowRight => ph::ARROW_RIGHT,
            Self::Home => ph::HOUSE,

            // Actions
            Self::Plus => ph::PLUS,
            Self::Minus => ph::MINUS,
            Self::Trash => ph::TRASH,
            Self::Edit => ph::PENCIL,
            Self::Copy => ph::COPY,
            Self::Download => ph::DOWNLOAD,
            Self::Upload => ph::UPLOAD,
            Self::Save => ph::FLOPPY_DISK,
            Self::Send => ph::PAPER_PLANE_TILT,
            Self::Refresh => ph::ARROW_CLOCKWISE,
            Self::Search => ph::MAGNIFYING_GLASS,
            Self::Filter => ph::FUNNEL,
            Self::Share => ph::SHARE,
            Self::DotsHorizontal => ph::DOTS_THREE,
            Self::DotsVertical => ph::DOTS_THREE_VERTICAL,
            Self::Menu => ph::LIST,

            // Infrastructure
            Self::Server => ph::HARD_DRIVES,
            Self::Database => ph::DATABASE,
            Self::Cpu => ph::CPU,
            Self::Cloud => ph::CLOUD,
            Self::Globe => ph::GLOBE,
            Self::Network => ph::NETWORK,
            Self::Activity => ph::ACTIVITY,
            Self::Lightning => ph::LIGHTNING,
            Self::Package => ph::PACKAGE,
            Self::Rocket => ph::ROCKET,
            Self::Recycle => ph::RECYCLE,

            // Files & code
            Self::File => ph::FILE,
            Self::FileCode => ph::FILE_CODE,
            Self::FileText => ph::FILE_TEXT,
            Self::Folder => ph::FOLDER,
            Self::FolderOpen => ph::FOLDER_OPEN,
            Self::Code => ph::CODE,
            Self::Terminal => ph::TERMINAL_WINDOW,
            Self::Bug => ph::BUG,
            Self::Brain => ph::BRAIN,

            // Git
            Self::GitBranch => ph::GIT_BRANCH,
            Self::GitCommit => ph::GIT_COMMIT,
            Self::GitPullRequest => ph::GIT_PULL_REQUEST,

            // Security
            Self::Lock => ph::LOCK,
            Self::Unlock => ph::LOCK_OPEN,
            Self::Key => ph::KEY,
            Self::Shield => ph::SHIELD,
            Self::ShieldCheck => ph::SHIELD_CHECK,
            Self::Eye => ph::EYE,
            Self::EyeSlash => ph::EYE_SLASH,

            // Comms
            Self::Bell => ph::BELL,
            Self::BellOff => ph::BELL_SLASH,
            Self::Envelope => ph::ENVELOPE,
            Self::Chat => ph::CHAT_CIRCLE,

            // People
            Self::User => ph::USER,
            Self::Users => ph::USERS,
            Self::UserCircle => ph::USER_CIRCLE,
            Self::UserGear => ph::USER_GEAR,

            // Time
            Self::Clock => ph::CLOCK,
            Self::Calendar => ph::CALENDAR,
            Self::Hourglass => ph::HOURGLASS,
            Self::Timer => ph::TIMER,

            // Misc
            Self::Settings => ph::GEAR,
            Self::Heart => ph::HEART,
            Self::Star => ph::STAR,
            Self::Bookmark => ph::BOOKMARK,
            Self::Flag => ph::FLAG,
            Self::Tag => ph::TAG,
            Self::Pin => ph::PUSH_PIN,
            Self::Link => ph::LINK,
            Self::Sun => ph::SUN,
            Self::Moon => ph::MOON,
            Self::Bullet => ph::DOT,
            Self::Leaf => ph::LEAF,

            // Escapes
            Self::Glyph(s) => s,
            Self::Custom(_) => "?",
        }
    }

    /// Like [`Self::codepoint`] but returns `None` for [`Icon::Custom`].
    pub(crate) fn glyph(self) -> Option<&'static str> {
        match self {
            Self::Custom(_) => None,
            other => Some(other.codepoint()),
        }
    }
}

/// Paint a Phosphor codepoint centered in `rect`. Phosphor glyphs are
/// designed to fill their EM box, so we use ~95% of the rect side as font
/// size to leave a hair of breathing room.
fn paint_glyph(painter: &Painter, rect: Rect, color: Color32, codepoint: &str) {
    let size = rect.width() * 0.95;
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        codepoint,
        FontId::new(size, FontFamily::Proportional),
        color,
    );
}

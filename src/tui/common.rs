use ratatui::style::Color;

pub const SECTION_TITLE_FG_COLOR: Color = Color::from_u32(0x282828);
pub const PRIMARY_COLOR: Color = Color::from_u32(0xb8bb26);
pub const SECONDARY_COLOR: Color = Color::from_u32(0x83a598);
pub const MESSAGE_COLOR: Color = Color::from_u32(0xd3869b);
pub const INACTIVE_PANE_TITLE_BG_COLOR: Color = Color::from_u32(0xbdae93);
pub const INACTIVE_PANE_BORDER_COLOR: Color = Color::from_u32(0x928374);
pub const INACTIVE_PANE_SELECTED_COLOR: Color = Color::from_u32(0xfabd2f);
pub const INFO_MESSAGE_COLOR: Color = Color::from_u32(0x83a598);
pub const ERROR_MESSAGE_COLOR: Color = Color::from_u32(0xfb4934);

pub const TITLE: &str = " ecscope ";
pub const MIN_TERMINAL_WIDTH: u16 = 80;
pub const MIN_TERMINAL_HEIGHT: u16 = 30;
pub const UNKNOWN_VALUE: &str = "unknown";

pub const CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS: u64 = 10;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Pane {
    ServicesList,
    ServiceDetails,
    TasksList,
    TaskDetails,
    ContainersList,
    ContainerDetails,
    Help,
}

impl std::fmt::Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pane::ServicesList => write!(f, "sl"),
            Pane::ServiceDetails => write!(f, "sd"),
            Pane::TasksList => write!(f, "tl"),
            Pane::TaskDetails => write!(f, "td"),
            Pane::ContainersList => write!(f, "cl"),
            Pane::ContainerDetails => write!(f, "cd"),
            Pane::Help => write!(f, "h"),
        }
    }
}

pub(super) struct TerminalDimensions {
    pub(super) width: u16,
    pub(super) height: u16,
}

use serde::Deserialize;
use std::path::PathBuf;

pub enum TmuxResponse {
    PaneCapture { target: String, content: String },
    Sessios(Vec<TmuxSession>),
    Windows(Vec<TmuxWindow>),
    Panes(Vec<TmuxPane>),
    NewServer { name: String, socket_path: PathBuf },
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxSession {
    #[serde(alias = "session_id")]
    pub id: String,
    #[serde(alias = "session_index")]
    pub index: u32,
    #[serde(alias = "session_name")]
    pub name: String,
    #[serde(alias = "session_attached")]
    pub attached: bool,
    #[serde(alias = "session_activity")]
    pub activity: u32,
    #[serde(skip)]
    pub windows: Vec<TmuxWindow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxWindow {
    #[serde(alias = "window_id")]
    pub id: String,
    #[serde(alias = "window_index")]
    pub index: u32,
    #[serde(alias = "window_name")]
    pub name: String,
    #[serde(alias = "window_activity")]
    pub activity: u32,
    #[serde(alias = "window_width")]
    pub width: u32,
    #[serde(alias = "window_height")]
    pub height: u32,
    #[serde(alias = "window_cell_width")]
    pub cell_width: u32,
    #[serde(alias = "window_cell_height")]
    pub cell_height: u32,
    #[serde(alias = "window_zoomed_flag")]
    pub is_zoomed: u32,
    #[serde(alias = "window_marked_flag")]
    pub is_marked: u32,
    #[serde(skip)]
    pub panes: Vec<TmuxPane>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxPane {
    #[serde(alias = "pane_id")]
    pub id: String,
    #[serde(alias = "pane_index")]
    pub index: u32,
    #[serde(alias = "pane_width")]
    pub width: u32,
    #[serde(alias = "pane_height")]
    pub height: u32,
    #[serde(alias = "pane_active")]
    pub active: bool,
    #[serde(alias = "pane_current_command")]
    pub current_command: String,
}

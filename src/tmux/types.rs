use anyhow::Result;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub enum TmuxCommand<'a> {
    NewSession {
        session_name: &'a str,
    },
    KillSession {
        session_name: &'a str,
    },
    RenameSession {
        old_session_name: &'a str,
        new_session_name: &'a str,
    },
    CapturePane {
        session_name: &'a str,
        window_index: u64,
        pane_index: u64,
    },
    GetSessions,
    GetWindows {
        session_name: &'a str,
    },
    GetPanes {
        session_name: &'a str,
        window_index: u64,
    },
}

pub enum TmuxResponse {
    PaneCapture(TmuxCapturePane),
    Sessios(Vec<TmuxSession>),
    Windows(Vec<TmuxWindow>),
    Panes(Vec<TmuxPane>),
}

#[derive(Debug, Clone)]
pub struct TmuxCapturePane {
    pub session_name: String,
    pub window_index: u64,
    pub pane_index: u64,
    pub buffer: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxSession {
    #[serde(rename = "session_id")]
    pub id: String,
    // #[serde(rename = "session_index")]
    // pub index: u64,
    #[serde(rename = "session_name")]
    pub name: String,
    #[serde(rename = "session_attached", deserialize_with = "bool_from_int")]
    pub attached: bool,
    #[serde(rename = "session_activity")]
    pub activity: u64,
    #[serde(skip)]
    pub windows: Vec<TmuxWindow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxWindow {
    #[serde(rename = "window_id")]
    pub id: String,
    #[serde(rename = "window_index")]
    pub index: u64,
    #[serde(rename = "window_name")]
    pub name: String,
    #[serde(rename = "window_activity")]
    pub activity: u64,
    #[serde(rename = "window_width")]
    pub width: u64,
    #[serde(rename = "window_height")]
    pub height: u64,
    #[serde(rename = "window_cell_width")]
    pub cell_width: u64,
    #[serde(rename = "window_cell_height")]
    pub cell_height: u64,
    #[serde(rename = "window_zoomed_flag", deserialize_with = "bool_from_int")]
    pub is_zoomed: bool,
    #[serde(rename = "window_marked_flag", deserialize_with = "bool_from_int")]
    pub is_marked: bool,
    #[serde(skip)]
    pub panes: Vec<TmuxPane>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxPane {
    #[serde(rename = "pane_id")]
    pub id: String,
    #[serde(rename = "pane_index")]
    pub index: u64,
    #[serde(rename = "pane_width")]
    pub width: u64,
    #[serde(rename = "pane_height")]
    pub height: u64,
    #[serde(rename = "pane_active", deserialize_with = "bool_from_int")]
    pub active: bool,
    #[serde(rename = "pane_current_command")]
    pub current_command: String,
}

/// helper function for deserializing integer to bool
fn bool_from_int<'a, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'a>,
{
    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        _ => Err(serde::de::Error::custom("invalid value")),
    }
}

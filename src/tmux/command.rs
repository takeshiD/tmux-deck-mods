use crate::tmux::types::{TmuxCapturePane, TmuxPane, TmuxResponse, TmuxSession, TmuxWindow};
use anyhow::{Result, anyhow};
use std::io;
use std::process::{Command, Output};

fn _get_sessions() -> io::Result<Output> {
    Command::new("tmux")
            .args([
                "list-sessions",
                "-F",
                "{\"session_id\": \"#{session_id}\",\"session_name\": \"#{session_name}\",  \"session_attached\": #{session_attached}, \"session_activity\": #{session_activity}}",
            ])
            .output()
}

pub fn get_sessions() -> Result<TmuxResponse> {
    match _get_sessions() {
        Ok(output) if output.status.success() => {
            let content = str::from_utf8(&output.stdout)?;
            let sessions: Vec<TmuxSession> = content
                .lines()
                .map(|line| {
                    serde_json::from_str(line).expect("failed to parse tmux session string")
                })
                .collect();
            Ok(TmuxResponse::Sessios(sessions))
        }
        Ok(output) => Err(anyhow!("failed to get sessions: output={:?}", output)),
        Err(e) => Err(anyhow!("failed to get sessions: occured {}", e)),
    }
}

fn _get_windows(session_name: &str) -> io::Result<Output> {
    Command::new("tmux")
            .args([
                "list-windows",
                "-t",
                session_name,
                "-F",
                "{\"window_id\": \"#{window_id}\", \"window_index\": #{window_index}, \"window_name\": \"#{window_name}\", \"window_activity\": #{window_activity}, \"window_width\": #{window_width}, \"window_height\": #{window_height}, \"window_cell_width\": #{window_cell_width}, \"window_cell_height\": #{window_cell_height}, \"window_zoomed_flag\": #{window_zoomed_flag}, \"window_marked_flag\": #{window_marked_flag}}",
            ])
            .output()
}

pub fn get_windows(session_name: &str) -> Result<TmuxResponse> {
    match _get_windows(session_name) {
        Ok(output) if output.status.success() => {
            let content = str::from_utf8(&output.stdout)?;
            let windows: Vec<TmuxWindow> = content
                .lines()
                .map(|line| {
                    serde_json::from_str(line).expect("failed to parse tmux windows string")
                })
                .collect();
            Ok(TmuxResponse::Windows(windows))
        }
        Ok(output) => Err(anyhow!(
            "failed to get windows info in '{}': output={:?}",
            session_name,
            output
        )),
        Err(e) => Err(anyhow!(
            "failed to get windows info in '{}': occured {}",
            session_name,
            e
        )),
    }
}

fn _get_panes(session_name: &str, window_index: u32) -> io::Result<Output> {
    let target = format!("{}:{}", session_name, window_index);
    Command::new("tmux")
            .args([
                "list-panes",
                "-t",
                &target,
                "-F",
                "{\"pane_id\": \"#{pane_id}\", \"pane_index\": #{pane_index}, \"pane_width\": #{pane_width}, \"pane_height\": #{pane_height}, \"pane_active\": #{pane_active}, \"pane_current_command\": \"#{pane_current_command}\"}",
            ])
            .output()
}

pub fn get_panes(session_name: &str, window_index: u32) -> Result<TmuxResponse> {
    let target = format!("{}:{}", session_name, window_index);
    match _get_panes(session_name, window_index) {
        Ok(output) if output.status.success() => {
            let content = str::from_utf8(&output.stdout)?;
            let panes: Vec<TmuxPane> = content
                .lines()
                .map(|line| serde_json::from_str(line).expect("failed to parse tmux pane string"))
                .collect();
            Ok(TmuxResponse::Panes(panes))
        }
        Ok(output) => Err(anyhow!(
            "failed to get panes info in '{}': output={:?}",
            target,
            output
        )),
        Err(e) => Err(anyhow!(
            "failed to get panes info in '{}': occured {}",
            target,
            e
        )),
    }
}

fn _capture_pane(session_name: &str, window_index: u64, pane_index: u64) -> io::Result<Output> {
    let target = format!("{session_name}:{window_index}.{pane_index}");
    Command::new("tmux")
        .args(["capture-pane", "-e", "-p", "-J", "-t", &target])
        .output()
}

pub fn capture_pane(
    session_name: &str,
    window_index: u64,
    pane_index: u64,
) -> Result<TmuxResponse> {
    let target = format!("{session_name}:{window_index}.{pane_index}");
    match _capture_pane(session_name, window_index, pane_index) {
        Ok(output) if output.status.success() => Ok(TmuxResponse::PaneCapture(TmuxCapturePane {
            session_name: session_name.to_string(),
            window_index,
            pane_index,
            buffer: String::from_utf8_lossy(&output.stdout).to_string(),
        })),
        Ok(output) => Err(anyhow!(
            "failed to capture pane '{}': output={:?}",
            target,
            output
        )),
        Err(e) => Err(anyhow!(
            "failed to capture pane '{}': occured {}",
            target,
            e
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_sessions() {
        let sessions = get_sessions();
        assert!(sessions.is_ok());
    }
    #[test]
    fn test_get_windows() {
        let windows = get_windows("tmux-deck");
        assert!(windows.is_ok());
    }
    #[test]
    fn test_get_panes() {
        let panes = get_panes("tmux-deck", 0);
        assert!(panes.is_ok());
    }
    #[test]
    fn test_capture_pane() {
        let panes = capture_pane("dotfiles", 0, 0);
        assert!(panes.is_ok());
    }
}

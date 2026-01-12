use crate::tmux::types::{TmuxPane, TmuxResponse, TmuxSession, TmuxWindow};
use anyhow::{Result, anyhow};
use std::io;
use std::process::{Command, Output};

pub fn capture_pane(target: &str) -> Result<TmuxResponse> {
    let output = Command::new("tmux")
        .args(["capture-pane", "-e", "-p", "-J", "-t", target])
        .output();
    match output {
        Ok(output) if output.status.success() => Ok(TmuxResponse::PaneCapture {
            target: target.to_string(),
            content: String::from_utf8_lossy(&output.stdout).to_string(),
        }),
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

fn _get_sessions() -> io::Result<Output> {
    Command::new("tmux")
            .args([
                "list-sessions",
                "-F",
                "{\"session_id\": \"#{session_id}\",\"session_index\": \"#{session_index}\",\"session_name\": \"#{session_name}\",  \"session_attached\": \"#{session_attached}\", \"session_activity\": \"#{session_activity}\"}",
            ])
            .output()
}

pub fn get_sessions() -> Result<TmuxResponse> {
    match _get_sessions() {
        Ok(output) if output.status.success() => {
            let content = str::from_utf8(&output.stdout)?;
            let panes: Vec<TmuxPane> = content
                .lines()
                .map(|line| serde_json::from_str(line).expect("failed to parse tmux pane string"))
                .collect();
            Ok(TmuxResponse::Panes(panes))
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
                "{\"window_id\": \"#{window_id}\", \"window_index\": \"#{window_index}\", \"window_name\": \"#{window_name}\", \"window_activity\": \"#{window_activity}\", \"window_width\": \"#{window_width}\", \"window_height\": \"#{window_height}\", \"window_cell_width\": \"#{window_cell_width}\", \"window_cell_height\": \"#{window_cell_height}\"}, \"window_zoomed_flag\": \"#{window_zoomed_flag}\", \"window_marked_flag\": \"#{window_markd_flag}\"}",
            ])
            .output()
}

pub fn get_windows(session_name: &str) -> Result<TmuxResponse> {
    match _get_windows(session_name) {
        Ok(output) if output.status.success() => {
            let content = str::from_utf8(&output.stdout)?;
            let panes: Vec<TmuxPane> = content
                .lines()
                .map(|line| serde_json::from_str(line).expect("failed to parse tmux pane string"))
                .collect();
            Ok(TmuxResponse::Panes(panes))
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
                "\"pane_id\": \"#{pane_id}\", \"pane_index\": \"#{pane_index}\", \"pane_width\": \"#{pane_width}\", \"pane_height\": \"#{pane_height}\", \"pane_active\": \"#{pane_active}\", \"pane_last\": \"#{pane_last}\", \"pane_current_command\": \"#{pane_current_command}\"",
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output};
    struct TestTmuxServer {
        socket_path: PathBuf,
    }
    impl TestTmuxServer {
        fn new(socket_path: &str) -> Result<Self> {
            let output = Command::new("tmux")
                .args(["-S", socket_path, "new"])
                .output();
            match output {
                Ok(output) if output.status.success() => Ok(Self {
                    socket_path: PathBuf::from(socket_path),
                }),
                _ => Err(anyhow!("failed to create tmux server")),
            }
        }
    }
    impl Drop for TestTmuxServer {
        fn drop(&mut self) {
            let socket_path = self.socket_path.to_str().expect("failed to convert str");
            Command::new("tmux")
                .args(["-S", socket_path, "kill-server"])
                .output();
            std::fs::remove_file(socket_path);
        }
    }
    fn enter_tmux_server(sock_name: &str) {
        let output = Command::new("tmux")
            .args(["-S", &format!("/tmp/{sock_name}"), "new"])
            .output();
        match output {
            Ok(output) if output.status.success() => Ok(),
        }
    }
    fn kill_server_exit() {}
    #[test]
    fn test_get_sessions() {}
}

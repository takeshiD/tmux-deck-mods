use anyhow::Result;

mod command;
mod types;

use types::{TmuxResponse};

pub struct Tmux {}

impl Tmux {
    pub fn get_sessions() -> Result<TmuxResponse> {
        unimplemented!()
    }
    pub fn get_windows() -> Result<TmuxResponse> {
        unimplemented!()
    }
    pub fn get_panes() -> Result<TmuxResponse> {
        unimplemented!()
    }
    pub fn rename_session() -> Result<TmuxResponse> {
        unimplemented!()
    }
    pub fn kill_session() -> Result<TmuxResponse> {
        unimplemented!()
    }
    pub fn new_session() -> Result<TmuxResponse> {
        unimplemented!()
    }
    pub fn capture_pane() -> Result<TmuxResponse> {
        unimplemented!()
    }
}

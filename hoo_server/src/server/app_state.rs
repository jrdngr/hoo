use std::sync::mpsc::Sender;

use hoo_base::HooCommand;

#[derive(Debug)]
pub struct AppState {
    pub sender: Sender<HooCommand>,
}

impl AppState {
    pub fn new(sender: &Sender<HooCommand>) -> Self {
        Self {
            sender: sender.clone(),
        }
    }
}

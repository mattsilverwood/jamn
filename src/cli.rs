use std::io;

use clap::{Parser, Subcommand};

use crate::tui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct JamnCli {
    #[command(subcommand)]
    action: Option<JamnActions>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum JamnActions {
    App, // Default, this enum simply exists for ease of adding functionality.
}

impl JamnCli {
    pub fn parse_and_dispatch() -> io::Result<()> {
        let arguments = Self::parse();

        match arguments.action {
            #[allow(clippy::single_match)]
            None | Some(JamnActions::App) => tui::JamnApp::init_and_run()?,
        };

        Ok(())
    }
}

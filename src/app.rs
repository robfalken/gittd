use std::time::Duration;

use crate::{
    events::KeyboardEvent,
    subapps::Branch,
    ui::{restore_terminal, setup_terminal},
};
use anyhow::Result;
use crossterm::event::{self};

use crate::cli::{Cli, Commands};

pub struct App {
    app: Branch,
}

pub enum LoopStatus {
    Break,
    Continue,
    BreakWithMessage(String),
}

impl App {
    pub fn init(args: Cli) -> Result<Self> {
        match args.command {
            Commands::Branch(args) => Ok(App {
                app: Branch::init(args)?,
            }),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = setup_terminal()?;

        loop {
            match self.handle_input() {
                Ok(status) => match status {
                    LoopStatus::Break => {
                        restore_terminal(&mut terminal)?;
                        break;
                    }
                    LoopStatus::BreakWithMessage(msg) => {
                        restore_terminal(&mut terminal)?;
                        println!("{}", msg);
                        break;
                    }
                    LoopStatus::Continue => {
                        terminal.draw(|frame| {
                            self.app.render(frame);
                        })?;
                    }
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self) -> Result<LoopStatus> {
        if event::poll(Duration::from_millis(250))? {
            let ev = event::read()?;

            return match KeyboardEvent::from_event(ev) {
                KeyboardEvent::Q => return Ok(LoopStatus::Break),
                ev => self.app.handle_input(ev),
            };
        }

        Ok(LoopStatus::Continue)
    }
}

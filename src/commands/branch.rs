use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use git2::Repository;
use ratatui::{prelude::*, widgets::*};
use std::{io::Stdout, time::Duration};

use crate::{
    branches::{self, Branch},
    cli::BranchArgs,
    ui,
};

enum Mode {
    CheckOut,
    Delete,
}

enum ShouldExit {
    Yes(Option<String>),
    No,
}

pub fn run(_args: &BranchArgs) -> Result<()> {
    match init() {
        Ok(mut app) => {
            app.run().context("app loop failed")?;
            Ok(())
        }
        Err(err) => {
            println!("{}", err);
            Ok(())
        }
    }
}

pub struct App {
    pub repo: Repository,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    mode: Mode,
}

impl App {
    fn handle_input(&mut self) -> Result<ShouldExit> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                match key.code {
                    KeyCode::Char('q') => return Ok(ShouldExit::Yes(None)),
                    KeyCode::Char('d') => {
                        self.mode = Mode::Delete;
                        return Ok(ShouldExit::No);
                    }
                    KeyCode::Char('0') => {
                        return Ok(self.handle_branch_at(0));
                    }
                    KeyCode::Char('1') => {
                        return Ok(self.handle_branch_at(1));
                    }
                    _ => return Ok(ShouldExit::No),
                }
            }
        }
        Ok(ShouldExit::No)
    }

    fn handle_branch_at(&self, index: usize) -> ShouldExit {
        let branches = branches::list_local(&self.repo);
        let branch = branches.get(index).unwrap();
        if branch.current {
            ShouldExit::No
        } else {
            branches::checkout(&self.repo, &branch.name);
            let msg = format!("Switched to branch '{}'", branch.name);
            ShouldExit::Yes(Some(msg))
        }
    }
}

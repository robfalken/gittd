use anyhow::Result;
use ratatui::Frame;
use ratatui::{prelude::*, widgets::*};

use crate::git_repo::GitRepo;
use crate::{app::LoopStatus, cli::BranchArgs, events::KeyboardEvent};

enum Mode {
    Checkout,
    Delete,
}

pub struct Branch {
    repo: GitRepo,
    mode: Mode,
}

impl Branch {
    pub fn init(args: BranchArgs) -> Result<Self> {
        let repo = GitRepo::init()?;

        let mode = match args.delete {
            true => Mode::Delete,
            false => Mode::Checkout,
        };

        Ok(Branch { repo, mode })
    }

    pub fn render(&self, frame: &mut Frame) {
        let branches = self.repo.list_local_branches();
        let mut items: Vec<Line> = branches
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, branch)| {
                // let head_mark = "* ".red();
                let name = Span::from(branch.name);
                let prefix = if branch.is_head {
                    Span::from(format!("* ")).red()
                } else {
                    Span::from(format!("{} ", index)).yellow()
                };

                Line::from(vec![prefix, name])
            })
            .collect::<Vec<Line>>()
            .to_owned();

        let block = Block::default()
            .title(self.title())
            .borders(Borders::ALL)
            .border_style(self.contextual_border_style())
            .title_style(self.contextual_title_style())
            .padding(Padding {
                top: 1,
                left: 1,
                bottom: 1,
                right: 1,
            });
        items.push(Line::default());
        items.push(footer());
        let list = List::new(items).block(block);
        frame.render_widget(list, frame.size());
    }

    fn contextual_border_style(&self) -> Style {
        match self.mode {
            Mode::Checkout => Style::default().fg(Color::DarkGray),
            Mode::Delete => Style::default().fg(Color::Red),
        }
    }

    fn contextual_title_style(&self) -> Style {
        match self.mode {
            Mode::Checkout => Style::default().fg(Color::Yellow),
            Mode::Delete => Style::default().fg(Color::Red),
        }
    }

    pub fn handle_input(&mut self, event: KeyboardEvent) -> Result<LoopStatus> {
        return match event {
            KeyboardEvent::D => {
                self.mode = Mode::Delete;
                Ok(LoopStatus::Continue)
            }
            KeyboardEvent::Number(n) => match self.mode {
                Mode::Checkout => Ok(self.checkout_branch_at_index(n)),
                Mode::Delete => match self.repo.delete_branch_at(n) {
                    Ok(_) => Ok(LoopStatus::Continue),
                    Err(err) => Ok(LoopStatus::BreakWithMessage(err.to_string())),
                },
            },
            _ => Ok(LoopStatus::Continue),
        };
    }

    fn checkout_branch_at_index(&self, index: usize) -> LoopStatus {
        let branches = self.repo.list_local_branches();
        match branches.get(index) {
            Some(branch) => {
                if branch.is_head {
                    LoopStatus::Continue
                } else {
                    match self.repo.checkout_branch(&branch.name) {
                        Ok(_) => {
                            let msg = format!("Switched to branch '{}'", branch.name);
                            LoopStatus::BreakWithMessage(msg)
                        }
                        Err(err) => LoopStatus::BreakWithMessage(err.to_string()),
                    }
                }
            }
            None => LoopStatus::Continue,
        }
    }

    fn title(&self) -> &'static str {
        match self.mode {
            Mode::Checkout => " Checkout Branch ",
            Mode::Delete => " Delete Branch ",
        }
    }
}

fn footer() -> Line<'static> {
    let parts = vec![Span::from("Press"), " q ".yellow(), Span::from("to quit")];
    Line::from(parts)
}

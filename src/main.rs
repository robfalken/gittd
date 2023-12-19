use app::App;
use clap::Parser;

mod app;
mod branches;
mod cli;
mod events;
mod git_repo;
mod subapps;
mod ui;

fn main() {
    let args = cli::Cli::parse();
    ui::initialize_panic_handler();

    match App::init(args) {
        Ok(mut app) => {
            if let Err(err) = app.run() {
                eprintln!("{}", err);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

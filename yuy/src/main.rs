mod cli;
mod commands;
mod config;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Download { model, quant }) => {
            commands::download::execute(&model, quant).await
        }
        Some(Commands::Run { model, runtime, quant, preset, resume, template }) => {
            commands::run::execute(&model, runtime, quant, preset, resume, template).await
        }
        Some(Commands::List { target }) => {
            commands::list::execute(target).await
        }
        Some(Commands::Info { model, variants }) => {
            commands::info::execute(&model, variants).await
        }
        Some(Commands::Remove { model }) => {
            commands::remove::execute(&model).await
        }
        Some(Commands::Runtime { action }) => {
            commands::runtime::execute(action).await
        }
        Some(Commands::Doctor) => {
            commands::doctor::execute().await
        }
        Some(Commands::Setup) => {
            commands::setup::execute().await
        }
        None => {
            print_banner();
            println!("{}", "Type 'yuy --help' for usage information\n".bright_cyan());
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".bright_red().bold(), e);
        std::process::exit(1);
    }
}

fn print_banner() {
    println!(
        "{}",
        r#"
$$\     $$\                    
\$$\   $$  |                   
 \$$\ $$  /$$\   $$\ $$\   $$\ 
  \$$$$  / $$ |  $$ |$$ |  $$ |
   \$$  /  $$ |  $$ |$$ |  $$ |
    $$ |   $$ |  $$ |$$ |  $$ |
    $$ |   \$$$$$$  |\$$$$$$$ |
    \__|    \______/  \____$$ |
                     $$\   $$ |
                     \$$$$$$  |
                      \______/ 
        "#
        .bright_magenta()
    );
    println!(
        "{}\n",
        "Yuuki CLI v0.1.0 - Official AI Model Manager"
            .bright_cyan()
            .bold()
    );
}

use clap::Parser;
use sora::cli::Run;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match sora::cli::Cmd::parse().run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

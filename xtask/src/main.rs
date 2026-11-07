//! Dev-задачи: `cargo xtask <cmd>`. Команды: bench, fetch-reference <ver>.
mod fetch_reference;
mod mappings;

use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("bench") => run(
            "cargo",
            &["bench", "-p", "ezzmc-benches", "--benches"],
            args.collect(),
        ),
        Some("fetch-reference") => match args.next() {
            Some(ver) => match fetch_reference::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("fetch-reference: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask fetch-reference <версия>");
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("xtask: команды: bench [фильтр] | fetch-reference <версия>");
            ExitCode::FAILURE
        }
    }
}

fn run(prog: &str, base: &[&str], extra: Vec<String>) -> ExitCode {
    let mut cmd = Command::new(prog);
    cmd.args(base);
    if !extra.is_empty() {
        cmd.arg("--").args(extra);
    }
    match cmd.status() {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

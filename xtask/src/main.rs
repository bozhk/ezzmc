//! Dev-задачи: `cargo xtask <cmd>`. Команды: bench, fetch-reference <ver>, gen-reports <ver>, gen-protocol <ver>, gen-registries <ver>, gen-blockstates <ver>, gen-blockprops <ver>, gen-tags <ver>.
mod capture_golden;
mod fetch_reference;
mod gen_blockprops;
mod gen_blockstates;
mod gen_protocol;
mod gen_registries;
mod gen_reports;
mod gen_tags;
mod mappings;
mod server;

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
        Some("gen-protocol") => match args.next() {
            Some(ver) => match gen_protocol::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("gen-protocol: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask gen-protocol <версия> | gen-registries <версия> | gen-blockstates <версия> | capture-golden <версия> <инструмент> | server <версия> [порт] [--reset]");
                ExitCode::FAILURE
            }
        },
        Some("gen-registries") => match args.next() {
            Some(ver) => match gen_registries::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("gen-registries: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask gen-registries <версия> | gen-blockstates <версия> | capture-golden <версия> <инструмент> | server <версия> [порт] [--reset]");
                ExitCode::FAILURE
            }
        },
        Some("gen-blockprops") => match args.next() {
            Some(ver) => match gen_blockprops::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("gen-blockprops: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask gen-blockprops <версия>");
                ExitCode::FAILURE
            }
        },
        Some("gen-tags") => match args.next() {
            Some(ver) => match gen_tags::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("gen-tags: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask gen-tags <версия>");
                ExitCode::FAILURE
            }
        },
        Some("gen-blockstates") => match args.next() {
            Some(ver) => match gen_blockstates::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("gen-blockstates: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask gen-blockstates <версия> | capture-golden <версия> <инструмент> | server <версия> [порт] [--reset]");
                ExitCode::FAILURE
            }
        },
        Some("capture-golden") => match (args.next(), args.next()) {
            (Some(ver), Some(tool)) => match capture_golden::run_cmd(&ver, &tool) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("capture-golden: {e}");
                    ExitCode::FAILURE
                }
            },
            _ => {
                eprintln!("использование: cargo xtask capture-golden <версия> <инструмент> | server <версия> [порт] [--reset]");
                ExitCode::FAILURE
            }
        },
        Some("record") => match (args.next(), args.next(), args.next(), args.next()) {
            (Some(proto), Some(listen), Some(target), Some(out)) => {
                match record_cmd(&proto, &listen, &target, &out) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(e) => {
                        eprintln!("record: {e}");
                        ExitCode::FAILURE
                    }
                }
            }
            _ => {
                eprintln!("использование: cargo xtask record <protocol> <listen host:port> <target host:port> <out.ezrec>");
                ExitCode::FAILURE
            }
        },
        Some("server") => match args.next() {
            Some(ver) => match server::run_cmd(&ver, &args.collect::<Vec<_>>()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("server: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("использование: cargo xtask server <версия> [порт] [--reset]");
                ExitCode::FAILURE
            }
        },
        Some("gen-reports") => match args.next() {
            Some(ver) => match gen_reports::run_cmd(&ver) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("gen-reports: {e}");
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!(
                    "использование: cargo xtask gen-reports <версия> | gen-protocol <версия>"
                );
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!(
                "xtask: команды: bench [фильтр] | fetch-reference <версия> | gen-reports <версия>"
            );
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

/// `cargo xtask record`: прокси-рекордер трафика (TODO-023), одно соединение.
fn record_cmd(proto: &str, listen: &str, target: &str, out: &str) -> Result<(), String> {
    let protocol: u32 = proto
        .parse()
        .map_err(|_| format!("protocol не число: {proto}"))?;
    let f = std::fs::File::create(out).map_err(|e| format!("{out}: {e}"))?;
    eprintln!("record: жду клиента на {listen} → {target}, запись в {out}");
    ezzmc_net::replay::record_proxy(listen, target, protocol, std::io::BufWriter::new(f))
        .map_err(|e| e.to_string())
}

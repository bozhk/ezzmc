//! Логирование (TODO-024): tracing, уровни через `EZZMC_LOG` / `RUST_LOG`,
//! опциональный лог в файл, лог пакетов — цель `packets` (включается флагом).
use std::{fs::File, io, path::Path, sync::Mutex};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

#[derive(Default)]
pub struct LogConfig<'a> {
    /// Директива уровня по умолчанию, если `EZZMC_LOG`/`RUST_LOG` не заданы.
    pub default_level: Option<&'a str>,
    /// Писать копию лога (без цветов) в этот файл.
    pub file: Option<&'a Path>,
    /// Включить подробный лог пакетов (цель `packets` на уровне trace).
    pub log_packets: bool,
}

/// Строит фильтр: окружение > `default_level` > `info`; `packets` — по флагу.
pub fn build_filter(cfg: &LogConfig) -> EnvFilter {
    let base = std::env::var("EZZMC_LOG")
        .or_else(|_| std::env::var("RUST_LOG"))
        .unwrap_or_else(|_| cfg.default_level.unwrap_or("info").to_owned());
    let mut f = EnvFilter::try_new(&base).unwrap_or_else(|_| EnvFilter::new("info"));
    // Лог пакетов огромный — по умолчанию выключен независимо от общего уровня.
    let pk = if cfg.log_packets {
        "packets=trace"
    } else {
        "packets=off"
    };
    if let Ok(d) = pk.parse() {
        f = f.add_directive(d);
    }
    f
}

/// Инициализирует глобальный subscriber. Повторный вызов — ошибка, не паника.
pub fn init(cfg: &LogConfig) -> io::Result<()> {
    let console = fmt::layer()
        .with_target(true)
        .with_filter(build_filter(cfg));
    let file_layer = match cfg.file {
        Some(p) => {
            if let Some(dir) = p.parent() {
                std::fs::create_dir_all(dir)?;
            }
            let f = Mutex::new(File::create(p)?);
            Some(
                fmt::layer()
                    .with_ansi(false)
                    .with_writer(f)
                    .with_filter(build_filter(cfg)),
            )
        }
        None => None,
    };
    tracing_subscriber::registry()
        .with(console)
        .with(file_layer)
        .try_init()
        .map_err(|e| io::Error::other(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packets_off_by_default() {
        let f = build_filter(&LogConfig::default()).to_string();
        assert!(f.contains("packets=off"), "{f}");
        let f = build_filter(&LogConfig {
            log_packets: true,
            ..Default::default()
        })
        .to_string();
        assert!(f.contains("packets=trace"), "{f}");
    }
}

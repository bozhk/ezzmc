//! Модель потоков (TODO-035).
//!
//! * **main** — winit-цикл, игровой тик, рендер (winit требует главный поток на macOS/Windows;
//!   рендер сам не блокируется на I/O — вся тяжёлая работа ниже).
//! * **`ezzmc-net`** — [`Worker`]: блокирующий сокет, разбор/декомпрессия пакетов.
//! * **`ezzmc-io`** — [`Worker`]: загрузка ресурсов/чанков с диска, запись скриншотов.
//! * **`ezzmc-mesh-N`** — rayon-пул ([`mesh_pool`]): генерация мешей, число потоков = ядра − 2.
//!
//! Связь только через каналы crossbeam: команды main→worker (неограниченный, не блокирует кадр),
//! события worker→main (`try_iter` в начале тика).
// Worker/mesh_pool подключаются сетью (гл. 2) и мешером (гл. 5); пока только каркас.
#![allow(dead_code)]

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread::{self, JoinHandle};

/// Поток-исполнитель с парой каналов. `Cmd` — вход, `Evt` — выход.
pub struct Worker<Cmd: Send + 'static, Evt: Send + 'static> {
    cmd_tx: Option<Sender<Cmd>>,
    evt_rx: Receiver<Evt>,
    handle: Option<JoinHandle<()>>,
}

impl<Cmd: Send + 'static, Evt: Send + 'static> Worker<Cmd, Evt> {
    /// Запускает поток `name`. `run` получает приёмник команд и отправитель событий и
    /// должна вернуться, когда канал команд закрыт (владелец уничтожен).
    pub fn spawn(
        name: &str,
        run: impl FnOnce(Receiver<Cmd>, Sender<Evt>) + Send + 'static,
    ) -> std::io::Result<Self> {
        let (cmd_tx, cmd_rx) = unbounded();
        let (evt_tx, evt_rx) = unbounded();
        let handle = thread::Builder::new()
            .name(name.to_owned())
            .spawn(move || run(cmd_rx, evt_tx))?;
        Ok(Self {
            cmd_tx: Some(cmd_tx),
            evt_rx,
            handle: Some(handle),
        })
    }

    /// Отправить команду. `false`, если поток уже завершился.
    pub fn send(&self, cmd: Cmd) -> bool {
        self.cmd_tx.as_ref().is_some_and(|tx| tx.send(cmd).is_ok())
    }

    /// Все накопившиеся события, без блокировки.
    pub fn poll(&self) -> impl Iterator<Item = Evt> + '_ {
        self.evt_rx.try_iter()
    }

    pub fn is_finished(&self) -> bool {
        self.handle.as_ref().is_none_or(JoinHandle::is_finished)
    }
}

impl<Cmd: Send + 'static, Evt: Send + 'static> Drop for Worker<Cmd, Evt> {
    fn drop(&mut self) {
        self.cmd_tx = None; // закрывает канал → поток выходит
        if let Some(h) = self.handle.take() {
            if h.join().is_err() {
                tracing::error!("рабочий поток завершился паникой");
            }
        }
    }
}

/// Число потоков меш-пула: ядра − 2 (main и net остаются свободными), минимум 1.
pub fn mesh_threads(available: usize) -> usize {
    available.saturating_sub(2).max(1)
}

pub fn mesh_pool() -> Result<rayon::ThreadPool, rayon::ThreadPoolBuildError> {
    let n = mesh_threads(thread::available_parallelism().map_or(4, |n| n.get()));
    rayon::ThreadPoolBuilder::new()
        .num_threads(n)
        .thread_name(|i| format!("ezzmc-mesh-{i}"))
        .build()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn worker_roundtrip_and_clean_shutdown() {
        let w: Worker<u32, (String, u32)> = Worker::spawn("ezzmc-test", |rx, tx| {
            let name = thread::current().name().unwrap_or("?").to_owned();
            for c in rx {
                let _ = tx.send((name.clone(), c * 2));
            }
        })
        .unwrap();
        assert!(w.send(21));
        let got = w.evt_rx.recv().unwrap();
        assert_eq!(got, ("ezzmc-test".to_owned(), 42));
        drop(w); // join без зависания
    }

    #[test]
    fn send_after_thread_exit_is_false() {
        let w: Worker<u32, ()> = Worker::spawn("ezzmc-test2", |_rx, _tx| {}).unwrap();
        while !w.is_finished() {
            thread::yield_now();
        }
        assert!(!w.send(1));
    }

    #[test]
    fn mesh_thread_count() {
        assert_eq!(mesh_threads(16), 14);
        assert_eq!(mesh_threads(2), 1);
        assert_eq!(mesh_threads(1), 1);
    }

    #[test]
    fn mesh_pool_runs_named_threads() {
        let pool = mesh_pool().unwrap();
        let name = pool.install(|| thread::current().name().map(str::to_owned));
        assert!(name.is_some_and(|n| n.starts_with("ezzmc-mesh-")));
    }
}

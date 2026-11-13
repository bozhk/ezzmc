//! ezzmc-launcher: выбор аккаунта (offline / Microsoft), настройки и запуск ezzmc или ванильной Java.
mod config;
mod java;
mod launch;

use config::{Account, Config, Kind};
use ezzmc_auth::microsoft;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};
use std::process::{Child, Command};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[allow(clippy::unwrap_used, clippy::all)]
mod ui {
    slint::include_modules!();
}
use ui::{AccountItem, MainWindow};

type Shared = Arc<Mutex<Config>>;

#[derive(Clone, Default)]
struct Runtime {
    children: Arc<Mutex<Vec<Child>>>,
    log: Arc<Mutex<String>>,
    ms_gen: Arc<AtomicU64>,
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn push_log(rt: &Runtime, ui: &Weak<MainWindow>, line: &str) {
    let text = {
        let mut l = lock(&rt.log);
        l.push_str(line);
        l.push('\n');
        if l.len() > 300_000 {
            let cut = l.len() - 200_000;
            let mut cut = cut;
            while !l.is_char_boundary(cut) {
                cut += 1;
            }
            l.drain(..cut);
        }
        l.clone()
    };
    let _ = ui.upgrade_in_event_loop(move |u| u.set_log(text.into()));
}

fn toast(ui: &Weak<MainWindow>, msg: &str) {
    let m = msg.to_string();
    let _ = ui.upgrade_in_event_loop({
        let m = m.clone();
        move |u| u.set_toast(m.into())
    });
    let ui = ui.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(3));
        let _ = ui.upgrade_in_event_loop(move |u| {
            if u.get_toast() == m.as_str() {
                u.set_toast("".into());
            }
        });
    });
}

fn account_items(c: &Config) -> Vec<AccountItem> {
    c.accounts
        .iter()
        .map(|a| AccountItem {
            name: a.name.clone().into(),
            sub: match a.kind {
                Kind::Offline => "Offline".into(),
                Kind::Microsoft => "Microsoft · лицензия".into(),
            },
            ms: a.kind == Kind::Microsoft,
            initial: a
                .name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_default()
                .into(),
        })
        .collect()
}

fn sync_accounts(ui: &MainWindow, c: &Config) {
    ui.set_accounts(ModelRc::new(VecModel::from(account_items(c))));
    ui.set_selected(c.selected as i32);
}

fn settings_to_ui(ui: &MainWindow, c: &Config) {
    let s = &c.settings;
    ui.set_target(s.target);
    ui.set_version(s.java_version.clone().into());
    ui.set_server(s.server.clone().into());
    ui.set_game_dir(s.game_dir.clone().into());
    ui.set_client_path(s.client_path.clone().into());
    ui.set_client_dir(s.client_dir.clone().into());
    ui.set_java_path(s.java_path.clone().into());
    ui.set_ram_min(s.ram_min_mb as f32);
    ui.set_ram_max(s.ram_max_mb as f32);
    ui.set_jvm_args(s.jvm_args.clone().into());
    ui.set_game_args(s.game_args.clone().into());
    ui.set_msa_client_id(s.msa_client_id.clone().into());
    ui.set_close_on_launch(s.close_on_launch);
    ui.set_fullscreen(s.fullscreen);
}

fn ui_to_settings(ui: &MainWindow, c: &mut Config) {
    let round = |v: f32| ((v / 256.0).round() as i32 * 256).max(256);
    let s = &mut c.settings;
    s.target = ui.get_target();
    s.java_version = ui.get_version().trim().to_string();
    s.server = ui.get_server().trim().to_string();
    s.game_dir = ui.get_game_dir().trim().to_string();
    s.client_path = ui.get_client_path().trim().to_string();
    s.client_dir = ui.get_client_dir().trim().to_string();
    let jp = ui.get_java_path().to_string();
    s.java_path = jp.split(" · Java").next().unwrap_or("").trim().to_string();
    s.ram_min_mb = round(ui.get_ram_min());
    s.ram_max_mb = round(ui.get_ram_max()).max(s.ram_min_mb);
    s.jvm_args = ui.get_jvm_args().trim().to_string();
    s.game_args = ui.get_game_args().trim().to_string();
    s.msa_client_id = ui.get_msa_client_id().trim().to_string();
    s.close_on_launch = ui.get_close_on_launch();
    s.fullscreen = ui.get_fullscreen();
    c.selected = (ui.get_selected().max(0) as usize).min(c.accounts.len().saturating_sub(1));
}

fn save(cfg: &Config, ui: &Weak<MainWindow>) {
    if let Err(e) = cfg.save() {
        toast(ui, &format!("Не удалось сохранить конфиг: {e}"));
    }
}

fn set_versions(ui: &MainWindow, list: Vec<String>) {
    let v: Vec<SharedString> = list.into_iter().map(Into::into).collect();
    ui.set_versions(ModelRc::new(VecModel::from(v)));
}

fn merged_versions(game: &std::path::Path, remote: &[String], current: &str) -> Vec<String> {
    let mut v = java::installed_versions(game);
    for r in remote {
        if !v.contains(r) {
            v.push(r.clone());
        }
    }
    if !current.is_empty() && !v.iter().any(|x| x == current) {
        v.insert(0, current.to_string());
    }
    v
}

/// Без окна: `--print-session` печатает JSON `{name, uuid, token, online}` выбранного аккаунта
/// (обновляя токен как при обычном запуске) — для MCP-запуска клиента тем же входом, что и у лаунчера.
fn print_session() -> i32 {
    let mut c = Config::load();
    let idx = c.selected;
    let Some(mut acc) = c.accounts.get(idx).cloned() else {
        eprintln!("нет аккаунта");
        return 1;
    };
    match launch::resolve_session(&mut acc, &c.msa_client_id()) {
        Ok(s) => {
            c.accounts[idx] = acc;
            let _ = c.save();
            println!(
                "{}",
                serde_json::json!({"name": s.name, "uuid": s.uuid, "token": s.token, "online": s.online})
            );
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    if std::env::args().any(|a| a == "--print-session") {
        std::process::exit(print_session());
    }
    let cfg: Shared = Arc::new(Mutex::new(Config::load()));
    let rt = Runtime::default();
    let ui = MainWindow::new()?;
    let weak = ui.as_weak();

    {
        let c = lock(&cfg);
        sync_accounts(&ui, &c);
        settings_to_ui(&ui, &c);
        set_versions(
            &ui,
            merged_versions(
                std::path::Path::new(&c.settings.game_dir),
                &[],
                &c.settings.java_version,
            ),
        );
    }
    // версии из манифеста — в фоне
    let fetch_versions = {
        let (cfg, weak) = (cfg.clone(), weak.clone());
        move || {
            let (cfg, weak) = (cfg.clone(), weak.clone());
            std::thread::spawn(move || {
                let remote: Vec<String> = java::remote_versions()
                    .map(|v| v.into_iter().map(|(id, _)| id).collect())
                    .unwrap_or_default();
                let (game, cur) = {
                    let c = lock(&cfg);
                    (c.settings.game_dir.clone(), c.settings.java_version.clone())
                };
                let list = merged_versions(std::path::Path::new(&game), &remote, &cur);
                let _ = weak.upgrade_in_event_loop(move |u| set_versions(&u, list));
            });
        }
    };
    fetch_versions();
    ui.on_refresh_versions({
        let f = fetch_versions.clone();
        move || f()
    });

    ui.on_save_settings({
        let (cfg, weak, f) = (cfg.clone(), weak.clone(), fetch_versions.clone());
        move || {
            let Some(u) = weak.upgrade() else { return };
            {
                let mut c = lock(&cfg);
                ui_to_settings(&u, &mut c);
                save(&c, &weak);
            }
            toast(&weak, "Настройки сохранены");
            f();
        }
    });

    ui.on_select_account({
        let (cfg, weak) = (cfg.clone(), weak.clone());
        move |i| {
            let Some(u) = weak.upgrade() else { return };
            let mut c = lock(&cfg);
            c.selected = (i.max(0) as usize).min(c.accounts.len().saturating_sub(1));
            sync_accounts(&u, &c);
            save(&c, &weak);
        }
    });

    ui.on_remove_account({
        let (cfg, weak) = (cfg.clone(), weak.clone());
        move |i| {
            let Some(u) = weak.upgrade() else { return };
            let mut c = lock(&cfg);
            if c.accounts.len() <= 1 {
                toast(&weak, "Нельзя удалить последний аккаунт");
                return;
            }
            let i = i.max(0) as usize;
            if i < c.accounts.len() {
                c.accounts.remove(i);
                if c.selected >= i && c.selected > 0 {
                    c.selected -= 1;
                }
            }
            sync_accounts(&u, &c);
            save(&c, &weak);
        }
    });

    ui.on_add_offline({
        let (cfg, weak) = (cfg.clone(), weak.clone());
        move |nick| {
            let Some(u) = weak.upgrade() else { return };
            let nick = nick.trim().to_string();
            if !config::valid_nick(&nick) {
                toast(&weak, "Ник: 3–16 символов, буквы/цифры/_");
                return;
            }
            let mut c = lock(&cfg);
            if c.accounts
                .iter()
                .any(|a| a.kind == Kind::Offline && a.name == nick)
            {
                toast(&weak, "Такой offline-аккаунт уже есть");
                return;
            }
            c.accounts.push(Account::offline(&nick));
            c.selected = c.accounts.len() - 1;
            u.set_new_nick("".into());
            sync_accounts(&u, &c);
            save(&c, &weak);
        }
    });

    ui.on_open_url(|url| {
        let _ = std::process::Command::new("xdg-open")
            .arg(url.as_str())
            .spawn();
    });
    ui.on_copy_text(|t| {
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text(t.to_string());
        }
    });
    ui.on_clear_log({
        let (rt, weak) = (rt.clone(), weak.clone());
        move || {
            lock(&rt.log).clear();
            let _ = weak.upgrade_in_event_loop(|u| u.set_log("".into()));
        }
    });

    ui.on_find_javas({
        let weak = weak.clone();
        move || {
            let weak = weak.clone();
            toast(&weak, "Ищу Java…");
            std::thread::spawn(move || {
                let list: Vec<SharedString> = java::find_javas()
                    .into_iter()
                    .map(|(p, m)| format!("{p} · Java {m}").into())
                    .collect();
                let n = list.len();
                let _ = weak.upgrade_in_event_loop(move |u| {
                    u.set_javas(ModelRc::new(VecModel::from(list)));
                });
                toast(&weak, &format!("Найдено Java: {n}"));
            });
        }
    });

    // ---- вход Microsoft ----
    ui.on_cancel_ms({
        let (rt, weak) = (rt.clone(), weak.clone());
        move || {
            rt.ms_gen.fetch_add(1, Ordering::SeqCst);
            let _ = weak.upgrade_in_event_loop(|u| u.set_ms_open(false));
        }
    });
    ui.on_add_ms({
        let (cfg, rt, weak) = (cfg.clone(), rt.clone(), weak.clone());
        move || {
            let client_id = lock(&cfg).msa_client_id();
            if client_id.is_empty() {
                toast(
                    &weak,
                    "Сначала задай Azure client_id в Настройках → Microsoft",
                );
                if let Some(u) = weak.upgrade() {
                    u.set_page(2);
                }
                return;
            }
            let gen = rt.ms_gen.fetch_add(1, Ordering::SeqCst) + 1;
            if let Some(u) = weak.upgrade() {
                u.set_ms_code("".into());
                u.set_ms_url("".into());
                u.set_ms_status("Запрашиваю код…".into());
                u.set_ms_open(true);
            }
            let (cfg, rt, weak) = (cfg.clone(), rt.clone(), weak.clone());
            std::thread::spawn(move || ms_login(cfg, rt, weak, client_id, gen));
        }
    });

    // ---- запуск ----
    ui.on_stop({
        let rt = rt.clone();
        move || {
            for c in lock(&rt.children).iter_mut() {
                let _ = c.kill();
            }
        }
    });
    let start = {
        let (cfg, rt, weak) = (cfg.clone(), rt.clone(), weak.clone());
        Rc::new(move |both: bool| {
            let Some(u) = weak.upgrade() else { return };
            let snapshot = {
                let mut c = lock(&cfg);
                ui_to_settings(&u, &mut c);
                save(&c, &weak);
                c.clone()
            };
            u.set_busy(true);
            u.set_progress(0.0);
            u.set_status("Подготовка…".into());
            let (cfg, rt, weak) = (cfg.clone(), rt.clone(), weak.clone());
            std::thread::spawn(move || {
                let r = run_game(&cfg, &rt, &weak, snapshot, both);
                let _ = weak.upgrade_in_event_loop(move |u| {
                    u.set_busy(false);
                    u.set_running(false);
                    u.set_progress(0.0);
                    let _ = u.show();
                    match r {
                        Ok(msg) => u.set_status(msg.into()),
                        Err(e) => u.set_status(format!("Ошибка: {e}").into()),
                    }
                });
            });
        })
    };
    ui.on_play({
        let start = start.clone();
        move || start(false)
    });
    ui.on_play_both(move || start(true));

    // Окно прячется на время игры, поэтому цикл не должен завершаться при hide().
    // Крестик окна завершает лаунчер (программный hide() этот обработчик не вызывает).
    ui.window().on_close_requested(|| {
        let _ = slint::quit_event_loop();
        slint::CloseRequestResponse::HideWindow
    });
    ui.show()?;
    slint::run_event_loop_until_quit()
}

fn ms_login(cfg: Shared, rt: Runtime, weak: Weak<MainWindow>, client_id: String, gen: u64) {
    let status = |w: &Weak<MainWindow>, s: &str| {
        let s = s.to_string();
        let _ = w.upgrade_in_event_loop(move |u| u.set_ms_status(s.into()));
    };
    let fail = |w: &Weak<MainWindow>, e: String| {
        status(w, &format!("Ошибка: {e}"));
    };
    let code = match microsoft::device_code(&client_id) {
        Ok(c) => c,
        Err(e) => return fail(&weak, e.to_string()),
    };
    {
        let (c, url) = (code.user_code.clone(), code.verification_uri.clone());
        let _ = weak.upgrade_in_event_loop(move |u| {
            u.set_ms_code(c.into());
            u.set_ms_url(url.into());
            u.set_ms_status("Жду подтверждения в браузере…".into());
        });
    }
    let deadline = std::time::Instant::now() + Duration::from_secs(code.expires_in);
    let mut interval = code.interval;
    let tokens = loop {
        std::thread::sleep(Duration::from_secs(interval));
        if rt.ms_gen.load(Ordering::SeqCst) != gen {
            return;
        }
        if std::time::Instant::now() > deadline {
            return fail(&weak, "код истёк".into());
        }
        match microsoft::poll(&client_id, &code) {
            Ok(microsoft::Poll::Pending) => {}
            Ok(microsoft::Poll::SlowDown) => interval += 5,
            Ok(microsoft::Poll::Done(t)) => break t,
            Err(e) => return fail(&weak, e.to_string()),
        }
    };
    status(&weak, "Вход в Xbox / Minecraft…");
    let mc = match microsoft::minecraft_login(&tokens.access_token) {
        Ok(m) => m,
        Err(e) => return fail(&weak, e.to_string()),
    };
    let acc = Account {
        kind: Kind::Microsoft,
        name: mc.name.clone(),
        uuid: mc.uuid.clone(),
        refresh_token: tokens.refresh_token,
        mc_token: mc.access_token,
        mc_expires: mc.expires_at,
    };
    {
        let mut c = lock(&cfg);
        if let Some(i) = c
            .accounts
            .iter()
            .position(|a| a.kind == Kind::Microsoft && a.uuid == acc.uuid)
        {
            c.accounts[i] = acc;
            c.selected = i;
        } else {
            c.accounts.push(acc);
            c.selected = c.accounts.len() - 1;
        }
        save(&c, &weak);
        let items = account_items(&c);
        let sel = c.selected as i32;
        let _ = weak.upgrade_in_event_loop(move |u| {
            u.set_accounts(ModelRc::new(VecModel::from(items)));
            u.set_selected(sel);
            u.set_ms_open(false);
        });
    }
    toast(&weak, &format!("Вошёл как {}", mc.name));
}

// try_wait() уже забрал статус завершённого процесса — зомби не остаётся.
#[allow(clippy::zombie_processes)]
fn run_game(
    cfg: &Shared,
    rt: &Runtime,
    weak: &Weak<MainWindow>,
    snap: Config,
    both: bool,
) -> Result<String, String> {
    let status = |s: &str, p: f32| {
        let s = s.to_string();
        let _ = weak.upgrade_in_event_loop(move |u| {
            u.set_status(s.into());
            u.set_progress(p);
        });
    };
    let idx = snap.selected;
    let mut acc = snap.accounts.get(idx).cloned().ok_or("нет аккаунта")?;
    status("Проверка аккаунта…", 0.0);
    let session = launch::resolve_session(&mut acc, &snap.msa_client_id())?;
    {
        let mut c = lock(cfg);
        if let Some(a) = c.accounts.get_mut(idx) {
            *a = acc;
        }
        save(&c, weak);
    }
    push_log(
        rt,
        weak,
        &format!(
            "== запуск: {} ({}) ==",
            session.name,
            if session.online {
                "Microsoft"
            } else {
                "offline"
            }
        ),
    );
    // (команда, метка в логе). «Оба»: сначала Java (может докачивать), потом ezzmc.
    let mut cmds: Vec<(Command, &str)> = Vec::new();
    if both || snap.settings.target != 0 {
        cmds.push((
            launch::build_java(&snap, &session, &|s, p| status(s, p))?,
            if both { "[java] " } else { "" },
        ));
    }
    if both || snap.settings.target == 0 {
        cmds.push((
            launch::build_ezzmc(&snap, &session)?,
            if both { "[ezzmc] " } else { "" },
        ));
    }
    let capture = !snap.settings.close_on_launch;
    for (cmd, tag) in cmds {
        let mut child = launch::spawn(cmd, capture)?;
        if capture {
            let streams: [Option<Box<dyn std::io::Read + Send>>; 2] = [
                child.stdout.take().map(|s| Box::new(s) as _),
                child.stderr.take().map(|s| Box::new(s) as _),
            ];
            for s in streams.into_iter().flatten() {
                let (rt, weak) = (rt.clone(), weak.clone());
                std::thread::spawn(move || {
                    use std::io::BufRead;
                    for l in std::io::BufReader::new(s).lines().map_while(Result::ok) {
                        push_log(&rt, &weak, &format!("{tag}{l}"));
                    }
                });
            }
        }
        lock(&rt.children).push(child);
    }
    let close_on_launch = snap.settings.close_on_launch;
    let _ = weak.upgrade_in_event_loop(move |u| {
        u.set_busy(false);
        u.set_running(true);
        u.set_status("Игра запущена".into());
        if !close_on_launch {
            // Лаунчер скрывается, пока идёт игра; вернётся в on_play после её закрытия.
            let _ = u.hide();
        }
    });
    if snap.settings.close_on_launch {
        std::thread::sleep(Duration::from_secs(2));
        let _ = slint::invoke_from_event_loop(|| {
            let _ = slint::quit_event_loop();
        });
        return Ok("Игра запущена".into());
    }
    loop {
        std::thread::sleep(Duration::from_millis(300));
        let mut g = lock(&rt.children);
        let mut i = 0;
        while i < g.len() {
            match g[i].try_wait() {
                Ok(Some(st)) => {
                    g.remove(i);
                    push_log(rt, weak, &format!("== игра завершилась: {st} =="));
                    if g.is_empty() {
                        return Ok(format!("Игра завершилась ({st})"));
                    }
                }
                Ok(None) => i += 1,
                Err(e) => return Err(e.to_string()),
            }
        }
        if g.is_empty() {
            break;
        }
    }
    Ok("Готово".into())
}

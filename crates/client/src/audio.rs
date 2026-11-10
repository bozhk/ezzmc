//! Звуковая система: sounds.json → выбор по весам → декод ogg → микшер cpal.
//! VANILLA: SoundEngine#play/calculateVolume/calculatePitch, WeighedSoundEvents#getSound,
//! Channel#linearAttenuation (ref 0, rolloff 1, max = max(volume,1)·attenuation_distance), SoundSource.
//! Вызовы из любого потока: `audio::play_at(...)`, `audio::play_ui(...)`; ничто не блокирует вызывающего.
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// VANILLA: SoundSource (порядок = ordinal в пакетах).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cat {
    Master,
    Music,
    Records,
    Weather,
    Blocks,
    Hostile,
    Neutral,
    Players,
    Ambient,
    Voice,
    Ui,
}

impl Cat {
    pub fn from_ordinal(i: i32) -> Cat {
        use Cat::*;
        [
            Master, Music, Records, Weather, Blocks, Hostile, Neutral, Players, Ambient, Voice, Ui,
        ]
        .get(i.max(0) as usize)
        .copied()
        .unwrap_or(Master)
    }
}

/// Диагностика для MCP: активные голоса и число запущенных звуков за сессию.
pub static ACTIVE: AtomicU32 = AtomicU32::new(0);
pub static STARTED: AtomicU32 = AtomicU32::new(0);
/// Число играющих голосов категории Music (для MusicManager).
pub static MUSIC_ACTIVE: AtomicU32 = AtomicU32::new(0);

static CAT_VOL: [AtomicU32; 11] = [const { AtomicU32::new(0x3f80_0000) }; 11];

/// Громкость категории 0..1 (VANILLA: Options#getFinalSoundSourceVolume).
pub fn set_volume(c: Cat, v: f32) {
    CAT_VOL[c as usize].store(v.clamp(0.0, 1.0).to_bits(), Ordering::Relaxed);
}
fn cat_volume(c: Cat) -> f32 {
    let g = |c: Cat| f32::from_bits(CAT_VOL[c as usize].load(Ordering::Relaxed));
    if c == Cat::Master {
        g(c)
    } else {
        g(c) * g(Cat::Master)
    }
}

#[derive(Clone, Copy)]
struct Listener {
    pos: [f64; 3],
    yaw: f32,
    pitch: f32,
}
static LISTENER: Mutex<Listener> = Mutex::new(Listener {
    pos: [0.0; 3],
    yaw: 0.0,
    pitch: 0.0,
});

/// Положение/ориентация слушателя (глаза камеры), градусы как в ванили.
pub fn set_listener(pos: [f64; 3], yaw: f32, pitch: f32) {
    if let Ok(mut l) = LISTENER.lock() {
        *l = Listener { pos, yaw, pitch };
    }
}

struct Req {
    event: String,
    cat: Cat,
    pos: Option<[f64; 3]>,
    volume: f32,
    pitch: f32,
    /// Зацикленный голос с регулируемой громкостью (f32 bits).
    gain: Option<Arc<AtomicU32>>,
}

enum Cmd {
    Play(Req),
    Stop(Option<String>, Option<Cat>),
}

enum MixCmd {
    Add(Voice),
    Stop(Option<String>, Option<Cat>),
}

struct Pcm {
    rate: u32,
    ch: usize,
    data: Vec<i16>,
}
impl Pcm {
    fn frames(&self) -> usize {
        self.data.len() / self.ch
    }
}

struct Voice {
    pcm: Arc<Pcm>,
    pos: f64,
    step: f64,
    vol: f32,
    at: Option<[f64; 3]>,
    max: f32,
    looping: bool,
    gain: Option<Arc<AtomicU32>>,
    event: String,
    cat: Cat,
}

static TX: OnceLock<Sender<Cmd>> = OnceLock::new();

/// Запустить звуковой поток (идемпотентно). Без устройства вывода всё молча игнорируется.
pub fn init() {
    TX.get_or_init(|| {
        let (tx, rx) = unbounded();
        let _ = std::thread::Builder::new()
            .name("audio".into())
            .spawn(move || worker(rx));
        tx
    });
}

fn send(c: Cmd) {
    if let Some(tx) = TX.get() {
        let _ = tx.send(c);
    }
}

/// Звук в точке мира.
pub fn play_at(event: &str, cat: Cat, x: f64, y: f64, z: f64, volume: f32, pitch: f32) {
    send(Cmd::Play(Req {
        event: norm(event),
        cat,
        pos: Some([x, y, z]),
        volume,
        pitch,
        gain: None,
    }));
}

/// Звук без позиции (GUI, музыка): VANILLA: SimpleSoundInstance.forUI / relative.
pub fn play_ui(event: &str, cat: Cat, volume: f32, pitch: f32) {
    send(Cmd::Play(Req {
        event: norm(event),
        cat,
        pos: None,
        volume,
        pitch,
        gain: None,
    }));
}

/// Зацикленный звук без позиции (VANILLA: BiomeAmbientSoundsHandler.LoopSoundInstance, relative).
/// Громкость 0..1 меняется записью f32-битов в `gain`; остановить — `stop(Some(event), Some(cat))`.
pub fn play_loop(event: &str, cat: Cat, gain: Arc<AtomicU32>) {
    send(Cmd::Play(Req {
        event: norm(event),
        cat,
        pos: None,
        volume: 1.0,
        pitch: 1.0,
        gain: Some(gain),
    }));
}

/// VANILLA: ClientboundStopSoundPacket.
pub fn stop(event: Option<&str>, cat: Option<Cat>) {
    send(Cmd::Stop(event.map(norm), cat));
}

fn norm(e: &str) -> String {
    e.strip_prefix("minecraft:").unwrap_or(e).to_owned()
}

// ───────────────────────── sounds.json ─────────────────────────

struct Entry {
    name: String,
    is_event: bool,
    volume: Dist,
    pitch: Dist,
    weight: i32,
    atten: f32,
}

#[derive(Clone, Copy)]
enum Dist {
    Const(f32),
    Uniform(f32, f32),
}
impl Dist {
    fn parse(v: Option<&serde_json::Value>) -> Dist {
        match v {
            Some(serde_json::Value::Number(n)) => Dist::Const(n.as_f64().unwrap_or(1.0) as f32),
            Some(o) if o.is_object() => {
                let f = |k: &str| o.get(k).and_then(|x| x.as_f64()).map(|x| x as f32);
                match (f("min"), f("max")) {
                    (Some(a), Some(b)) => Dist::Uniform(a, b),
                    _ => Dist::Const(f("value").unwrap_or(1.0)),
                }
            }
            _ => Dist::Const(1.0),
        }
    }
    fn sample(self, r: &mut Rng) -> f32 {
        match self {
            Dist::Const(v) => v,
            Dist::Uniform(a, b) => a + (b - a) * r.f32(),
        }
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn f32(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
    fn below(&mut self, n: i32) -> i32 {
        (self.next() % n.max(1) as u64) as i32
    }
}

struct Registry {
    events: HashMap<String, Vec<Entry>>,
    store: ezzmc_assets::source::ObjectStore,
}

fn load_registry() -> Option<Registry> {
    let inst = ezzmc_assets::install::GameInstall::locate("26.3").ok()?;
    let idx = inst.index_path().ok()??;
    let objects = inst.assets_dir.as_ref()?.join("objects");
    let store = ezzmc_assets::source::ObjectStore::open(&idx, &objects).ok()?;
    use ezzmc_assets::source::ResourceSource;
    let raw = store.read("assets/minecraft/sounds.json")?;
    let j: serde_json::Value = serde_json::from_slice(&raw).ok()?;
    let mut events = HashMap::new();
    for (k, v) in j.as_object()? {
        let mut list = Vec::new();
        for s in v
            .get("sounds")
            .and_then(|x| x.as_array())
            .into_iter()
            .flatten()
        {
            let e = match s {
                serde_json::Value::String(n) => Entry {
                    name: n.clone(),
                    is_event: false,
                    volume: Dist::Const(1.0),
                    pitch: Dist::Const(1.0),
                    weight: 1,
                    atten: 16.0,
                },
                o => Entry {
                    name: o
                        .get("name")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_owned(),
                    is_event: o.get("type").and_then(|x| x.as_str()) == Some("event"),
                    volume: Dist::parse(o.get("volume")),
                    pitch: Dist::parse(o.get("pitch")),
                    weight: o.get("weight").and_then(|x| x.as_i64()).unwrap_or(1) as i32,
                    atten: o
                        .get("attenuation_distance")
                        .and_then(|x| x.as_f64())
                        .unwrap_or(16.0) as f32,
                },
            };
            list.push(e);
        }
        events.insert(k.clone(), list);
    }
    Some(Registry { events, store })
}

struct Picked {
    path: String,
    volume: f32,
    pitch: f32,
    atten: f32,
}

impl Registry {
    /// VANILLA: WeighedSoundEvents#getSound (вложенные `type: event` раскрываются рекурсивно).
    fn pick(&self, event: &str, rng: &mut Rng, depth: u8) -> Option<Picked> {
        let list = self.events.get(event)?;
        let total: i32 = list.iter().map(|e| e.weight.max(0)).sum();
        if list.is_empty() || total == 0 || depth > 4 {
            return None;
        }
        let mut i = rng.below(total);
        let e = list.iter().find(|e| {
            i -= e.weight.max(0);
            i < 0
        })?;
        if e.is_event {
            let mut p = self.pick(&norm(&e.name), rng, depth + 1)?;
            p.volume *= e.volume.sample(rng);
            p.pitch *= e.pitch.sample(rng);
            return Some(p);
        }
        let (ns, path) = e.name.split_once(':').unwrap_or(("minecraft", &e.name));
        Some(Picked {
            path: format!("assets/{ns}/sounds/{path}.ogg"),
            volume: e.volume.sample(rng),
            pitch: e.pitch.sample(rng),
            atten: e.atten,
        })
    }
}

fn decode(bytes: Vec<u8>) -> Option<Pcm> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::io::MediaSourceStream;
    let mss = MediaSourceStream::new(Box::new(Cursor::new(bytes)), Default::default());
    let probed = symphonia::default::get_probe()
        .format(
            &symphonia::core::probe::Hint::new(),
            mss,
            &Default::default(),
            &Default::default(),
        )
        .ok()?;
    let mut fmt = probed.format;
    let track = fmt.default_track()?;
    let (id, rate) = (track.id, track.codec_params.sample_rate?);
    let mut dec = symphonia::default::get_codecs()
        .make(&track.codec_params, &Default::default())
        .ok()?;
    let mut data: Vec<i16> = Vec::new();
    let mut ch = 0;
    while let Ok(pkt) = fmt.next_packet() {
        if pkt.track_id() != id {
            continue;
        }
        let Ok(buf) = dec.decode(&pkt) else { continue };
        let spec = *buf.spec();
        let n = spec.channels.count();
        let mut sb = SampleBuffer::<i16>::new(buf.capacity() as u64, spec);
        sb.copy_interleaved_ref(buf);
        if n <= 2 {
            ch = n;
            data.extend_from_slice(sb.samples());
        } else {
            // >2 каналов: берём первые два.
            ch = 2;
            for f in sb.samples().chunks(n) {
                data.extend_from_slice(&f[..2]);
            }
        }
    }
    data.shrink_to_fit();
    (ch > 0 && !data.is_empty()).then_some(Pcm { rate, ch, data })
}

const MAX_VOICES: usize = 247;
/// Потолок кэша коротких звуков (байт PCM).
const PCM_CACHE_MAX: usize = 64 << 20;

fn worker(rx: Receiver<Cmd>) {
    let (mtx, mrx) = unbounded::<MixCmd>();
    let Some((_stream, out_rate)) = open_output(mrx) else {
        tracing::warn!("звук: нет устройства вывода — звук отключён");
        return;
    };
    let Some(reg) = load_registry() else {
        tracing::warn!("звук: не найден sounds.json (assets индекс) — звук отключён");
        return;
    };
    let mut cache: HashMap<String, Option<Arc<Pcm>>> = HashMap::new();
    let mut cache_bytes = 0usize;
    let mut warned: std::collections::HashSet<String> = Default::default();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ std::process::id() as u64);
    use ezzmc_assets::source::ResourceSource;
    while let Ok(c) = rx.recv() {
        match c {
            Cmd::Stop(e, c) => {
                let _ = mtx.send(MixCmd::Stop(e, c));
            }
            Cmd::Play(r) => {
                let Some(p) = reg.pick(&r.event, &mut rng, 0) else {
                    if !reg.events.contains_key(&r.event) && warned.insert(r.event.clone()) {
                        tracing::debug!("звук: неизвестное событие {}", r.event);
                    }
                    continue;
                };
                // VANILLA: SimpleSoundInstance volume·=sound.volume, pitch·=sound.pitch.
                let vol_inst = r.volume * p.volume;
                // Громкость категории применяется в микшере на лету (VANILLA: updateSourceVolume).
                let vol = vol_inst.clamp(0.0, 1.0);
                let long = r.gain.is_some() || matches!(r.cat, Cat::Music | Cat::Records);
                if vol <= 0.0 || (!long && cat_volume(r.cat) <= 0.0) {
                    continue;
                }
                // Длинные звуки (музыка/пластинки/петли) не кэшируем: трек ≈ 30 МБ PCM,
                // освобождается вместе с голосом. Короткие — в кэше с потолком по размеру.
                let pcm = if long {
                    reg.store.read(&p.path).and_then(decode).map(Arc::new)
                } else {
                    if cache_bytes > PCM_CACHE_MAX {
                        cache.clear();
                        cache_bytes = 0;
                    }
                    cache
                        .entry(p.path.clone())
                        .or_insert_with(|| {
                            let d = reg.store.read(&p.path).and_then(decode);
                            if d.is_none() {
                                tracing::debug!("звук: не декодирован {}", p.path);
                            }
                            d.map(|d| {
                                cache_bytes += d.data.len() * 2;
                                Arc::new(d)
                            })
                        })
                        .clone()
                };
                let Some(pcm) = pcm else { continue };
                let pitch = (r.pitch * p.pitch).clamp(0.5, 2.0);
                let looping = r.gain.is_some();
                let _ = mtx.send(MixCmd::Add(Voice {
                    step: f64::from(pcm.rate) / f64::from(out_rate) * f64::from(pitch),
                    pcm,
                    pos: 0.0,
                    vol,
                    at: r.pos,
                    max: vol_inst.max(1.0) * p.atten,
                    looping,
                    gain: r.gain,
                    event: r.event,
                    cat: r.cat,
                }));
            }
        }
    }
}

// ───────────────────────── вывод и микшер ─────────────────────────

fn open_output(rx: Receiver<MixCmd>) -> Option<(cpal::Stream, u32)> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    let dev = cpal::default_host().default_output_device()?;
    let cfg = dev.default_output_config().ok()?;
    let rate = cfg.sample_rate().0;
    let ch = usize::from(cfg.channels());
    let conf: cpal::StreamConfig = cfg.clone().into();
    let err = |e| tracing::warn!("звук: ошибка потока: {e}");
    let stream = match cfg.sample_format() {
        cpal::SampleFormat::F32 => dev.build_output_stream(&conf, mixer::<f32>(rx, ch), err, None),
        cpal::SampleFormat::I16 => dev.build_output_stream(&conf, mixer::<i16>(rx, ch), err, None),
        cpal::SampleFormat::U16 => dev.build_output_stream(&conf, mixer::<u16>(rx, ch), err, None),
        _ => return None,
    }
    .ok()?;
    stream.play().ok()?;
    Some((stream, rate))
}

fn mixer<T: cpal::SizedSample + cpal::FromSample<f32>>(
    rx: Receiver<MixCmd>,
    out_ch: usize,
) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) + Send + 'static {
    let mut voices: Vec<Voice> = Vec::new();
    let mut acc: Vec<f32> = Vec::new();
    move |out: &mut [T], _| {
        for c in rx.try_iter() {
            match c {
                MixCmd::Add(v) => {
                    if voices.len() < MAX_VOICES {
                        STARTED.fetch_add(1, Ordering::Relaxed);
                        voices.push(v);
                    }
                }
                MixCmd::Stop(e, c) => voices.retain(|v| {
                    !(e.as_ref().is_none_or(|e| *e == v.event) && c.is_none_or(|c| c == v.cat))
                }),
            }
        }
        let l = LISTENER.try_lock().map(|g| *g).unwrap_or(Listener {
            pos: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
        });
        let frames = out.len() / out_ch.max(1);
        acc.clear();
        acc.resize(frames * 2, 0.0);
        let basis = Basis::of(&l);
        voices.retain_mut(|v| {
            let (mut gl, mut gr) = voice_gains(v, &l, &basis);
            let ck = cat_volume(v.cat);
            gl *= ck;
            gr *= ck;
            if let Some(g) = &v.gain {
                let k = f32::from_bits(g.load(Ordering::Relaxed));
                gl *= k;
                gr *= k;
            }
            let n = v.pcm.frames();
            let st = v.pcm.ch;
            for f in 0..frames {
                let i = v.pos as usize;
                if i >= n {
                    if v.looping {
                        v.pos -= n as f64;
                    } else {
                        return false;
                    }
                }
                let i = v.pos as usize;
                let t = (v.pos - i as f64) as f32;
                let j = if i + 1 < n {
                    i + 1
                } else if v.looping {
                    0
                } else {
                    i
                };
                let s = |k: usize, c: usize| f32::from(v.pcm.data[k * st + c]) / 32768.0;
                let (a, b) = (s(i, 0) + (s(j, 0) - s(i, 0)) * t, {
                    let c = st - 1;
                    s(i, c) + (s(j, c) - s(i, c)) * t
                });
                if v.at.is_some() || st == 1 {
                    let m = if st == 1 { a } else { (a + b) * 0.5 };
                    acc[f * 2] += m * gl;
                    acc[f * 2 + 1] += m * gr;
                } else {
                    acc[f * 2] += a * gl;
                    acc[f * 2 + 1] += b * gr;
                }
                v.pos += v.step;
            }
            true
        });
        ACTIVE.store(voices.len() as u32, Ordering::Relaxed);
        MUSIC_ACTIVE.store(
            voices.iter().filter(|v| v.cat == Cat::Music).count() as u32,
            Ordering::Relaxed,
        );
        for (f, o) in out.chunks_mut(out_ch.max(1)).enumerate() {
            let (a, b) = (acc[f * 2].clamp(-1.0, 1.0), acc[f * 2 + 1].clamp(-1.0, 1.0));
            for (c, s) in o.iter_mut().enumerate() {
                *s = T::from_sample(match (out_ch, c) {
                    (1, _) => (a + b) * 0.5,
                    (_, 0) => a,
                    (_, 1) => b,
                    _ => 0.0,
                });
            }
        }
    }
}

/// Оси слушателя (вперёд/вверх/вправо) из yaw/pitch Minecraft.
struct Basis {
    fwd: [f64; 3],
    up: [f64; 3],
    right: [f64; 3],
}
impl Basis {
    fn of(l: &Listener) -> Basis {
        let (y, p) = (
            f64::from(l.yaw).to_radians(),
            f64::from(l.pitch).to_radians(),
        );
        let fwd = [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()];
        let right = [-y.cos(), 0.0, -y.sin()];
        let up = [
            right[1] * fwd[2] - right[2] * fwd[1],
            right[2] * fwd[0] - right[0] * fwd[2],
            right[0] * fwd[1] - right[1] * fwd[0],
        ];
        Basis { fwd, up, right }
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Усиление левого/правого канала: расстояние (линейное) + стерео-панорама.
fn voice_gains(v: &Voice, l: &Listener, b: &Basis) -> (f32, f32) {
    let Some(at) = v.at else {
        return (v.vol, v.vol);
    };
    let d = [at[0] - l.pos[0], at[1] - l.pos[1], at[2] - l.pos[2]];
    let dist = dot(d, d).sqrt() as f32;
    let g = v.vol * (1.0 - dist.min(v.max) / v.max).clamp(0.0, 1.0);
    let [pl, pr] = pan(dot(d, b.right), dot(d, b.up), dot(d, b.fwd));
    (g * pl, g * pr)
}

/// Панорама моно-источника на стерео, как у OpenAL Soft без HRTF: передний сектор растянут в 1.5 раза
/// (60° от центра — полностью в канал), затем декодер первого порядка (W=0.5, Y=0.5, X=0.0957).
fn pan(right: f64, up: f64, fwd: f64) -> [f32; 2] {
    let len = (right * right + up * up + fwd * fwd).sqrt();
    if len < 1.0e-6 {
        return pan(0.0, 0.0, 1.0);
    }
    let el = (up / len).clamp(-1.0, 1.0).asin();
    let mut az = right.atan2(fwd);
    let half_pi = std::f64::consts::FRAC_PI_2;
    if az.abs() < half_pi {
        az = (az.abs() * 1.5).min(half_pi).copysign(az);
    }
    let y = az.sin() * el.cos();
    let x = az.cos() * el.cos() * 0.0957;
    [(0.5 - 0.5 * y + x) as f32, (0.5 + 0.5 * y + x) as f32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pan_center_and_sides() {
        let c = pan(0.0, 0.0, 1.0);
        assert!((c[0] - c[1]).abs() < 1e-6 && (c[0] - 0.596).abs() < 1e-3);
        let r = pan(1.0, 0.0, 0.0);
        assert!(r[1] > 0.9 && r[0] < 0.1);
        let b = pan(0.0, 0.0, -1.0);
        assert!((b[0] - 0.404).abs() < 1e-3);
    }

    #[test]
    fn right_is_right() {
        // Смотрим на юг (yaw 0): запад (−x) — справа.
        let b = Basis::of(&Listener {
            pos: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
        });
        assert!(dot([-1.0, 0.0, 0.0], b.right) > 0.99);
        assert!(dot([0.0, 1.0, 0.0], b.up) > 0.99);
    }

    #[test]
    fn dist_attenuation() {
        let v = Voice {
            pcm: Arc::new(Pcm {
                rate: 44100,
                ch: 1,
                data: vec![0; 4],
            }),
            pos: 0.0,
            step: 1.0,
            vol: 1.0,
            at: Some([8.0, 0.0, 0.0]),
            max: 16.0,
            looping: false,
            gain: None,
            event: String::new(),
            cat: Cat::Blocks,
        };
        let l = Listener {
            pos: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
        };
        let (a, b) = voice_gains(&v, &l, &Basis::of(&l));
        assert!(((a + b) - 0.0).abs() >= 0.0 && a + b > 0.0);
    }
}

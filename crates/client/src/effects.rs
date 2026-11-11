//! Активные эффекты локального игрока (TODO-317, 153).
//! VANILLA: MobEffectInstance (клиентская часть: duration, BlendState), LivingEntity#forceAddEffect /
//! removeEffectNoUpdate / tickEffects, GameRenderer#nightVisionScale.
use ezzmc_protocol::generated::r26_3::MOB_EFFECT;

/// VANILLA: MobEffectInstance.INFINITE_DURATION.
pub const INFINITE: i32 = -1;

const FLAG_AMBIENT: u8 = 1;
const FLAG_VISIBLE: u8 = 2;
const FLAG_SHOW_ICON: u8 = 4;
const FLAG_BLEND: u8 = 8;

/// VANILLA: MobEffect#setBlendDuration(in, out, outAdvance) — у остальных эффектов 0.
fn blend_params(name: &str) -> (i32, i32, i32) {
    match name {
        "nausea" => (150, 20, 60),
        "darkness" => (22, 22, 22),
        _ => (0, 0, 0),
    }
}

#[derive(Clone, Debug)]
pub struct Effect {
    pub id: i32,
    pub amplifier: i32,
    pub duration: i32,
    pub ambient: bool,
    pub visible: bool,
    pub show_icon: bool,
    factor: f32,
    factor_prev: f32,
}

impl Effect {
    pub fn name(&self) -> &'static str {
        MOB_EFFECT.get(self.id as usize).copied().unwrap_or("?")
    }

    fn is_infinite(&self) -> bool {
        self.duration == INFINITE
    }

    /// VANILLA: MobEffectInstance#endsWithin.
    pub fn ends_within(&self, ticks: i32) -> bool {
        !self.is_infinite() && self.duration <= ticks
    }

    /// VANILLA: BlendState#hasEffect.
    fn blend_target(&self) -> f32 {
        let advance = blend_params(self.name()).2;
        if self.ends_within(advance) {
            0.0
        } else {
            1.0
        }
    }

    /// VANILLA: MobEffectInstance#skipBlending → BlendState#setImmediate.
    fn set_immediate(&mut self) {
        self.factor = self.blend_target();
        self.factor_prev = self.factor;
    }

    /// VANILLA: MobEffectInstance#tickClient.
    fn tick(&mut self) {
        // Бесконечный эффект (-1) не тикает: duration > 0 его исключает.
        if self.duration > 0 {
            self.duration -= 1;
        }
        // BlendState#tick
        self.factor_prev = self.factor;
        let target = self.blend_target();
        if self.factor != target {
            let (bin, bout, _) = blend_params(self.name());
            let dur = if target > 0.5 { bin } else { bout };
            if dur == 0 {
                self.factor = target;
            } else {
                let max = 1.0 / dur as f32;
                self.factor += (target - self.factor).clamp(-max, max);
            }
        }
    }

    /// VANILLA: MobEffectInstance#getBlendFactor.
    pub fn blend(&self, partial: f32) -> f32 {
        self.factor_prev + (self.factor - self.factor_prev) * partial
    }
}

#[derive(Default, Debug)]
pub struct Effects {
    list: Vec<Effect>,
    /// Счётчик клиентских тиков с входа в мир (замена `LivingEntity#tickCount` для пульсации Darkness).
    pub ticks: u32,
}

impl Effects {
    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Effect> {
        self.list.iter()
    }

    /// VANILLA: ClientPacketListener#handleUpdateMobEffect + LivingEntity#forceAddEffect.
    pub fn update(&mut self, id: i32, amplifier: i32, duration: i32, flags: u8) {
        let mut e = Effect {
            id,
            amplifier,
            duration,
            ambient: flags & FLAG_AMBIENT != 0,
            visible: flags & FLAG_VISIBLE != 0,
            show_icon: flags & FLAG_SHOW_ICON != 0,
            factor: 0.0,
            factor_prev: 0.0,
        };
        if flags & FLAG_BLEND == 0 {
            e.set_immediate();
        }
        match self.list.iter_mut().find(|x| x.id == id) {
            Some(old) => {
                // VANILLA: newEffect.copyBlendState(previousEffect).
                e.factor = old.factor;
                e.factor_prev = old.factor_prev;
                *old = e;
            }
            None => self.list.push(e),
        }
    }

    /// VANILLA: ClientPacketListener#handleRemoveMobEffect → removeEffectNoUpdate.
    pub fn remove(&mut self, id: i32) {
        self.list.retain(|e| e.id != id);
    }

    /// VANILLA: LivingEntity#tickEffects (клиентская ветка) — раз в игровой тик.
    pub fn tick(&mut self) {
        self.ticks = self.ticks.wrapping_add(1);
        for e in &mut self.list {
            e.tick();
        }
    }

    pub fn get(&self, name: &str) -> Option<&Effect> {
        self.list.iter().find(|e| e.name() == name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// VANILLA: LivingEntity#getEffectBlendFactor.
    pub fn blend_factor(&self, name: &str, partial: f32) -> f32 {
        self.get(name).map_or(0.0, |e| e.blend(partial))
    }

    /// VANILLA: GameRenderer#nightVisionScale (нужен эффект night_vision).
    pub fn night_vision_scale(&self, partial: f32) -> f32 {
        match self.get("night_vision") {
            Some(e) if e.ends_within(200) => {
                0.7 + ((e.duration as f32 - partial) * std::f32::consts::PI * 0.2).sin() * 0.3
            }
            Some(_) => 1.0,
            None => 0.0,
        }
    }

    /// Яркость тумана от ночного зрения: FogRenderer#computeFogColor (вне воды; не при Darkness).
    pub fn fog_night_vision(&self, partial: f32) -> f32 {
        if self.has("night_vision") && !self.has("darkness") {
            self.night_vision_scale(partial)
        } else {
            0.0
        }
    }

    /// VANILLA: Camera → doesMobEffectBlockSky.
    pub fn blocks_sky(&self) -> bool {
        self.has("blindness") || self.has("darkness")
    }
}

/// VANILLA: ClientboundUpdateMobEffectPacket (varint entity, varint effect, varint amp, varint dur, byte flags).
pub fn parse_update(body: &[u8], own_id: i32) -> Option<(i32, i32, i32, u8)> {
    let mut r = ezzmc_protocol::wire::Reader::new(body);
    let ent = r.var_int().ok()?;
    if ent != own_id {
        return None;
    }
    let id = r.var_int().ok()?;
    let amp = r.var_int().ok()?;
    let dur = r.var_int().ok()?;
    let flags = r.u8().ok()?;
    Some((id, amp, dur, flags))
}

/// VANILLA: ClientboundRemoveMobEffectPacket (varint entity, varint effect).
pub fn parse_remove(body: &[u8], own_id: i32) -> Option<i32> {
    let mut r = ezzmc_protocol::wire::Reader::new(body);
    let ent = r.var_int().ok()?;
    if ent != own_id {
        return None;
    }
    r.var_int().ok()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn id(name: &str) -> i32 {
        MOB_EFFECT.iter().position(|n| *n == name).unwrap() as i32
    }

    #[test]
    fn darkness_blends_in_over_22_ticks() {
        let mut fx = Effects::default();
        fx.update(id("darkness"), 0, 600, FLAG_BLEND | FLAG_VISIBLE);
        assert_eq!(fx.blend_factor("darkness", 0.0), 0.0);
        for _ in 0..11 {
            fx.tick();
        }
        let half = fx.blend_factor("darkness", 1.0);
        assert!((half - 0.5).abs() < 1e-5, "{half}");
        for _ in 0..20 {
            fx.tick();
        }
        assert_eq!(fx.blend_factor("darkness", 1.0), 1.0);
    }

    #[test]
    fn no_blend_flag_is_immediate_and_expiry_blends_out() {
        let mut fx = Effects::default();
        fx.update(id("darkness"), 0, 30, FLAG_VISIBLE);
        assert_eq!(fx.blend_factor("darkness", 0.0), 1.0);
        for _ in 0..8 {
            fx.tick();
        }
        // duration 22 → endsWithin(22): цель 0, спад 1/22 за тик
        assert!(fx.blend_factor("darkness", 1.0) < 1.0);
        fx.remove(id("darkness"));
        assert!(!fx.has("darkness"));
    }

    #[test]
    fn night_vision_flashes_in_last_ten_seconds() {
        let mut fx = Effects::default();
        fx.update(id("night_vision"), 0, 1000, FLAG_VISIBLE);
        assert_eq!(fx.night_vision_scale(0.0), 1.0);
        fx.update(id("night_vision"), 0, 100, FLAG_VISIBLE);
        let s = fx.night_vision_scale(0.0);
        assert!((0.4..=1.0).contains(&s));
        assert_ne!(s, 1.0);
        fx.update(id("night_vision"), 0, INFINITE, FLAG_VISIBLE);
        assert_eq!(fx.night_vision_scale(0.0), 1.0);
    }

    #[test]
    fn parse_packets_only_for_own_entity() {
        let body = [5u8, 14, 0, 100, 0b110];
        assert_eq!(parse_update(&body, 5), Some((14, 0, 100, 6)));
        assert_eq!(parse_update(&body, 6), None);
        assert_eq!(parse_remove(&[5, 14], 5), Some(14));
    }
}

//! Статистика: пакет award_stats, счётчик и форматтеры.
//! VANILLA: ClientboundAwardStatsPacket, StatsCounter, Stats, StatFormatter (26.3).
use std::collections::HashMap;

use ezzmc_protocol::generated::r26_3 as reg;
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;

/// VANILLA: BuiltInRegistries.STAT_TYPE — порядок реестра (id в пакете).
pub const MINED: u32 = 0;
pub const CRAFTED: u32 = 1;
pub const USED: u32 = 2;
pub const BROKEN: u32 = 3;
pub const PICKED_UP: u32 = 4;
pub const DROPPED: u32 = 5;
pub const KILLED: u32 = 6;
pub const KILLED_BY: u32 = 7;
pub const CUSTOM: u32 = 8;

/// VANILLA: StatsCounter — (тип статистики, id значения в реестре типа) → число.
#[derive(Default, Debug)]
pub struct Stats {
    map: HashMap<(u32, u32), i32>,
    /// VANILLA: StatsScreen#onStatsUpdated — ответ на REQUEST_STATS уже получен.
    pub received: bool,
}

/// Разобранный `award_stats`: (тип, значение, количество).
pub type Update = Vec<(u32, u32, i32)>;

/// VANILLA: ClientboundAwardStatsPacket — Map<Stat, VarInt>; Stat = тип (VarInt) + значение (VarInt).
pub fn parse(body: &[u8]) -> ProtocolResult<Update> {
    let mut r = Reader::new(body);
    let n = r.var_int()?.max(0);
    let mut out = Vec::new();
    for _ in 0..n {
        let ty = r.var_int()?;
        let val = r.var_int()?;
        let amount = r.var_int()?;
        out.push((ty.max(0) as u32, val.max(0) as u32, amount));
    }
    Ok(out)
}

impl Stats {
    /// VANILLA: ClientPacketListener#handleAwardStats — put для каждой записи.
    pub fn apply(&mut self, u: Update) {
        for (t, v, a) in u {
            self.map.insert((t, v), a);
        }
        self.received = true;
    }

    /// VANILLA: StatsCounter#getValue(Stat) — по умолчанию 0.
    pub fn get(&self, ty: u32, value: u32) -> i32 {
        self.map.get(&(ty, value)).copied().unwrap_or(0)
    }
}

/// VANILLA: Stats — форматтер пользовательской статистики.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Formatter {
    Default,
    DivideByTen,
    Distance,
    Time,
}

/// VANILLA: Stats#makeCustomStat — форматтер по имени.
pub fn custom_formatter(name: &str) -> Formatter {
    match name {
        "play_time" | "total_world_time" | "time_since_death" | "time_since_rest"
        | "sneak_time" => Formatter::Time,
        "damage_dealt"
        | "damage_dealt_absorbed"
        | "damage_dealt_resisted"
        | "damage_taken"
        | "damage_blocked_by_shield"
        | "damage_absorbed"
        | "damage_resisted" => Formatter::DivideByTen,
        n if n.ends_with("_one_cm") => Formatter::Distance,
        _ => Formatter::Default,
    }
}

/// VANILLA: StatFormatter.DECIMAL_FORMAT — "########0.00" (HALF_EVEN, без группировки).
fn decimal(v: f64) -> String {
    format!("{v:.2}")
}

/// VANILLA: StatFormatter.DEFAULT — NumberFormat.getIntegerInstance(Locale.US): разделитель тысяч «,».
fn grouped(v: i32) -> String {
    let s = v.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if v < 0 {
        out.insert(0, '-');
    }
    out
}

/// VANILLA: Double.toString для значений вида n/20 (секунды): целые — с «.0».
fn java_double(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{v:.1}")
    } else {
        format!("{v}")
    }
}

impl Formatter {
    /// VANILLA: StatFormatter#format.
    pub fn format(self, v: i32) -> String {
        match self {
            Formatter::Default => grouped(v),
            Formatter::DivideByTen => decimal(f64::from(v) * 0.1),
            Formatter::Distance => {
                let meters = f64::from(v) / 100.0;
                let km = meters / 1000.0;
                if km > 0.5 {
                    format!("{} km", decimal(km))
                } else if meters > 0.5 {
                    format!("{} m", decimal(meters))
                } else {
                    format!("{v} cm")
                }
            }
            Formatter::Time => {
                let seconds = f64::from(v) / 20.0;
                let minutes = seconds / 60.0;
                let hours = minutes / 60.0;
                let days = hours / 24.0;
                let years = days / 365.0;
                if years > 0.5 {
                    format!("{} y", decimal(years))
                } else if days > 0.5 {
                    format!("{} d", decimal(days))
                } else if hours > 0.5 {
                    format!("{} h", decimal(hours))
                } else if minutes > 0.5 {
                    format!("{} min", decimal(minutes))
                } else {
                    format!("{} s", java_double(seconds))
                }
            }
        }
    }
}

/// VANILLA: BlockItem — id блока для предмета-блока (одноимённая запись реестра).
/// VANILLA-UNSURE: настенные варианты (wall_torch и т. п.) в `Item.BY_BLOCK` указывают на обычный предмет.
pub fn item_block(item: u32) -> Option<u32> {
    let name = reg::ITEM.get(item as usize)?;
    reg::BLOCK.iter().position(|b| b == name).map(|b| b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatters() {
        assert_eq!(Formatter::Default.format(1234567), "1,234,567");
        assert_eq!(Formatter::Default.format(12), "12");
        assert_eq!(Formatter::DivideByTen.format(25), "2.50");
        assert_eq!(Formatter::Distance.format(30), "30 cm");
        assert_eq!(Formatter::Distance.format(250), "2.50 m");
        assert_eq!(Formatter::Distance.format(200_000), "2.00 km");
        assert_eq!(Formatter::Time.format(100), "5.0 s");
        assert_eq!(Formatter::Time.format(1), "0.05 s");
        assert_eq!(Formatter::Time.format(2400), "2.00 min");
    }

    #[test]
    fn parse_award() {
        // 2 записи: (custom=8, jump=?, 5) и (mined=0, блок 1, 3)
        let b = [2, 8, 10, 5, 0, 1, 3];
        let u = parse(&b).unwrap();
        assert_eq!(u, vec![(8, 10, 5), (0, 1, 3)]);
        let mut s = Stats::default();
        s.apply(u);
        assert_eq!(s.get(8, 10), 5);
        assert_eq!(s.get(0, 1), 3);
        assert_eq!(s.get(0, 2), 0);
        assert!(s.received);
    }

    #[test]
    fn block_items() {
        let stone = reg::BLOCK.iter().position(|b| *b == "stone").unwrap() as u32;
        let it = reg::ITEM.iter().position(|i| *i == "stone").unwrap() as u32;
        assert_eq!(item_block(it), Some(stone));
    }
}

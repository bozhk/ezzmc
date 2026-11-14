//! Карты (`filled_map`): данные с сервера, палитра, значки и отрисовка в рамке.
//! VANILLA: ClientboundMapItemDataPacket, MapItemSavedData.MapPatch, MapColor, MapDecorationTypes,
//! MapTextureManager, MapRenderer (26.3).

use std::collections::HashMap;

use ezzmc_protocol::item::ItemStack;
use ezzmc_protocol::nbt;
use ezzmc_protocol::text::Component;
use ezzmc_protocol::wire::Reader;
use ezzmc_protocol::ProtocolResult;
use ezzmc_render::particledraw::{push_vertex_raw, ParticleDraw};
use glam::{Mat4, Vec3};

/// Сторона карты в пикселях (MapRenderer.WIDTH/HEIGHT).
pub const SIZE: usize = 128;
/// Атлас карт: `SLOTS`×`SLOTS` ячеек по 128²; последняя ячейка — значки.
pub const ATLAS: u32 = 2048;
const SLOTS: usize = (ATLAS as usize) / SIZE;
const DECO_SLOT: usize = SLOTS * SLOTS - 1;

/// VANILLA: MapColor — базовые цвета по id (0 — NONE, прозрачный).
const BASE: [u32; 62] = [
    0, 8368696, 16247203, 13092807, 16711680, 10526975, 10987431, 31744, 16777215, 10791096,
    9923917, 7368816, 4210943, 9402184, 16776437, 14188339, 11685080, 6724056, 15066419, 8375321,
    15892389, 5000268, 10066329, 5013401, 8339378, 3361970, 6704179, 6717235, 10040115, 1644825,
    16445005, 6085589, 4882687, 55610, 8476209, 7340544, 13742497, 10441252, 9787244, 7367818,
    12223780, 6780213, 10505550, 3746083, 8874850, 5725276, 8014168, 4996700, 4993571, 5001770,
    9321518, 2430480, 12398641, 9715553, 6035741, 1474182, 3837580, 5647422, 1356933, 6579300,
    14200723, 8365974,
];
/// VANILLA: MapColor.Brightness — LOW, NORMAL, HIGH, LOWEST.
const BRIGHT: [u32; 4] = [180, 220, 255, 135];

/// VANILLA: MapColor#getColorFromPackedId → RGBA.
pub fn color(packed: u8) -> [u8; 4] {
    let id = usize::from(packed >> 2);
    let Some(&c) = BASE.get(id).filter(|_| id != 0) else {
        return [0; 4];
    };
    let m = BRIGHT[usize::from(packed & 3)];
    let ch = |s: u32| ((c >> s & 0xFF) * m / 255) as u8;
    [ch(16), ch(8), ch(0), 255]
}

/// VANILLA: MapDecorationTypes — (asset, showOnItemFrame) в порядке регистрации.
pub const DECO_TYPES: [(&str, bool); 40] = [
    ("player", false),
    ("frame", true),
    ("red_marker", false),
    ("blue_marker", false),
    ("target_x", true),
    ("target_point", true),
    ("player_off_map", false),
    ("player_off_limits", false),
    ("woodland_mansion", true),
    ("ocean_monument", true),
    ("white_banner", true),
    ("orange_banner", true),
    ("magenta_banner", true),
    ("light_blue_banner", true),
    ("yellow_banner", true),
    ("lime_banner", true),
    ("pink_banner", true),
    ("gray_banner", true),
    ("light_gray_banner", true),
    ("cyan_banner", true),
    ("purple_banner", true),
    ("blue_banner", true),
    ("brown_banner", true),
    ("green_banner", true),
    ("red_banner", true),
    ("black_banner", true),
    ("red_x", true),
    ("desert_village", true),
    ("plains_village", true),
    ("savanna_village", true),
    ("snowy_village", true),
    ("taiga_village", true),
    ("jungle_temple", true),
    ("swamp_hut", true),
    ("trial_chambers", true),
    ("abandoned_camp", true),
    ("ancient_city", true),
    ("desert_pyramid", true),
    ("mineshaft", true),
    ("warm_ocean_ruins", true),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Deco {
    pub ty: usize,
    pub x: i8,
    pub y: i8,
    pub rot: u8,
    pub name: Option<Component>,
}

/// Разобранный `map_item_data`.
#[derive(Clone, Debug, PartialEq)]
pub struct MapEvt {
    pub id: i32,
    pub decos: Option<Vec<Deco>>,
    /// (startX, startY, width, height, colors).
    pub patch: Option<(usize, usize, usize, usize, Vec<u8>)>,
}

/// VANILLA: ClientboundMapItemDataPacket.STREAM_CODEC.
pub fn parse(body: &[u8]) -> ProtocolResult<MapEvt> {
    let mut r = Reader::new(body);
    let id = r.var_int()?;
    let _scale = r.i8()?;
    let _locked = r.bool()?;
    let decos = if r.bool()? {
        let n = r.var_int()?.clamp(0, 4096);
        let mut v = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let ty = r.var_int()?.max(0) as usize;
            let (x, y) = (r.i8()?, r.i8()?);
            // VANILLA: MapDecoration — rot & 15.
            let rot = r.u8()? & 15;
            let name = if r.bool()? {
                match nbt::read_network(&mut r)? {
                    Some(t) => Some(Component::from_nbt(&t)?),
                    None => None,
                }
            } else {
                None
            };
            v.push(Deco {
                ty,
                x,
                y,
                rot,
                name,
            });
        }
        Some(v)
    } else {
        None
    };
    // VANILLA: MapPatch#read — ширина 0 = патча нет.
    let w = usize::from(r.u8()?);
    let patch = if w > 0 {
        let h = usize::from(r.u8()?);
        let (sx, sy) = (usize::from(r.u8()?), usize::from(r.u8()?));
        let n = r.var_int()?.max(0) as usize;
        Some((sx, sy, w, h, r.bytes(n)?.to_vec()))
    } else {
        None
    };
    Ok(MapEvt { id, decos, patch })
}

/// Расположение карты и значков в текстуре (UV): начало карты, размер карты, начало значков, размер значка.
#[derive(Clone, Copy)]
pub struct Layout {
    pub map: (f32, f32),
    pub size: (f32, f32),
    pub deco: (f32, f32),
    pub icon: (f32, f32),
}

/// Атлас карт в руке (512×128): основная рука, вторая рука, фон с данными, фон без данных, значки.
pub const HAND_ATLAS: (u32, u32) = (512, 128);

/// Раскладка `HAND_ATLAS` для карты в руке (`slot` 0 — основная, 1 — вторая).
pub fn hand_layout(slot: usize) -> Layout {
    Layout {
        map: (slot as f32 * 0.25, 0.0),
        size: (0.25, 1.0),
        deco: (0.75, 0.5),
        icon: (8.0 / 512.0, 8.0 / 128.0),
    }
}

/// UV фона карты в `HAND_ATLAS`: (u0, v0, u1, v1). VANILLA: map_background[_checkerboard].png.
pub fn hand_background(has_data: bool) -> [f32; 4] {
    if has_data {
        [0.5, 0.0, 0.75, 1.0]
    } else {
        [0.75, 0.0, 0.875, 0.5]
    }
}

fn blit(px: &mut [u8], aw: usize, x: usize, y: usize, img: &[u8], w: usize, h: usize) {
    for r in 0..h {
        let d = (x + (y + r) * aw) * 4;
        px[d..d + w * 4].copy_from_slice(&img[r * w * 4..(r + 1) * w * 4]);
    }
}

fn blit_decos(
    px: &mut [u8],
    aw: usize,
    ox: usize,
    oy: usize,
    rm: &ezzmc_assets::pack::ResourceManager,
) {
    for (i, (name, _)) in DECO_TYPES.iter().enumerate() {
        let path = format!("assets/minecraft/textures/map/decorations/{name}.png");
        if let Some(img) = png(rm, &path, 8) {
            blit(px, aw, ox + (i % 16) * 8, oy + (i / 16) * 8, &img, 8, 8);
        }
    }
}

fn png(rm: &ezzmc_assets::pack::ResourceManager, path: &str, side: u32) -> Option<Vec<u8>> {
    rm.read(path)
        .and_then(|d| ezzmc_assets::texture::Image::decode_png(&d).ok())
        .filter(|i| i.width == side && i.height == side)
        .map(|i| i.to_rgba8())
}

pub struct MapData {
    pub colors: Vec<u8>,
    pub decos: Vec<Deco>,
    /// Растёт при каждом изменении цветов (для перезаливки текстуры карты в руке).
    pub version: u32,
    slot: usize,
    dirty: bool,
}

/// Все карты клиента (VANILLA: ClientLevel#mapData + MapTextureManager).
#[derive(Default)]
pub struct Maps {
    pub map: HashMap<i32, MapData>,
    next: usize,
}

/// `map_id` предмета (VANILLA: DataComponents.MAP_ID).
pub fn map_id(s: &ItemStack) -> Option<i32> {
    use ezzmc_protocol::generated::r26_3::DATA_COMPONENT_TYPE;
    let id = DATA_COMPONENT_TYPE.iter().position(|x| *x == "map_id")? as u32;
    let b = s.patch.added.iter().find(|(i, _)| *i == id)?;
    Reader::new(&b.1).var_int().ok()
}

fn slot_uv(slot: usize) -> (f32, f32) {
    let s = SIZE as f32 / ATLAS as f32;
    ((slot % SLOTS) as f32 * s, (slot / SLOTS) as f32 * s)
}

impl Maps {
    /// VANILLA: ClientPacketListener#handleMapItemData + packet.applyToMap.
    pub fn apply(&mut self, e: MapEvt) {
        if !self.map.contains_key(&e.id) {
            // Ячейки атласа по кругу: при переполнении вытесняется карта, занимавшая ячейку.
            let slot = self.next % DECO_SLOT;
            self.next += 1;
            self.map.retain(|_, m| m.slot != slot);
            self.map.insert(
                e.id,
                MapData {
                    colors: vec![0; SIZE * SIZE],
                    decos: Vec::new(),
                    version: 0,
                    slot,
                    dirty: true,
                },
            );
        }
        let Some(m) = self.map.get_mut(&e.id) else {
            return;
        };
        // VANILLA: MapItemSavedData#addClientSideDecorations — список заменяется целиком.
        if let Some(d) = e.decos {
            m.decos = d;
        }
        // VANILLA: MapPatch#applyToMap.
        if let Some((sx, sy, w, h, colors)) = e.patch {
            for y in 0..h {
                for x in 0..w {
                    let (Some(&c), true) = (colors.get(x + y * w), sx + x < SIZE && sy + y < SIZE)
                    else {
                        continue;
                    };
                    m.colors[sx + x + (sy + y) * SIZE] = c;
                }
            }
            m.dirty = true;
            m.version = m.version.wrapping_add(1);
        }
    }

    /// Пиксели карты RGBA 128². VANILLA: MapTextureManager.MapInstance#updateTextureIfNeeded.
    pub fn rgba(&self, id: i32) -> Option<Vec<u8>> {
        let m = self.map.get(&id)?;
        Some(m.colors.iter().flat_map(|&c| color(c)).collect())
    }

    /// Начальное содержимое `HAND_ATLAS`: фоны и значки.
    pub fn hand_atlas_init(rm: &ezzmc_assets::pack::ResourceManager) -> Vec<u8> {
        let (w, h) = (HAND_ATLAS.0 as usize, HAND_ATLAS.1 as usize);
        let mut px = vec![0u8; w * h * 4];
        let dir = "assets/minecraft/textures/map";
        if let Some(i) = png(rm, &format!("{dir}/map_background_checkerboard.png"), 128) {
            blit(&mut px, w, 256, 0, &i, 128, 128);
        }
        if let Some(i) = png(rm, &format!("{dir}/map_background.png"), 64) {
            blit(&mut px, w, 384, 0, &i, 64, 64);
        }
        blit_decos(&mut px, w, 384, 64, rm);
        px
    }

    /// Начальное содержимое атласа: пусто + значки в последней ячейке (16 в ряд по 8×8).
    pub fn atlas_init(rm: Option<&ezzmc_assets::pack::ResourceManager>) -> Vec<u8> {
        let a = ATLAS as usize;
        let mut px = vec![0u8; a * a * 4];
        let (ox, oy) = ((DECO_SLOT % SLOTS) * SIZE, (DECO_SLOT / SLOTS) * SIZE);
        if let Some(rm) = rm {
            blit_decos(&mut px, a, ox, oy, rm);
        }
        px
    }

    /// VANILLA: MapTextureManager.MapInstance#updateTextureIfNeeded — только изменившиеся карты.
    pub fn upload(&mut self, queue: &wgpu::Queue, draw: &ParticleDraw) {
        for m in self.map.values_mut().filter(|m| m.dirty) {
            m.dirty = false;
            let rgba: Vec<u8> = m.colors.iter().flat_map(|&c| color(c)).collect();
            let (x, y) = ((m.slot % SLOTS) * SIZE, (m.slot / SLOTS) * SIZE);
            draw.write_region(queue, x as u32, y as u32, SIZE as u32, SIZE as u32, &rgba);
        }
    }

    /// Карта в координатах 0..128 (матрица `m` уже содержит масштаб 1/128 и сдвиги вызывающего).
    /// VANILLA: MapRenderer#render. Свет — 0..240. `lay` — раскладка текстуры (None = атлас мира).
    /// `false`, если данных карты нет.
    pub fn bake(
        &self,
        id: i32,
        m: Mat4,
        only_frame: bool,
        light: (u16, u16),
        lay: Option<Layout>,
        out: &mut Vec<u8>,
    ) -> bool {
        let Some(map) = self.map.get(&id) else {
            return false;
        };
        let lay = lay.unwrap_or_else(|| {
            let s = SIZE as f32 / ATLAS as f32;
            let t = 8.0 / ATLAS as f32;
            Layout {
                map: slot_uv(map.slot),
                size: (s, s),
                deco: slot_uv(DECO_SLOT),
                icon: (t, t),
            }
        });
        let ((u, v), (su, sv)) = (lay.map, lay.size);
        let mut quad = |m: Mat4, c: [([f32; 3], [f32; 2]); 4]| {
            for (p, uv) in c {
                let q = m.transform_point3(Vec3::from(p));
                push_vertex_raw(out, q.to_array(), uv, [255; 4], light);
            }
        };
        quad(
            m,
            [
                ([0.0, 128.0, -0.01], [u, v + sv]),
                ([128.0, 128.0, -0.01], [u + su, v + sv]),
                ([128.0, 0.0, -0.01], [u + su, v]),
                ([0.0, 0.0, -0.01], [u, v]),
            ],
        );
        let ((du, dv), (tu, tv)) = (lay.deco, lay.icon);
        let mut count = 0;
        for d in &map.decos {
            let Some(&(_, on_frame)) = DECO_TYPES.get(d.ty) else {
                continue;
            };
            if only_frame && !on_frame {
                continue;
            }
            let dm =
                m * Mat4::from_translation(Vec3::new(
                    f32::from(d.x) / 2.0 + 64.0,
                    f32::from(d.y) / 2.0 + 64.0,
                    -0.02,
                )) * Mat4::from_rotation_z((f32::from(d.rot) * 360.0 / 16.0).to_radians())
                    * Mat4::from_scale(Vec3::new(4.0, 4.0, 3.0))
                    * Mat4::from_translation(Vec3::new(-0.125, 0.125, 0.0));
            let (u0, v0) = (du + (d.ty % 16) as f32 * tu, dv + (d.ty / 16) as f32 * tv);
            let z = count as f32 * -0.001;
            quad(
                dm,
                [
                    ([-1.0, 1.0, z], [u0, v0]),
                    ([1.0, 1.0, z], [u0 + tu, v0]),
                    ([1.0, -1.0, z], [u0 + tu, v0 + tv]),
                    ([-1.0, -1.0, z], [u0, v0 + tv]),
                ],
            );
            // TODO-526: подписи значков (decoration.name) в мире не рисуются.
            count += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette() {
        assert_eq!(color(0), [0; 4]);
        assert_eq!(color(3), [0; 4]);
        // GRASS (8368696 = 7F B2 38): HIGH — как есть, LOW ×180/255.
        assert_eq!(color(1 << 2 | 2), [0x7F, 0xB2, 0x38, 255]);
        assert_eq!(color(1 << 2), [89, 125, 39, 255]);
    }

    #[test]
    fn packet_patch_and_decos() {
        // id 5, scale 0, locked, 1 значок (frame, x=-10, y=20, rot 0x13→3, без имени), патч 2×1 в (3,4).
        let body = [5, 0, 1, 1, 1, 1, 0xF6, 20, 0x13, 0, 2, 1, 3, 4, 2, 4, 8];
        let e = parse(&body).unwrap();
        assert_eq!(e.id, 5);
        let d = &e.decos.as_ref().unwrap()[0];
        assert_eq!((d.ty, d.x, d.y, d.rot), (1, -10, 20, 3));
        let mut maps = Maps::default();
        maps.apply(e);
        let m = &maps.map[&5];
        assert_eq!(m.colors[3 + 4 * SIZE], 4);
        assert_eq!(m.colors[4 + 4 * SIZE], 8);
        // Без значков и патча: значки остаются.
        maps.apply(parse(&[5, 0, 1, 0, 0]).unwrap());
        assert_eq!(maps.map[&5].decos.len(), 1);
    }
}

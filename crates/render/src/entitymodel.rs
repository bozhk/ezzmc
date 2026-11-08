//! База моделей сущностей (игрок, мобы, ...). VANILLA: ModelPart / ModelPart.Cube / PartPose /
//! CubeListBuilder / LayerDefinition (26.3). Части лежат плоским списком «родитель раньше детей»;
//! поза каждой части: сдвиг в пикселях (1/16 блока) + повороты в радианах (порядок ZYX).
//! Пространство модели — как в ванили: Y вниз, один блок = 16 единиц.

use crate::particledraw::push_vertex;
use glam::{Mat4, Quat, Vec3};

/// VANILLA: PartPose (x, y, z, xRot, yRot, zRot).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x_rot: f32,
    pub y_rot: f32,
    pub z_rot: f32,
}

impl Pose {
    pub const fn offset(x: f32, y: f32, z: f32) -> Self {
        Pose {
            x,
            y,
            z,
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
        }
    }

    /// VANILLA: ModelPart#translateAndRotate.
    pub fn matrix(&self) -> Mat4 {
        let t = Mat4::from_translation(Vec3::new(self.x, self.y, self.z) / 16.0);
        if self.x_rot == 0.0 && self.y_rot == 0.0 && self.z_rot == 0.0 {
            return t;
        }
        // Quaternionf#rotationZYX(z, y, x) = Rz · Ry · Rx.
        t * Mat4::from_quat(Quat::from_euler(
            glam::EulerRot::ZYX,
            self.z_rot,
            self.y_rot,
            self.x_rot,
        ))
    }
}

#[derive(Clone, Copy, Debug)]
struct Vert {
    pos: [f32; 3],
    uv: [f32; 2],
}

/// Грань куба: 4 вершины и нормаль.
#[derive(Clone, Copy, Debug)]
struct Polygon {
    v: [Vert; 4],
    normal: [f32; 3],
}

/// VANILLA: ModelPart.Cube.
#[derive(Clone, Debug)]
pub struct Cube {
    polys: [Polygon; 6],
}

impl Cube {
    /// VANILLA: ModelPart.Cube#<init>. `origin`/`dim` в пикселях, `grow` — раздутие по осям,
    /// `tex` — левый верхний угол развёртки в пикселях, `tex_size` — размер текстуры.
    pub fn new(
        origin: [f32; 3],
        dim: [f32; 3],
        grow: [f32; 3],
        mirror: bool,
        tex: [f32; 2],
        tex_size: [f32; 2],
    ) -> Self {
        let (w, h, d) = (dim[0], dim[1], dim[2]);
        let mut x0 = origin[0];
        let mut y0 = origin[1];
        let mut z0 = origin[2];
        let mut x1 = x0 + w;
        let y1_ = y0 + h;
        let z1_ = z0 + d;
        x0 -= grow[0];
        y0 -= grow[1];
        z0 -= grow[2];
        x1 += grow[0];
        let y1 = y1_ + grow[1];
        let z1 = z1_ + grow[2];
        if mirror {
            std::mem::swap(&mut x0, &mut x1);
        }
        let p = |x, y, z| [x, y, z];
        let v000 = p(x0, y0, z0);
        let v100 = p(x1, y0, z0);
        let v110 = p(x1, y1, z0);
        let v010 = p(x0, y1, z0);
        let v011 = p(x0, y1, z1);
        let v001 = p(x0, y0, z1);
        let v101 = p(x1, y0, z1);
        let v111 = p(x1, y1, z1);
        let (tx, ty) = (tex[0], tex[1]);
        let u0 = tx;
        let u1 = tx + d;
        let u2 = tx + d + w;
        let u22 = tx + d + w + w;
        let u3 = tx + d + w + d;
        let u4 = tx + d + w + d + w;
        let v0 = ty;
        let v1 = ty + d;
        let v2 = ty + d + h;
        let mk = |verts: [[f32; 3]; 4], ua: f32, va: f32, ub: f32, vb: f32, n: [f32; 3]| {
            let uv = |u: f32, v: f32| [u / tex_size[0], v / tex_size[1]];
            // VANILLA: Polygon#<init> — порядок UV (u1,v0) (u0,v0) (u0,v1) (u1,v1).
            let mut vs = [
                Vert {
                    pos: verts[0],
                    uv: uv(ub, va),
                },
                Vert {
                    pos: verts[1],
                    uv: uv(ua, va),
                },
                Vert {
                    pos: verts[2],
                    uv: uv(ua, vb),
                },
                Vert {
                    pos: verts[3],
                    uv: uv(ub, vb),
                },
            ];
            let mut normal = n;
            if mirror {
                vs.swap(0, 3);
                vs.swap(1, 2);
                normal[0] = -normal[0];
            }
            Polygon { v: vs, normal }
        };
        Cube {
            polys: [
                // DOWN, UP, WEST, NORTH, EAST, SOUTH (порядок и вершины как в ModelPart.Cube 26.3)
                mk([v101, v001, v000, v100], u1, v0, u2, v1, [0.0, -1.0, 0.0]),
                mk([v110, v010, v011, v111], u2, v1, u22, v0, [0.0, 1.0, 0.0]),
                mk([v000, v001, v011, v010], u0, v1, u1, v2, [-1.0, 0.0, 0.0]),
                mk([v100, v000, v010, v110], u1, v1, u2, v2, [0.0, 0.0, -1.0]),
                mk([v101, v100, v110, v111], u2, v1, u3, v2, [1.0, 0.0, 0.0]),
                mk([v001, v101, v111, v011], u3, v1, u4, v2, [0.0, 0.0, 1.0]),
            ],
        }
    }
}

/// VANILLA: ModelPart.
#[derive(Clone, Debug)]
pub struct Part {
    pub name: &'static str,
    parent: Option<usize>,
    pub initial: Pose,
    pub pose: Pose,
    /// VANILLA: ModelPart#xScale/yScale/zScale (меняется keyframe-анимацией).
    pub scale: Vec3,
    pub visible: bool,
    cubes: Vec<Cube>,
}

/// Модель: набор частей + размер текстуры. VANILLA: Model + LayerDefinition (baked).
#[derive(Clone, Debug)]
pub struct Model {
    pub parts: Vec<Part>,
    pub tex_size: [f32; 2],
}

/// Описание куба для билдера (VANILLA: CubeListBuilder#addBox).
#[derive(Clone, Copy, Debug)]
pub struct BoxDef {
    pub tex: [f32; 2],
    pub origin: [f32; 3],
    pub dim: [f32; 3],
    pub grow: f32,
    pub mirror: bool,
}

impl BoxDef {
    pub const fn new(tex: [f32; 2], origin: [f32; 3], dim: [f32; 3]) -> Self {
        BoxDef {
            tex,
            origin,
            dim,
            grow: 0.0,
            mirror: false,
        }
    }
    pub const fn grow(mut self, g: f32) -> Self {
        self.grow = g;
        self
    }
    pub const fn mirror(mut self) -> Self {
        self.mirror = true;
        self
    }
}

impl Model {
    pub fn new(tex_size: [f32; 2]) -> Self {
        Model {
            parts: Vec::new(),
            tex_size,
        }
    }

    /// VANILLA: PartDefinition#addOrReplaceChild. Возвращает индекс части.
    pub fn add(
        &mut self,
        name: &'static str,
        parent: Option<usize>,
        pose: Pose,
        boxes: &[BoxDef],
    ) -> usize {
        let cubes = boxes
            .iter()
            .map(|b| Cube::new(b.origin, b.dim, [b.grow; 3], b.mirror, b.tex, self.tex_size))
            .collect();
        self.parts.push(Part {
            name,
            parent,
            initial: pose,
            pose,
            scale: Vec3::ONE,
            visible: true,
            cubes,
        });
        self.parts.len() - 1
    }

    pub fn index(&self, name: &str) -> Option<usize> {
        self.parts.iter().position(|p| p.name == name)
    }

    /// VANILLA: Model#resetPose — все части в исходное положение.
    pub fn reset_pose(&mut self) {
        for p in &mut self.parts {
            p.pose = p.initial;
            p.scale = Vec3::ONE;
        }
    }

    /// Матрицы частей относительно `root` (VANILLA: ModelPart#render, стек поз).
    pub fn part_matrices(&self, root: Mat4) -> Vec<Mat4> {
        let mut m: Vec<Mat4> = Vec::with_capacity(self.parts.len());
        for p in &self.parts {
            let base = p.parent.map_or(root, |i| m[i]);
            let mut local = p.pose.matrix();
            if p.scale != Vec3::ONE {
                local *= Mat4::from_scale(p.scale);
            }
            m.push(base * local);
        }
        m
    }

    /// Запекает видимые части в вершины формата `ParticleDraw` (32 Б, квады по 4 вершины).
    /// `root` переводит модель в пространство, где камера в начале координат.
    /// `light` — (block, sky) 0..15. Освещение граней: VANILLA minecraft_mix_light (LEVEL).
    /// Возвращает число квадов.
    pub fn bake(&self, root: Mat4, color: [u8; 4], light: (u8, u8), out: &mut Vec<u8>) -> u32 {
        let mats = self.part_matrices(root);
        let mut quads = 0;
        for (i, part) in self.parts.iter().enumerate() {
            if !part.visible {
                continue;
            }
            // Невидимый родитель скрывает и детей (VANILLA: ModelPart#render).
            let mut hidden = false;
            let mut cur = part.parent;
            while let Some(p) = cur {
                if !self.parts[p].visible {
                    hidden = true;
                    break;
                }
                cur = self.parts[p].parent;
            }
            if hidden {
                continue;
            }
            let m = mats[i];
            for cube in &part.cubes {
                for poly in &cube.polys {
                    let n = m
                        .transform_vector3(Vec3::from(poly.normal))
                        .normalize_or_zero();
                    let shade = mix_light(n);
                    let c = [
                        (f32::from(color[0]) * shade) as u8,
                        (f32::from(color[1]) * shade) as u8,
                        (f32::from(color[2]) * shade) as u8,
                        color[3],
                    ];
                    for v in &poly.v {
                        let p = m.transform_point3(Vec3::from(v.pos) / 16.0);
                        push_vertex(out, p.to_array(), v.uv, c, light);
                    }
                    quads += 1;
                }
            }
        }
        quads
    }
}

/// VANILLA: Lighting (Entry.LEVEL) — направления света и minecraft_mix_light.
fn mix_light(n: Vec3) -> f32 {
    let l0 = Vec3::new(0.2, 1.0, -0.7).normalize();
    let l1 = Vec3::new(-0.2, 1.0, 0.7).normalize();
    let (a, b) = (l0.dot(n).max(0.0), l1.dot(n).max(0.0));
    ((a + b) * 0.6 + 0.4).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_uv_layout_matches_vanilla_head() {
        // Голова 8×8×8 в (0,0), текстура 64×64: верх занимает u 8..16, v 0..8.
        let c = Cube::new(
            [-4.0, -8.0, -4.0],
            [8.0; 3],
            [0.0; 3],
            false,
            [0.0, 0.0],
            [64.0, 64.0],
        );
        let up = &c.polys[0]; // «DOWN» в пространстве Y-вниз = верх головы
        let us: Vec<f32> = up.v.iter().map(|v| v.uv[0] * 64.0).collect();
        let vs: Vec<f32> = up.v.iter().map(|v| v.uv[1] * 64.0).collect();
        assert_eq!(us.iter().cloned().fold(f32::MAX, f32::min), 8.0);
        assert_eq!(us.iter().cloned().fold(f32::MIN, f32::max), 16.0);
        assert_eq!(vs.iter().cloned().fold(f32::MIN, f32::max), 8.0);
    }

    #[test]
    fn bake_counts_six_quads_per_cube() {
        let mut m = Model::new([64.0, 64.0]);
        m.add(
            "a",
            None,
            Pose::default(),
            &[BoxDef::new([0.0; 2], [0.0; 3], [1.0; 3])],
        );
        let mut out = Vec::new();
        assert_eq!(m.bake(Mat4::IDENTITY, [255; 4], (0, 15), &mut out), 6);
        assert_eq!(out.len(), 6 * 4 * 32);
    }

    #[test]
    fn child_follows_parent_pose() {
        let mut m = Model::new([16.0, 16.0]);
        let p = m.add("p", None, Pose::offset(16.0, 0.0, 0.0), &[]);
        m.add("c", Some(p), Pose::offset(0.0, 16.0, 0.0), &[]);
        let mats = m.part_matrices(Mat4::IDENTITY);
        assert_eq!(mats[1].w_axis.truncate(), Vec3::new(1.0, 1.0, 0.0));
    }
}

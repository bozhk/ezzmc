//! Сортировка прозрачных квадов по расстоянию до камеры (TODO-142).
//! VANILLA: MeshData#unpackQuadCentroids / SortState#buildSortedIndexBuffer / VertexSorting#byDistance /
//! TranslucencyPointOfView (1.21.11).
use crate::vertex::PackedVertex;

/// Центроиды квадов слоя (`(v0 + v2) / 2`) в локальных координатах секции.
#[derive(Clone, Debug, Default)]
pub struct SortState {
    centroids: Vec<[f32; 3]>,
}

impl SortState {
    pub fn from_vertices(v: &[PackedVertex]) -> SortState {
        let centroids = v
            .chunks_exact(4)
            .map(|q| {
                let (a, c) = (q[0].position(), q[2].position());
                [
                    (a[0] + c[0]) / 2.0,
                    (a[1] + c[1]) / 2.0,
                    (a[2] + c[2]) / 2.0,
                ]
            })
            .collect();
        SortState { centroids }
    }

    pub fn quad_count(&self) -> usize {
        self.centroids.len()
    }

    /// Порядок квадов от дальнего к ближнему (стабильно, как `IntArrays.mergeSort`).
    /// `camera` — позиция камеры относительно начала секции.
    pub fn sorted_order(&self, camera: [f32; 3]) -> Vec<u32> {
        let d: Vec<f32> = self
            .centroids
            .iter()
            .map(|c| {
                let (x, y, z) = (c[0] - camera[0], c[1] - camera[1], c[2] - camera[2]);
                x * x + y * y + z * z
            })
            .collect();
        let mut idx: Vec<u32> = (0..self.centroids.len() as u32).collect();
        idx.sort_by(|&a, &b| d[b as usize].total_cmp(&d[a as usize]));
        idx
    }

    /// Индексный буфер: на квад `0,1,2,2,3,0` (+4·номер квада).
    pub fn sorted_indices(&self, camera: [f32; 3]) -> Vec<u32> {
        let mut out = Vec::with_capacity(self.centroids.len() * 6);
        for q in self.sorted_order(camera) {
            let b = q * 4;
            out.extend_from_slice(&[b, b + 1, b + 2, b + 2, b + 3, b]);
        }
        out
    }
}

/// Индексы в порядке исходной генерации (не прозрачные слои): `0,1,2,2,3,0`.
pub fn identity_indices(quads: usize) -> Vec<u32> {
    let mut out = Vec::with_capacity(quads * 6);
    for q in 0..quads as u32 {
        let b = q * 4;
        out.extend_from_slice(&[b, b + 1, b + 2, b + 2, b + 3, b]);
    }
    out
}

/// VANILLA: TranslucencyPointOfView — положение камеры относительно секции, по осям −1/0/1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointOfView {
    x: i8,
    y: i8,
    z: i8,
}

impl PointOfView {
    pub fn of(camera: [f64; 3], section: [i32; 3]) -> PointOfView {
        let c = |v: f64, s: i32| ((v.floor() as i32 >> 4) - s).clamp(-1, 1) as i8;
        PointOfView {
            x: c(camera[0], section[0]),
            y: c(camera[1], section[1]),
            z: c(camera[2], section[2]),
        }
    }

    pub fn is_axis_aligned(&self) -> bool {
        self.x == 0 || self.y == 0 || self.z == 0
    }

    /// Нужна ли пересортировка: точка обзора сменилась, либо камера «в створе» секции (порядок меняется при каждом шаге).
    // VANILLA: SectionRenderDispatcher — `!isDifferentPointOfView(pov) && !pov.isAxisAligned()` → пропустить
    pub fn needs_resort(old: Option<PointOfView>, new: PointOfView) -> bool {
        old != Some(new) || new.is_axis_aligned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quad(x: f32, z: f32) -> [PackedVertex; 4] {
        let p = |x, z| PackedVertex::pack([x, 0.0, z], [0.0; 2], [255; 4], 0, 0);
        [p(x, z), p(x, z + 1.0), p(x + 1.0, z + 1.0), p(x + 1.0, z)]
    }

    #[test]
    fn far_to_near_and_stable() {
        let mut v = Vec::new();
        for x in [2.0, 8.0, 5.0] {
            v.extend(quad(x, 0.0));
        }
        let s = SortState::from_vertices(&v);
        assert_eq!(s.sorted_order([0.0, 0.0, 0.0]), vec![1, 2, 0]);
        assert_eq!(s.sorted_order([16.0, 0.0, 0.0]), vec![0, 2, 1]);
        let i = s.sorted_indices([0.0; 3]);
        assert_eq!(&i[..6], &[4, 5, 6, 6, 7, 4]);
        // равные расстояния — исходный порядок
        let mut w = quad(4.0, 0.0).to_vec();
        w.extend(quad(4.0, 0.0));
        assert_eq!(
            SortState::from_vertices(&w).sorted_order([0.0; 3]),
            vec![0, 1]
        );
        assert_eq!(identity_indices(1), vec![0, 1, 2, 2, 3, 0]);
    }

    #[test]
    fn point_of_view_resort_rules() {
        let a = PointOfView::of([8.0, 8.0, 8.0], [0, 0, 0]);
        assert!(a.is_axis_aligned() && PointOfView::needs_resort(Some(a), a));
        let far = PointOfView::of([100.0, 100.0, 100.0], [0, 0, 0]);
        assert!(!far.is_axis_aligned());
        assert!(!PointOfView::needs_resort(Some(far), far));
        assert!(PointOfView::needs_resort(None, far));
        assert!(PointOfView::needs_resort(
            Some(far),
            PointOfView::of([-100.0, 100.0, 100.0], [0, 0, 0])
        ));
    }
}

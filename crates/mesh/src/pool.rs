//! Многопоточная сборка мешей (TODO-134): rayon-пул, результаты — через crossbeam-канал,
//! устаревшие результаты отбрасываются по счётчику поколения секции. Рендер-поток не блокируется.
use crate::section::{mesh_section, ModelProvider, Neighborhood, SectionMesh, SectionPos};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MeshResult {
    pub pos: SectionPos,
    pub mesh: SectionMesh,
}

pub struct MeshPool {
    pool: rayon::ThreadPool,
    tx: Sender<(SectionPos, u64, SectionMesh)>,
    rx: Receiver<(SectionPos, u64, SectionMesh)>,
    generation: HashMap<SectionPos, u64>,
    pending: usize,
}

impl MeshPool {
    pub fn new(threads: usize) -> Result<Self, rayon::ThreadPoolBuildError> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads.max(1))
            .thread_name(|i| format!("mesh-{i}"))
            .build()?;
        let (tx, rx) = unbounded();
        Ok(Self {
            pool,
            tx,
            rx,
            generation: HashMap::new(),
            pending: 0,
        })
    }

    /// Поставить секцию в сборку; более ранние незавершённые результаты этой секции станут устаревшими.
    pub fn submit(&mut self, pos: SectionPos, hood: Neighborhood, models: Arc<dyn ModelProvider>) {
        let g = self.generation.entry(pos).or_insert(0);
        *g += 1;
        let gen = *g;
        let tx = self.tx.clone();
        self.pending += 1;
        self.pool.spawn(move || {
            let mesh = mesh_section(pos, &hood, models.as_ref());
            let _ = tx.send((pos, gen, mesh));
        });
    }

    /// Секция выгружена: все её результаты в полёте устаревают.
    pub fn cancel(&mut self, pos: &SectionPos) {
        if let Some(g) = self.generation.get_mut(pos) {
            *g += 1;
        }
    }

    /// Готовые, актуальные результаты (не блокирует).
    pub fn poll(&mut self) -> Vec<MeshResult> {
        let mut out = Vec::new();
        while let Ok((pos, gen, mesh)) = self.rx.try_recv() {
            self.pending -= 1;
            if self.generation.get(&pos) == Some(&gen) {
                out.push(MeshResult { pos, mesh });
            }
        }
        out
    }

    pub fn pending(&self) -> usize {
        self.pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::RenderLayer;
    use crate::section::ModelPart;
    use ezzmc_world::block::BlockState;
    use std::time::{Duration, Instant};

    struct NoModels;
    impl ModelProvider for NoModels {
        fn parts<'a>(&'a self, _: BlockState, _: i32, _: i32, _: i32, _: &mut Vec<ModelPart<'a>>) {}
        fn layer(&self, _: BlockState) -> RenderLayer {
            RenderLayer::Solid
        }
        fn tint(
            &self,
            _: BlockState,
            _: i32,
            _: &Neighborhood,
            _: SectionPos,
            _: i32,
            _: i32,
            _: i32,
        ) -> [u8; 3] {
            [255; 3]
        }
    }

    #[test]
    fn results_delivered_and_stale_dropped() {
        let mut p = MeshPool::new(2).unwrap_or_else(|_| unreachable!());
        let a = SectionPos { x: 0, y: 0, z: 0 };
        let b = SectionPos { x: 1, y: 0, z: 0 };
        let m: Arc<dyn ModelProvider> = Arc::new(NoModels);
        p.submit(a, Neighborhood::default(), m.clone());
        p.submit(a, Neighborhood::default(), m.clone());
        p.submit(b, Neighborhood::default(), m);
        p.cancel(&b);
        let mut got = Vec::new();
        let t = Instant::now();
        while p.pending() > 0 && t.elapsed() < Duration::from_secs(5) {
            got.extend(p.poll());
            std::thread::sleep(Duration::from_millis(5));
        }
        got.extend(p.poll());
        assert_eq!(p.pending(), 0);
        assert_eq!(got.len(), 1); // a: актуальное поколение 2; b отменена
        assert_eq!(got[0].pos, a);
        assert!(got[0].mesh.is_empty());
    }
}

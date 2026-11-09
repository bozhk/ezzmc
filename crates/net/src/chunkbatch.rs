//! VANILLA: ChunkBatchSizeCalculator — сколько чанков за тик просить у сервера по скорости приёма.
use std::time::Instant;

pub struct ChunkBatchSize {
    aggregated_nanos_per_chunk: f64,
    old_samples_weight: u32,
    start: Instant,
}

impl Default for ChunkBatchSize {
    fn default() -> Self {
        Self {
            aggregated_nanos_per_chunk: 2_000_000.0,
            old_samples_weight: 1,
            start: Instant::now(),
        }
    }
}

impl ChunkBatchSize {
    pub fn on_batch_start(&mut self) {
        self.start = Instant::now();
    }

    pub fn on_batch_finished(&mut self, size: i32) {
        if size > 0 {
            let per_chunk = self.start.elapsed().as_nanos() as f64 / f64::from(size);
            let a = self.aggregated_nanos_per_chunk;
            let v = per_chunk.clamp(a / 3.0, a * 3.0);
            let w = f64::from(self.old_samples_weight);
            self.aggregated_nanos_per_chunk = (a * w + v) / (w + 1.0);
            self.old_samples_weight = (self.old_samples_weight + 1).min(49);
        }
    }

    pub fn desired_chunks_per_tick(&self) -> f32 {
        (7_000_000.0 / self.aggregated_nanos_per_chunk) as f32
    }
}

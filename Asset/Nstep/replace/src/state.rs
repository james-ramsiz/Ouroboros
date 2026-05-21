use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use crate::config::SimConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct Checkpoint {
    pub seed: u64,
    pub mutation_rate: f32,
    pub color_phase: f32,
    pub frame_count: u64,
    pub width: u32,
    pub height: u32,
    pub history: VecDeque<(f32, f32)>,
}

pub struct SimState {
    pub rng: SmallRng,
    pub mutation_rate: f32,
    pub color_phase: f32,
    pub frame_count: u64,
    pub width: u32,
    pub height: u32,
    pub history: VecDeque<(f32, f32)>,
    pub history_len: usize,
    // Adaptive resolution bounds
    pub min_res: u32,
    pub max_res: u32,
    pub res_step: u32,
    pub adaptive_enabled: bool,
}

impl SimState {
    pub fn new(cfg: &SimConfig) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(cfg.seed),
            mutation_rate: cfg.rules.base_mutation_rate,
            color_phase: 0.0,
            frame_count: 0,
            width: cfg.general.width,
            height: cfg.general.height,
            history: VecDeque::with_capacity(cfg.history.length),
            history_len: cfg.history.length,
            min_res: cfg.adaptive.min_res,
            max_res: cfg.adaptive.max_res,
            res_step: cfg.adaptive.resolution_step,
            adaptive_enabled: cfg.adaptive.enabled,
        }
    }

    pub fn load_checkpoint(cfg: &SimConfig, data: &[u8]) -> Self {
        let cp: Checkpoint = serde_json::from_slice(data).expect("Invalid checkpoint JSON");
        Self {
            rng: SmallRng::seed_from_u64(cp.seed),
            mutation_rate: cp.mutation_rate,
            color_phase: cp.color_phase,
            frame_count: cp.frame_count,
            width: cp.width,
            height: cp.height,
            history: cp.history,
            history_len: cfg.history.length,
            min_res: cfg.adaptive.min_res,
            max_res: cfg.adaptive.max_res,
            res_step: cfg.adaptive.resolution_step,
            adaptive_enabled: cfg.adaptive.enabled,
        }
    }

    pub fn save_checkpoint(&self) -> Vec<u8> {
        let cp = Checkpoint {
            seed: 42, // Simplified: SmallRng state isn't trivially serializable, but seed+frame is usually enough for determinism
            mutation_rate: self.mutation_rate,
            color_phase: self.color_phase,
            frame_count: self.frame_count,
            width: self.width,
            height: self.height,
            history: self.history.clone(),
        };
        serde_json::to_vec_pretty(&cp).expect("Checkpoint serialization failed")
    }

    /// Meta feedback with N-step memory & adaptive resolution
    pub fn evolve_rules(&mut self, ssim: f32, entropy: f32, cfg: &SimConfig) {
        // Push to history ring buffer
        self.history.push_back((ssim, entropy));
        if self.history.len() > self.history_len {
            self.history.pop_front();
        }

        // Weighted historical metrics (recent frames matter more)
        let (hist_ssim, hist_entropy) = self.history.iter().enumerate().fold((0.0, 0.0), |(s, e), (i, (ss, en))| {
            let w = (i as f32 + 1.0) / self.history_len as f32;
            (s + ss * w, e + en * w)
        });

        // Adaptive resolution drift
        if self.adaptive_enabled {
            let stability = hist_ssim;
            let chaos = hist_entropy;
            
            // High stability -> expand resolution to force complexity
            // High chaos -> contract resolution to restore order
            if stability > 0.6 && chaos < 0.4 {
                self.width = (self.width + self.res_step).min(self.max_res);
                self.height = (self.height + self.res_step).min(self.max_res);
            } else if stability < 0.3 && chaos > 0.7 {
                self.width = (self.width - self.res_step).max(self.min_res);
                self.height = (self.height - self.res_step).max(self.min_res);
            }
            // Snap to grid
            self.width = (self.width / self.res_step) * self.res_step;
            self.height = (self.height / self.res_step) * self.res_step;
        }

        // Rule mutation driven by historical pressure
        let divergence = (1.0 - hist_ssim) * 0.02;
        let damping = (1.0 - hist_entropy) * 0.015;
        
        self.mutation_rate = (self.mutation_rate + divergence - damping * 0.5).clamp(0.005, 0.25);
        self.color_phase += cfg.rules.color_shift_speed + damping - divergence * 0.3;
        self.frame_count += 1;
    }
}

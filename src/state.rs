use crate::config::SimConfig;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct SimState {
    pub rng: SmallRng,
    pub mutation_rate: f32,
    pub color_phase: f32,
    pub frame_count: u64,
    pub width: u32,
    pub height: u32,
}

impl SimState {
    pub fn new(cfg: &SimConfig) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(cfg.seed),
            mutation_rate: cfg.base_mutation_rate,
            color_phase: 0.0,
            frame_count: 0,
            width: cfg.width,
            height: cfg.height,
        }
    }

    /// Meta feedback: adjust rules based on self-similarity/entropy metrics
    pub fn evolve_rules(&mut self, cfg: &SimConfig, prev_entropy: f32) {
        let drift = self.rng.gen_range(-0.01..0.01) * cfg.feedback_weight;
        self.mutation_rate = (self.mutation_rate + drift).clamp(0.01, 0.5);
        self.color_phase += cfg.color_shift_speed + drift;
        self.frame_count += 1;
    }
}

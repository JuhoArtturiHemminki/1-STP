use crate::fractal_feedback::{PHI, H_C};

pub struct TerminalPreserver {
    pub cathodic_bias: f64,
    pub oxidation_inhibitor: f64,
}

impl TerminalPreserver {
    pub const fn new() -> Self {
        Self {
            cathodic_bias: 0.0,
            oxidation_inhibitor: 1.0,
        }
    }

    pub fn sync_potential(&mut self, ambient_noise: f64, drift: &mut f64) 
-> f64 {
        let potential_shift = (ambient_noise * PHI) / H_C;
        *drift += potential_shift * 0.1;
        self.cathodic_bias = (potential_shift * 1000.0).abs();
        self.oxidation_inhibitor = 1.0 / (1.0 + (*drift).abs());
        self.oxidation_inhibitor
    }
}


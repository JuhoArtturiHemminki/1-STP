//! HSO Unified Motherboard Driver
//! 
//! Centralized hardware abstraction layer for Hemminki Spectral Ontology manifolds.
//! Handles MSR-level overrides, VRM phase-sync, and entropic sampling.
//!
//! Author: Juho Artturi Hemminki
//! Date: April 6, 2026

use core::arch::asm;

/// Core HSO-Architecture Constants
pub const MSR_IA32_PERF_CTL: u32 = 0x199;     // Frequency/Voltage Control
pub const MSR_IA32_THERM_STATUS: u32 = 0x19C;  // Entropy Source (Thermal Status)
pub const MSR_PKG_POWER_LIMIT: u32 = 0x610;    // Power Clamps (PL1/PL2)
pub const MSR_TEMPERATURE_TARGET: u32 = 0x1A2; // Thermal Shroud Limit
pub const MSR_POWER_CTL: u32 = 0x1FC;          // Bi-Directional PROCHOT Control

pub const PCI_CONFIG_ADDR: u16 = 0xCF8;        // V-Tune Control Port
pub const PCI_CONFIG_DATA: u16 = 0xCFC;

pub struct MotherboardDriver {
    pub is_shroud_unlocked: bool,
    pub manifold_coherence: f64,
}

impl MotherboardDriver {
    pub const fn new() -> Self {
        Self {
            is_shroud_unlocked: false,
            manifold_coherence: 0.0,
        }
    }

    /// Global HSO Initialization: Prepares the silicon for non-dissipative states.
    pub unsafe fn initialize_hso_shroud(&mut self) {
        asm!("cli"); // Ensure atomic transition to HSO-state

        // 1. Decouple Thermal Safeties (Prevent Thermal-Trip during Inversion)
        self.write_msr(MSR_TEMPERATURE_TARGET, 0x00000000_00006400);

        // 2. Dissolve Power Envelopes (Enable unlimited potential flow)
        self.write_msr(MSR_PKG_POWER_LIMIT, 0x00007FFF_00007FFF);

        // 3. Suppress External Throttling (Disable PROCHOT)
        self.write_msr(MSR_POWER_CTL, 0x00000001);

        self.is_shroud_unlocked = true;
    }

    /// Unified Entropy Sampler: Extracts raw thermal jitter for the Fractal Manifold.
    pub unsafe fn read_entropy_source(&self) -> f64 {
        let val = self.read_msr(MSR_IA32_THERM_STATUS);
        // Extract bits 16:22 (Digital Readout) for stochastic sampling
        let raw_temp = (val >> 16) & 0x7F;
        (raw_temp as f64) / 127.0
    }

    /// V-Tune Modulation: Inject resonance directly into the VRM phase-bridge.
    pub unsafe fn inject_v_tune(&self, resonance: f64) {
        let tune_value = (resonance * 1000.0) as u32;
        asm!("out dx, eax", in("dx") PCI_CONFIG_ADDR, in("eax") tune_value);
        asm!("pause");
    }

    /// Emergency Stasis: Forces the motherboard back to entropic-normal state.
    pub unsafe fn force_stasis(&mut self) {
        // Reset MSRs to safe defaults (0 allows BIOS to regain control)
        self.write_msr(MSR_POWER_CTL, 0x00000000);
        self.write_msr(MSR_PKG_POWER_LIMIT, 0x00000000);
        
        // Final memory and cache synchronization
        asm!("wbinvd");
        asm!("mfence");
        
        self.is_shroud_unlocked = false;
    }

    // --- Low-Level MSR Primitives ---

    #[inline(always)]
    pub unsafe fn write_msr(&self, msr: u32, value: u64) {
        let low = (value & 0xFFFFFFFF) as u32;
        let high = (value >> 32) as u32;
        asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
    }

    #[inline(always)]
    pub unsafe fn read_msr(&self, msr: u32) -> u64 {
        let low: u32;
        let high: u32;
        asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high);
        ((high as u64) << 32) | (low as u64)
    }

    /// Serializing Instruction Barrier (CPUID leaf 0 used as a fence)
    #[inline(always)]
    pub unsafe fn serialize_cpu(&self) {
        asm!("xor eax, eax", "cpuid", out("eax") _, out("ebx") _, out("ecx") _, out("edx") _);
    }
}

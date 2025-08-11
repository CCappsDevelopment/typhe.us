use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

#[wasm_bindgen]
pub fn generate_sine_wave(freq: f32, duration_secs: f32, sample_rate: f32) -> Vec<f32> {
    let total_samples = (duration_secs * sample_rate).round() as usize;
    (0..total_samples)
        .map(|i| ((2.0 * PI * freq * i as f32) / sample_rate).sin())
        .collect()
}

#[wasm_bindgen]
pub fn generate_sawtooth_wave(freq: f32, duration_secs: f32, sample_rate: f32) -> Vec<f32> {
    let total_samples = (duration_secs * sample_rate).round() as usize;
    let period = sample_rate / freq;
    
    (0..total_samples)
        .map(|i| {
            let phase = (i as f32 % period) / period;
            2.0 * phase - 1.0
        })
        .collect()
}

#[wasm_bindgen]
pub fn generate_square_wave(freq: f32, duration_secs: f32, sample_rate: f32) -> Vec<f32> {
    let total_samples = (duration_secs * sample_rate).round() as usize;
    let period = sample_rate / freq;
    
    (0..total_samples)
        .map(|i| {
            let phase = (i as f32 % period) / period;
            if phase < 0.5 { 1.0 } else { -1.0 }
        })
        .collect()
}

#[wasm_bindgen]
pub fn generate_triangle_wave(freq: f32, duration_secs: f32, sample_rate: f32) -> Vec<f32> {
    let total_samples = (duration_secs * sample_rate).round() as usize;
    let period = sample_rate / freq;
    
    (0..total_samples)
        .map(|i| {
            let phase = (i as f32 % period) / period;
            if phase < 0.5 {
                // Rising edge: 0 to 0.5 maps to -1.0 to 1.0
                4.0 * phase - 1.0
            } else {
                // Falling edge: 0.5 to 1.0 maps to 1.0 to -1.0
                3.0 - 4.0 * phase
            }
        })
        .collect()
}
use crate::config_loader::{get_material, MATERIALS};
use crate::models::*;
use std::collections::HashMap;

const BELL_VIBRATION_MODES: [(usize, usize); 8] = [
    (2, 0), (3, 0), (4, 0), (2, 1), (5, 0), (3, 1), (6, 0), (4, 1),
];

const FREQUENCY_CALIBRATION: f64 = 2.2;

const IDEAL_HARMONIC_RATIOS: [f64; 8] = [
    0.5,    // hum
    1.0,    // fundamental
    1.19,   // tierce
    1.5,    // quint
    2.0,    // nominal
    2.51,   // sixth
    3.0,    // octave nominal
    4.0,    // double octave
];

pub fn compare_alloys(req: &AlloyComparisonRequest, bell: Option<&Bell>) -> AlloyComparisonResult {
    let (height, diameter, ref_freq) = match bell {
        Some(b) => (b.height_m, b.diameter_m, b.expected_freq_hz),
        None => {
            let h = req.height_m.unwrap_or(2.0);
            let d = req.diameter_m.unwrap_or(1.5);
            let f = estimate_fundamental_from_dims(h, d);
            (h, d, f)
        }
    };

    let alloys_to_compare: Vec<String> = if req.alloy_keys.is_empty() {
        MATERIALS.keys().cloned().collect()
    } else {
        req.alloy_keys.clone()
    };

    let mut metrics_list = Vec::new();
    for alloy_key in &alloys_to_compare {
        if let Some(mat) = get_material(alloy_key) {
            metrics_list.push(compute_alloy_metrics(alloy_key, &mat, height, diameter, ref_freq));
        }
    }

    let ref_key = metrics_list
        .first()
        .map(|m| m.alloy_key.clone())
        .unwrap_or_default();

    let comparison_table = build_comparison_table(&metrics_list);
    let radar_data = build_radar_chart_data(&metrics_list);
    let recommendations = generate_alloy_recommendations(&metrics_list, &ref_key);

    AlloyComparisonResult {
        reference_alloy: ref_key,
        reference_freq_hz: ref_freq,
        metrics: metrics_list,
        comparison_table,
        radar_chart_data: radar_data,
        recommendations,
    }
}

fn estimate_fundamental_from_dims(height_m: f64, diameter_m: f64) -> f64 {
    let effective_diameter = (height_m + diameter_m) / 2.0;
    let t = diameter_m * 0.04;
    let r = diameter_m / 2.0;
    let h = height_m;
    let e = 1.15e11_f64;
    let rho = 8800.0_f64;
    let nu = 0.34;
    let stiffness = (e / (12.0 * rho * (1.0 - nu * nu))).sqrt();
    let geom = t / (r * h);
    let mode_factor = ((BELL_VIBRATION_MODES[0].0.pow(2) * (BELL_VIBRATION_MODES[0].1 + 1).pow(2)) as f64).sqrt();
    stiffness * geom * mode_factor * FREQUENCY_CALIBRATION
}

fn compute_alloy_metrics(
    key: &str,
    mat: &crate::config_loader::Material,
    height_m: f64,
    diameter_m: f64,
    ref_freq: f64,
) -> AlloyAcousticMetrics {
    let young_modulus = mat.young_modulus;
    let rho = mat.density;
    let nu = mat.poisson_ratio;
    let t = diameter_m * 0.04;
    let r = diameter_m / 2.0;
    let h = height_m;
    let sound_speed = (young_modulus / rho).sqrt();
    let stiffness_coeff = (young_modulus / (12.0 * rho * (1.0 - nu * nu))).sqrt();
    let geom_factor = t / (r * h);

    let fundamental_mode = ((BELL_VIBRATION_MODES[0].0.pow(2) * (BELL_VIBRATION_MODES[0].1 + 1).pow(2)) as f64).sqrt();
    let fundamental = stiffness_coeff * geom_factor * fundamental_mode * FREQUENCY_CALIBRATION;

    let sn_ratio = mat.composition.get("sn").copied().unwrap_or(0.14);
    let pb_ratio = mat.composition.get("pb").copied().unwrap_or(0.02);
    let inharm_tweak = (pb_ratio * 2.0 - sn_ratio * 0.5) * 0.01;

    let mut freqs = Vec::with_capacity(IDEAL_HARMONIC_RATIOS.len());
    for (i, ratio) in IDEAL_HARMONIC_RATIOS.iter().enumerate() {
        let mode_correction = 1.0 + inharm_tweak * (i as f64).powi(2) * 0.02;
        freqs.push(fundamental * ratio * mode_correction);
    }

    let pitch_dev_cents = if ref_freq > 0.0 {
        1200.0 * (fundamental / ref_freq).log2()
    } else {
        0.0
    };

    let harmonic_ratios: Vec<f64> = freqs.iter().map(|f| f / fundamental).collect();

    let mut inharm = 0.0;
    for (i, ratio) in harmonic_ratios.iter().enumerate() {
        if i < IDEAL_HARMONIC_RATIOS.len() {
            inharm += ((ratio - IDEAL_HARMONIC_RATIOS[i]) / IDEAL_HARMONIC_RATIOS[i]).powi(2);
        }
    }
    let inharmonicity = (inharm / harmonic_ratios.len() as f64).sqrt() * 100.0;

    let mut energy = vec![0.0; freqs.len()];
    let mut total_energy = 0.0;
    let sn_ratio = mat.composition.get("sn").copied().unwrap_or(0.14);
    let pb_ratio = mat.composition.get("pb").copied().unwrap_or(0.02);
    let damping_per_octave = 0.3 + pb_ratio * 3.0 - sn_ratio * 0.5;
    for (i, f) in freqs.iter().enumerate() {
        let amplitude = 1.0 / (i as f64 + 1.0).powi(2);
        let freq_ratio = f / fundamental;
        let damping_factor = (-damping_per_octave * freq_ratio.log2()).exp();
        energy[i] = amplitude * amplitude * damping_factor;
        total_energy += energy[i];
    }
    for e in &mut energy {
        *e /= total_energy;
    }

    let high_freq_energy: f64 = energy.iter().skip(4).sum();
    let low_freq_energy: f64 = energy.iter().take(3).sum();
    let brightness = high_freq_energy * 100.0;
    let warmth = low_freq_energy * 100.0;

    let sn_ratio = mat.composition.get("sn").copied().unwrap_or(0.14) * 100.0;
    let pb_ratio = mat.composition.get("pb").copied().unwrap_or(0.02) * 100.0;
    let damping_factor = pb_ratio * 0.08 + (100.0 - sn_ratio - pb_ratio) * 0.005;
    let decay_time = (2.5 + sn_ratio * 0.05 - damping_factor * 0.3).max(0.5);

    let bell_surface = 2.0 * std::f64::consts::PI * r * h * 1.5;
    let sound_power = 0.5 * rho * 343.0 * bell_surface * 1e-4;

    let directivity = 3.0 + sn_ratio * 0.05 - inharmonicity * 0.01;

    let ringing = (1.0 - inharmonicity / 50.0).max(0.0) * 50.0 + 50.0;

    let pitch_score = (100.0 - pitch_dev_cents.abs()).max(0.0);
    let inharm_score = (100.0 - inharmonicity * 2.0).max(0.0);
    let decay_score = (decay_time / 4.0 * 100.0).min(100.0).max(0.0);
    let power_score = (sound_power * 100.0).min(100.0).max(0.0);
    let overall = (pitch_score * 0.25 + inharm_score * 0.3 + decay_score * 0.2 + power_score * 0.1 + ringing * 0.15);

    let mut composition = HashMap::new();
    for (k, v) in &mat.composition {
        composition.insert(k.clone(), *v);
    }

    AlloyAcousticMetrics {
        alloy_key: key.to_string(),
        alloy_name: mat.name.clone(),
        composition,
        natural_frequencies: freqs,
        fundamental_hz: fundamental,
        pitch_deviation_from_ref_cents: pitch_dev_cents,
        sound_power_w: sound_power,
        directivity_index: directivity.max(1.0).min(6.0),
        decay_time_s: decay_time,
        harmonic_ratios,
        harmonic_energy_distribution: energy,
        inharmonicity_coefficient: inharmonicity,
        brightness_index: brightness,
        warmth_index: warmth,
        ringing_quality: ringing.min(100.0).max(0.0),
        overall_quality_score: overall.min(100.0).max(0.0),
        density: rho,
        young_modulus_pa: young_modulus,
        sound_speed_m_s: sound_speed,
    }
}

fn build_comparison_table(metrics: &[AlloyAcousticMetrics]) -> Vec<AlloyComparisonRow> {
    let rows_data: Vec<(&str, &str, Box<dyn Fn(&AlloyAcousticMetrics) -> f64 + Send + Sync>)> = vec![
        ("基频", "Hz", Box::new(|m| m.fundamental_hz)),
        ("音准偏差", "cents", Box::new(|m| m.pitch_deviation_from_ref_cents.abs())),
        ("声功率", "W", Box::new(|m| m.sound_power_w)),
        ("指向性指数", "dB", Box::new(|m| m.directivity_index)),
        ("衰减时间", "s", Box::new(|m| m.decay_time_s)),
        ("非谐系数", "%", Box::new(|m| m.inharmonicity_coefficient)),
        ("明亮度", "index", Box::new(|m| m.brightness_index)),
        ("温暖度", "index", Box::new(|m| m.warmth_index)),
        ("延音品质", "%", Box::new(|m| m.ringing_quality)),
        ("综合评分", "%", Box::new(|m| m.overall_quality_score)),
        ("密度", "kg/m³", Box::new(|m| m.density)),
        ("杨氏模量", "GPa", Box::new(|m| m.young_modulus_pa / 1e9)),
        ("声速", "m/s", Box::new(|m| m.sound_speed_m_s)),
    ];

    let mut result = Vec::new();
    for (metric_name, unit, accessor) in rows_data {
        let mut values = HashMap::new();
        let mut best_val = f64::INFINITY;
        let mut worst_val = f64::NEG_INFINITY;
        let mut best_key = String::new();
        let mut worst_key = String::new();

        let lower_is_better = matches!(
            metric_name,
            "音准偏差" | "非谐系数" | "密度"
        );

        for m in metrics {
            let val = accessor(m);
            values.insert(m.alloy_key.clone(), val);

            let is_better = if lower_is_better {
                val < best_val
            } else {
                val > best_val
            };
            if is_better || best_val == f64::INFINITY {
                best_val = val;
                best_key = m.alloy_key.clone();
            }

            let is_worse = if lower_is_better {
                val > worst_val
            } else {
                val < worst_val
            };
            if is_worse || worst_val == f64::NEG_INFINITY {
                worst_val = val;
                worst_key = m.alloy_key.clone();
            }
        }

        result.push(AlloyComparisonRow {
            metric: metric_name.to_string(),
            unit: unit.to_string(),
            values,
            best_alloy: best_key,
            worst_alloy: worst_key,
        });
    }

    result
}

fn build_radar_chart_data(metrics: &[AlloyAcousticMetrics]) -> AlloyRadarData {
    let labels = vec![
        "音准稳定性".to_string(),
        "音色纯度".to_string(),
        "延音品质".to_string(),
        "投射能力".to_string(),
        "明亮度".to_string(),
        "温暖度".to_string(),
        "综合评分".to_string(),
    ];

    let mut datasets = HashMap::new();
    for m in metrics {
        let pitch_score = (100.0 - m.pitch_deviation_from_ref_cents.abs() * 2.0).max(0.0).min(100.0);
        let purity_score = (100.0 - m.inharmonicity_coefficient * 2.0).max(0.0).min(100.0);
        let decay_score = (m.decay_time_s / 4.0 * 100.0).min(100.0).max(0.0);
        let projection = (m.directivity_index / 6.0 * 50.0 + m.sound_power_w.min(1.0) * 50.0).min(100.0).max(0.0);

        datasets.insert(
            m.alloy_key.clone(),
            vec![
                pitch_score,
                purity_score,
                m.ringing_quality,
                projection,
                m.brightness_index,
                m.warmth_index,
                m.overall_quality_score,
            ],
        );
    }

    AlloyRadarData { labels, datasets }
}

fn generate_alloy_recommendations(metrics: &[AlloyAcousticMetrics], ref_key: &str) -> Vec<String> {
    let mut recs = Vec::new();

    if let Some(best_overall) = metrics.iter().max_by(|a, b| a.overall_quality_score.partial_cmp(&b.overall_quality_score).unwrap()) {
        recs.push(format!(
            "综合音质最佳: {} (评分 {:.1}/100)，适合追求顶级音效的应用场景",
            best_overall.alloy_name, best_overall.overall_quality_score
        ));
    }

    if let Some(best_pitch) = metrics.iter().min_by(|a, b| a.pitch_deviation_from_ref_cents.abs().partial_cmp(&b.pitch_deviation_from_ref_cents.abs()).unwrap()) {
        recs.push(format!(
            "音准最精确: {} (偏差 {:.1} 音分)，适合与其他乐器合奏或精确定音需求",
            best_pitch.alloy_name, best_pitch.pitch_deviation_from_ref_cents.abs()
        ));
    }

    if let Some(best_ring) = metrics.iter().max_by(|a, b| a.ringing_quality.partial_cmp(&b.ringing_quality).unwrap()) {
        recs.push(format!(
            "延音最出色: {} (延音品质 {:.1}%)，适合营造悠远、空灵的钟声氛围",
            best_ring.alloy_name, best_ring.ringing_quality
        ));
    }

    if let Some(best_pure) = metrics.iter().min_by(|a, b| a.inharmonicity_coefficient.partial_cmp(&b.inharmonicity_coefficient).unwrap()) {
        recs.push(format!(
            "音色最纯净: {} (非谐系数 {:.2}%)，泛音结构最接近理想谐波关系",
            best_pure.alloy_name, best_pure.inharmonicity_coefficient
        ));
    }

    if metrics.len() >= 2 {
        let mut sorted: Vec<&AlloyAcousticMetrics> = metrics.iter().collect();
        sorted.sort_by(|a, b| b.overall_quality_score.partial_cmp(&a.overall_quality_score).unwrap());
        if sorted[0].overall_quality_score - sorted[1].overall_quality_score < 5.0 {
            recs.push(format!(
                "{} 与 {} 音质接近，可根据材料成本和历史文化因素选择",
                sorted[0].alloy_name, sorted[1].alloy_name
            ));
        }
    }

    if let Some(ref_m) = metrics.iter().find(|m| m.alloy_key == ref_key) {
        for m in metrics {
            if m.alloy_key != ref_key {
                let diff = m.overall_quality_score - ref_m.overall_quality_score;
                if diff > 10.0 {
                    recs.push(format!(
                        "升级建议: 将 {} 替换为 {} 可提升综合音质约 {:.1}%",
                        ref_m.alloy_name, m.alloy_name, diff
                    ));
                }
            }
        }
    }

    recs
}

pub fn get_alloy_composition_suggestion(target_freq_hz: f64, max_deviation_cents: f64) -> HashMap<String, f64> {
    let mut best = HashMap::new();
    let (mut cu, mut sn, mut pb) = (82.0_f64, 15.0_f64, 2.0_f64);
    let step = 0.5_f64;
    let mut best_dev = f64::MAX;

    let sn_steps = ((22.0 - 10.0) / (step * 2.0)) as i32;
    let pb_steps = ((4.0 - 0.5) / step) as i32;

    for sn_i in 0..=sn_steps {
        let sn_test = 10.0 + sn_i as f64 * step * 2.0;
        for pb_i in 0..=pb_steps {
            let pb_test = 0.5 + pb_i as f64 * step;
            let cu_test = 100.0 - sn_test - pb_test;
            if cu_test < 74.0 || cu_test > 88.0 {
                continue;
            }
            let approx_e = 1.15e11 + (sn_test - 14.0) * 1e9;
            let approx_rho = 8800.0 + (sn_test - 14.0) * 10.0;
            let freq_factor = (approx_e / approx_rho).sqrt();
            let ref_factor = (1.15e11 / 8800.0_f64).sqrt();
            let estimated_freq = target_freq_hz * (freq_factor / ref_factor);
            let dev = (1200.0 * (estimated_freq / target_freq_hz).log2()).abs();
            if dev < best_dev && dev <= max_deviation_cents {
                best_dev = dev;
                cu = cu_test;
                sn = sn_test;
                pb = pb_test;
            }
        }
    }
    best.insert("cu".to_string(), cu / 100.0);
    best.insert("sn".to_string(), sn / 100.0);
    best.insert("pb".to_string(), pb / 100.0);
    best.insert("tolerance_cents".to_string(), best_dev);
    best
}

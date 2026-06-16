//! 虚拟敲钟模块 - 模拟敲击交互的物理与声学真实感

use crate::models::*;
use uuid::Uuid;

const POSITION_MODES: [&str; 5] = ["lip", "waist", "shoulder", "crown", "rim"];
const MALLET_HARDNESS: [&str; 4] = ["soft", "medium", "hard", "metal"];

const IDEAL_RATIOS: [f64; 8] = [0.5, 1.0, 1.19, 1.5, 2.0, 2.5, 3.0, 4.0];

const POS_AMP: [f64; 5] = [1.2, 1.0, 0.8, 0.5, 1.1];
const POS_INDEX: [usize; 5] = [0, 1, 2, 3, 4];

const POS_BIAS: [[f64; 8]; 5] = [
    [1.0, 1.2, 0.9, 1.1, 0.8, 1.0, 0.7, 0.9],
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    [0.9, 0.8, 1.0, 1.1, 1.2, 1.0, 1.1, 1.0],
    [0.6, 0.5, 0.7, 1.0, 1.1, 1.2, 1.3, 1.1],
    [1.1, 1.1, 0.8, 0.9, 0.9, 1.0, 0.8, 0.7],
];

const MALLET_SPECTRAL: [[f64; 8]; 4] = [
    [1.0, 0.8, 0.6, 0.45, 0.35, 0.25, 0.2, 0.15],
    [1.0, 1.0, 0.85, 0.7, 0.6, 0.5, 0.4, 0.3],
    [1.0, 1.1, 1.15, 1.1, 1.0, 0.9, 0.8, 0.7],
    [0.8, 1.0, 1.2, 1.4, 1.5, 1.6, 1.5, 1.4],
];

const MALLET_MASS: [f64; 4] = [2.0, 4.0, 6.0, 3.0];
const MALLET_CONTACT_MOD: [f64; 4] = [1e7, 5e7, 2e8, 5e8];
const MALLET_DAMPING: [f64; 4] = [0.95, 0.85, 0.75, 0.7];
const MALLET_ATTACK_MS: [f64; 4] = [25.0, 12.0, 5.0, 3.0];
const CONTACT_MOD_REF: f64 = 5e7;

fn pos_idx(key: &str) -> usize {
    match key {
        "lip" => 0,
        "waist" => 1,
        "shoulder" => 2,
        "crown" => 3,
        "rim" => 4,
        _ => 1,
    }
}

fn mallet_idx(key: &str) -> usize {
    match key {
        "soft" => 0,
        "medium" => 1,
        "hard" => 2,
        "metal" => 3,
        _ => 1,
    }
}

pub fn compute_strike_impact(
    params: &VirtualStrikeParams,
    bell: Option<&Bell>,
) -> VirtualStrikeResult {
    let strike_force = params.strike_force.clamp(0.1, 1.0);
    let pos = params.strike_position.to_lowercase();
    let pos_key = if POSITION_MODES.contains(&pos.as_str()) {
        pos.as_str()
    } else {
        "waist"
    };
    let pi = pos_idx(pos_key);

    let mallet = params.mallet_hardness.to_lowercase();
    let mallet_key = if MALLET_HARDNESS.contains(&mallet.as_str()) {
        mallet.as_str()
    } else {
        "medium"
    };
    let mi = mallet_idx(mallet_key);

    let bell_weight = bell.map(|b| b.weight_kg).unwrap_or(5000.0);
    let bell_freq = bell.map(|b| b.expected_freq_hz).unwrap_or(256.0);

    let expected_speed = 6.0;
    let impact_vel = expected_speed * strike_force.powf(0.5);

    let mallet_mass = MALLET_MASS[mi];
    let contact_modulus = MALLET_CONTACT_MOD[mi];

    let kinetic_energy = 0.5 * mallet_mass * impact_vel.powi(2);
    let peak_force_n = (kinetic_energy * contact_modulus).powf(2.0 / 3.0) * 0.01;
    let base_contact_ms = (1.0 / bell_freq * 1000.0) * (2.0 - strike_force) * 0.7;
    let hardness_factor = (CONTACT_MOD_REF / contact_modulus).powf(0.4);
    let contact_ms = base_contact_ms * hardness_factor;

    let pos_amplitude_factor = POS_AMP[pi];
    let pos_harmonic_bias = &POS_BIAS[pi];
    let mallet_spectral_bias = &MALLET_SPECTRAL[mi];

    let mut harmonic_amps = [0.0f64; 8];
    let mut total_amp = 0.0;
    for i in 0..8 {
        let base = 1.0 / (i as f64 + 1.0).powi(2);
        let amp = base * strike_force * pos_amplitude_factor
            * pos_harmonic_bias[i]
            * mallet_spectral_bias[i];
        harmonic_amps[i] = amp;
        total_amp += amp;
    }

    let ref_pressure = 2e-5_f64;
    let sound_pressure = total_amp * 0.5;
    let spl_db = 20.0 * (sound_pressure / ref_pressure).log10();

    let freq_weighted_phon = if bell_freq < 100.0 {
        spl_db - 15.0
    } else if bell_freq < 400.0 {
        spl_db - 5.0
    } else {
        spl_db
    };
    let phon = (freq_weighted_phon * strike_force.powf(0.5)).min(120.0).max(30.0);

    let damping_penalty = MALLET_DAMPING[mi];
    let mass_factor = (bell_weight / 5000.0).powf(0.3);
    let base_decay = (2.5 + (bell_weight.log10() - 3.0) * 0.5).max(1.0);
    let estimated_decay = base_decay * mass_factor * damping_penalty * (1.0 + pos_amplitude_factor * 0.1);

    let quality = classify_strike_quality(
        strike_force, pos_key, mallet_key, phon, estimated_decay, bell_freq,
    );

    let mut partials = Vec::with_capacity(8);
    for i in 0..8 {
        let freq_ratio = IDEAL_RATIOS[i] * (1.0 + (i as f64 * 0.003));
        let detune = match mi {
            3 => 0.0,
            _ => 0.0,
        };
        let decay = (estimated_decay / freq_ratio.powf(0.6)).max(0.2);
        let gain = (harmonic_amps[i] / total_amp * 3.0).min(1.0);
        partials.push(PartialParams {
            freq_ratio,
            gain,
            decay_s: decay,
            detune_cents: detune,
        });
    }

    let attack_ms = MALLET_ATTACK_MS[mi];
    let master_gain = 0.3 + strike_force * 0.5;

    VirtualStrikeResult {
        strike_id: Uuid::new_v4(),
        impact_velocity: impact_vel,
        peak_contact_force_n: peak_force_n,
        contact_duration_ms: contact_ms,
        estimated_decay_s: estimated_decay,
        perceived_loudness_phon: phon,
        quality_description: quality,
        harmonic_amplitudes: harmonic_amps.to_vec(),
        audio_synthesis_params: AudioSynthParams {
            fundamental_hz: bell_freq,
            partials,
            attack_ms,
            master_gain,
            strike_timestamp_ms: js_sys_now(),
        },
    }
}

fn classify_strike_quality(
    force: f64,
    pos: &str,
    mallet: &str,
    phon: f64,
    decay: f64,
    freq: f64,
) -> String {
    let mut notes = Vec::new();

    let pos_name = match pos {
        "lip" => "钟口",
        "waist" => "钟腰",
        "shoulder" => "钟肩",
        "crown" => "钟顶",
        "rim" => "唇边",
        _ => "钟身",
    };
    let mallet_name = match mallet {
        "soft" => "软质槌 (毡/布包)",
        "medium" => "中硬槌 (木槌)",
        "hard" => "硬质槌 (红木/牛角)",
        "metal" => "金属槌 (铜/铁)",
        _ => "木槌",
    };
    let force_desc = if force < 0.25 {
        "轻柔"
    } else if force < 0.5 {
        "中等"
    } else if force < 0.8 {
        "有力"
    } else {
        "猛烈"
    };

    notes.push(format!("{} 敲击 {} ({})", force_desc, mallet_name, pos_name));

    if phon > 110.0 {
        notes.push("⚠️ 音量极大，超过舒适阈值，长时间聆听可能损伤听力".to_string());
    } else if phon > 95.0 {
        notes.push("🔊 音量饱满，适合远距离传播".to_string());
    } else if phon > 75.0 {
        notes.push("🔈 音量适中，室内欣赏最佳".to_string());
    } else {
        notes.push("🔇 音量轻柔，适合近场静听".to_string());
    }

    if decay > 6.0 {
        notes.push("✨ 延音极长，钟声余韵悠远".to_string());
    } else if decay > 4.0 {
        notes.push("⭐ 延音充足，钟声余韵优美".to_string());
    } else if decay > 2.0 {
        notes.push("👍 延音适中，节奏感清晰".to_string());
    } else {
        notes.push("📎 延音偏短，适合快速旋律".to_string());
    }

    match (pos, mallet) {
        ("lip", "medium") | ("lip", "soft") => {
            notes.push("♪ 经典编钟敲法：钟口+中硬槌，主音清晰，泛音丰富".to_string());
        }
        ("waist", "hard") | ("waist", "metal") => {
            notes.push("♫ 钟声明亮穿透力强，适合广场/庆典".to_string());
        }
        ("shoulder", "soft") => {
            notes.push("♬ 音色温润醇厚，适合诵经背景音".to_string());
        }
        ("crown", "hard") => {
            notes.push("♩ 泛音突出，产生空灵效果，适合独奏".to_string());
        }
        _ => {}
    }

    if freq < 120.0 && force > 0.7 {
        notes.push("💡 低频大钟大力敲击，可产生大地颤动的次声波体感".to_string());
    }
    if freq > 500.0 && force < 0.3 {
        notes.push("💡 高频小钟轻敲，音色如银铃般清脆".to_string());
    }

    notes.join("；")
}

fn js_sys_now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

pub fn get_position_options() -> Vec<(&'static str, &'static str, f64)> {
    vec![
        ("lip", "钟口 (Lip) - 最常用敲击位置，主音饱满", 0.88),
        ("waist", "钟腰 (Waist) - 中部，音色均衡", 0.65),
        ("shoulder", "钟肩 (Shoulder) - 上部，音色柔和", 0.40),
        ("rim", "唇边 (Rim) - 钟口边缘，清脆明亮", 0.80),
        ("crown", "钟顶 (Crown) - 顶端，以泛音为主", 0.12),
    ]
}

pub fn get_mallet_options() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("soft", "软质槌 (毡包/布包)", "温暖醇厚，激发低频为主，延音最长"),
        ("medium", "中硬槌 (硬木/枣木)", "均衡饱满，传统编钟标准用槌"),
        ("hard", "硬质槌 (红木/牛角/玉石)", "明亮清脆，中高频丰富，穿透力强"),
        ("metal", "金属槌 (铜/铁槌)", "金石之声，高频炸裂，极具穿透力，易损伤钟体"),
    ]
}

pub fn generate_strike_tutorial() -> Vec<String> {
    vec![
        "🎓 古代编钟敲击技法入门:".to_string(),
        "".to_string(),
        "【基本握法】".to_string(),
        "  - 右手握槌柄中后段，手腕放松，手指自然环绕".to_string(),
        "  - 以肘关节为轴，小臂带动槌头做圆弧运动".to_string(),
        "  - 接触瞬间有'点触'感，忌死压钟面".to_string(),
        "".to_string(),
        "【力度控制】 (对应滑块 0.0 - 1.0)".to_string(),
        "  - 0.1 - 0.25: 轻抚/拨奏，mp - p力度，适合室内清奏".to_string(),
        "  - 0.25 - 0.5: 标准敲击，mf力度，日常练习推荐".to_string(),
        "  - 0.5 - 0.8: 重槌击奏，f力度，集体演奏".to_string(),
        "  - 0.8 - 1.0: 全力敲击，ff力度，庆典/祭祀/报时".to_string(),
        "".to_string(),
        "【位置差异】".to_string(),
        "  - 钟口 (正鼓部): 两正鼓音区交汇处，激发主音最充分".to_string(),
        "  - 钟腰 (中鼓部): 振动均匀，适合合奏".to_string(),
        "  - 钟肩: 振动偏弱，辅助音色".to_string(),
        "  - 唇边 (侧鼓部): 曾侯乙编钟的第二基音位置!".to_string(),
        "  - 钟顶 (舞部): 几乎不激发基频，纯泛音效果".to_string(),
        "".to_string(),
        "【编钟特殊技巧】 (一钟双音原理)".to_string(),
        "  - 曾侯乙每套编钟可敲出两个大三度/小三度音".to_string(),
        "  - 正鼓部 (钟口正中) → 第一基音 (正鼓音)".to_string(),
        "  - 侧鼓部 (钟口侧面) → 第二基音 (侧鼓音)".to_string(),
        "  - 两个音互不干扰，现代声学揭秘: 节线分布".to_string(),
        "".to_string(),
        "【保养禁忌】".to_string(),
        "  - 禁止金属槌直接敲击古钟，会造成永久性凹陷".to_string(),
        "  - 同一位置连续重击不超过3次，防止金属疲劳".to_string(),
        "  - 冬季低温时先轻敲预热，骤热骤冷会开裂".to_string(),
        "".to_string(),
        "✨ 听辨练习: 调节不同位置+不同槌+不同力度组合，仔细听泛音结构变化!".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::VirtualStrikeParams;
    use uuid::Uuid;

    fn make_test_params(pos: &str, mallet: &str, force: f64) -> VirtualStrikeParams {
        VirtualStrikeParams {
            bell_id: Uuid::new_v4(),
            strike_force: force,
            strike_position: pos.to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: mallet.to_string(),
        }
    }

    #[test]
    fn test_compute_strike_impact_standard() {
        let params = make_test_params("lip", "medium", 0.5);
        let result = compute_strike_impact(&params, None);
        assert!(result.audio_synthesis_params.fundamental_hz > 0.0);
        assert!(result.audio_synthesis_params.partials.len() >= 8);
        assert!(result.perceived_loudness_phon > 0.0);
    }

    #[test]
    fn test_compute_strike_impact_min_force() {
        let params = make_test_params("lip", "medium", 0.01);
        let result = compute_strike_impact(&params, None);
        assert!(result.peak_contact_force_n > 0.0);
    }

    #[test]
    fn test_compute_strike_impact_max_force() {
        let params = make_test_params("lip", "medium", 1.0);
        let result = compute_strike_impact(&params, None);
        assert!(result.peak_contact_force_n > 0.0);
    }

    #[test]
    fn test_compute_strike_impact_negative_force_clamped() {
        let params = make_test_params("lip", "medium", -0.5);
        let result = compute_strike_impact(&params, None);
        assert!(result.impact_velocity > 0.0);
    }

    #[test]
    fn test_compute_strike_impact_invalid_position() {
        let params = make_test_params("invalid_pos", "medium", 0.5);
        let result = compute_strike_impact(&params, None);
        assert!(result.audio_synthesis_params.fundamental_hz > 0.0);
    }

    #[test]
    fn test_compute_strike_impact_invalid_mallet() {
        let params = make_test_params("lip", "invalid_mallet", 0.5);
        let result = compute_strike_impact(&params, None);
        assert!(result.audio_synthesis_params.fundamental_hz > 0.0);
    }

    #[test]
    fn test_compute_strike_impact_all_positions() {
        let positions = ["lip", "rim", "waist", "shoulder", "crown"];
        for pos in positions.iter() {
            let params = make_test_params(pos, "medium", 0.5);
            let result = compute_strike_impact(&params, None);
            assert!(result.estimated_decay_s > 0.0);
            assert!(!result.quality_description.is_empty());
        }
    }

    #[test]
    fn test_compute_strike_impact_mallet_hardness_effect() {
        let mallets = ["soft", "medium", "hard", "metal"];
        let mut results = Vec::new();
        for m in mallets.iter() {
            let params = make_test_params("lip", m, 0.5);
            let result = compute_strike_impact(&params, None);
            results.push(result);
        }
        let soft_attack = results[0].audio_synthesis_params.attack_ms;
        let metal_attack = results[3].audio_synthesis_params.attack_ms;
        assert!(soft_attack > metal_attack);
    }

    #[test]
    fn test_compute_strike_impact_force_amplitude_relationship() {
        let params1 = make_test_params("lip", "medium", 0.2);
        let params2 = make_test_params("lip", "medium", 0.8);
        let r1 = compute_strike_impact(&params1, None);
        let r2 = compute_strike_impact(&params2, None);
        assert!(r2.peak_contact_force_n > r1.peak_contact_force_n);
        assert!(r2.perceived_loudness_phon > r1.perceived_loudness_phon);
    }

    #[test]
    fn test_get_position_options() {
        let opts = get_position_options();
        assert_eq!(opts.len(), 5);
        for (key, name, factor) in opts {
            assert!(!key.is_empty());
            assert!(!name.is_empty());
            assert!(factor > 0.0 && factor <= 1.5);
        }
    }

    #[test]
    fn test_get_mallet_options() {
        let opts = get_mallet_options();
        assert_eq!(opts.len(), 4);
        let keys: Vec<&str> = opts.iter().map(|(k, _, _)| *k).collect();
        assert!(keys.contains(&"soft"));
        assert!(keys.contains(&"medium"));
        assert!(keys.contains(&"hard"));
        assert!(keys.contains(&"metal"));
    }

    #[test]
    fn test_generate_strike_tutorial() {
        let tutorial = generate_strike_tutorial();
        assert!(!tutorial.is_empty());
        assert!(tutorial.len() >= 5);
        let content: String = tutorial.join(" ");
        assert!(content.contains("编钟"));
        assert!(content.contains("力度"));
    }

    #[test]
    fn test_strike_idempotency_for_deterministic_fields() {
        let params = make_test_params("waist", "medium", 0.5);
        let r1 = compute_strike_impact(&params, None);
        let r2 = compute_strike_impact(&params, None);
        assert!((r1.impact_velocity - r2.impact_velocity).abs() < 0.001);
        assert!((r1.peak_contact_force_n - r2.peak_contact_force_n).abs() < 0.001);
        assert!((r1.contact_duration_ms - r2.contact_duration_ms).abs() < 0.001);
    }

    #[test]
    fn test_audio_synth_params_web_audio_compatible() {
        let params = make_test_params("lip", "medium", 0.5);
        let result = compute_strike_impact(&params, None);
        let synth = result.audio_synthesis_params;
        assert!(synth.fundamental_hz >= 20.0);
        assert!(synth.fundamental_hz <= 20000.0);
        assert!(synth.master_gain >= 0.0 && synth.master_gain <= 2.0);
        assert!(synth.attack_ms >= 0.0);
        for p in &synth.partials {
            assert!(p.freq_ratio > 0.0);
            assert!(p.gain >= 0.0);
            assert!(p.decay_s > 0.0);
        }
    }

    #[test]
    fn test_hertz_contact_force_realism() {
        let params = make_test_params("lip", "hard", 0.9);
        let result = compute_strike_impact(&params, None);
        assert!(result.peak_contact_force_n > 10.0);
        assert!(result.peak_contact_force_n < 100000.0);
        assert!(result.contact_duration_ms > 0.1);
        assert!(result.contact_duration_ms < 100.0);
        assert!(result.impact_velocity > 0.0);
    }

    #[test]
    fn test_harmonic_amplitudes_valid() {
        let params = make_test_params("lip", "medium", 0.5);
        let result = compute_strike_impact(&params, None);
        assert_eq!(result.harmonic_amplitudes.len(), 8);
        for &amp in &result.harmonic_amplitudes {
            assert!(amp >= 0.0);
        }
    }

    #[test]
    fn test_quality_description_not_empty() {
        let params = make_test_params("lip", "medium", 0.5);
        let result = compute_strike_impact(&params, None);
        assert!(!result.quality_description.is_empty());
    }
}

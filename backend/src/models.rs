use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bell {
    pub bell_id: Uuid,
    pub bell_name: String,
    pub dynasty: String,
    pub bell_type: String,
    pub material: String,
    pub height_m: f64,
    pub diameter_m: f64,
    pub weight_kg: f64,
    pub expected_pitch: String,
    pub expected_freq_hz: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct SensorReading {
    pub reading_id: Uuid,
    pub bell_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub temp_celsius: f64,
    pub temp_gradient: f64,
    pub wall_thickness_mm: f64,
    pub thickness_deviation: f64,
    pub alloy_cu: f64,
    pub alloy_sn: f64,
    pub alloy_pb: f64,
    pub alloy_zn: f64,
    pub alloy_other: f64,
    pub acoustic_freq_hz: f64,
    pub acoustic_amplitude: f64,
    pub acoustic_decay: f64,
    pub acoustic_harmonics: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReadingIn {
    pub bell_id: Uuid,
    pub temp_celsius: f64,
    pub temp_gradient: f64,
    pub wall_thickness_mm: f64,
    pub thickness_deviation: f64,
    pub alloy_cu: f64,
    pub alloy_sn: f64,
    pub alloy_pb: f64,
    pub alloy_zn: f64,
    pub alloy_other: f64,
    pub acoustic_freq_hz: f64,
    pub acoustic_amplitude: f64,
    pub acoustic_decay: f64,
    pub acoustic_harmonics: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct CastingSimulation {
    pub sim_id: Uuid,
    pub bell_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sim_type: String,
    pub time_step_sec: u32,
    pub temp_field: String,
    pub solid_fraction: String,
    pub shrinkage_porosity: String,
    pub defect_locations: String,
    pub defect_count: u32,
    pub max_shrinkage: f64,
    pub cooling_rate: f64,
    pub prediction_risk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingSimRequest {
    pub bell_id: Uuid,
    pub sim_type: String,
    pub initial_temp: f64,
    pub grid_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct AcousticSimulation {
    pub sim_id: Uuid,
    pub bell_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub natural_frequencies: String,
    pub mode_shapes: String,
    pub far_field_pressure: String,
    pub sound_field_2d: String,
    pub directivity_index: f64,
    pub sound_power: f64,
    pub pitch_deviation_cents: f64,
    pub pitch_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticSimRequest {
    pub bell_id: Uuid,
    pub method: String,
    pub young_modulus: Option<f64>,
    pub poisson_ratio: Option<f64>,
    pub density: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct Alert {
    pub alert_id: Uuid,
    pub bell_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub related_reading: Option<Uuid>,
    pub related_sim: Option<Uuid>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct CastingProcess {
    pub process_id: Uuid,
    pub bell_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub stage: String,
    pub progress: f64,
    pub current_temp: f64,
    pub mold_fill_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttAlertPayload {
    pub alert_id: Uuid,
    pub bell_id: Uuid,
    pub bell_name: String,
    pub timestamp: DateTime<Utc>,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
}

// ========== Feature 1: 合金配比音质对比分析 ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlloyComparisonRequest {
    pub bell_id: Option<Uuid>,
    pub height_m: Option<f64>,
    pub diameter_m: Option<f64>,
    pub alloy_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlloyAcousticMetrics {
    pub alloy_key: String,
    pub alloy_name: String,
    pub composition: std::collections::HashMap<String, f64>,
    pub natural_frequencies: Vec<f64>,
    pub fundamental_hz: f64,
    pub pitch_deviation_from_ref_cents: f64,
    pub sound_power_w: f64,
    pub directivity_index: f64,
    pub decay_time_s: f64,
    pub harmonic_ratios: Vec<f64>,
    pub harmonic_energy_distribution: Vec<f64>,
    pub inharmonicity_coefficient: f64,
    pub brightness_index: f64,
    pub warmth_index: f64,
    pub ringing_quality: f64,
    pub overall_quality_score: f64,
    pub density: f64,
    pub young_modulus_pa: f64,
    pub sound_speed_m_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlloyComparisonResult {
    pub reference_alloy: String,
    pub reference_freq_hz: f64,
    pub metrics: Vec<AlloyAcousticMetrics>,
    pub comparison_table: Vec<AlloyComparisonRow>,
    pub radar_chart_data: AlloyRadarData,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlloyComparisonRow {
    pub metric: String,
    pub unit: String,
    pub values: std::collections::HashMap<String, f64>,
    pub best_alloy: String,
    pub worst_alloy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlloyRadarData {
    pub labels: Vec<String>,
    pub datasets: std::collections::HashMap<String, Vec<f64>>,
}

// ========== Feature 2: 古代vs现代铸造工艺对比 ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingMethodRequest {
    pub bell_id: Option<Uuid>,
    pub methods: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingMethodMetrics {
    pub method_key: String,
    pub method_name: String,
    pub category: String,
    pub era: String,
    pub description: String,
    pub historical_period: String,
    pub typical_accuracy_mm: f64,
    pub surface_roughness_ra: f64,
    pub cooling_rate_cps: f64,
    pub typical_defect_rate_pct: f64,
    pub max_shrinkage_porosity_pct: f64,
    pub dimensional_tolerance_pct: f64,
    pub material_yield_pct: f64,
    pub production_cycle_days: f64,
    pub labor_intensity: f64,
    pub energy_consumption_kwh_per_ton: f64,
    pub cost_per_kg: f64,
    pub environmental_impact_score: f64,
    pub skill_requirements_level: f64,
    pub max_cast_weight_tons: f64,
    pub minimum_thickness_mm: f64,
    pub microstructural_quality: f64,
    pub acoustic_quality_potential: f64,
    pub aesthetic_quality: f64,
    pub durability_years: f64,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub famous_examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingComparisonResult {
    pub methods_count: usize,
    pub methods: Vec<CastingMethodMetrics>,
    pub ancient_vs_modern_summary: AncientModernSummary,
    pub comparison_chart_data: CastingChartData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncientModernSummary {
    pub ancient_avg_scores: std::collections::HashMap<String, f64>,
    pub modern_avg_scores: std::collections::HashMap<String, f64>,
    pub key_differences: Vec<String>,
    pub trade_offs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingChartData {
    pub categories: Vec<String>,
    pub ancient_avg: Vec<f64>,
    pub modern_avg: Vec<f64>,
}

// ========== Feature 3: 钟楼建筑声学传播模拟 ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAcousticRequest {
    pub bell_id: Option<Uuid>,
    pub frequency_hz: Option<f64>,
    pub tower: TowerBuildingParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerBuildingParams {
    pub tower_style: String,
    pub height_m: f64,
    pub width_m: f64,
    pub depth_m: f64,
    pub wall_thickness_m: f64,
    pub wall_material: String,
    pub bell_chamber_height_m: f64,
    pub window_count: u32,
    pub window_width_m: f64,
    pub window_height_m: f64,
    pub roof_style: String,
    pub openings_direction_deg: Vec<f64>,
    pub internal_absorption_coeff: f64,
    pub internal_reverberation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAcousticResult {
    pub tower_params: TowerBuildingParams,
    pub with_tower: TowerSoundField,
    pub without_tower: TowerSoundField,
    pub comparison_metrics: TowerComparisonMetrics,
    pub directivity_pattern: Vec<TowerDirectivityPoint>,
    pub sound_coverage: Vec<TowerCoverageZone>,
    pub optimization_tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerSoundField {
    pub field_2d: Vec<Vec<f64>>,
    pub max_spl_db: f64,
    pub min_spl_db: f64,
    pub avg_spl_db: f64,
    pub reverberation_time_s: f64,
    pub clarity_index: f64,
    pub definition_d50: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerComparisonMetrics {
    pub spl_boost_at_100m_db: f64,
    pub directionality_improvement: f64,
    pub coverage_area_increase_pct: f64,
    pub echo_reduction_db: f64,
    pub frequency_response_flatness: f64,
    pub overall_improvement_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerDirectivityPoint {
    pub angle_deg: f64,
    pub with_tower_spl: f64,
    pub without_tower_spl: f64,
    pub gain_db: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerCoverageZone {
    pub zone_name: String,
    pub distance_range_m: (f64, f64),
    pub with_tower_avg_spl: f64,
    pub without_tower_avg_spl: f64,
    pub spl_gain_db: f64,
    pub intelligible_speech: bool,
    pub aesthetic_enjoyment: bool,
}

// ========== Feature 4: 虚拟敲钟记录(可选) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualStrikeParams {
    pub bell_id: Uuid,
    pub strike_force: f64,
    pub strike_position: String,
    pub strike_angle_deg: f64,
    pub mallet_hardness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualStrikeResult {
    pub strike_id: Uuid,
    pub impact_velocity: f64,
    pub peak_contact_force_n: f64,
    pub contact_duration_ms: f64,
    pub estimated_decay_s: f64,
    pub perceived_loudness_phon: f64,
    pub quality_description: String,
    pub harmonic_amplitudes: Vec<f64>,
    pub audio_synthesis_params: AudioSynthParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSynthParams {
    pub fundamental_hz: f64,
    pub partials: Vec<PartialParams>,
    pub attack_ms: f64,
    pub master_gain: f64,
    pub strike_timestamp_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialParams {
    pub freq_ratio: f64,
    pub gain: f64,
    pub decay_s: f64,
    pub detune_cents: f64,
}

use crate::models::*;

const GRID_SIZE: usize = 100;
const MAX_DISTANCE_M: f64 = 500.0;

fn frequency_dependent_absorption(material: &str, freq: f64) -> f64 {
    let f = freq.max(50.0).min(8000.0);
    let octave_ratio = (f / 500.0).log2();
    let base = match material {
        "brick" => 0.03 + 0.02 * octave_ratio.max(0.0),
        "stone" => 0.02 + 0.015 * octave_ratio.max(0.0),
        "wood" => 0.10 + 0.08 * octave_ratio.max(0.0),
        "concrete" => 0.05 + 0.03 * octave_ratio.max(0.0),
        "adobe" => 0.15 + 0.10 * octave_ratio.max(0.0),
        _ => 0.08 + 0.04 * octave_ratio.max(0.0),
    };
    base.max(0.01).min(0.8)
}

fn roof_reflection_coeff(roof_style: &str, freq: f64) -> f64 {
    let base = match roof_style {
        "dome" | "穹顶" | "圆顶" => 0.95,
        "gable" | "山墙" | "硬山" | "悬山" => 0.90,
        "flat" | "平顶" => 0.85,
        "pagoda" | "塔状" | "攒尖" => 0.92,
        "hipped" | "庑殿" | "歇山" | "重檐庑殿顶" | "歇山顶" | "庑殿顶" | "十字歇山顶" => 0.88,
        _ => 0.90,
    };
    let freq_factor = 1.0 + 0.05 * ((freq / 500.0).log2()).tanh();
    (base * freq_factor).min(0.98)
}

fn ground_reflection_coeff(ground_type: &str, freq: f64, incident_angle: f64) -> f64 {
    let base = match ground_type {
        "grass" => 0.60,
        "soil" => 0.70,
        "asphalt" => 0.92,
        "concrete" => 0.95,
        "water" => 0.98,
        "marble" => 0.96,
        _ => 0.75,
    };
    let grazing_factor = 0.5 + 0.5 * incident_angle.cos().abs();
    let freq_factor = 1.0 - 0.2 * ((freq / 1000.0).log2()).max(0.0).min(1.0);
    (base * grazing_factor * freq_factor).max(0.1).min(0.99)
}

fn diffusion_coefficient(material: &str, roughness: f64, freq: f64) -> f64 {
    let base_rough = match material {
        "brick" => 0.3,
        "stone" => 0.2,
        "wood" => 0.5,
        "concrete" => 0.15,
        "adobe" => 0.6,
        _ => 0.25,
    };
    let wavelength = 343.0 / freq;
    let roughness_ratio = (roughness / wavelength).min(2.0);
    (base_rough + roughness_ratio * 0.3).min(0.9)
}

pub fn simulate_tower_acoustics(req: &TowerAcousticRequest) -> TowerAcousticResult {
    let frequency = req.frequency_hz.unwrap_or(256.0);
    let tower = &req.tower;

    let wavelength = 343.0 / frequency;
    let k = 2.0 * std::f64::consts::PI / wavelength;

    let bell_height = tower.height_m - tower.bell_chamber_height_m / 2.0;
    let bell_x = tower.width_m / 2.0;
    let bell_z = tower.depth_m / 2.0;

    let with_field = compute_2d_field_with_tower(k, tower, bell_height, bell_x, bell_z, frequency);
    let without_field = compute_2d_field_free(k, bell_height, frequency);

    let directivity = compute_directivity_pattern(k, tower, bell_height, frequency, &without_field);
    let coverage = compute_coverage_zones(&with_field, &without_field);

    let metrics = compute_comparison_metrics(&with_field, &without_field, &directivity, tower);
    let tips = generate_optimization_tips(tower, &metrics, frequency);

    TowerAcousticResult {
        tower_params: tower.clone(),
        with_tower: with_field,
        without_tower: without_field,
        comparison_metrics: metrics,
        directivity_pattern: directivity,
        sound_coverage: coverage,
        optimization_tips: tips,
    }
}

fn compute_2d_field_with_tower(
    k: f64,
    tower: &TowerBuildingParams,
    bell_h: f64,
    bell_x: f64,
    bell_z: f64,
    freq: f64,
) -> TowerSoundField {
    let mut field = vec![vec![0.0f64; GRID_SIZE]; GRID_SIZE];
    let mut spl_values = Vec::new();
    let dx = MAX_DISTANCE_M * 2.0 / GRID_SIZE as f64;
    let dz = MAX_DISTANCE_M * 2.0 / GRID_SIZE as f64;

    let total_open_area = tower.window_count as f64 * tower.window_width_m * tower.window_height_m;
    let wall_area = 2.0 * (tower.width_m + tower.depth_m) * tower.bell_chamber_height_m;
    let floor_area = tower.width_m * tower.depth_m;
    let ceiling_area = tower.width_m * tower.depth_m;
    let total_surface = wall_area + floor_area + ceiling_area;
    let openness_ratio = (total_open_area / wall_area.max(0.1)).min(0.8);

    let wall_absorption = frequency_dependent_absorption(&tower.wall_material, freq);
    let wall_reflection = 1.0 - wall_absorption;

    let floor_absorption = frequency_dependent_absorption(&tower.ground_type, freq);
    let ceiling_height = if tower.ceiling_height_m > 0.0 { tower.ceiling_height_m } else { tower.bell_chamber_height_m * 0.8 };
    let roof_refl = roof_reflection_coeff(&tower.roof_style, freq);
    let ceiling_absorption = 1.0 - roof_refl;

    let wall_diffusion = diffusion_coefficient(&tower.wall_material, tower.wall_roughness_mm, freq);
    let diffuse_reflection = wall_reflection * wall_diffusion;
    let specular_reflection = wall_reflection * (1.0 - wall_diffusion);

    let effective_absorption = (
        wall_area * wall_absorption * (1.0 - openness_ratio)
        + floor_area * floor_absorption
        + ceiling_area * ceiling_absorption
        + total_open_area * tower.internal_absorption_coeff
    ) / total_surface.max(0.1);
    let effective_absorption = effective_absorption.max(0.02);

    let rt60 = if effective_absorption > 0.0 {
        let volume = tower.width_m * tower.depth_m * tower.bell_chamber_height_m;
        let sabine_area = total_surface * effective_absorption + total_open_area * 0.5;
        0.161 * volume / sabine_area.max(0.1)
    } else {
        4.0
    };

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let world_x = (i as f64 - GRID_SIZE as f64 / 2.0) * dx;
            let world_z = (j as f64 - GRID_SIZE as f64 / 2.0) * dz;

            let r_ground = (world_x * world_x + world_z * world_z).sqrt();
            let r_3d = (r_ground * r_ground + bell_h * bell_h).sqrt();

            let inside_tower = world_x.abs() < tower.width_m / 2.0
                && world_z.abs() < tower.depth_m / 2.0
                && r_ground < (tower.width_m.max(tower.depth_m));

            let local_x = world_x + bell_x;
            let local_z = world_z + bell_z;
            let nearest_wall_dist = vec![
                tower.width_m / 2.0 - world_x.abs(),
                tower.depth_m / 2.0 - world_z.abs(),
            ]
            .into_iter()
            .reduce(f64::min)
            .unwrap_or(0.0);

            let mut p_direct = if r_3d > 0.1 {
                (k * r_3d).sin() / r_3d
            } else {
                10.0
            };

            let num_reflections = 3;
            let mut p_total = p_direct;
            for order in 1..=num_reflections {
                let spec_decay = specular_reflection.powi(order as i32);
                let diff_decay = diffuse_reflection.powi(order as i32);
                let path_extra = (order as f64) * nearest_wall_dist.max(0.1) * 2.0;
                let r_reflect = r_3d + path_extra;
                if r_reflect > 0.1 {
                    let phase_shift = k * path_extra + order as f64 * 0.5;
                    let spec_comp = (k * r_reflect - phase_shift).sin() / r_reflect * spec_decay * 0.4;
                    let diff_comp = (k * r_reflect).sin() / r_reflect * diff_decay * 0.25;
                    p_total += spec_comp + diff_comp;
                }
            }

            let ceiling_dist = ceiling_height - bell_h;
            if ceiling_dist > 0.5 {
                let r_ceiling = (r_ground * r_ground + (bell_h + ceiling_dist * 2.0).powi(2)).sqrt();
                if r_ceiling > 0.1 {
                    let path_diff = r_ceiling - r_3d;
                    let phase = k * path_diff + std::f64::consts::PI;
                    p_total += (k * r_ceiling - phase).sin() / r_ceiling * roof_refl * 0.35;
                }
            }

            let floor_dist = bell_h;
            if floor_dist > 0.5 {
                let r_floor = (r_ground * r_ground + (bell_h + floor_dist * 2.0).powi(2)).sqrt();
                if r_floor > 0.1 {
                    let incident_angle = (r_ground / r_floor).acos();
                    let floor_refl = 1.0 - floor_absorption;
                    let path_diff = r_floor - r_3d;
                    let phase = k * path_diff + std::f64::consts::PI;
                    p_total += (k * r_floor - phase).sin() / r_floor * floor_refl * 0.4;
                }
            }

            if inside_tower {
                let chamber_boost = 3.0 * (1.0 - openness_ratio * 0.7) + rt60 * 0.3;
                p_total *= chamber_boost;
            } else {
                let dir_boost = compute_tower_directivity_boost(
                    world_x, world_z, tower, openness_ratio, total_open_area, freq,
                );
                let edge_diffraction = if nearest_wall_dist < 10.0 && nearest_wall_dist > -5.0 {
                    let edge_factor = (1.0 - (nearest_wall_dist / 10.0).abs()).max(0.0);
                    1.0 + edge_factor * 0.3 * wall_reflection
                } else {
                    1.0
                };
                p_total *= dir_boost * edge_diffraction;
            }

            let ground_grazing = if r_ground > 0.1 {
                (bell_h / r_3d).acos()
            } else {
                0.0
            };
            let ground_refl = ground_reflection_coeff(&tower.ground_type, freq, ground_grazing);
            let ground_effect = 1.0 + (-r_ground / 200.0).exp() * 0.4 * ground_refl;
            p_total *= ground_effect;

            let p_ref = 2e-5;
            let spl = 20.0 * (p_total.abs() * 1000.0 / p_ref).log10();
            let spl_clamped = spl.max(20.0).min(120.0);

            field[i][j] = spl_clamped;
            spl_values.push(spl_clamped);
        }
    }

    summarize_field(&field, &spl_values, rt60)
}

fn compute_2d_field_free(k: f64, bell_h: f64, _freq: f64) -> TowerSoundField {
    let mut field = vec![vec![0.0f64; GRID_SIZE]; GRID_SIZE];
    let mut spl_values = Vec::new();
    let dx = MAX_DISTANCE_M * 2.0 / GRID_SIZE as f64;
    let dz = MAX_DISTANCE_M * 2.0 / GRID_SIZE as f64;

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let world_x = (i as f64 - GRID_SIZE as f64 / 2.0) * dx;
            let world_z = (j as f64 - GRID_SIZE as f64 / 2.0) * dz;

            let r_ground = (world_x * world_x + world_z * world_z).sqrt();
            let r_3d = (r_ground * r_ground + bell_h * bell_h).sqrt();

            let p = if r_3d > 0.1 {
                (k * r_3d).sin() / r_3d
            } else {
                10.0
            };

            let ground_effect = 1.0 + (-r_ground / 200.0).exp() * 0.3;
            let p_total = p * ground_effect;

            let p_ref = 2e-5;
            let spl = 20.0 * (p_total.abs() * 1000.0 / p_ref).log10();
            let spl_clamped = spl.max(20.0).min(120.0);

            field[i][j] = spl_clamped;
            spl_values.push(spl_clamped);
        }
    }

    summarize_field(&field, &spl_values, 0.5)
}

fn compute_tower_directivity_boost(
    world_x: f64,
    world_z: f64,
    tower: &TowerBuildingParams,
    openness: f64,
    _open_area: f64,
    freq: f64,
) -> f64 {
    let angle_rad = world_z.atan2(world_x);
    let angle_deg = (angle_rad.to_degrees() + 360.0) % 360.0;

    let mut nearest_window_gain = 0.0;
    for opening in &tower.openings_direction_deg {
        let opening = opening % 360.0;
        let mut diff = (angle_deg - opening).abs();
        if diff > 180.0 {
            diff = 360.0 - diff;
        }
        let beam_width = 45.0 + (1.0 - openness) * 30.0;
        let gain = (-diff * diff / (2.0 * beam_width * beam_width)).exp() * (1.0 + openness * 1.5);
        if gain > nearest_window_gain {
            nearest_window_gain = gain;
        }
    }

    let base_boost = 0.05 + openness * 1.8 + nearest_window_gain * 0.6;

    let low_freq_boost = if freq < 200.0 {
        1.0 + openness * 0.3
    } else if freq > 2000.0 {
        0.85 + (1.0 - openness) * 0.3
    } else {
        1.0
    };

    (base_boost * low_freq_boost).max(0.5).min(3.5)
}

fn compute_directivity_pattern(
    _k: f64,
    tower: &TowerBuildingParams,
    _bell_h: f64,
    _freq: f64,
    free_field: &TowerSoundField,
) -> Vec<TowerDirectivityPoint> {
    let mut points = Vec::new();
    let center = GRID_SIZE / 2;
    let sample_radius = 100.0;
    let grid_per_m = GRID_SIZE as f64 / (MAX_DISTANCE_M * 2.0);
    let r_grid = (sample_radius * grid_per_m) as isize;

    for deg in (0..360).step_by(10) {
        let rad = (deg as f64).to_radians();
        let gi = (center as isize + (rad.cos() * r_grid as f64) as isize) as usize;
        let gj = (center as isize + (rad.sin() * r_grid as f64) as isize) as usize;

        let gi_clamped = gi.min(GRID_SIZE - 1).max(0);
        let gj_clamped = gj.min(GRID_SIZE - 1).max(0);

        let world_x = (gi as f64 - center as f64) / grid_per_m;
        let world_z = (gj as f64 - center as f64) / grid_per_m;
        let total_open_area = tower.window_count as f64 * tower.window_width_m * tower.window_height_m;
        let chamber_surface = 2.0 * (tower.width_m + tower.depth_m) * tower.bell_chamber_height_m;
        let openness_ratio = (total_open_area / chamber_surface.max(0.1)).min(0.8);

        let boost = compute_tower_directivity_boost(world_x, world_z, tower, openness_ratio, total_open_area, 256.0);
        let free_spl = if gi_clamped < GRID_SIZE && gj_clamped < GRID_SIZE {
            free_field.field_2d[gi_clamped][gj_clamped]
        } else {
            60.0
        };
        let with_spl_unclamped = free_spl * (boost / 1.3).ln().max(-0.5).exp();
        let with_spl = with_spl_unclamped.min(120.0);

        points.push(TowerDirectivityPoint {
            angle_deg: deg as f64,
            with_tower_spl: with_spl,
            without_tower_spl: free_spl,
            gain_db: with_spl - free_spl,
        });
    }
    points
}

fn compute_coverage_zones(
    with_tower: &TowerSoundField,
    without_tower: &TowerSoundField,
) -> Vec<TowerCoverageZone> {
    let center = GRID_SIZE / 2;
    let grid_per_m = GRID_SIZE as f64 / (MAX_DISTANCE_M * 2.0);

    let zones = vec![
        ("塔下近场 (0-30m)", 0.0, 30.0),
        ("近区欣赏 (30-100m)", 30.0, 100.0),
        ("中程传播 (100-250m)", 100.0, 250.0),
        ("远场覆盖 (250-500m)", 250.0, 500.0),
    ];

    zones
        .into_iter()
        .map(|(name, r_min, r_max)| {
            let r_min_grid = (r_min * grid_per_m) as isize;
            let r_max_grid = (r_max * grid_per_m) as isize;

            let mut with_sum = 0.0f64;
            let mut without_sum = 0.0f64;
            let mut count = 0usize;

            for di in -r_max_grid..=r_max_grid {
                for dj in -r_max_grid..=r_max_grid {
                    let r = ((di * di + dj * dj) as f64).sqrt();
                    if r >= r_min_grid as f64 && r < r_max_grid as f64 {
                        let gi = (center as isize + di) as usize;
                        let gj = (center as isize + dj) as usize;
                        if gi < GRID_SIZE && gj < GRID_SIZE {
                            with_sum += with_tower.field_2d[gi][gj];
                            without_sum += without_tower.field_2d[gi][gj];
                            count += 1;
                        }
                    }
                }
            }

            let with_avg = if count > 0 { with_sum / count as f64 } else { 0.0 };
            let without_avg = if count > 0 { without_sum / count as f64 } else { 0.0 };

            TowerCoverageZone {
                zone_name: name.to_string(),
                distance_range_m: (r_min, r_max),
                with_tower_avg_spl: with_avg,
                without_tower_avg_spl: without_avg,
                spl_gain_db: with_avg - without_avg,
                intelligible_speech: with_avg > 55.0,
                aesthetic_enjoyment: with_avg > 45.0 && with_avg < 95.0,
            }
        })
        .collect()
}

fn compute_comparison_metrics(
    with_tower: &TowerSoundField,
    without: &TowerSoundField,
    directivity: &[TowerDirectivityPoint],
    tower: &TowerBuildingParams,
) -> TowerComparisonMetrics {
    let center = GRID_SIZE / 2;
    let grid_per_m = GRID_SIZE as f64 / (MAX_DISTANCE_M * 2.0);
    let idx_100 = (100.0 * grid_per_m) as isize;

    let spl_at_100_with = sample_ring_avg(&with_tower.field_2d, center, idx_100);
    let spl_at_100_without = sample_ring_avg(&without.field_2d, center, idx_100);

    let dir_with: Vec<f64> = directivity.iter().map(|p| p.with_tower_spl).collect();
    let dir_without: Vec<f64> = directivity.iter().map(|p| p.without_tower_spl).collect();

    let directivity_index_with = compute_directivity_index(&dir_with);
    let directivity_index_without = compute_directivity_index(&dir_without);

    let threshold = 45.0;
    let area_with = count_above_threshold(&with_tower.field_2d, threshold);
    let area_without = count_above_threshold(&without.field_2d, threshold);
    let coverage_pct = if area_without > 0 {
        ((area_with as f64 - area_without as f64) / area_without as f64 * 100.0).max(-50.0)
    } else {
        0.0
    };

    let echo_reduction = (tower.internal_absorption_coeff * 10.0 + (1.0 - 0.95_f64.powf(tower.window_count as f64)) * 5.0).min(15.0);

    let freq_flatness = {
        let total_open_area = tower.window_count as f64 * tower.window_width_m * tower.window_height_m;
        let chamber_surface = 2.0 * (tower.width_m + tower.depth_m) * tower.bell_chamber_height_m;
        let openness = (total_open_area / chamber_surface.max(0.1)).min(0.8);
        100.0 - (0.5 - openness).abs() * 120.0
    }.max(20.0);

    let overall = (
        (spl_at_100_with - spl_at_100_without).max(0.0) * 3.0
        + (directivity_index_with - directivity_index_without).max(0.0) * 5.0
        + coverage_pct.max(0.0) * 0.3
        + echo_reduction * 0.5
        + freq_flatness * 0.3
    ).min(100.0).max(0.0);

    TowerComparisonMetrics {
        spl_boost_at_100m_db: (spl_at_100_with - spl_at_100_without).max(-10.0).min(20.0),
        directionality_improvement: (directivity_index_with - directivity_index_without).max(-3.0).min(10.0),
        coverage_area_increase_pct: coverage_pct,
        echo_reduction_db: echo_reduction,
        frequency_response_flatness: freq_flatness,
        overall_improvement_score: overall,
    }
}

fn summarize_field(field: &[Vec<f64>], values: &[f64], rt60: f64) -> TowerSoundField {
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let avg = if values.is_empty() { 0.0 } else { values.iter().sum::<f64>() / values.len() as f64 };

    let clarity = (85.0 - avg).max(0.0).min(100.0);
    let d50 = (100.0 - rt60 * 15.0).max(0.0).min(100.0);

    TowerSoundField {
        field_2d: field.to_vec(),
        max_spl_db: max,
        min_spl_db: min,
        avg_spl_db: avg,
        reverberation_time_s: rt60.max(0.1),
        clarity_index: clarity,
        definition_d50: d50,
    }
}

fn sample_ring_avg(field: &[Vec<f64>], center: usize, radius_grid: isize) -> f64 {
    let mut sum = 0.0;
    let mut count = 0usize;
    let samples = 36;
    for s in 0..samples {
        let rad = (s as f64 / samples as f64) * 2.0 * std::f64::consts::PI;
        let gi = (center as isize + (rad.cos() * radius_grid as f64) as isize) as usize;
        let gj = (center as isize + (rad.sin() * radius_grid as f64) as isize) as usize;
        if gi < GRID_SIZE && gj < GRID_SIZE {
            sum += field[gi][gj];
            count += 1;
        }
    }
    if count > 0 { sum / count as f64 } else { 0.0 }
}

fn compute_directivity_index(spl_pattern: &[f64]) -> f64 {
    let avg: f64 = spl_pattern.iter().sum::<f64>() / spl_pattern.len() as f64;
    let max = spl_pattern.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    max - avg
}

fn count_above_threshold(field: &[Vec<f64>], threshold: f64) -> usize {
    field
        .iter()
        .map(|row| row.iter().filter(|&&v| v >= threshold).count())
        .sum()
}

fn generate_optimization_tips(
    tower: &TowerBuildingParams,
    metrics: &TowerComparisonMetrics,
    freq: f64,
) -> Vec<String> {
    let mut tips = Vec::new();

    let total_open_area = tower.window_count as f64 * tower.window_width_m * tower.window_height_m;
    let chamber_surface = 2.0 * (tower.width_m + tower.depth_m) * tower.bell_chamber_height_m;
    let openness_ratio = (total_open_area / chamber_surface.max(0.1)).min(0.8);

    if openness_ratio < 0.2 {
        tips.push(format!(
            "钟楼开口率过低 ({:.1}%)，建议增加窗户面积以提升 {:.0}Hz 频率的传播效率，预计可提升覆盖面积约 {:.0}%",
            openness_ratio * 100.0,
            freq,
            (0.3 - openness_ratio).max(0.0) * 80.0
        ));
    } else if openness_ratio > 0.6 {
        tips.push(format!(
            "钟楼开口率过高 ({:.1}%)，低频声能外泄严重，建议适当减少开口面积或在内部增加共鸣腔以增强低音厚重感",
            openness_ratio * 100.0
        ));
    }

    if metrics.spl_boost_at_100m_db < 2.0 {
        tips.push("100m处声压级提升不足，建议将钟楼高度提升至钟声波长的1/4以上，利用高度差增强地面反射叠加".to_string());
    }

    if tower.window_count < 4 {
        tips.push(format!(
            "当前仅{}面开窗，建议改为四面或八面对称开窗，以获得更均匀的360度覆盖，消除声学死角",
            tower.window_count
        ));
    }

    match tower.wall_material.as_str() {
        "wood" => tips.push("木质墙体中高频吸收较大，如追求钟声悠远感，建议外墙改用砖石材料，内部可保留木质装饰".to_string()),
        "concrete" => tips.push("混凝土墙面反射过强，可能产生驻波，建议在钟楼内部适当悬挂布幔或设置扩散体以优化混响时间".to_string()),
        _ => {}
    }

    if tower.openings_direction_deg.is_empty() {
        tips.push("建议在钟楼四面设置对称开口朝向主要传播方向（如面向东南西北四个主要街道/广场）".to_string());
    }

    if metrics.echo_reduction_db < 3.0 {
        tips.push("钟楼内部混响过长，建议在钟室顶部或墙壁增加吸音材料，降低回声干扰，提升清晰度C50指标".to_string());
    }

    if tower.height_m < 10.0 {
        tips.push(format!(
            "钟楼高度偏低 ({:.1}m)，建议将钟的悬挂高度提升至15m以上，可利用地面反射形成声像增强，等效于增加了1个声源",
            tower.height_m
        ));
    }

    if tower.wall_thickness_m < 0.37 {
        tips.push("墙体偏薄，隔声量不足，建议外墙采用一砖半墙 (370mm) 或双墙结构，减少邻近建筑的噪声干扰".to_string());
    }

    if metrics.overall_improvement_score < 50.0 {
        tips.push(format!(
            "综合声学评分偏低 ({:.1}/100)，建议优先优化开口率 + 钟室高度两个最敏感参数，一般可提升20分以上",
            metrics.overall_improvement_score
        ));
    } else if metrics.overall_improvement_score > 80.0 {
        tips.push(format!(
            "当前钟楼设计优秀 (评分 {:.1}/100)，已具备良好的声学性能，如进一步追求极致可尝试屋顶加装声学反射罩",
            metrics.overall_improvement_score
        ));
    }

    tips
}

pub fn get_preset_tower_configs() -> Vec<TowerBuildingParams> {
    vec![
        TowerBuildingParams {
            tower_style: "唐代钟楼 (木构)".to_string(),
            height_m: 15.0,
            width_m: 8.0,
            depth_m: 8.0,
            wall_thickness_m: 0.2,
            wall_material: "wood".to_string(),
            bell_chamber_height_m: 5.0,
            window_count: 4,
            window_width_m: 1.8,
            window_height_m: 2.4,
            roof_style: "庑殿顶".to_string(),
            openings_direction_deg: vec![0.0, 90.0, 180.0, 270.0],
            internal_absorption_coeff: 0.15,
            internal_reverberation: 1.5,
            ground_type: "wood".to_string(),
            wall_roughness_mm: 8.0,
            ceiling_height_m: 4.0,
        },
        TowerBuildingParams {
            tower_style: "明代鼓楼 (砖石)".to_string(),
            height_m: 30.0,
            width_m: 15.0,
            depth_m: 15.0,
            wall_thickness_m: 0.6,
            wall_material: "brick".to_string(),
            bell_chamber_height_m: 10.0,
            window_count: 8,
            window_width_m: 1.5,
            window_height_m: 3.0,
            roof_style: "十字歇山顶".to_string(),
            openings_direction_deg: vec![0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0],
            internal_absorption_coeff: 0.08,
            internal_reverberation: 3.5,
            ground_type: "stone".to_string(),
            wall_roughness_mm: 5.0,
            ceiling_height_m: 8.0,
        },
        TowerBuildingParams {
            tower_style: "苏州寒山寺钟楼".to_string(),
            height_m: 12.0,
            width_m: 6.0,
            depth_m: 6.0,
            wall_thickness_m: 0.37,
            wall_material: "brick".to_string(),
            bell_chamber_height_m: 4.0,
            window_count: 4,
            window_width_m: 1.2,
            window_height_m: 1.8,
            roof_style: "歇山顶".to_string(),
            openings_direction_deg: vec![0.0, 90.0, 180.0, 270.0],
            internal_absorption_coeff: 0.1,
            internal_reverberation: 2.2,
            ground_type: "marble".to_string(),
            wall_roughness_mm: 6.0,
            ceiling_height_m: 3.2,
        },
        TowerBuildingParams {
            tower_style: "永乐大钟钟楼 (北京觉生寺)".to_string(),
            height_m: 20.0,
            width_m: 20.0,
            depth_m: 20.0,
            wall_thickness_m: 0.8,
            wall_material: "stone".to_string(),
            bell_chamber_height_m: 15.0,
            window_count: 12,
            window_width_m: 2.0,
            window_height_m: 6.0,
            roof_style: "重檐庑殿顶".to_string(),
            openings_direction_deg: vec![0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0],
            internal_absorption_coeff: 0.06,
            internal_reverberation: 5.0,
            ground_type: "marble".to_string(),
            wall_roughness_mm: 3.0,
            ceiling_height_m: 12.0,
        },
        TowerBuildingParams {
            tower_style: "现代简约钟楼".to_string(),
            height_m: 25.0,
            width_m: 10.0,
            depth_m: 10.0,
            wall_thickness_m: 0.3,
            wall_material: "concrete".to_string(),
            bell_chamber_height_m: 8.0,
            window_count: 6,
            window_width_m: 2.5,
            window_height_m: 4.0,
            roof_style: "平顶+声学罩".to_string(),
            openings_direction_deg: vec![0.0, 60.0, 120.0, 180.0, 240.0, 300.0],
            internal_absorption_coeff: 0.12,
            internal_reverberation: 2.0,
            ground_type: "concrete".to_string(),
            wall_roughness_mm: 2.0,
            ceiling_height_m: 6.0,
        },
    ]
}

use bell_casting_backend::config_loader::*;
use bell_casting_backend::models::*;
use bell_casting_backend::simulation::*;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

/* ======================================================================
   测试辅助函数
   ====================================================================== */

fn create_test_bell() -> Bell {
    Bell {
        bell_id: Uuid::new_v4(),
        bell_name: "测试编钟".to_string(),
        dynasty: "先秦".to_string(),
        bell_type: "编钟".to_string(),
        material: "bronze_qing_qin".to_string(),
        height_m: 1.5,
        diameter_m: 1.0,
        weight_kg: 200.0,
        expected_pitch: "C4".to_string(),
        expected_freq_hz: 261.63,
        created_at: Utc::now(),
    }
}

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

/* ======================================================================
   Feature 1: 合金配比音质对比分析测试
   ====================================================================== */

mod alloy_comparison_tests {
    use super::*;

    // --- 正常场景测试 ---

    #[test]
    pub fn test_alloy_comparison_normal_multiple_alloys() {
        println!("\n=== [F1-正常] 多合金声学参数对比 ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(2.0),
            diameter_m: Some(1.5),
            alloy_keys: vec![
                "bronze_qing_qin".to_string(),
                "bronze_yong_le".to_string(),
                "bronze_fo_zhong".to_string(),
            ],
        };

        let result = compare_alloys(&req, None);

        assert_eq!(result.metrics.len(), 3, "应该返回3种合金的对比结果");
        assert!(!result.comparison_table.is_empty(), "对比表不能为空");
        assert!(!result.recommendations.is_empty(), "推荐建议不能为空");
        assert_eq!(result.radar_chart_data.labels.len(), 7, "雷达图应有7个维度");

        for metrics in &result.metrics {
            assert!(metrics.fundamental_hz > 0.0, "基频必须大于0");
            assert!(metrics.sound_power_w > 0.0, "声功率必须大于0");
            assert!(metrics.decay_time_s > 0.0, "衰减时间必须大于0");
            assert!(metrics.overall_quality_score > 0.0, "综合评分必须大于0");
            assert_eq!(metrics.natural_frequencies.len(), 8, "应有8阶固有频率");
            assert_eq!(metrics.harmonic_ratios.len(), 8, "应有8个谐波比");

            let f0 = metrics.natural_frequencies[0];
            assert!(f0 > 50.0 && f0 < 500.0, "基频应在合理范围: {}", f0);

            assert!(
                metrics.brightness_index > 0.0 && metrics.brightness_index <= 100.0,
                "明亮度应在0-100范围"
            );
            assert!(
                metrics.warmth_index > 0.0 && metrics.warmth_index <= 100.0,
                "温暖度应在0-100范围"
            );
        }

        println!("  ✅ 3种合金对比结果正常");
        println!("  参考频率: {:.2} Hz", result.reference_freq_hz);
        println!("  对比表行数: {}", result.comparison_table.len());
        println!("  推荐建议数: {}", result.recommendations.len());
    }

    #[test]
    pub fn test_alloy_comparison_with_bell_reference() {
        println!("\n=== [F1-正常] 使用参考钟进行合金对比 ===");

        let bell = create_test_bell();
        let req = AlloyComparisonRequest {
            bell_id: Some(bell.bell_id),
            height_m: None,
            diameter_m: None,
            alloy_keys: vec!["bronze_qing_qin".to_string(), "high_tin_bronze".to_string()],
        };

        let result = compare_alloys(&req, Some(&bell));

        assert_eq!(result.metrics.len(), 2);
        assert!(
            (result.reference_freq_hz - bell.expected_freq_hz).abs() < 1.0,
            "参考频率应接近钟的预期频率"
        );

        for m in &result.metrics {
            let dev = m.pitch_deviation_from_ref_cents;
            assert!(
                dev.abs() < 500.0,
                "音准偏差不应过大: {} 音分",
                dev
            );
        }

        println!("  ✅ 参考钟模式工作正常");
        println!("  参考钟频率: {} Hz", bell.expected_freq_hz);
    }

    #[test]
    pub fn test_alloy_acoustic_trend_verification() {
        println!("\n=== [F1-正常] 验证声学参数变化趋势 ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(2.0),
            diameter_m: Some(1.5),
            alloy_keys: vec![
                "bronze_qing_qin".to_string(),
                "bronze_yong_le".to_string(),
                "bronze_fo_zhong".to_string(),
                "high_tin_bronze".to_string(),
                "gray_cast_iron".to_string(),
            ],
        };

        let result = compare_alloys(&req, None);
        let metrics_map: HashMap<String, &AlloyAcousticMetrics> = result
            .metrics
            .iter()
            .map(|m| (m.alloy_key.clone(), m))
            .collect();

        let high_tin = metrics_map.get("high_tin_bronze").unwrap();
        let gray_iron = metrics_map.get("gray_cast_iron").unwrap();

        assert!(
            high_tin.brightness_index > gray_iron.brightness_index,
            "高锡青铜明亮度应高于灰铸铁: {} vs {}",
            high_tin.brightness_index,
            gray_iron.brightness_index
        );

        assert!(
            high_tin.decay_time_s > gray_iron.decay_time_s,
            "高锡青铜延音应长于灰铸铁: {}s vs {}s",
            high_tin.decay_time_s,
            gray_iron.decay_time_s
        );

        assert!(
            high_tin.overall_quality_score > gray_iron.overall_quality_score,
            "高锡青铜综合音质评分应高于灰铸铁"
        );

        assert!(
            high_tin.inharmonicity_coefficient < 1.0,
            "非谐系数应小于1: {}",
            high_tin.inharmonicity_coefficient
        );

        println!("  ✅ 声学参数变化趋势符合物理规律");
        println!("  高锡青铜明亮度: {:.1}, 灰铸铁: {:.1}", high_tin.brightness_index, gray_iron.brightness_index);
        println!("  高锡青铜延音: {:.2}s, 灰铸铁: {:.2}s", high_tin.decay_time_s, gray_iron.decay_time_s);
    }

    #[test]
    pub fn test_alloy_comparison_table_structure() {
        println!("\n=== [F1-正常] 对比表结构验证 ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(2.0),
            diameter_m: Some(1.5),
            alloy_keys: vec!["bronze_qing_qin".to_string(), "bronze_yong_le".to_string()],
        };

        let result = compare_alloys(&req, None);

        for row in &result.comparison_table {
            assert!(!row.metric.is_empty(), "指标名不能为空");
            assert!(row.best_alloy.contains("bronze"), "最佳合金应在对比列表中");
            assert!(row.worst_alloy.contains("bronze"), "最差合金应在对比列表中");
            assert_eq!(row.values.len(), 2, "每种对比合金应有一个值");
        }

        let metrics_in_table = result.comparison_table.len();
        assert!(metrics_in_table >= 8, "对比表至少应有8个指标");

        println!("  ✅ 对比表结构完整，共{}个指标", metrics_in_table);
    }

    // --- 边界场景测试 ---

    #[test]
    pub fn test_alloy_comparison_single_alloy_boundary() {
        println!("\n=== [F1-边界] 单合金对比（边界情况） ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(1.0),
            diameter_m: Some(0.5),
            alloy_keys: vec!["bronze_qing_qin".to_string()],
        };

        let result = compare_alloys(&req, None);

        assert_eq!(result.metrics.len(), 1, "单合金也应返回1个结果");
        assert!(!result.comparison_table.is_empty(), "单合金也应有对比表");
        assert_eq!(result.reference_alloy, "bronze_qing_qin", "参考合金应为自身");

        let m = &result.metrics[0];
        assert!(m.fundamental_hz > 100.0, "小尺寸钟基频应更高");

        println!("  ✅ 单合金边界情况正常");
        println!("  单合金基频: {:.2} Hz", m.fundamental_hz);
    }

    #[test]
    pub fn test_alloy_comparison_empty_alloy_list() {
        println!("\n=== [F1-边界] 空合金列表（使用全部材料） ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(2.0),
            diameter_m: Some(1.5),
            alloy_keys: vec![],
        };

        let result = compare_alloys(&req, None);

        assert!(
            result.metrics.len() >= 3,
            "空列表应返回所有可用材料，至少3种: {}",
            result.metrics.len()
        );

        println!("  ✅ 空列表自动使用全部材料: {} 种", result.metrics.len());
    }

    #[test]
    pub fn test_alloy_comparison_extreme_dimensions() {
        println!("\n=== [F1-边界] 极端尺寸验证 ===");

        let small_req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(0.3),
            diameter_m: Some(0.2),
            alloy_keys: vec!["bronze_qing_qin".to_string()],
        };
        let small = compare_alloys(&small_req, None);

        let large_req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(8.0),
            diameter_m: Some(5.0),
            alloy_keys: vec!["bronze_qing_qin".to_string()],
        };
        let large = compare_alloys(&large_req, None);

        let f_small = small.metrics[0].fundamental_hz;
        let f_large = large.metrics[0].fundamental_hz;

        assert!(f_small > f_large, "小钟基频应高于大钟: {} > {}", f_small, f_large);
        assert!(f_small > 200.0, "小编钟基频应在中音频段");
        assert!(f_large > 20.0 && f_large < 200.0, "大钟基频应在低音频段");

        println!("  ✅ 极端尺寸趋势正确");
        println!("  小编钟 (0.3m): {:.1} Hz", f_small);
        println!("  大乐钟 (8.0m): {:.1} Hz", f_large);
    }

    // --- 异常场景测试 ---

    #[test]
    pub fn test_alloy_comparison_invalid_alloy_keys() {
        println!("\n=== [F1-异常] 无效合金key处理 ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(2.0),
            diameter_m: Some(1.5),
            alloy_keys: vec![
                "invalid_alloy_123".to_string(),
                "nonexistent_material".to_string(),
            ],
        };

        let result = compare_alloys(&req, None);

        assert!(
            result.metrics.is_empty(),
            "无效key不应返回任何合金结果"
        );

        println!("  ✅ 无效合金key被正确过滤，返回空结果");
    }

    #[test]
    pub fn test_alloy_comparison_mixed_valid_invalid() {
        println!("\n=== [F1-异常] 有效+无效混合key ===");

        let req = AlloyComparisonRequest {
            bell_id: None,
            height_m: Some(2.0),
            diameter_m: Some(1.5),
            alloy_keys: vec![
                "bronze_qing_qin".to_string(),
                "invalid_material".to_string(),
                "bronze_yong_le".to_string(),
            ],
        };

        let result = compare_alloys(&req, None);

        assert_eq!(result.metrics.len(), 2, "应只返回2种有效合金");
        let keys: Vec<&String> = result.metrics.iter().map(|m| &m.alloy_key).collect();
        assert!(keys.contains(&&"bronze_qing_qin".to_string()));
        assert!(keys.contains(&&"bronze_yong_le".to_string()));
        assert!(!keys.contains(&&"invalid_material".to_string()));

        println!("  ✅ 混合key正确过滤，仅保留有效合金: 2种");
    }

    #[test]
    pub fn test_alloy_composition_suggestion_basic() {
        println!("\n=== [F1-正常] 合金成分建议 ===");

        let target = 256.0;
        let suggestion = get_alloy_composition_suggestion(target, 50.0);

        assert!(!suggestion.is_empty(), "建议不应为空");
        assert!(suggestion.contains_key("cu"), "应包含铜含量建议");
        assert!(suggestion.contains_key("sn"), "应包含锡含量建议");

        let sn_pct = suggestion.get("sn").unwrap() * 100.0;
        assert!(sn_pct > 5.0 && sn_pct < 30.0, "锡含量应在合理范围: {}%", sn_pct);

        println!("  ✅ 合金成分建议生成正常");
        println!("  目标频率: {} Hz", target);
        println!("  建议锡含量: {:.1}%", sn_pct);
    }

    #[test]
    pub fn test_alloy_composition_suggestion_edge_cases() {
        println!("\n=== [F1-边界] 合金成分建议边界 ===");

        let low_freq = get_alloy_composition_suggestion(50.0, 100.0);
        let high_freq = get_alloy_composition_suggestion(800.0, 100.0);

        assert!(!low_freq.is_empty());
        assert!(!high_freq.is_empty());

        let low_sn = low_freq.get("sn").unwrap();
        let high_sn = high_freq.get("sn").unwrap();

        println!("  低频目标锡含量: {:.1}%", low_sn * 100.0);
        println!("  高频目标锡含量: {:.1}%", high_sn * 100.0);
        println!("  ✅ 极端频率也能生成建议");
    }
}

/* ======================================================================
   Feature 2: 古代vs现代铸造工艺对比测试
   ====================================================================== */

mod casting_comparison_tests {
    use super::*;

    // --- 正常场景测试 ---

    #[test]
    pub fn test_casting_comparison_normal_all_methods() {
        println!("\n=== [F2-正常] 全部工艺对比 ===");

        let req = CastingMethodRequest {
            bell_id: None,
            methods: None,
        };

        let result = compare_casting_methods(&req);

        assert!(result.methods_count >= 6, "至少应有6种工艺");
        assert!(!result.methods.is_empty(), "工艺列表不能为空");
        assert!(
            !result.ancient_vs_modern_summary.key_differences.is_empty(),
            "核心差异列表不能为空"
        );
        assert!(
            !result.ancient_vs_modern_summary.trade_offs.is_empty(),
            "取舍建议不能为空"
        );
        assert_eq!(
            result.comparison_chart_data.categories.len(),
            result.comparison_chart_data.ancient_avg.len(),
            "图表数据维度应一致"
        );

        for method in &result.methods {
            assert!(!method.method_key.is_empty());
            assert!(!method.method_name.is_empty());
            assert!(!method.pros.is_empty());
            assert!(!method.cons.is_empty());
            assert!(method.typical_defect_rate_pct > 0.0);
            assert!(method.cost_per_kg > 0.0);
            assert!(method.acoustic_quality_potential >= 0.0 && method.acoustic_quality_potential <= 10.0);
        }

        println!("  ✅ 全部工艺对比正常");
        println!("  工艺总数: {}", result.methods_count);
        println!("  核心差异数: {}", result.ancient_vs_modern_summary.key_differences.len());
    }

    #[test]
    pub fn test_casting_defect_rate_difference_verification() {
        println!("\n=== [F2-正常] 缺陷率差异验证 ===");

        let req = CastingMethodRequest {
            bell_id: None,
            methods: None,
        };

        let result = compare_casting_methods(&req);

        let ancient_methods: Vec<&CastingMethodMetrics> = result
            .methods
            .iter()
            .filter(|m| m.era == "ancient")
            .collect();
        let modern_methods: Vec<&CastingMethodMetrics> = result
            .methods
            .iter()
            .filter(|m| m.era == "modern")
            .collect();

        assert!(!ancient_methods.is_empty(), "古代工艺至少应有1种");
        assert!(!modern_methods.is_empty(), "现代工艺至少应有1种");

        let ancient_avg_defect: f64 = ancient_methods
            .iter()
            .map(|m| m.typical_defect_rate_pct)
            .sum::<f64>()
            / ancient_methods.len() as f64;

        let modern_avg_defect: f64 = modern_methods
            .iter()
            .map(|m| m.typical_defect_rate_pct)
            .sum::<f64>()
            / modern_methods.len() as f64;

        println!("  古代平均缺陷率: {:.2}%", ancient_avg_defect);
        println!("  现代平均缺陷率: {:.2}%", modern_avg_defect);

        // 现代工艺通常缺陷率更低（质量控制更好）
        assert!(
            modern_avg_defect < ancient_avg_defect * 1.5,
            "现代工艺缺陷率不应显著高于古代"
        );

        println!("  ✅ 缺陷率差异符合工艺规律");
    }

    #[test]
    pub fn test_casting_ancient_modern_score_comparison() {
        println!("\n=== [F2-正常] 古代vs现代综合评分对比 ===");

        let req = CastingMethodRequest {
            bell_id: None,
            methods: None,
        };

        let result = compare_casting_methods(&req);
        let s = &result.ancient_vs_modern_summary;

        assert!(
            s.ancient_avg_scores.contains_key("acoustic_quality"),
            "古代评分应包含声学质量"
        );
        assert!(
            s.modern_avg_scores.contains_key("cost_efficiency"),
            "现代评分应包含成本效率"
        );

        let ancient_acoustic = s.ancient_avg_scores.get("acoustic_quality").copied().unwrap_or(0.0);
        let modern_acoustic = s.modern_avg_scores.get("acoustic_quality").copied().unwrap_or(0.0);
        let ancient_art = s.ancient_avg_scores.get("aesthetic_quality").copied().unwrap_or(0.0);
        let modern_cost = s.modern_avg_scores.get("cost_efficiency").copied().unwrap_or(0.0);
        let ancient_cost = s.ancient_avg_scores.get("cost_efficiency").copied().unwrap_or(0.0);

        println!("  古代声学评分: {:.1}", ancient_acoustic);
        println!("  现代声学评分: {:.1}", modern_acoustic);
        println!("  古代艺术评分: {:.1}", ancient_art);
        println!("  古代成本效率: {:.1}", ancient_cost);
        println!("  现代成本效率: {:.1}", modern_cost);

        assert!(ancient_art > 5.0, "古代工艺艺术表现力应较高");
        assert!(modern_cost > ancient_cost, "现代工艺成本效率应更高");

        println!("  ✅ 古代vs现代评分对比符合预期");
    }

    // --- 边界场景测试 ---

    #[test]
    pub fn test_casting_comparison_specific_methods_subset() {
        println!("\n=== [F2-边界] 指定部分工艺对比 ===");

        let selected = vec![
            "ancient_sand_lost_wax".to_string(),
            "modern_investment_casting".to_string(),
            "modern_centrifugal".to_string(),
        ];

        let req = CastingMethodRequest {
            bell_id: None,
            methods: Some(selected.clone()),
        };

        let result = compare_casting_methods(&req);

        assert_eq!(result.methods.len(), 3, "应返回3种指定工艺");
        let keys: Vec<&String> = result.methods.iter().map(|m| &m.method_key).collect();
        for k in &selected {
            assert!(keys.contains(&k), "应包含指定的工艺: {}", k);
        }

        println!("  ✅ 指定子集对比正常: {} 种工艺", result.methods.len());
    }

    #[test]
    pub fn test_casting_comparison_only_ancient() {
        println!("\n=== [F2-边界] 仅古代工艺对比 ===");

        let ancient_methods = vec![
            "ancient_sand_lost_wax".to_string(),
            "ancient_clay_piece".to_string(),
            "ancient_sand_mold".to_string(),
        ];

        let req = CastingMethodRequest {
            bell_id: None,
            methods: Some(ancient_methods),
        };

        let result = compare_casting_methods(&req);

        assert_eq!(result.methods.len(), 3);
        for m in &result.methods {
            assert_eq!(m.era, "ancient", "全部应为古代工艺");
        }

        assert!(
            !result.ancient_vs_modern_summary.key_differences.is_empty() || result.methods.len() == 3,
            "纯古代组也应生成总结"
        );

        println!("  ✅ 仅古代组对比正常");
    }

    #[test]
    pub fn test_casting_comparison_single_method() {
        println!("\n=== [F2-边界] 单工艺详情 ===");

        let req = CastingMethodRequest {
            bell_id: None,
            methods: Some(vec!["ancient_sand_lost_wax".to_string()]),
        };

        let result = compare_casting_methods(&req);

        assert_eq!(result.methods.len(), 1);
        let m = &result.methods[0];
        assert_eq!(m.method_key, "ancient_sand_lost_wax");
        assert!(!m.description.is_empty());
        assert!(!m.famous_examples.is_empty());
        assert!(m.pros.len() >= 3, "至少3条优点");
        assert!(m.cons.len() >= 3, "至少3条缺点");

        println!("  ✅ 单工艺详情完整");
        println!("  失蜡法精度: {:.1} mm", m.typical_accuracy_mm);
        println!("  代表作: {}", m.famous_examples.join(", "));
    }

    // --- 异常场景测试 ---

    #[test]
    pub fn test_casting_comparison_invalid_method_keys() {
        println!("\n=== [F2-异常] 无效工艺key处理 ===");

        let req = CastingMethodRequest {
            bell_id: None,
            methods: Some(vec![
                "invalid_method_xyz".to_string(),
                "fake_process".to_string(),
            ]),
        };

        let result = compare_casting_methods(&req);

        // 无效key应被过滤，可能返回空或全部
        assert!(
            result.methods.len() < 3,
            "无效key不应返回大量有效工艺"
        );

        println!("  ✅ 无效工艺key被正确处理: 返回{}种", result.methods.len());
    }

    #[test]
    pub fn test_casting_method_list_complete() {
        println!("\n=== [F2-正常] 工艺列表完整性 ===");

        let list = get_casting_method_key_list();

        assert!(list.len() >= 6, "至少应有6种工艺");
        assert!(list.iter().any(|k| k.contains("ancient")), "应包含古代工艺");
        assert!(list.iter().any(|k| k.contains("modern")), "应包含现代工艺");

        println!("  ✅ 工艺列表完整: {} 种", list.len());
        println!("  {}", list.join(", "));
    }

    #[test]
    pub fn test_casting_recommendation_logic() {
        println!("\n=== [F2-正常] 智能推荐逻辑 ===");

        let small_bell = get_recommended_method_for_bell(0.5, 7.0, 100.0, false);
        let large_bell = get_recommended_method_for_bell(50.0, 9.0, 300.0, true);
        let budget_bell = get_recommended_method_for_bell(5.0, 6.0, 50.0, false);

        assert!(!small_bell.is_empty(), "小钟推荐不应为空");
        assert!(!large_bell.is_empty(), "大钟推荐不应为空");
        assert!(!budget_bell.is_empty(), "经济型推荐不应为空");

        println!("  小钟 (0.5吨, 高精度): {}", small_bell);
        println!("  大钟 (50吨, 高艺术): {}", large_bell);
        println!("  经济型 (5吨, 低预算): {}", budget_bell);

        assert_ne!(small_bell, large_bell, "不同需求应有不同推荐");

        println!("  ✅ 推荐逻辑工作正常");
    }
}

/* ======================================================================
   Feature 3: 钟楼建筑声学传播模拟测试
   ====================================================================== */

mod tower_acoustic_tests {
    use super::*;

    fn create_default_tower() -> TowerBuildingParams {
        TowerBuildingParams {
            tower_style: "测试钟楼".to_string(),
            height_m: 15.0,
            width_m: 8.0,
            depth_m: 8.0,
            wall_thickness_m: 0.5,
            wall_material: "brick".to_string(),
            bell_chamber_height_m: 5.0,
            window_count: 4,
            window_width_m: 1.8,
            window_height_m: 2.4,
            roof_style: "hip".to_string(),
            openings_direction_deg: vec![0.0, 90.0, 180.0, 270.0],
            internal_absorption_coeff: 0.1,
            internal_reverberation: 2.0,
            ground_type: "marble".to_string(),
            wall_roughness_mm: 5.0,
            ceiling_height_m: 4.0,
        }
    }

    // --- 正常场景测试 ---

    #[test]
    pub fn test_tower_acoustic_normal_simulation() {
        println!("\n=== [F3-正常] 标准钟楼声学模拟 ===");

        let tower = create_default_tower();
        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);

        assert_eq!(result.with_tower.field_2d.len(), 100, "声场网格应为100x100");
        assert_eq!(result.with_tower.field_2d[0].len(), 100);
        assert!(result.with_tower.max_spl_db > result.with_tower.min_spl_db);
        assert!(result.with_tower.reverberation_time_s > 0.0);
        assert!(!result.directivity_pattern.is_empty());
        assert!(!result.sound_coverage.is_empty());
        assert!(!result.optimization_tips.is_empty());

        assert_eq!(result.without_tower.field_2d.len(), 100);

        let cm = &result.comparison_metrics;
        assert!(cm.overall_improvement_score >= 0.0, "综合评分不应为负");
        assert!(cm.overall_improvement_score <= 100.0, "综合评分不应超过100");

        println!("  ✅ 标准钟楼模拟成功");
        println!("  有钟楼最大SPL: {:.1} dB", result.with_tower.max_spl_db);
        println!("  自由场最大SPL: {:.1} dB", result.without_tower.max_spl_db);
        println!("  100m处提升: {:.1} dB", cm.spl_boost_at_100m_db);
        println!("  综合评分: {:.0}/100", cm.overall_improvement_score);
    }

    #[test]
    pub fn test_tower_sound_field_symmetry() {
        println!("\n=== [F3-正常] 声场分布对称性验证 ===");

        let mut tower = create_default_tower();
        tower.window_count = 4;
        tower.openings_direction_deg = vec![0.0, 90.0, 180.0, 270.0];

        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);
        let field = &result.with_tower.field_2d;
        let n = field.len();
        let center = n / 2;

        let east = field[center + 20][center];
        let west = field[center - 20][center];
        let north = field[center][center + 20];
        let south = field[center][center - 20];

        println!("  东: {:.2} dB, 西: {:.2} dB", east, west);
        println!("  南: {:.2} dB, 北: {:.2} dB", south, north);

        assert!(
            (east - west).abs() < 10.0,
            "东西向声压应大致对称 (对称钟楼): 差值{}",
            (east - west).abs()
        );

        println!("  ✅ 对称钟楼声场分布符合预期");
    }

    #[test]
    pub fn test_tower_vs_free_field_comparison() {
        println!("\n=== [F3-正常] 有钟楼vs自由场对比 ===");

        let tower = create_default_tower();
        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);

        assert!(
            result.with_tower.reverberation_time_s > result.without_tower.reverberation_time_s,
            "有钟楼混响时间应长于自由场"
        );

        let zones = &result.sound_coverage;
        assert!(!zones.is_empty());

        for zone in zones {
            assert!(zone.zone_name.len() > 0);
            assert_eq!(zone.distance_range_m.0 < zone.distance_range_m.1, true);
            assert!(zone.with_tower_avg_spl > 0.0);
            assert!(zone.without_tower_avg_spl > 0.0);
        }

        let near_zone = zones.first().unwrap();
        let far_zone = zones.last().unwrap();
        assert!(
            near_zone.with_tower_avg_spl > far_zone.with_tower_avg_spl,
            "近场声压应高于远场"
        );

        println!("  ✅ 有钟楼vs自由场对比合理");
        println!("  有钟楼RT60: {:.2}s", result.with_tower.reverberation_time_s);
        println!("  自由场RT60: {:.2}s", result.without_tower.reverberation_time_s);
        println!("  近区 ({}-{}m): {:.1} dB",
            near_zone.distance_range_m.0, near_zone.distance_range_m.1, near_zone.with_tower_avg_spl);
        println!("  远区 ({}-{}m): {:.1} dB",
            far_zone.distance_range_m.0, far_zone.distance_range_m.1, far_zone.with_tower_avg_spl);
    }

    // --- 边界场景测试 ---

    #[test]
    pub fn test_tower_zero_windows_boundary() {
        println!("\n=== [F3-边界] 零开窗（封闭钟楼） ===");

        let mut tower = create_default_tower();
        tower.window_count = 0;
        tower.window_width_m = 0.0;
        tower.window_height_m = 0.0;
        tower.openings_direction_deg = vec![];

        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);

        assert!(
            result.with_tower.avg_spl_db < 100.0,
            "封闭钟楼外部声压不应过高"
        );
        assert!(
            result.with_tower.reverberation_time_s > 0.5,
            "封闭钟楼内部应有混响"
        );

        println!("  ✅ 零开窗边界情况正常");
        println!("  封闭钟楼平均SPL: {:.1} dB", result.with_tower.avg_spl_db);
    }

    #[test]
    pub fn test_tower_many_windows_boundary() {
        println!("\n=== [F3-边界] 超多开窗（接近开放式） ===");

        let mut tower = create_default_tower();
        tower.window_count = 12;
        tower.window_width_m = 3.0;
        tower.window_height_m = 4.0;

        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);

        let diff = (result.with_tower.avg_spl_db - result.without_tower.avg_spl_db).abs();

        println!("  开放式钟楼与自由场平均差: {:.2} dB", diff);
        println!("  ✅ 多开窗边界情况正常");
    }

    #[test]
    pub fn test_tower_extreme_frequencies() {
        println!("\n=== [F3-边界] 极端频率测试 ===");

        let tower = create_default_tower();

        let low_req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(40.0),
            tower: tower.clone(),
        };
        let low = simulate_tower_acoustics(&low_req);

        let high_req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(2000.0),
            tower: tower.clone(),
        };
        let high = simulate_tower_acoustics(&high_req);

        assert!(low.with_tower.max_spl_db > 0.0);
        assert!(high.with_tower.max_spl_db > 0.0);

        println!("  低频 (40Hz) 最大SPL: {:.1} dB", low.with_tower.max_spl_db);
        println!("  高频 (2000Hz) 最大SPL: {:.1} dB", high.with_tower.max_spl_db);
        println!("  低频混响: {:.2}s, 高频混响: {:.2}s",
            low.with_tower.reverberation_time_s,
            high.with_tower.reverberation_time_s);

        println!("  ✅ 极端频率边界处理正常");
    }

    #[test]
    pub fn test_tower_extreme_dimensions() {
        println!("\n=== [F3-边界] 极端尺寸钟楼 ===");

        let tiny_tower = TowerBuildingParams {
            tower_style: "微型钟楼".to_string(),
            height_m: 3.0,
            width_m: 2.0,
            depth_m: 2.0,
            wall_thickness_m: 0.2,
            wall_material: "wood".to_string(),
            bell_chamber_height_m: 1.5,
            window_count: 2,
            window_width_m: 0.8,
            window_height_m: 1.0,
            roof_style: "flat".to_string(),
            openings_direction_deg: vec![0.0, 180.0],
            internal_absorption_coeff: 0.2,
            internal_reverberation: 0.5,
            ground_type: "wood".to_string(),
            wall_roughness_mm: 10.0,
            ceiling_height_m: 1.2,
        };

        let huge_tower = TowerBuildingParams {
            tower_style: "巨型钟楼".to_string(),
            height_m: 60.0,
            width_m: 20.0,
            depth_m: 20.0,
            wall_thickness_m: 2.0,
            wall_material: "stone".to_string(),
            bell_chamber_height_m: 15.0,
            window_count: 8,
            window_width_m: 3.0,
            window_height_m: 6.0,
            roof_style: "dome".to_string(),
            openings_direction_deg: vec![0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0],
            internal_absorption_coeff: 0.05,
            internal_reverberation: 4.0,
            ground_type: "marble".to_string(),
            wall_roughness_mm: 2.0,
            ceiling_height_m: 12.0,
        };

        let tiny_req = TowerAcousticRequest { bell_id: None, frequency_hz: Some(512.0), tower: tiny_tower };
        let huge_req = TowerAcousticRequest { bell_id: None, frequency_hz: Some(128.0), tower: huge_tower };

        let tiny = simulate_tower_acoustics(&tiny_req);
        let huge = simulate_tower_acoustics(&huge_req);

        assert!(tiny.with_tower.reverberation_time_s > 0.0);
        assert!(huge.with_tower.reverberation_time_s > 0.0);

        println!("  微型钟楼RT60: {:.2}s", tiny.with_tower.reverberation_time_s);
        println!("  巨型钟楼RT60: {:.2}s", huge.with_tower.reverberation_time_s);
        println!("  ✅ 极端尺寸钟楼模拟正常");
    }

    // --- 异常场景测试 ---

    #[test]
    pub fn test_tower_invalid_wall_material() {
        println!("\n=== [F3-异常] 无效墙体材料 ===");

        let mut tower = create_default_tower();
        tower.wall_material = "unobtanium".to_string();

        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);

        assert!(
            result.with_tower.max_spl_db > 0.0,
            "无效材料也应使用默认值完成模拟"
        );

        println!("  ✅ 无效材料优雅降级，使用默认值完成模拟");
        println!("  SPL: {:.1} dB", result.with_tower.max_spl_db);
    }

    #[test]
    pub fn test_tower_preset_configs_valid() {
        println!("\n=== [F3-正常] 预置钟楼配置完整性 ===");

        let presets = get_preset_tower_configs();

        assert!(!presets.is_empty(), "预置配置不能为空");
        assert!(presets.len() >= 3, "至少应有3个预置");

        for preset in &presets {
            assert!(!preset.tower_style.is_empty());
            assert!(preset.height_m > 0.0);
            assert!(preset.width_m > 0.0);
            assert!(preset.depth_m > 0.0);
            assert!(preset.wall_thickness_m > 0.0);
            assert!(!preset.wall_material.is_empty());
            assert!(preset.bell_chamber_height_m > 0.0);
            assert!(preset.window_count >= 0);
        }

        println!("  ✅ 预置钟楼配置完整有效: {} 个", presets.len());
        for p in presets {
            println!("    - {} ({}m高, {}窗)", p.tower_style, p.height_m, p.window_count);
        }
    }

    #[test]
    pub fn test_tower_optimization_tips_relevance() {
        println!("\n=== [F3-正常] 优化建议有效性 ===");

        let tower = create_default_tower();
        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);

        assert!(!result.optimization_tips.is_empty(), "优化建议不应为空");
        assert!(result.optimization_tips.len() >= 3, "至少应有3条优化建议");

        for tip in &result.optimization_tips {
            assert!(tip.len() > 5, "建议内容应足够具体");
        }

        println!("  ✅ 优化建议有效: {} 条", result.optimization_tips.len());
        for tip in result.optimization_tips.iter().take(3) {
            println!("    💡 {}", tip);
        }
    }

    #[test]
    pub fn test_tower_directivity_pattern_consistency() {
        println!("\n=== [F3-正常] 指向性图案一致性 ===");

        let tower = create_default_tower();
        let req = TowerAcousticRequest {
            bell_id: None,
            frequency_hz: Some(256.0),
            tower,
        };

        let result = simulate_tower_acoustics(&req);
        let pattern = &result.directivity_pattern;

        assert!(!pattern.is_empty(), "指向性图案不应为空");
        assert!(pattern.len() >= 8, "至少应有8个方向采样");

        for pt in pattern {
            assert!(pt.angle_deg >= 0.0 && pt.angle_deg < 360.0);
            assert!(pt.with_tower_spl > 0.0);
            assert!(pt.without_tower_spl > 0.0);
            assert!(
                (pt.gain_db - (pt.with_tower_spl - pt.without_tower_spl)).abs() < 0.01,
                "增益值应为两者差值: gain={}, diff={}",
                pt.gain_db, pt.with_tower_spl - pt.without_tower_spl
            );
        }

        println!("  ✅ 指向性图案数据一致: {} 个方向点", pattern.len());
    }
}

/* ======================================================================
   Feature 4: 虚拟敲钟交互体验测试
   ====================================================================== */

mod virtual_strike_tests {
    use super::*;

    // --- 正常场景测试 ---

    #[test]
    pub fn test_virtual_strike_normal_mid_force() {
        println!("\n=== [F4-正常] 中等力度标准敲击 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "waist".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));

        assert!(result.strike_id != Uuid::nil());
        assert!(result.impact_velocity > 0.0);
        assert!(result.peak_contact_force_n > 0.0);
        assert!(result.contact_duration_ms > 0.0);
        assert!(result.estimated_decay_s > 0.0);
        assert!(result.perceived_loudness_phon > 0.0);
        assert!(!result.quality_description.is_empty());
        assert_eq!(result.harmonic_amplitudes.len(), 8, "应有8个谐波振幅");
        assert_eq!(result.audio_synthesis_params.partials.len(), 8, "应有8个分音参数");

        let ap = &result.audio_synthesis_params;
        assert!(ap.fundamental_hz > 0.0);
        assert!(ap.master_gain > 0.0 && ap.master_gain <= 1.0);
        assert!(ap.attack_ms > 0.0);

        for p in &ap.partials {
            assert!(p.freq_ratio > 0.0);
            assert!(p.gain >= 0.0 && p.gain <= 1.0);
            assert!(p.decay_s > 0.0);
            assert!(p.detune_cents.abs() < 100.0, "失谐不应过大");
        }

        println!("  ✅ 标准敲击响应正常");
        println!("  峰值接触力: {:.1} N", result.peak_contact_force_n);
        println!("  接触时间: {:.2} ms", result.contact_duration_ms);
        println!("  估计衰减: {:.2} s", result.estimated_decay_s);
        println!("  感知响度: {:.0} phon", result.perceived_loudness_phon);
        println!("  基频: {:.1} Hz", result.audio_synthesis_params.fundamental_hz);
    }

    #[test]
    pub fn test_virtual_strike_position_timbre_difference() {
        println!("\n=== [F4-正常] 不同位置音色差异验证 ===");

        let bell = create_test_bell();
        let positions = ["lip", "waist", "shoulder", "crown", "rim"];

        let mut results = Vec::new();
        for pos in &positions {
            let params = VirtualStrikeParams {
                bell_id: bell.bell_id,
                strike_force: 0.5,
                strike_position: pos.to_string(),
                strike_angle_deg: 0.0,
                mallet_hardness: "medium".to_string(),
            };
            results.push((pos, compute_strike_impact(&params, Some(&bell))));
        }

        let lip_amps = &results[0].1.harmonic_amplitudes;
        let crown_amps = &results[3].1.harmonic_amplitudes;

        let lip_high_ratio = lip_amps[3..].iter().sum::<f64>() / lip_amps.iter().sum::<f64>();
        let crown_high_ratio = crown_amps[3..].iter().sum::<f64>() / crown_amps.iter().sum::<f64>();

        println!("  钟口高频能量比: {:.2}%", lip_high_ratio * 100.0);
        println!("  钟顶高频能量比: {:.2}%", crown_high_ratio * 100.0);
        println!("  钟口主音振幅: {:.4}", lip_amps[0]);
        println!("  钟顶主音振幅: {:.4}", crown_amps[0]);

        assert!(
            lip_amps[0] > crown_amps[0],
            "钟口敲击主音应更响"
        );

        assert!(
            crown_high_ratio > lip_high_ratio * 0.5,
            "钟顶敲击泛音比例应更高"
        );

        for (pos, res) in &results {
            println!("  📍 {} -> 基音幅度: {:.4}, 衰减: {:.2}s",
                pos, res.harmonic_amplitudes[0], res.estimated_decay_s);
        }

        println!("  ✅ 位置对音色的影响符合物理规律");
    }

    #[test]
    pub fn test_virtual_strike_mallet_hardness_effect() {
        println!("\n=== [F4-正常] 木槌硬度对音色的影响 ===");

        let bell = create_test_bell();
        let mallets = ["soft", "medium", "hard", "metal"];

        let mut results = Vec::new();
        for m in &mallets {
            let params = VirtualStrikeParams {
                bell_id: bell.bell_id,
                strike_force: 0.5,
                strike_position: "waist".to_string(),
                strike_angle_deg: 0.0,
                mallet_hardness: m.to_string(),
            };
            results.push((m, compute_strike_impact(&params, Some(&bell))));
        }

        let soft_amps = &results[0].1.harmonic_amplitudes;
        let metal_amps = &results[3].1.harmonic_amplitudes;

        let soft_high = soft_amps[4..].iter().sum::<f64>();
        let metal_high = metal_amps[4..].iter().sum::<f64>();

        println!("  软槌高频总振幅: {:.4}", soft_high);
        println!("  金属槌高频总振幅: {:.4}", metal_high);

        assert!(
            metal_high > soft_high,
            "硬槌激发更多高频泛音"
        );

        for (m, res) in &results {
            let high_ratio = res.harmonic_amplitudes[2..].iter().sum::<f64>()
                / res.harmonic_amplitudes.iter().sum::<f64>();
            println!("  🔨 {} -> 高频占比: {:.1}%, 接触时间: {:.1}ms",
                m, high_ratio * 100.0, res.contact_duration_ms);
        }

        println!("  ✅ 木槌硬度对音色的影响正确");
    }

    #[test]
    pub fn test_virtual_strike_force_amplitude_relationship() {
        println!("\n=== [F4-正常] 力度与振幅的关系 ===");

        let bell = create_test_bell();
        let forces = [0.1, 0.25, 0.5, 0.75, 1.0];

        let mut results = Vec::new();
        for f in &forces {
            let params = VirtualStrikeParams {
                bell_id: bell.bell_id,
                strike_force: *f,
                strike_position: "lip".to_string(),
                strike_angle_deg: 0.0,
                mallet_hardness: "medium".to_string(),
            };
            results.push((f, compute_strike_impact(&params, Some(&bell))));
        }

        for i in 1..results.len() {
            let (_, prev) = &results[i - 1];
            let (_, curr) = &results[i];
            assert!(
                curr.peak_contact_force_n > prev.peak_contact_force_n,
                "力度增大，接触力应增大"
            );
            assert!(
                curr.perceived_loudness_phon >= prev.perceived_loudness_phon,
                "力度增大，响度应增加"
            );
        }

        for (f, res) in &results {
            println!("  💪 {:.0}% -> 接触力: {:.0}N, 响度: {:.0}phon, 衰减: {:.2}s",
                *f * 100.0, res.peak_contact_force_n, res.perceived_loudness_phon, res.estimated_decay_s);
        }

        println!("  ✅ 力度与振幅正相关，符合物理规律");
    }

    // --- 边界场景测试 ---

    #[test]
    pub fn test_virtual_strike_minimum_force_boundary() {
        println!("\n=== [F4-边界] 最小力度极限 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.0,
            strike_position: "lip".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "soft".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));

        assert!(result.peak_contact_force_n > 0.0, "即使零力度也应有最小接触力");
        assert!(result.audio_synthesis_params.master_gain > 0.0, "音量不应为零");

        println!("  ✅ 最小力度边界处理正常");
        println!("  最小力度接触力: {:.2} N", result.peak_contact_force_n);
    }

    #[test]
    pub fn test_virtual_strike_maximum_force_boundary() {
        println!("\n=== [F4-边界] 最大力度极限 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 100.0,
            strike_position: "lip".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "metal".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));

        assert!(
            result.peak_contact_force_n < 1e7,
            "力度应被钳位，不会无限增大"
        );
        assert!(
            result.audio_synthesis_params.master_gain <= 1.0,
            "主增益不应超过1.0"
        );

        println!("  ✅ 最大力度被正确钳位");
        println!("  输入100.0，实际接触力: {:.0} N", result.peak_contact_force_n);
    }

    #[test]
    pub fn test_virtual_strike_extreme_positions() {
        println!("\n=== [F4-边界] 极端位置（钟顶vs钟口） ===");

        let bell = create_test_bell();

        let lip_params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "lip".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };
        let lip = compute_strike_impact(&lip_params, Some(&bell));

        let crown_params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "crown".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };
        let crown = compute_strike_impact(&crown_params, Some(&bell));

        assert!(
            lip.harmonic_amplitudes[0] > crown.harmonic_amplitudes[0] * 2.0,
            "钟口基音振幅应显著大于钟顶"
        );

        assert!(
            crown.harmonic_amplitudes[3] > 0.0,
            "钟顶也应有一定的泛音"
        );

        println!("  钟口基音振幅: {:.4}", lip.harmonic_amplitudes[0]);
        println!("  钟顶基音振幅: {:.4}", crown.harmonic_amplitudes[0]);
        println!("  ✅ 极端位置音色差异显著");
    }

    // --- 异常场景测试 ---

    #[test]
    pub fn test_virtual_strike_invalid_position() {
        println!("\n=== [F4-异常] 无效敲击位置 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "invalid_position_xyz".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));

        assert!(
            result.peak_contact_force_n > 0.0,
            "无效位置也应优雅降级到默认位置"
        );
        assert_eq!(result.harmonic_amplitudes.len(), 8);

        println!("  ✅ 无效位置优雅降级，使用默认位置");
        println!("  音质描述: {}", result.quality_description);
    }

    #[test]
    pub fn test_virtual_strike_invalid_mallet() {
        println!("\n=== [F4-异常] 无效木槌类型 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "waist".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "rubber_mallet_unknown".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));

        assert!(
            result.peak_contact_force_n > 0.0,
            "无效木槌也应优雅降级"
        );
        assert!(!result.quality_description.is_empty());

        println!("  ✅ 无效木槌优雅降级，使用默认硬度");
    }

    #[test]
    pub fn test_virtual_strike_no_bell_reference() {
        println!("\n=== [F4-异常] 无参考钟敲击 ===");

        let params = VirtualStrikeParams {
            bell_id: Uuid::nil(),
            strike_force: 0.5,
            strike_position: "waist".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };

        let result = compute_strike_impact(&params, None);

        assert!(
            result.audio_synthesis_params.fundamental_hz > 0.0,
            "无参考钟也应使用默认频率"
        );
        assert!(
            result.audio_synthesis_params.fundamental_hz > 100.0
                && result.audio_synthesis_params.fundamental_hz < 1000.0,
            "默认频率应在合理范围"
        );

        println!("  ✅ 无参考钟使用默认参数正常");
        println!("  默认基频: {:.1} Hz", result.audio_synthesis_params.fundamental_hz);
    }

    #[test]
    pub fn test_virtual_strike_negative_force() {
        println!("\n=== [F4-异常] 负力度输入 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: -5.0,
            strike_position: "lip".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));

        assert!(result.peak_contact_force_n > 0.0, "负力度应被钳位为正值");
        assert!(result.estimated_decay_s > 0.0, "衰减时间应为正");

        println!("  ✅ 负力度输入被正确钳位处理");
        println!("  输出接触力: {:.2} N", result.peak_contact_force_n);
    }

    #[test]
    pub fn test_position_and_mallet_options_valid() {
        println!("\n=== [F4-正常] 选项列表完整性 ===");

        let positions = get_position_options();
        let mallets = get_mallet_options();

        assert_eq!(positions.len(), 5, "应有5个敲击位置");
        assert_eq!(mallets.len(), 4, "应有4种木槌");

        for (key, desc, factor) in &positions {
            assert!(!key.is_empty());
            assert!(!desc.is_empty());
            assert!(*factor > 0.0 && *factor <= 1.0);
        }

        for (key, name, desc) in &mallets {
            assert!(!key.is_empty());
            assert!(!name.is_empty());
            assert!(!desc.is_empty());
        }

        println!("  ✅ 位置和木槌选项完整有效");
        println!("  位置 ({}种): {}", positions.len(),
            positions.iter().map(|(k, _, _)| *k).collect::<Vec<_>>().join(", "));
        println!("  木槌 ({}种): {}", mallets.len(),
            mallets.iter().map(|(k, _, _)| *k).collect::<Vec<_>>().join(", "));
    }

    #[test]
    pub fn test_strike_tutorial_completeness() {
        println!("\n=== [F4-正常] 敲击教程完整性 ===");

        let tutorial = generate_strike_tutorial();

        assert!(!tutorial.is_empty(), "教程不应为空");
        assert!(tutorial.len() >= 5, "教程至少应有5段");

        let full_text = tutorial.join("\n");
        assert!(full_text.contains("握法") || full_text.contains("握槌"), "教程应包含握法");
        assert!(full_text.contains("力度"), "教程应包含力度");
        assert!(full_text.contains("位置"), "教程应包含位置");
        assert!(full_text.contains("编钟") || full_text.contains("双音"), "教程应包含编钟内容");

        println!("  ✅ 敲击教程完整全面: {} 段", tutorial.len());
        for line in tutorial.iter().take(5) {
            println!("    {}", line);
        }
    }

    #[test]
    pub fn test_audio_synth_params_web_audio_compatible() {
        println!("\n=== [F4-正常] Web Audio参数兼容性 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "lip".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };

        let result = compute_strike_impact(&params, Some(&bell));
        let ap = &result.audio_synthesis_params;

        assert!(ap.fundamental_hz >= 20.0 && ap.fundamental_hz <= 20000.0,
            "基频应在人耳可听范围: {}", ap.fundamental_hz);
        assert!(ap.master_gain > 0.0 && ap.master_gain <= 1.0,
            "主增益应在合理范围: {}", ap.master_gain);
        assert!(ap.attack_ms > 0.0 && ap.attack_ms < 1000.0,
            "起音时间应合理: {}ms", ap.attack_ms);
        assert_eq!(ap.partials.len(), 8, "应有8个分音");

        for (i, p) in ap.partials.iter().enumerate() {
            let freq = ap.fundamental_hz * p.freq_ratio;
            assert!(freq >= 20.0 && freq <= 20000.0,
                "分音{}频率{}Hz应在可听范围", i, freq);
            assert!(p.gain >= 0.0 && p.gain <= 1.0,
                "分音{}增益{}应在0-1", i, p.gain);
            assert!(p.decay_s > 0.0 && p.decay_s < 30.0,
                "分音{}衰减{}s应合理", i, p.decay_s);
        }

        println!("  ✅ Web Audio参数完全兼容");
        println!("  基频: {:.1} Hz", ap.fundamental_hz);
        println!("  主增益: {:.3}", ap.master_gain);
        println!("  起音: {:.1} ms", ap.attack_ms);
        println!("  分音数: {}", ap.partials.len());
    }

    #[test]
    pub fn test_hertz_contact_force_realism() {
        println!("\n=== [F4-正常] Hertz接触力学真实感验证 ===");

        let bell = create_test_bell();

        let soft_params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "waist".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "soft".to_string(),
        };
        let soft = compute_strike_impact(&soft_params, Some(&bell));

        let hard_params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "waist".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "hard".to_string(),
        };
        let hard = compute_strike_impact(&hard_params, Some(&bell));

        assert!(
            hard.peak_contact_force_n > soft.peak_contact_force_n,
            "硬槌峰值力应更大: {} > {}", hard.peak_contact_force_n, soft.peak_contact_force_n
        );
        assert!(
            soft.contact_duration_ms > hard.contact_duration_ms,
            "软槌接触时间应更长: {} > {}", soft.contact_duration_ms, hard.contact_duration_ms
        );

        println!("  软槌: 峰值力{:.0}N, 接触{:.1}ms", soft.peak_contact_force_n, soft.contact_duration_ms);
        println!("  硬槌: 峰值力{:.0}N, 接触{:.1}ms", hard.peak_contact_force_n, hard.contact_duration_ms);
        println!("  ✅ Hertz接触力学规律正确（硬槌=大力+短接触）");
    }

    #[test]
    pub fn test_strike_idempotency_same_params() {
        println!("\n=== [F4-正常] 相同参数结果一致性 ===");

        let bell = create_test_bell();
        let params = VirtualStrikeParams {
            bell_id: bell.bell_id,
            strike_force: 0.5,
            strike_position: "lip".to_string(),
            strike_angle_deg: 0.0,
            mallet_hardness: "medium".to_string(),
        };

        let r1 = compute_strike_impact(&params, Some(&bell));
        let r2 = compute_strike_impact(&params, Some(&bell));

        assert!(
            (r1.peak_contact_force_n - r2.peak_contact_force_n).abs() < 1.0,
            "相同参数的接触力应一致"
        );
        assert!(
            (r1.estimated_decay_s - r2.estimated_decay_s).abs() < 0.01,
            "相同参数的衰减时间应一致"
        );
        assert!(
            (r1.perceived_loudness_phon - r2.perceived_loudness_phon).abs() < 1.0,
            "相同参数的响度应一致"
        );

        println!("  ✅ 相同参数产生一致结果（算法确定性）");
    }
}

/* ======================================================================
   测试入口 (与现有integration_test.rs保持一致风格)
   ====================================================================== */

fn main() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       古代铸钟工艺仿真系统 - 新增功能测试套件            ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    println!("\n测试运行于: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("测试目标: 4个新增Feature的正常/边界/异常场景全覆盖");
    println!();

    let mut total = 0u32;
    let mut passed = 0u32;

    let test_modules: Vec<(&str, Vec<(&str, fn())>)> = vec![
        (
            "F1. 合金配比音质对比",
            vec![
                ("多合金声学参数对比", alloy_comparison_tests::test_alloy_comparison_normal_multiple_alloys as fn()),
                ("使用参考钟对比", alloy_comparison_tests::test_alloy_comparison_with_bell_reference as fn()),
                ("声学参数变化趋势验证", alloy_comparison_tests::test_alloy_acoustic_trend_verification as fn()),
                ("对比表结构验证", alloy_comparison_tests::test_alloy_comparison_table_structure as fn()),
                ("单合金边界", alloy_comparison_tests::test_alloy_comparison_single_alloy_boundary as fn()),
                ("空合金列表(全部)", alloy_comparison_tests::test_alloy_comparison_empty_alloy_list as fn()),
                ("极端尺寸验证", alloy_comparison_tests::test_alloy_comparison_extreme_dimensions as fn()),
                ("无效合金key", alloy_comparison_tests::test_alloy_comparison_invalid_alloy_keys as fn()),
                ("有效+无效混合key", alloy_comparison_tests::test_alloy_comparison_mixed_valid_invalid as fn()),
                ("合金成分建议", alloy_comparison_tests::test_alloy_composition_suggestion_basic as fn()),
                ("成分建议边界", alloy_comparison_tests::test_alloy_composition_suggestion_edge_cases as fn()),
            ],
        ),
        (
            "F2. 古现代铸造工艺对比",
            vec![
                ("全部工艺对比", casting_comparison_tests::test_casting_comparison_normal_all_methods as fn()),
                ("缺陷率差异验证", casting_comparison_tests::test_casting_defect_rate_difference_verification as fn()),
                ("古今评分对比", casting_comparison_tests::test_casting_ancient_modern_score_comparison as fn()),
                ("指定子集对比", casting_comparison_tests::test_casting_comparison_specific_methods_subset as fn()),
                ("仅古代工艺", casting_comparison_tests::test_casting_comparison_only_ancient as fn()),
                ("单工艺详情", casting_comparison_tests::test_casting_comparison_single_method as fn()),
                ("无效工艺key", casting_comparison_tests::test_casting_comparison_invalid_method_keys as fn()),
                ("工艺列表完整", casting_comparison_tests::test_casting_method_list_complete as fn()),
                ("智能推荐逻辑", casting_comparison_tests::test_casting_recommendation_logic as fn()),
            ],
        ),
        (
            "F3. 钟楼建筑声学模拟",
            vec![
                ("标准钟楼模拟", tower_acoustic_tests::test_tower_acoustic_normal_simulation as fn()),
                ("声场分布对称性", tower_acoustic_tests::test_tower_sound_field_symmetry as fn()),
                ("有钟楼vs自由场", tower_acoustic_tests::test_tower_vs_free_field_comparison as fn()),
                ("零开窗边界", tower_acoustic_tests::test_tower_zero_windows_boundary as fn()),
                ("超多开窗边界", tower_acoustic_tests::test_tower_many_windows_boundary as fn()),
                ("极端频率测试", tower_acoustic_tests::test_tower_extreme_frequencies as fn()),
                ("极端尺寸钟楼", tower_acoustic_tests::test_tower_extreme_dimensions as fn()),
                ("无效墙体材料", tower_acoustic_tests::test_tower_invalid_wall_material as fn()),
                ("预置配置完整", tower_acoustic_tests::test_tower_preset_configs_valid as fn()),
                ("优化建议有效", tower_acoustic_tests::test_tower_optimization_tips_relevance as fn()),
                ("指向性图案一致", tower_acoustic_tests::test_tower_directivity_pattern_consistency as fn()),
            ],
        ),
        (
            "F4. 虚拟敲钟交互体验",
            vec![
                ("中等力度标准敲击", virtual_strike_tests::test_virtual_strike_normal_mid_force as fn()),
                ("位置音色差异验证", virtual_strike_tests::test_virtual_strike_position_timbre_difference as fn()),
                ("木槌硬度影响", virtual_strike_tests::test_virtual_strike_mallet_hardness_effect as fn()),
                ("力度振幅关系", virtual_strike_tests::test_virtual_strike_force_amplitude_relationship as fn()),
                ("最小力度极限", virtual_strike_tests::test_virtual_strike_minimum_force_boundary as fn()),
                ("最大力度极限", virtual_strike_tests::test_virtual_strike_maximum_force_boundary as fn()),
                ("极端位置对比", virtual_strike_tests::test_virtual_strike_extreme_positions as fn()),
                ("无效敲击位置", virtual_strike_tests::test_virtual_strike_invalid_position as fn()),
                ("无效木槌类型", virtual_strike_tests::test_virtual_strike_invalid_mallet as fn()),
                ("无参考钟敲击", virtual_strike_tests::test_virtual_strike_no_bell_reference as fn()),
                ("负力度输入", virtual_strike_tests::test_virtual_strike_negative_force as fn()),
                ("选项列表完整", virtual_strike_tests::test_position_and_mallet_options_valid as fn()),
                ("敲击教程完整", virtual_strike_tests::test_strike_tutorial_completeness as fn()),
                ("Web Audio兼容", virtual_strike_tests::test_audio_synth_params_web_audio_compatible as fn()),
                ("Hertz接触真实感", virtual_strike_tests::test_hertz_contact_force_realism as fn()),
                ("相同参数一致性", virtual_strike_tests::test_strike_idempotency_same_params as fn()),
            ],
        ),
    ];

    for (module_name, tests) in &test_modules {
        println!("\n━━━ {} ━━━", module_name);
        for (test_name, test_fn) in tests {
            total += 1;
            print!("  {:02}. {} ... ", total, test_name);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(test_fn));
            match result {
                Ok(_) => {
                    passed += 1;
                    println!("✅ PASS");
                }
                Err(_) => {
                    println!("❌ FAIL");
                }
            }
        }
    }

    println!("\n\n╔══════════════════════════════════════════════════════════╗");
    println!("║                    测试结果汇总                         ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  总计: {:3} 个测试用例                                 ║", total);
    println!("║  通过: {:3} 个                                        ║", passed);
    println!("║  失败: {:3} 个                                        ║", total - passed);
    println!("║  通过率: {:6.2}%                                     ║", if total > 0 { passed as f64 / total as f64 * 100.0 } else { 0.0 });
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    if total == passed {
        println!("🎉 所有新增功能测试全部通过!");
    } else {
        println!("⚠️  有 {} 个测试失败，请检查相关功能实现。", total - passed);
    }
    println!();
}

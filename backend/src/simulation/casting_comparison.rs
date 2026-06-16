use crate::models::*;
use std::collections::HashMap;

fn build_casting_method_database() -> Vec<CastingMethodMetrics> {
    vec![
        CastingMethodMetrics {
            method_key: "ancient_sand_lost_wax".to_string(),
            method_name: "古代失蜡法".to_string(),
            category: "砂型铸造".to_string(),
            era: "ancient".to_string(),
            description: "使用蜂蜡制模，外覆耐火泥砂，加热失蜡后浇铸。先秦青铜礼器的核心工艺。".to_string(),
            historical_period: "商代 - 汉代 (约公元前1600年 - 公元220年)".to_string(),
            typical_accuracy_mm: 1.5,
            surface_roughness_ra: 12.5,
            cooling_rate_cps: 2.5,
            typical_defect_rate_pct: 8.5,
            max_shrinkage_porosity_pct: 6.0,
            dimensional_tolerance_pct: 1.2,
            material_yield_pct: 65.0,
            production_cycle_days: 90.0,
            labor_intensity: 9.5,
            energy_consumption_kwh_per_ton: 1800.0,
            cost_per_kg: 380.0,
            environmental_impact_score: 3.0,
            skill_requirements_level: 9.5,
            max_cast_weight_tons: 50.0,
            minimum_thickness_mm: 4.0,
            microstructural_quality: 7.0,
            acoustic_quality_potential: 9.2,
            aesthetic_quality: 9.0,
            durability_years: 2500.0,
            pros: vec![
                "极致的艺术表现力，花纹极其精细".to_string(),
                "可铸造复杂镂空结构".to_string(),
                "单件独特性，具有收藏价值".to_string(),
                "历史文化价值极高".to_string(),
            ],
            cons: vec![
                "工序极其繁琐，无法批量生产".to_string(),
                "模料一次性使用，成本高".to_string(),
                "成品率低，失败率可达30%".to_string(),
                "对工匠技艺要求极高，传承人稀缺".to_string(),
            ],
            famous_examples: vec![
                "曾侯乙尊盘 (战国)".to_string(),
                "四羊方尊 (商)".to_string(),
                "莲鹤方壶 (春秋)".to_string(),
            ],
            standard_code: "CN-ARCH-001".to_string(),
            standard_reference: "中国古代铸造技艺非遗名录".to_string(),
            data_source: "archaeological_evidence".to_string(),
            quality_grade: "premium".to_string(),
        },
        CastingMethodMetrics {
            method_key: "ancient_clay_piece".to_string(),
            method_name: "古代分范法".to_string(),
            category: "sand_mold".to_string(),
            era: "ancient".to_string(),
            description: "使用泥土烧制的组合范块拼合铸型，编钟的标准铸造工艺。".to_string(),
            historical_period: "西周 - 明清 (约公元前1046年 - 1911年)".to_string(),
            typical_accuracy_mm: 2.0,
            surface_roughness_ra: 25.0,
            cooling_rate_cps: 3.0,
            typical_defect_rate_pct: 6.5,
            max_shrinkage_porosity_pct: 5.0,
            dimensional_tolerance_pct: 1.5,
            material_yield_pct: 70.0,
            production_cycle_days: 60.0,
            labor_intensity: 8.0,
            energy_consumption_kwh_per_ton: 1500.0,
            cost_per_kg: 280.0,
            environmental_impact_score: 2.5,
            skill_requirements_level: 8.5,
            max_cast_weight_tons: 80.0,
            minimum_thickness_mm: 5.0,
            microstructural_quality: 7.5,
            acoustic_quality_potential: 9.5,
            aesthetic_quality: 7.5,
            durability_years: 2000.0,
            pros: vec![
                "编钟音准控制最佳，壁厚均匀".to_string(),
                "可重复制作相同规格的钟".to_string(),
                "范块可部分重复使用".to_string(),
                "永乐大钟级别的超大型铸件".to_string(),
            ],
            cons: vec![
                "合范缝会留下痕迹".to_string(),
                "花纹精细度不如失蜡法".to_string(),
                "大型铸钟需数百块范，组装复杂".to_string(),
            ],
            famous_examples: vec![
                "曾侯乙编钟 (战国)".to_string(),
                "永乐大钟 (明)".to_string(),
                "寒山寺大钟 (唐)".to_string(),
            ],
            standard_code: "CN-ARCH-002".to_string(),
            standard_reference: "中国古代铸造技艺非遗名录 - 范铸工艺".to_string(),
            data_source: "archaeological_evidence".to_string(),
            quality_grade: "premium".to_string(),
        },
        CastingMethodMetrics {
            method_key: "ancient_sand_mold".to_string(),
            method_name: "古代砂型法".to_string(),
            category: "sand_mold".to_string(),
            era: "ancient".to_string(),
            description: "使用细砂加黏土粘结剂制作铸型，民间中小型佛钟常用工艺。".to_string(),
            historical_period: "汉代 - 现代民间 (约公元前202年 - 至今)".to_string(),
            typical_accuracy_mm: 4.0,
            surface_roughness_ra: 50.0,
            cooling_rate_cps: 5.0,
            typical_defect_rate_pct: 12.0,
            max_shrinkage_porosity_pct: 8.0,
            dimensional_tolerance_pct: 2.5,
            material_yield_pct: 60.0,
            production_cycle_days: 30.0,
            labor_intensity: 7.0,
            energy_consumption_kwh_per_ton: 1200.0,
            cost_per_kg: 150.0,
            environmental_impact_score: 4.0,
            skill_requirements_level: 6.0,
            max_cast_weight_tons: 30.0,
            minimum_thickness_mm: 8.0,
            microstructural_quality: 5.5,
            acoustic_quality_potential: 7.0,
            aesthetic_quality: 5.0,
            durability_years: 1200.0,
            pros: vec![
                "工艺简单，成本低廉".to_string(),
                "适合民间批量生产".to_string(),
                "材料随处可得".to_string(),
            ],
            cons: vec![
                "精度低，表面粗糙".to_string(),
                "音准一致性差".to_string(),
                "容易产生砂眼缺陷".to_string(),
            ],
            famous_examples: vec![
                "各地民间寺庙中小型佛钟".to_string(),
            ],
            standard_code: "CN-ARCH-003".to_string(),
            standard_reference: "民间传统工艺普查资料".to_string(),
            data_source: "folklore_survey".to_string(),
            quality_grade: "standard".to_string(),
        },
        CastingMethodMetrics {
            method_key: "modern_sand_green".to_string(),
            method_name: "现代湿砂型铸造".to_string(),
            category: "sand_mold".to_string(),
            era: "modern".to_string(),
            description: "机械化砂型铸造，使用膨润土粘结湿砂，现代工业标准工艺。".to_string(),
            historical_period: "20世纪初 - 至今".to_string(),
            typical_accuracy_mm: 1.2,
            surface_roughness_ra: 6.3,
            cooling_rate_cps: 8.0,
            typical_defect_rate_pct: 2.5,
            max_shrinkage_porosity_pct: 2.5,
            dimensional_tolerance_pct: 0.5,
            material_yield_pct: 88.0,
            production_cycle_days: 3.0,
            labor_intensity: 2.0,
            energy_consumption_kwh_per_ton: 900.0,
            cost_per_kg: 45.0,
            environmental_impact_score: 5.5,
            skill_requirements_level: 3.0,
            max_cast_weight_tons: 200.0,
            minimum_thickness_mm: 6.0,
            microstructural_quality: 6.0,
            acoustic_quality_potential: 6.5,
            aesthetic_quality: 4.0,
            durability_years: 800.0,
            pros: vec![
                "高度机械化，效率极高".to_string(),
                "成本最低，适合大批量".to_string(),
                "缺陷率可控".to_string(),
                "材料利用率高".to_string(),
            ],
            cons: vec![
                "表面质量一般".to_string(),
                "尺寸精度有限".to_string(),
                "冷却速度快，晶粒粗大，影响声学品质".to_string(),
                "文化艺术价值低".to_string(),
            ],
            famous_examples: vec![
                "工业用阀体、泵壳".to_string(),
                "现代机床铸件".to_string(),
            ],
            standard_code: "GB/T 11352-2009".to_string(),
            standard_reference: "一般工程用铸造碳钢件".to_string(),
            data_source: "national_standard".to_string(),
            quality_grade: "commercial".to_string(),
        },
        CastingMethodMetrics {
            method_key: "modern_resin_sand".to_string(),
            method_name: "现代树脂砂铸造".to_string(),
            category: "sand_mold".to_string(),
            era: "modern".to_string(),
            description: "使用树脂粘结剂的自硬砂型，精度高，表面质量好，现代化首选工艺。".to_string(),
            historical_period: "20世纪中叶 - 至今".to_string(),
            typical_accuracy_mm: 0.6,
            surface_roughness_ra: 3.2,
            cooling_rate_cps: 6.0,
            typical_defect_rate_pct: 1.8,
            max_shrinkage_porosity_pct: 2.0,
            dimensional_tolerance_pct: 0.3,
            material_yield_pct: 92.0,
            production_cycle_days: 5.0,
            labor_intensity: 3.0,
            energy_consumption_kwh_per_ton: 1100.0,
            cost_per_kg: 85.0,
            environmental_impact_score: 6.0,
            skill_requirements_level: 4.5,
            max_cast_weight_tons: 150.0,
            minimum_thickness_mm: 5.0,
            microstructural_quality: 7.5,
            acoustic_quality_potential: 8.5,
            aesthetic_quality: 7.0,
            durability_years: 1500.0,
            pros: vec![
                "精度高，尺寸一致性好".to_string(),
                "表面光洁，少机加工".to_string(),
                "冷却速度适中，适合声学铸件".to_string(),
                "可制作大型钟体".to_string(),
                "音准一致性极佳".to_string(),
            ],
            cons: vec![
                "树脂成本较高".to_string(),
                "固化过程有VOC排放".to_string(),
                "砂回收处理复杂".to_string(),
            ],
            famous_examples: vec![
                "现代仿制编钟 (如湖北省博物馆复制)".to_string(),
                "出口国外的艺术铜钟".to_string(),
            ],
            standard_code: "GB/T 16746-2018".to_string(),
            standard_reference: "铸造工艺导则 树脂自硬砂".to_string(),
            data_source: "national_standard".to_string(),
            quality_grade: "precision".to_string(),
        },
        CastingMethodMetrics {
            method_key: "modern_investment_casting".to_string(),
            method_name: "现代熔模精密铸造".to_string(),
            category: "investment_casting".to_string(),
            era: "modern".to_string(),
            description: "工业级失蜡法，使用硅溶胶+硅酸乙酯型壳，航空航天级精度。".to_string(),
            historical_period: "20世纪中叶 - 至今".to_string(),
            typical_accuracy_mm: 0.15,
            surface_roughness_ra: 0.8,
            cooling_rate_cps: 10.0,
            typical_defect_rate_pct: 1.0,
            max_shrinkage_porosity_pct: 1.0,
            dimensional_tolerance_pct: 0.1,
            material_yield_pct: 95.0,
            production_cycle_days: 14.0,
            labor_intensity: 4.0,
            energy_consumption_kwh_per_ton: 2500.0,
            cost_per_kg: 350.0,
            environmental_impact_score: 7.0,
            skill_requirements_level: 6.5,
            max_cast_weight_tons: 5.0,
            minimum_thickness_mm: 1.0,
            microstructural_quality: 8.5,
            acoustic_quality_potential: 7.5,
            aesthetic_quality: 9.5,
            durability_years: 1800.0,
            pros: vec![
                "近净成型，几乎无需机加工".to_string(),
                "可制造极其复杂精细的花纹".to_string(),
                "表面可达到镜面级光洁".to_string(),
                "合金材质范围广".to_string(),
            ],
            cons: vec![
                "铸件尺寸受限，不适合大钟".to_string(),
                "成本极高，工艺复杂".to_string(),
                "型壳制作周期长".to_string(),
                "冷却速度快，薄壁件声学性能存疑".to_string(),
            ],
            famous_examples: vec![
                "航空涡轮叶片".to_string(),
                "精密医疗植入物".to_string(),
                "高端艺术品小件".to_string(),
            ],
            standard_code: "GB/T 12214-2019".to_string(),
            standard_reference: "熔模铸造碳钢件 技术条件".to_string(),
            data_source: "national_standard".to_string(),
            quality_grade: "ultra_precision".to_string(),
        },
        CastingMethodMetrics {
            method_key: "modern_centrifugal".to_string(),
            method_name: "现代离心铸造".to_string(),
            category: "centrifugal_casting".to_string(),
            era: "modern".to_string(),
            description: "旋转模具利用离心力成型，组织致密，圆筒/管类零件理想工艺。".to_string(),
            historical_period: "20世纪初 - 至今".to_string(),
            typical_accuracy_mm: 0.8,
            surface_roughness_ra: 1.6,
            cooling_rate_cps: 15.0,
            typical_defect_rate_pct: 0.8,
            max_shrinkage_porosity_pct: 0.5,
            dimensional_tolerance_pct: 0.2,
            material_yield_pct: 98.0,
            production_cycle_days: 2.0,
            labor_intensity: 2.5,
            energy_consumption_kwh_per_ton: 1400.0,
            cost_per_kg: 120.0,
            environmental_impact_score: 5.0,
            skill_requirements_level: 5.0,
            max_cast_weight_tons: 40.0,
            minimum_thickness_mm: 3.0,
            microstructural_quality: 9.5,
            acoustic_quality_potential: 8.0,
            aesthetic_quality: 5.5,
            durability_years: 2000.0,
            pros: vec![
                "组织致密度极高，几乎无缩孔".to_string(),
                "晶粒细化，力学性能好".to_string(),
                "圆筒类壁厚极其均匀".to_string(),
                "成品率接近100%".to_string(),
                "耐腐蚀性极佳".to_string(),
            ],
            cons: vec![
                "仅适合回转体/圆筒类".to_string(),
                "异形钟体/花纹无法制作".to_string(),
                "内表面质量差".to_string(),
                "直径受设备限制".to_string(),
            ],
            famous_examples: vec![
                "青铜轴套、衬套".to_string(),
                "大型管道、缸套".to_string(),
                "一些现代小型圆筒钟".to_string(),
            ],
            standard_code: "GB/T 15114-2009".to_string(),
            standard_reference: "铝合金离心铸件".to_string(),
            data_source: "national_standard".to_string(),
            quality_grade: "high_density".to_string(),
        },
        CastingMethodMetrics {
            method_key: "modern_low_pressure".to_string(),
            method_name: "现代低压铸造".to_string(),
            category: "low_pressure_casting".to_string(),
            era: "modern".to_string(),
            description: "气体压力平稳充型，凝固顺序可控，铝合金轮毂标准工艺，适合高质量青铜件。".to_string(),
            historical_period: "20世纪中叶 - 至今".to_string(),
            typical_accuracy_mm: 0.5,
            surface_roughness_ra: 1.6,
            cooling_rate_cps: 4.0,
            typical_defect_rate_pct: 0.6,
            max_shrinkage_porosity_pct: 0.8,
            dimensional_tolerance_pct: 0.25,
            material_yield_pct: 96.0,
            production_cycle_days: 7.0,
            labor_intensity: 3.5,
            energy_consumption_kwh_per_ton: 1600.0,
            cost_per_kg: 180.0,
            environmental_impact_score: 5.0,
            skill_requirements_level: 5.5,
            max_cast_weight_tons: 60.0,
            minimum_thickness_mm: 2.5,
            microstructural_quality: 9.0,
            acoustic_quality_potential: 9.0,
            aesthetic_quality: 7.5,
            durability_years: 2200.0,
            pros: vec![
                "充型平稳，无湍流卷气".to_string(),
                "凝固顺序可控，自下而上补缩".to_string(),
                "缩孔缩松极少，声学品质极佳".to_string(),
                "壁厚可薄至2mm".to_string(),
                "力学性能优异".to_string(),
            ],
            cons: vec![
                "设备投资大".to_string(),
                "升液管损耗增加成本".to_string(),
                "生产效率比高压铸造低".to_string(),
                "对模具设计要求高".to_string(),
            ],
            famous_examples: vec![
                "高端铝合金汽车轮毂".to_string(),
                "航空发动机机匣".to_string(),
                "定制青铜乐器 (实验中)".to_string(),
            ],
            standard_code: "GB/T 24168-2009".to_string(),
            standard_reference: "低压铸造铸件 技术条件".to_string(),
            data_source: "national_standard".to_string(),
            quality_grade: "high_quality".to_string(),
        },
    ]
}

pub fn compare_casting_methods(req: &CastingMethodRequest) -> CastingComparisonResult {
    let all_methods = build_casting_method_database();

    let selected = if let Some(methods) = &req.methods {
        if methods.is_empty() {
            all_methods
        } else {
            all_methods
                .into_iter()
                .filter(|m| methods.contains(&m.method_key))
                .collect()
        }
    } else {
        all_methods
    };

    let (ancient, modern): (Vec<_>, Vec<_>) = selected.iter().partition(|m| m.era == "ancient");

    let categories = vec![
        "dimensional_accuracy".to_string(),
        "surface_quality".to_string(),
        "microstructure_quality".to_string(),
        "acoustic_quality".to_string(),
        "aesthetic_quality".to_string(),
        "durability".to_string(),
        "production_efficiency".to_string(),
        "material_yield".to_string(),
        "cost_efficiency".to_string(),
        "environmental_friendliness".to_string(),
    ];

    let category_scorers: Vec<Box<dyn Fn(&CastingMethodMetrics) -> f64>> = vec![
        Box::new(|m| (10.0 - m.typical_accuracy_mm).max(0.0).min(10.0) * 10.0),
        Box::new(|m| (10.0 - m.surface_roughness_ra / 5.0).max(0.0).min(10.0) * 10.0),
        Box::new(|m| m.microstructural_quality * 10.0),
        Box::new(|m| m.acoustic_quality_potential * 10.0),
        Box::new(|m| m.aesthetic_quality * 10.0),
        Box::new(|m| (m.durability_years / 250.0).min(100.0)),
        Box::new(|m| (30.0 / m.production_cycle_days.max(1.0)).min(100.0)),
        Box::new(|m| m.material_yield_pct),
        Box::new(|m| (1000.0 / m.cost_per_kg.max(10.0)).min(100.0)),
        Box::new(|m| (10.0 - m.environmental_impact_score) * 10.0),
    ];

    let score_methods = |list: &[&CastingMethodMetrics]| -> Vec<f64> {
        if list.is_empty() {
            return vec![0.0; categories.len()];
        }
        let mut result = vec![0.0; categories.len()];
        for m in list {
            for (i, scorer) in category_scorers.iter().enumerate() {
                result[i] += scorer(m);
            }
        }
        for r in &mut result {
            *r /= list.len() as f64;
            *r = (*r).round() / 1.0;
        }
        result
    };

    let ancient_scores = score_methods(&ancient);
    let modern_scores = score_methods(&modern);

    let mut ancient_avg_map = HashMap::new();
    let mut modern_avg_map = HashMap::new();
    for (i, cat) in categories.iter().enumerate() {
        ancient_avg_map.insert(cat.clone(), ancient_scores[i]);
        modern_avg_map.insert(cat.clone(), modern_scores[i]);
    }

    let key_differences = generate_key_differences(&ancient, &modern);
    let trade_offs = generate_trade_offs();

    let summary = AncientModernSummary {
        ancient_avg_scores: ancient_avg_map,
        modern_avg_scores: modern_avg_map,
        key_differences,
        trade_offs,
    };

    let chart_data = CastingChartData {
        categories: categories.clone(),
        ancient_avg: ancient_scores,
        modern_avg: modern_scores,
    };

    CastingComparisonResult {
        methods_count: selected.len(),
        methods: selected,
        ancient_vs_modern_summary: summary,
        comparison_chart_data: chart_data,
    }
}

fn generate_key_differences(
    ancient: &[&CastingMethodMetrics],
    modern: &[&CastingMethodMetrics],
) -> Vec<String> {
    let mut diffs = Vec::new();

    if !ancient.is_empty() && !modern.is_empty() {
        let avg_acc_a: f64 = ancient.iter().map(|m| m.typical_accuracy_mm).sum::<f64>() / ancient.len() as f64;
        let avg_acc_m: f64 = modern.iter().map(|m| m.typical_accuracy_mm).sum::<f64>() / modern.len() as f64;
        diffs.push(format!(
            "尺寸精度: 古代工艺平均精度 {}mm vs 现代工艺 {}mm，现代工艺精度提升约 {:.0}%",
            avg_acc_a, avg_acc_m,
            ((avg_acc_a - avg_acc_m) / avg_acc_a * 100.0).max(0.0)
        ));

        let avg_cycle_a: f64 = ancient.iter().map(|m| m.production_cycle_days).sum::<f64>() / ancient.len() as f64;
        let avg_cycle_m: f64 = modern.iter().map(|m| m.production_cycle_days).sum::<f64>() / modern.len() as f64;
        diffs.push(format!(
            "生产周期: 古代平均 {:.0}天 vs 现代平均 {:.0}天，现代效率提升约 {:.0}倍",
            avg_cycle_a, avg_cycle_m,
            (avg_cycle_a / avg_cycle_m.max(1.0))
        ));

        let avg_defect_a: f64 = ancient.iter().map(|m| m.typical_defect_rate_pct).sum::<f64>() / ancient.len() as f64;
        let avg_defect_m: f64 = modern.iter().map(|m| m.typical_defect_rate_pct).sum::<f64>() / modern.len() as f64;
        diffs.push(format!(
            "缺陷率: 古代平均 {:.1}% vs 现代平均 {:.1}%，现代工艺缺陷率降低约 {:.0}%",
            avg_defect_a, avg_defect_m,
            ((avg_defect_a - avg_defect_m) / avg_defect_a * 100.0).max(0.0)
        ));

        let avg_yield_a: f64 = ancient.iter().map(|m| m.material_yield_pct).sum::<f64>() / ancient.len() as f64;
        let avg_yield_m: f64 = modern.iter().map(|m| m.material_yield_pct).sum::<f64>() / modern.len() as f64;
        diffs.push(format!(
            "材料利用率: 古代 {:.0}% vs 现代 {:.0}%，节省了大量贵重金属",
            avg_yield_a, avg_yield_m
        ));

        let avg_acoustic_a: f64 = ancient.iter().map(|m| m.acoustic_quality_potential).sum::<f64>() / ancient.len() as f64;
        let avg_acoustic_m: f64 = modern.iter().map(|m| m.acoustic_quality_potential).sum::<f64>() / modern.len() as f64;
        if avg_acoustic_a > avg_acoustic_m {
            diffs.push(format!(
                "声学品质: 古代工艺评分 {:.1} 略高于现代 {:.1}，这得益于古代慢冷却产生的更均匀金相组织",
                avg_acoustic_a, avg_acoustic_m
            ));
        } else {
            diffs.push(format!(
                "声学品质: 现代工艺 (低压/树脂砂) 评分 {:.1} 不逊于古代 {:.1}，新工艺同样能制作高品质钟",
                avg_acoustic_m, avg_acoustic_a
            ));
        }
    }

    diffs
}

fn generate_trade_offs() -> Vec<String> {
    vec![
        "追求极致文化价值与艺术表现力 → 选择古代失蜡法或分范法，接受高成本和长周期".to_string(),
        "追求音准一致性与量产能力 → 选择现代树脂砂或低压铸造，牺牲部分手工艺术感".to_string(),
        "追求极低缺陷率与超长寿命 → 选择现代离心铸造或低压铸造，但花纹表现力受限".to_string(),
        "大尺寸 (>20吨) 巨型钟 → 古代分范法(唯一历史验证方案)或现代树脂砂铸造".to_string(),
        "小尺寸精密花纹纪念钟 → 现代熔模精密铸造，可达镜面效果".to_string(),
        "预算有限的普及型佛钟 → 现代湿砂型铸造，成本最低".to_string(),
        "顶级音质定制编钟 → 现代低压铸造(最佳声学组织) + 手工调音，质量超越古代".to_string(),
    ]
}

pub fn get_casting_method_key_list() -> Vec<String> {
    build_casting_method_database()
        .iter()
        .map(|m| m.method_key.clone())
        .collect()
}

pub fn get_recommended_method_for_bell(
    bell_weight_tons: f64,
    acoustic_requirement: f64,
    budget_per_kg: f64,
    need_complex_artwork: bool,
) -> String {
    let db = build_casting_method_database();

    let mut scored: Vec<(f64, String)> = db
        .iter()
        .map(|m| {
            let mut score = 0.0;
            if m.max_cast_weight_tons >= bell_weight_tons {
                score += 20.0;
            }
            score += m.acoustic_quality_potential * acoustic_requirement / 10.0;
            if m.cost_per_kg <= budget_per_kg {
                score += 25.0;
            }
            if need_complex_artwork {
                score += m.aesthetic_quality * 2.0;
            }
            (score, m.method_key.clone())
        })
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    scored.first().map(|s| s.1.clone()).unwrap_or_default()
}

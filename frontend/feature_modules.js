/* ================================================================
   feature_modules.js - 4个新功能的前端面板
   1. AlloyComparisonPanel  - 合金配比音质对比分析
   2. CastingMethodPanel    - 古代vs现代铸造工艺对比
   3. TowerAcousticPanel    - 钟楼建筑声学传播模拟
   4. VirtualStrikePanel    - 公众虚拟敲钟体验
   ================================================================ */

const COLOR_PALETTE = [
    '#E63946', '#457B9D', '#2A9D8F', '#F4A261',
    '#E76F51', '#264653', '#606C38', '#BC6C25',
    '#DDA15E', '#60A5FA'
];

/* =================================================================
   Feature 1: AlloyComparisonPanel - 合金配比音质对比分析
   ================================================================= */
class AlloyComparisonPanel {
    constructor(container, apiBase, bellOptions) {
        this.container = typeof container === 'string'
            ? document.getElementById(container)
            : container;
        this.apiBase = apiBase;
        this.bellOptions = bellOptions;
        this.selectedAlloys = new Set(['bronze_qing_qin', 'bronze_yong_le', 'bronze_fo_zhong', 'gray_cast_iron']);
        this.alloyList = [
            { key: 'bronze_qing_qin', name: '先秦锡青铜 (曾侯乙编钟)' },
            { key: 'bronze_yong_le', name: '明代低锡青铜 (永乐大钟)' },
            { key: 'bronze_fo_zhong', name: '清代高锡青铜 (佛钟)' },
            { key: 'gray_cast_iron', name: '灰铸铁 (近代铁钟)' },
            { key: 'high_tin_bronze', name: '高锡青铜 (22%Sn, 声学优化)' },
        ];
        this.result = null;
        this.render();
    }

    render() {
        this.container.innerHTML = `
            <div style="padding:16px;background:#fafafa;border-radius:8px;margin-bottom:16px;">
                <h3 style="margin-top:0;color:#264653;">🔬 功能1: 合金配比对钟声音质的影响分析</h3>
                <p style="color:#555;font-size:13px;">
                    不同Cu-Sn-Pb合金比例会极大影响钟的声学特性：杨氏模量、密度、阻尼系数决定了基频、
                    泛音结构、延音品质。锡含量提高→音色更清亮；铅含量提高→延音变短，音色更厚重。
                </p>
                <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-bottom:12px;">
                    <div>
                        <label style="font-size:12px;color:#666;">选择参考钟 (可选)</label>
                        <select id="alloy_bell_select" style="width:100%;padding:6px;">
                            <option value="">-- 使用自定义尺寸 --</option>
                            ${this.bellOptions.map(b => `<option value="${b.bell_id}">${b.bell_name} (${b.expected_pitch}, ${b.expected_freq_hz.toFixed(0)}Hz)</option>`).join('')}
                        </select>
                    </div>
                    <div id="custom_size_fields">
                        <label style="font-size:12px;color:#666;">自定义尺寸 (m)</label>
                        <div style="display:flex;gap:6px;">
                            <input type="number" id="custom_h" placeholder="高度" step="0.1" value="2.0" style="flex:1;padding:6px;">
                            <input type="number" id="custom_d" placeholder="直径" step="0.1" value="1.5" style="flex:1;padding:6px;">
                        </div>
                    </div>
                </div>
                <div style="margin-bottom:12px;">
                    <label style="font-size:12px;color:#666;display:block;margin-bottom:6px;">选择对比合金 (至少选2种):</label>
                    <div style="display:grid;grid-template-columns:repeat(auto-fill, minmax(240px, 1fr));gap:6px;">
                        ${this.alloyList.map(a => `
                            <label style="background:white;padding:6px 10px;border-radius:4px;border:1px solid #ddd;cursor:pointer;font-size:13px;">
                                <input type="checkbox" class="alloy_cb" value="${a.key}" ${this.selectedAlloys.has(a.key) ? 'checked' : ''}>
                                <b>${a.name}</b>
                            </label>
                        `).join('')}
                    </div>
                </div>
                <button id="run_alloy_compare" style="background:#264653;color:white;padding:8px 24px;border:none;border-radius:4px;cursor:pointer;font-weight:bold;">
                    🚀 运行合金对比分析
                </button>
            </div>
            <div id="alloy_result_area"></div>
        `;
        document.getElementById('run_alloy_compare').addEventListener('click', () => this.runAnalysis());
    }

    async runAnalysis() {
        const resultArea = document.getElementById('alloy_result_area');
        resultArea.innerHTML = '<div style="color:#666;padding:20px;text-align:center;">⏳ 正在计算合金声学特性...</div>';

        const checkboxes = document.querySelectorAll('.alloy_cb:checked');
        const alloys = Array.from(checkboxes).map(c => c.value);
        if (alloys.length < 2) {
            resultArea.innerHTML = '<div style="color:#E63946;padding:20px;">⚠️ 请至少选择2种合金进行对比</div>';
            return;
        }

        const bellId = document.getElementById('alloy_bell_select').value;
        const payload = { alloy_keys: alloys };
        if (bellId) {
            payload.bell_id = bellId;
        } else {
            payload.height_m = parseFloat(document.getElementById('custom_h').value) || 2.0;
            payload.diameter_m = parseFloat(document.getElementById('custom_d').value) || 1.5;
        }

        try {
            const res = await fetch(this.apiBase + '/analysis/alloy-comparison', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });
            const data = await res.json();
            if (!data.success) throw new Error(data.error);
            this.result = data.data;
            this.renderResult();
        } catch (e) {
            resultArea.innerHTML = `<div style="color:#E63946;padding:20px;">❌ 分析失败: ${e.message}</div>`;
        }
    }

    renderResult() {
        const r = this.result;
        const area = document.getElementById('alloy_result_area');
        const metrics = r.metrics;

        const rowsHtml = r.comparison_table.map(row => {
            const keys = Object.keys(row.values);
            const cells = keys.map(k => {
                const v = row.values[k];
                const isBest = k === row.best_alloy;
                const isWorst = k === row.worst_alloy;
                const bg = isBest ? '#E8F5E9;color:#1B5E20;font-weight:bold;'
                    : isWorst ? '#FFEBEE;color:#B71C1C;' : '';
                return `<td style="padding:6px 10px;border:1px solid #eee;text-align:right;${bg}">${v.toFixed(2)}</td>`;
            }).join('');
            return `<tr>
                <td style="padding:6px 10px;border:1px solid #eee;font-weight:bold;background:#f3f4f6;">${row.metric}</td>
                <td style="padding:6px 10px;border:1px solid #eee;color:#888;font-size:12px;">${row.unit}</td>
                ${cells}
            </tr>`;
        }).join('');

        const headersHtml = Object.keys(r.comparison_table[0].values).map(k => {
            const alloy = metrics.find(m => m.alloy_key === k);
            const color = COLOR_PALETTE[metrics.findIndex(m => m.alloy_key === k) % 10];
            return `<th style="padding:8px;background:${color};color:white;">${alloy ? alloy.alloy_name : k}</th>`;
        }).join('');

        const freqChartHtml = this.drawFreqChart(metrics);
        const radarHtml = this.drawRadarChart(r.radar_chart_data);
        const energyHtml = this.drawEnergyBars(metrics);
        const recsHtml = r.recommendations.map(r => `<li style="margin:6px 0;padding:6px;background:#FFF8E1;border-radius:4px;">${r}</li>`).join('');

        area.innerHTML = `
            <div style="background:white;padding:16px;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,0.05);">
                <h4 style="margin-top:0;color:#264653;">📊 参考基频: ${r.reference_freq_hz.toFixed(2)} Hz</h4>

                <h4 style="color:#457B9D;">🎵 8阶固有频率对比</h4>
                ${freqChartHtml}

                <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin:16px 0;">
                    <div>
                        <h4 style="color:#2A9D8F;">🎯 七维音质雷达图</h4>
                        ${radarHtml}
                    </div>
                    <div>
                        <h4 style="color:#E76F51;">⚡ 泛音能量分布</h4>
                        ${energyHtml}
                    </div>
                </div>

                <h4 style="color:#264653;margin-top:24px;">📋 综合对比表 <span style="font-size:12px;color:#666;font-weight:normal;">(绿=最佳 / 红=最差)</span></h4>
                <div style="overflow-x:auto;">
                    <table style="border-collapse:collapse;width:100%;font-size:13px;">
                        <thead>
                            <tr>
                                <th style="padding:8px;background:#333;color:white;text-align:left;">指标</th>
                                <th style="padding:8px;background:#333;color:white;">单位</th>
                                ${headersHtml}
                            </tr>
                        </thead>
                        <tbody>${rowsHtml}</tbody>
                    </table>
                </div>

                <h4 style="color:#BC6C25;margin-top:24px;">💡 智能建议</h4>
                <ul style="padding-left:20px;list-style:none;">${recsHtml}</ul>
            </div>
        `;
    }

    drawFreqChart(metrics) {
        const w = 720, h = 260, pad = 50;
        const maxF = Math.max(...metrics.map(m => Math.max(...m.natural_frequencies)));
        const minF = Math.min(...metrics.map(m => Math.min(...m.natural_frequencies)));

        let svg = `<svg viewBox="0 0 ${w} ${h}" style="width:100%;max-width:${w}px;">`;
        // 网格
        for (let i = 0; i <= 5; i++) {
            const y = pad + (h - 2 * pad) * i / 5;
            const fv = maxF - (maxF - minF) * i / 5;
            svg += `<line x1="${pad}" y1="${y}" x2="${w - pad / 2}" y2="${y}" stroke="#eee"/>`;
            svg += `<text x="${pad - 8}" y="${y + 4}" font-size="10" fill="#666" text-anchor="end">${fv.toFixed(0)}Hz</text>`;
        }
        // 模态标签
        const modes = ['(2,0)', '(3,0)', '(4,0)', '(2,1)', '(5,0)', '(3,1)', '(6,0)', '(4,1)'];
        modes.forEach((m, i) => {
            const x = pad + (w - 2 * pad) * i / (modes.length - 1);
            svg += `<text x="${x}" y="${h - 20}" font-size="10" fill="#666" text-anchor="middle">模${m}</text>`;
            svg += `<circle cx="${x}" cy="${h - pad}" r="2" fill="#aaa"/>`;
        });
        // 数据线
        metrics.forEach((m, idx) => {
            const color = COLOR_PALETTE[idx % 10];
            let path = '';
            m.natural_frequencies.forEach((f, i) => {
                const x = pad + (w - 2 * pad) * i / (modes.length - 1);
                const y = pad + (h - 2 * pad) * (1 - (f - minF) / (maxF - minF || 1));
                path += (i === 0 ? 'M' : 'L') + x.toFixed(1) + ',' + y.toFixed(1) + ' ';
                svg += `<circle cx="${x}" cy="${y}" r="4" fill="${color}"/>`;
            });
            svg += `<path d="${path}" fill="none" stroke="${color}" stroke-width="2"/>`;
            svg += `<text x="${w - pad / 2}" y="${pad + 20 + idx * 16}" font-size="11" fill="${color}" text-anchor="end">■ ${m.alloy_name} (基频 ${m.fundamental_hz.toFixed(0)}Hz)</text>`;
        });
        svg += '</svg>';
        return svg;
    }

    drawRadarChart(data) {
        const cx = 150, cy = 150, R = 100;
        const N = data.labels.length;
        let svg = `<svg viewBox="0 0 300 320" style="width:100%;max-width:300px;">`;
        // 网格
        for (let r = 1; r <= 5; r++) {
            let ring = '';
            for (let i = 0; i < N; i++) {
                const ang = -Math.PI / 2 + i * 2 * Math.PI / N;
                const rr = R * r / 5;
                const x = cx + rr * Math.cos(ang), y = cy + rr * Math.sin(ang);
                ring += (i === 0 ? 'M' : 'L') + x.toFixed(1) + ',' + y.toFixed(1) + ' ';
            }
            svg += `<path d="${ring}Z" fill="none" stroke="#ddd"/>`;
            svg += `<text x="${cx}" y="${cy - R * r / 5 + 4}" font-size="9" fill="#aaa" text-anchor="middle">${r * 20}</text>`;
        }
        // 轴
        for (let i = 0; i < N; i++) {
            const ang = -Math.PI / 2 + i * 2 * Math.PI / N;
            const x = cx + R * Math.cos(ang), y = cy + R * Math.sin(ang);
            const lx = cx + (R + 18) * Math.cos(ang), ly = cy + (R + 18) * Math.sin(ang);
            svg += `<line x1="${cx}" y1="${cy}" x2="${x}" y2="${y}" stroke="#ddd"/>`;
            svg += `<text x="${lx}" y="${ly + 4}" font-size="10" fill="#333" text-anchor="middle">${data.labels[i]}</text>`;
        }
        // 数据多边形
        const keys = Object.keys(data.datasets);
        keys.forEach((k, idx) => {
            const color = COLOR_PALETTE[idx % 10];
            const arr = data.datasets[k];
            let path = '';
            arr.forEach((v, i) => {
                const ang = -Math.PI / 2 + i * 2 * Math.PI / N;
                const rr = R * Math.min(v, 100) / 100;
                const x = cx + rr * Math.cos(ang), y = cy + rr * Math.sin(ang);
                path += (i === 0 ? 'M' : 'L') + x.toFixed(1) + ',' + y.toFixed(1) + ' ';
            });
            svg += `<path d="${path}Z" fill="${color}33" stroke="${color}" stroke-width="2"/>`;
            svg += `<text x="10" y="290 + idx * 14" font-size="10" fill="${color}">■ ${k}</text>`;
        });
        svg += '</svg>';
        return svg;
    }

    drawEnergyBars(metrics) {
        const N = metrics.length;
        const H = 240, W = 300, barW = 28;
        let html = `<svg viewBox="0 0 ${W} ${H + 20}" style="width:100%;max-width:${W}px;">`;
        const maxE = 1.0;
        for (let row = 0; row < 8; row++) {
            const y = 10 + row * (H / 8);
            html += `<text x="${W - 10}" y="${y + 10}" font-size="9" fill="#666" text-anchor="end">阶${row + 1}</text>`;
            metrics.forEach((m, col) => {
                const x = 10 + col * (W - 40) / N;
                const e = m.harmonic_energy_distribution[row] || 0;
                const bw = barW * (e / maxE);
                const color = COLOR_PALETTE[col % 10];
                html += `<rect x="${x}" y="${y}" width="${bw.toFixed(1)}" height="${(H / 8) - 4}" fill="${color}" opacity="0.85"/>`;
            });
        }
        metrics.forEach((m, col) => {
            const color = COLOR_PALETTE[col % 10];
            html += `<text x="${10 + col * (W - 40) / N}" y="${H + 14}" font-size="9" fill="${color}" transform="rotate(-35 ${10 + col * (W - 40) / N},${H + 14})">${m.alloy_key.slice(0, 8)}</text>`;
        });
        html += '</svg>';
        return html;
    }
}

/* =================================================================
   Feature 2: CastingMethodPanel - 古代vs现代铸造工艺对比
   ================================================================= */
class CastingMethodPanel {
    constructor(container, apiBase) {
        this.container = typeof container === 'string'
            ? document.getElementById(container)
            : container;
        this.apiBase = apiBase;
        this.result = null;
        this.render();
    }

    render() {
        this.container.innerHTML = `
            <div style="padding:16px;background:#fafafa;border-radius:8px;margin-bottom:16px;">
                <h3 style="margin-top:0;color:#264653;">🏺 功能2: 古代铸钟工艺 vs 现代铸造对比</h3>
                <p style="color:#555;font-size:13px;">
                    从商代的泥范法、失蜡法，到现代的树脂砂、低压铸造、离心铸造，3000年铸造技术的演进。
                    对比精度、缺陷率、声学潜力、艺术表现力、成本效率等10个维度。
                </p>
                <div style="margin-bottom:12px;">
                    <button id="run_casting_compare" style="background:#BC6C25;color:white;padding:8px 24px;border:none;border-radius:4px;cursor:pointer;font-weight:bold;">
                        📊 运行古代vs现代工艺对比 (8种工艺)
                    </button>
                    <span style="margin-left:16px;color:#888;font-size:12px;">
                        含: 古代失蜡法/分范法/砂型法 + 现代湿砂/树脂砂/熔模/离心/低压铸造
                    </span>
                </div>
            </div>
            <div id="casting_result_area"></div>
        `;
        document.getElementById('run_casting_compare').addEventListener('click', () => this.runAnalysis());
    }

    async runAnalysis() {
        const area = document.getElementById('casting_result_area');
        area.innerHTML = '<div style="color:#666;padding:20px;text-align:center;">⏳ 正在加载8种铸造工艺数据库...</div>';
        try {
            const res = await fetch(this.apiBase + '/analysis/casting-methods', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({})
            });
            const data = await res.json();
            if (!data.success) throw new Error(data.error);
            this.result = data.data;
            this.renderResult();
        } catch (e) {
            area.innerHTML = `<div style="color:#E63946;padding:20px;">❌ 加载失败: ${e.message}</div>`;
        }
    }

    renderResult() {
        const r = this.result;
        const area = document.getElementById('casting_result_area');

        const ancientMethods = r.methods.filter(m => m.era === 'ancient');
        const modernMethods = r.methods.filter(m => m.era === 'modern');

        const chartHtml = this.drawAncientModernChart(r.comparison_chart_data);
        const keyDiffs = r.ancient_vs_modern_summary.key_differences.map(d => `<li style="margin:6px 0;">${d}</li>`).join('');
        const tradeOffs = r.ancient_vs_modern_summary.trade_offs.map(t => `<li style="margin:6px 0;padding:6px;background:#E3F2FD;border-radius:4px;">🎯 ${t}</li>`).join('');

        const methodCards = r.methods.map(m => {
            const isAncient = m.era === 'ancient';
            const headerColor = isAncient ? 'linear-gradient(135deg,#BC6C25,#DDA15E)' : 'linear-gradient(135deg,#0077B6,#00B4D8)';
            const prosHtml = m.pros.map(p => `<span style="display:inline-block;background:#E8F5E9;color:#2E7D32;padding:3px 8px;border-radius:3px;font-size:11px;margin:2px;">✓ ${p}</span>`).join('');
            const consHtml = m.cons.map(c => `<span style="display:inline-block;background:#FFEBEE;color:#C62828;padding:3px 8px;border-radius:3px;font-size:11px;margin:2px;">✗ ${c}</span>`).join('');
            const famHtml = m.famous_examples.map(f => `<span style="display:inline-block;background:#FFF8E1;color:#F57F17;padding:3px 8px;border-radius:3px;font-size:11px;margin:2px;">⭐ ${f}</span>`).join('');

            return `
            <div style="border:1px solid #ddd;border-radius:8px;overflow:hidden;box-shadow:0 2px 4px rgba(0,0,0,0.05);">
                <div style="background:${headerColor};color:white;padding:12px;">
                    <div style="font-weight:bold;font-size:15px;">${m.method_name}</div>
                    <div style="font-size:11px;opacity:0.9;">${m.category} · ${m.historical_period}</div>
                </div>
                <div style="padding:12px;font-size:12px;line-height:1.6;">
                    <div style="margin-bottom:8px;color:#444;">${m.description}</div>
                    <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;margin:8px 0;">
                        <div>🏭 典型精度: <b>${m.typical_accuracy_mm}mm</b></div>
                        <div>🎨 表面Ra: <b>${m.surface_roughness_ra}μm</b></div>
                        <div>❌ 缺陷率: <b>${m.typical_defect_rate_pct}%</b></div>
                        <div>📐 公差: <b>±${m.dimensional_tolerance_pct}%</b></div>
                        <div>♻️ 利用率: <b>${m.material_yield_pct}%</b></div>
                        <div>⏱️ 周期: <b>${m.production_cycle_days}天</b></div>
                        <div>💰 成本: <b>¥${m.cost_per_kg}/kg</b></div>
                        <div>🔊 声学潜力: <b>${m.acoustic_quality_potential}/10</b></div>
                        <div>🎭 艺术表现: <b>${m.aesthetic_quality}/10</b></div>
                        <div>🏛️ 耐久: <b>${(m.durability_years/100).toFixed(0)}百年</b></div>
                    </div>
                    <div style="margin:8px 0;"><div style="color:#1B5E20;font-weight:bold;margin-bottom:4px;">✅ 优点:</div>${prosHtml}</div>
                    <div style="margin:8px 0;"><div style="color:#B71C1C;font-weight:bold;margin-bottom:4px;">❌ 缺点:</div>${consHtml}</div>
                    ${famHtml ? `<div style="margin:8px 0;"><div style="color:#E65100;font-weight:bold;margin-bottom:4px;">🏆 代表作品:</div>${famHtml}</div>` : ''}
                </div>
            </div>`;
        }).join('');

        area.innerHTML = `
            <div style="background:white;padding:16px;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,0.05);margin-bottom:16px;">
                <h4 style="margin-top:0;color:#264653;">📊 古代工艺 (${ancientMethods.length}种) vs 现代工艺 (${modernMethods.length}种)</h4>
                ${chartHtml}

                <h4 style="color:#0077B6;margin-top:24px;">📌 核心差异总结</h4>
                <ul style="padding-left:20px;">${keyDiffs}</ul>

                <h4 style="color:#DDA15E;margin-top:16px;">🎯 场景取舍建议</h4>
                <ul style="padding-left:20px;list-style:none;">${tradeOffs}</ul>
            </div>

            <h4 style="color:#264653;">🏛️ ${r.methods_count}种铸造工艺详细卡片</h4>
            <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(340px,1fr));gap:16px;">
                ${methodCards}
            </div>
        `;
    }

    drawAncientModernChart(chart) {
        const N = chart.categories.length;
        const w = 760, h = 300, pad = 60;
        let svg = `<svg viewBox="0 0 ${w} ${h + 30}" style="width:100%;max-width:${w}px;">`;
        // Y轴网格
        for (let i = 0; i <= 5; i++) {
            const y = pad + (h - 2 * pad) * i / 5;
            svg += `<line x1="${pad}" y1="${y}" x2="${w - pad / 2}" y2="${y}" stroke="#eee"/>`;
            svg += `<text x="${pad - 8}" y="${y + 4}" font-size="10" fill="#666" text-anchor="end">${100 - i * 20}</text>`;
        }
        svg += `<text x="10" y="${h / 2}" font-size="11" fill="#666" transform="rotate(-90 10,${h / 2})">评分 (0-100)</text>`;
        // X轴分类
        const groupW = (w - 2 * pad) / N;
        chart.categories.forEach((cat, i) => {
            const gx = pad + groupW * i + groupW / 2;
            const av = chart.ancient_avg[i], mv = chart.modern_avg[i];
            const ah = (h - 2 * pad) * av / 100, mh = (h - 2 * pad) * mv / 100;
            const ay = h - pad - ah, my = h - pad - mh;
            const barW = groupW * 0.35;
            svg += `<rect x="${gx - barW - 2}" y="${ay}" width="${barW}" height="${ah}" fill="#BC6C25" opacity="0.85"/>`;
            svg += `<rect x="${gx + 2}" y="${my}" width="${barW}" height="${mh}" fill="#0077B6" opacity="0.85"/>`;
            svg += `<text x="${gx - barW / 2 - 2}" y="${ay - 4}" font-size="10" fill="#BC6C25" text-anchor="middle">${av.toFixed(0)}</text>`;
            svg += `<text x="${gx + barW / 2 + 2}" y="${my - 4}" font-size="10" fill="#0077B6" text-anchor="middle">${mv.toFixed(0)}</text>`;
            svg += `<text x="${gx}" y="${h - pad / 2 + 8}" font-size="11" fill="#333" text-anchor="middle" transform="rotate(-25 ${gx},${h - pad / 2 + 8})">${cat}</text>`;
        });
        svg += `<text x="${w - 120}" y="20" font-size="12" fill="#BC6C25">■ 古代工艺平均</text>`;
        svg += `<text x="${w - 120}" y="38" font-size="12" fill="#0077B6">■ 现代工艺平均</text>`;
        svg += '</svg>';
        return svg;
    }
}

/* =================================================================
   Feature 3: TowerAcousticPanel - 钟楼建筑声学传播模拟
   ================================================================= */
class TowerAcousticPanel {
    constructor(container, apiBase) {
        this.container = typeof container === 'string'
            ? document.getElementById(container)
            : container;
        this.apiBase = apiBase;
        this.presets = [];
        this.selectedPreset = 0;
        this.result = null;
        this.init();
    }

    async init() {
        this.render();
        try {
            const res = await fetch(this.apiBase + '/analysis/tower-presets');
            const data = await res.json();
            if (data.success) {
                this.presets = data.data;
                this.refreshPresetSelect();
            }
        } catch (e) { console.warn(e); }
    }

    refreshPresetSelect() {
        const sel = document.getElementById('tower_preset');
        if (!sel) return;
        sel.innerHTML = this.presets.map((p, i) => `<option value="${i}">${p.tower_style}</option>`).join('');
    }

    render() {
        this.container.innerHTML = `
            <div style="padding:16px;background:#fafafa;border-radius:8px;margin-bottom:16px;">
                <h3 style="margin-top:0;color:#264653;">🏛️ 功能3: 钟楼建筑对钟声传播的影响模拟</h3>
                <p style="color:#555;font-size:13px;">
                    计算100×100m范围 (覆盖500m×500m) 的2D声压场分布，对比自由场与有钟楼时的差异。
                    钟楼的开口率、墙面材料、窗洞方向、悬挂高度都会极大影响钟声的传播距离和音质。
                </p>
                <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:10px;margin-bottom:10px;">
                    <div>
                        <label style="font-size:12px;color:#666;">📐 经典钟楼预设</label>
                        <select id="tower_preset" onchange="window.__loadTowerPreset()" style="width:100%;padding:6px;">
                            <option value="-1">-- 选择预设 --</option>
                        </select>
                    </div>
                    <div><label style="font-size:12px;color:#666;">高度 (m)</label><input id="tw_h" type="number" value="15" step="1" style="width:100%;padding:6px;"></div>
                    <div><label style="font-size:12px;color:#666;">宽度×深度 (m)</label>
                        <div style="display:flex;gap:4px;">
                            <input id="tw_w" type="number" value="8" step="1" style="flex:1;padding:6px;">
                            <input id="tw_d" type="number" value="8" step="1" style="flex:1;padding:6px;">
                        </div>
                    </div>
                    <div><label style="font-size:12px;color:#666;">墙厚 (m)</label><input id="tw_th" type="number" value="0.37" step="0.05" style="width:100%;padding:6px;"></div>
                    <div>
                        <label style="font-size:12px;color:#666;">墙体材料</label>
                        <select id="tw_mat" style="width:100%;padding:6px;">
                            <option value="wood">木结构 (反射弱)</option>
                            <option value="brick" selected>砖石结构 (中等)</option>
                            <option value="stone">石材 (反射强)</option>
                            <option value="concrete">混凝土</option>
                            <option value="adobe">夯土/土坯</option>
                        </select>
                    </div>
                    <div><label style="font-size:12px;color:#666;">钟室高度 (m)</label><input id="tw_bch" type="number" value="5" step="1" style="width:100%;padding:6px;"></div>
                    <div><label style="font-size:12px;color:#666;">开窗数量×尺寸</label>
                        <div style="display:flex;gap:4px;">
                            <input id="tw_wn" type="number" value="4" step="1" min="0" max="16" style="flex:1;padding:6px;">
                            <input id="tw_ww" type="number" value="1.8" step="0.2" placeholder="宽" style="flex:1;padding:6px;">
                            <input id="tw_wh" type="number" value="2.4" step="0.2" placeholder="高" style="flex:1;padding:6px;">
                        </div>
                    </div>
                    <div><label style="font-size:12px;color:#666;">模拟频率 (Hz)</label><input id="tw_freq" type="number" value="256" step="32" style="width:100%;padding:6px;"></div>
                </div>
                <button id="run_tower_sim" style="background:#0077B6;color:white;padding:8px 24px;border:none;border-radius:4px;cursor:pointer;font-weight:bold;">
                    🎯 运行钟楼声学模拟 (自由场 vs 有钟楼)
                </button>
            </div>
            <div id="tower_result_area"></div>
        `;
        window.__loadTowerPreset = () => this.loadPreset();
        document.getElementById('run_tower_sim').addEventListener('click', () => this.runSim());
    }

    loadPreset() {
        const idx = parseInt(document.getElementById('tower_preset').value);
        if (idx < 0 || idx >= this.presets.length) return;
        const p = this.presets[idx];
        document.getElementById('tw_h').value = p.height_m;
        document.getElementById('tw_w').value = p.width_m;
        document.getElementById('tw_d').value = p.depth_m;
        document.getElementById('tw_th').value = p.wall_thickness_m;
        document.getElementById('tw_mat').value = p.wall_material;
        document.getElementById('tw_bch').value = p.bell_chamber_height_m;
        document.getElementById('tw_wn').value = p.window_count;
        document.getElementById('tw_ww').value = p.window_width_m;
        document.getElementById('tw_wh').value = p.window_height_m;
    }

    collectParams() {
        const windowCount = parseInt(document.getElementById('tw_wn').value) || 4;
        const directions = [];
        for (let i = 0; i < windowCount; i++) {
            directions.push((360 / windowCount) * i);
        }
        return {
            tower_style: '自定义',
            height_m: parseFloat(document.getElementById('tw_h').value) || 15,
            width_m: parseFloat(document.getElementById('tw_w').value) || 8,
            depth_m: parseFloat(document.getElementById('tw_d').value) || 8,
            wall_thickness_m: parseFloat(document.getElementById('tw_th').value) || 0.37,
            wall_material: document.getElementById('tw_mat').value,
            bell_chamber_height_m: parseFloat(document.getElementById('tw_bch').value) || 5,
            window_count: windowCount,
            window_width_m: parseFloat(document.getElementById('tw_ww').value) || 1.8,
            window_height_m: parseFloat(document.getElementById('tw_wh').value) || 2.4,
            roof_style: 'custom',
            openings_direction_deg: directions,
            internal_absorption_coeff: 0.1,
            internal_reverberation: 2.0,
        };
    }

    async runSim() {
        const area = document.getElementById('tower_result_area');
        area.innerHTML = '<div style="color:#666;padding:40px;text-align:center;">⏳ 正在计算2D声压场 (100×100网格，含镜像反射+衍射)...</div>';
        try {
            const freq = parseFloat(document.getElementById('tw_freq').value) || 256;
            const payload = {
                frequency_hz: freq,
                tower: this.collectParams()
            };
            const res = await fetch(this.apiBase + '/analysis/tower-acoustic', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });
            const data = await res.json();
            if (!data.success) throw new Error(data.error);
            this.result = data.data;
            this.renderResult();
        } catch (e) {
            area.innerHTML = `<div style="color:#E63946;padding:20px;">❌ 模拟失败: ${e.message}</div>`;
        }
    }

    renderResult() {
        const r = this.result;
        const area = document.getElementById('tower_result_area');
        const p = r.tower_params;
        const cm = r.comparison_metrics;

        const withCanvas = this.renderHeatmap(r.with_tower.field_2d, 260, '有钟楼');
        const withoutCanvas = this.renderHeatmap(r.without_tower.field_2d, 260, '自由场');
        const diffCanvas = this.renderDiff(r.with_tower.field_2d, r.without_tower.field_2d, 260);
        const dirChart = this.renderDirectivity(r.directivity_pattern);

        const zonesHtml = r.sound_coverage.map(z => `
            <tr>
                <td style="padding:8px;border:1px solid #eee;font-weight:bold;">${z.zone_name}</td>
                <td style="padding:8px;border:1px solid #eee;text-align:center;">${z.distance_range_m[0]}-${z.distance_range_m[1]}m</td>
                <td style="padding:8px;border:1px solid #eee;text-align:right;background:#E3F2FD;">${z.with_tower_avg_spl.toFixed(1)} dB</td>
                <td style="padding:8px;border:1px solid #eee;text-align:right;">${z.without_tower_avg_spl.toFixed(1)} dB</td>
                <td style="padding:8px;border:1px solid #eee;text-align:right;background:${z.spl_gain_db > 0 ? '#E8F5E9;color:#2E7D32' : '#FFEBEE;color:#C62828'};font-weight:bold;">
                    ${z.spl_gain_db > 0 ? '+' : ''}${z.spl_gain_db.toFixed(1)} dB
                </td>
                <td style="padding:8px;border:1px solid #eee;text-align:center;">${z.intelligible_speech ? '✅ 可辨' : '❌ 过弱'}</td>
                <td style="padding:8px;border:1px solid #eee;text-align:center;">${z.aesthetic_enjoyment ? '🎵 适宜' : '😐 一般'}</td>
            </tr>
        `).join('');

        const tipsHtml = r.optimization_tips.map(t => `<li style="margin:6px 0;padding:8px;background:#FFF3E0;border-radius:4px;font-size:13px;">${t}</li>`).join('');

        area.innerHTML = `
            <div style="background:white;padding:16px;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,0.05);margin-bottom:16px;">
                <h4 style="margin-top:0;color:#264653;">📐 参数: ${p.tower_style} H=${p.height_m}m W=${p.width_m}m 墙厚=${p.wall_thickness_m}m 窗=${p.window_count}面</h4>

                <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px;margin:12px 0;">
                    <div style="background:linear-gradient(135deg,#0077B6,#00B4D8);color:white;padding:12px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">100m处声压提升</div>
                        <div style="font-size:24px;font-weight:bold;">${cm.spl_boost_at_100m_db > 0 ? '+' : ''}${cm.spl_boost_at_100m_db.toFixed(1)} dB</div>
                    </div>
                    <div style="background:linear-gradient(135deg,#2A9D8F,#52B788);color:white;padding:12px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">指向性提升</div>
                        <div style="font-size:24px;font-weight:bold;">+${cm.directionality_improvement.toFixed(1)} dB</div>
                    </div>
                    <div style="background:linear-gradient(135deg,#E76F51,#F4A261);color:white;padding:12px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">覆盖面积增加</div>
                        <div style="font-size:24px;font-weight:bold;">${cm.coverage_area_increase_pct > 0 ? '+' : ''}${cm.coverage_area_increase_pct.toFixed(0)}%</div>
                    </div>
                    <div style="background:linear-gradient(135deg,#606C38,#A4AC86);color:white;padding:12px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">综合声学评分</div>
                        <div style="font-size:24px;font-weight:bold;">${cm.overall_improvement_score.toFixed(0)}/100</div>
                    </div>
                </div>

                <h4 style="color:#264653;">🌍 2D声压场云图 (dB SPL, 俯视)</h4>
                <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin:12px 0;">
                    <div style="text-align:center;">
                        <div style="font-weight:bold;color:#0077B6;margin-bottom:6px;">🏛️ 有钟楼</div>
                        ${withCanvas.outerHTML}
                        <div style="font-size:11px;color:#666;">
                            Max=${r.with_tower.max_spl_db.toFixed(0)}dB / Avg=${r.with_tower.avg_spl_db.toFixed(0)}dB<br>
                            RT60=${r.with_tower.reverberation_time_s.toFixed(1)}s 清晰度C=${r.with_tower.clarity_index.toFixed(0)}
                        </div>
                    </div>
                    <div style="text-align:center;">
                        <div style="font-weight:bold;color:#666;margin-bottom:6px;">🌳 无钟楼 (自由场)</div>
                        ${withoutCanvas.outerHTML}
                        <div style="font-size:11px;color:#666;">
                            Max=${r.without_tower.max_spl_db.toFixed(0)}dB / Avg=${r.without_tower.avg_spl_db.toFixed(0)}dB<br>
                            RT60=${r.without_tower.reverberation_time_s.toFixed(1)}s
                        </div>
                    </div>
                    <div style="text-align:center;">
                        <div style="font-weight:bold;color:#E76F51;margin-bottom:6px;">📈 增益分布 (钟楼-自由场)</div>
                        ${diffCanvas.outerHTML}
                        <div style="font-size:11px;color:#666;">暖色=钟楼增强 / 冷色=钟楼削弱</div>
                    </div>
                </div>

                <h4 style="color:#264653;margin-top:16px;">🧭 水平指向性对比 (100m处)</h4>
                <div style="text-align:center;">${dirChart.outerHTML}</div>

                <h4 style="color:#0077B6;margin-top:16px;">📊 传播距离分层分析</h4>
                <div style="overflow-x:auto;">
                    <table style="border-collapse:collapse;width:100%;font-size:13px;">
                        <thead>
                            <tr style="background:#264653;color:white;">
                                <th style="padding:10px;text-align:left;">区域</th>
                                <th style="padding:10px;">距离</th>
                                <th style="padding:10px;">有钟楼SPL</th>
                                <th style="padding:10px;">自由场SPL</th>
                                <th style="padding:10px;">增益</th>
                                <th style="padding:10px;">语言可懂</th>
                                <th style="padding:10px;">欣赏适宜</th>
                            </tr>
                        </thead>
                        <tbody>${zonesHtml}</tbody>
                    </table>
                </div>

                <h4 style="color:#BC6C25;margin-top:20px;">💡 钟楼声学优化建议</h4>
                <ul style="padding-left:20px;list-style:none;">${tipsHtml}</ul>
            </div>
        `;
    }

    renderHeatmap(grid, size, title) {
        const c = document.createElement('canvas');
        c.width = size; c.height = size;
        const n = grid.length;
        const ctx = c.getContext('2d');
        const cell = size / n;
        let min = Infinity, max = -Infinity;
        grid.forEach(row => row.forEach(v => { min = Math.min(min, v); max = Math.max(max, v); }));
        for (let i = 0; i < n; i++) {
            for (let j = 0; j < n; j++) {
                const v = grid[i][j];
                const t = Math.max(0, Math.min(1, (v - min) / (max - min || 1)));
                // 蓝->青->绿->黄->红
                const r = Math.round(255 * Math.max(0, (t - 0.5) * 2));
                const g = Math.round(255 * (t < 0.5 ? t * 2 : (1 - (t - 0.5) * 2)));
                const b = Math.round(255 * (1 - Math.min(1, t * 2)));
                ctx.fillStyle = `rgb(${r},${g},${b})`;
                ctx.fillRect(i * cell, j * cell, cell + 1, cell + 1);
            }
        }
        ctx.strokeStyle = 'rgba(255,255,255,0.8)';
        ctx.lineWidth = 2;
        ctx.strokeRect(size / 2 - 4, size / 2 - 4, 8, 8);
        ctx.fillStyle = 'white';
        ctx.font = 'bold 11px sans-serif';
        ctx.fillText('中心(钟位置)', 6, 14);
        return c;
    }

    renderDiff(gridA, gridB, size) {
        const c = document.createElement('canvas');
        c.width = size; c.height = size;
        const n = gridA.length;
        const ctx = c.getContext('2d');
        const cell = size / n;
        let maxDiff = 0;
        for (let i = 0; i < n; i++) {
            for (let j = 0; j < n; j++) {
                maxDiff = Math.max(maxDiff, Math.abs(gridA[i][j] - gridB[i][j]));
            }
        }
        maxDiff = Math.max(maxDiff, 0.1);
        for (let i = 0; i < n; i++) {
            for (let j = 0; j < n; j++) {
                const diff = gridA[i][j] - gridB[i][j];
                const t = Math.max(-1, Math.min(1, diff / maxDiff));
                let r, g, b;
                if (t > 0) { r = 255; g = Math.round(255 * (1 - t)); b = Math.round(255 * (1 - t)); }
                else { r = Math.round(255 * (1 + t)); g = Math.round(255 * (1 + t)); b = 255; }
                ctx.fillStyle = `rgb(${r},${g},${b})`;
                ctx.fillRect(i * cell, j * cell, cell + 1, cell + 1);
            }
        }
        return c;
    }

    renderDirectivity(points) {
        const size = 300, cx = size / 2, cy = size / 2, R = 110;
        const c = document.createElement('canvas');
        c.width = size; c.height = size + 20;
        const ctx = c.getContext('2d');
        ctx.fillStyle = '#fafafa';
        ctx.fillRect(0, 0, size, size);
        // 同心圆
        for (let k = 1; k <= 4; k++) {
            ctx.strokeStyle = '#ddd';
            ctx.beginPath();
            ctx.arc(cx, cy, R * k / 4, 0, Math.PI * 2);
            ctx.stroke();
        }
        // 角度线
        for (let a = 0; a < 360; a += 45) {
            const rad = a * Math.PI / 180;
            ctx.strokeStyle = '#eee';
            ctx.beginPath();
            ctx.moveTo(cx, cy);
            ctx.lineTo(cx + R * Math.cos(rad), cy + R * Math.sin(rad));
            ctx.stroke();
            ctx.fillStyle = '#888';
            ctx.font = '10px sans-serif';
            ctx.fillText(a + '°', cx + (R + 12) * Math.cos(rad) - 10, cy + (R + 12) * Math.sin(rad));
        }
        // 归一化
        const allVals = points.flatMap(p => [p.with_tower_spl, p.without_tower_spl]);
        const minS = Math.min(...allVals), maxS = Math.max(...allVals);
        const drawPoly = (getter, color, fill) => {
            ctx.beginPath();
            points.forEach((pt, i) => {
                const rad = pt.angle_deg * Math.PI / 180;
                const v = getter(pt);
                const r = 20 + (R - 20) * (v - minS) / (maxS - minS || 1);
                const x = cx + r * Math.cos(rad), y = cy + r * Math.sin(rad);
                if (i === 0) ctx.moveTo(x, y);
                else ctx.lineTo(x, y);
            });
            ctx.closePath();
            if (fill) { ctx.fillStyle = color + '33'; ctx.fill(); }
            ctx.strokeStyle = color; ctx.lineWidth = 2; ctx.stroke();
        };
        drawPoly(p => p.without_tower_spl, '#999', true);
        drawPoly(p => p.with_tower_spl, '#0077B6', true);
        // 图例
        ctx.fillStyle = '#0077B6';
        ctx.fillRect(10, size - 14, 14, 10);
        ctx.fillStyle = '#333'; ctx.font = '11px sans-serif';
        ctx.fillText('■ 有钟楼', 28, size - 5);
        ctx.fillStyle = '#999';
        ctx.fillRect(100, size - 14, 14, 10);
        ctx.fillStyle = '#333';
        ctx.fillText('■ 自由场', 118, size - 5);
        return c;
    }
}

/* =================================================================
   Feature 4: VirtualStrikePanel - 公众虚拟敲钟体验
   ================================================================= */
class VirtualStrikePanel {
    constructor(container, apiBase, bellOptions, bell3dInstance, audioCtx) {
        this.container = typeof container === 'string'
            ? document.getElementById(container)
            : container;
        this.apiBase = apiBase;
        this.bellOptions = bellOptions;
        this.bell3d = bell3dInstance;
        this.audioCtx = audioCtx;
        this.currentBell = bellOptions[0];
        this.selectedPos = 'lip';
        this.selectedMallet = 'medium';
        this.strikeForce = 0.5;
        this.dragging = false;
        this.strikeHistory = [];
        this.render();
        this.setupAudioContext();
    }

    setupAudioContext() {
        if (!this.audioCtx) {
            try {
                this.audioCtx = new (window.AudioContext || window.webkitAudioContext)();
            } catch (e) {
                console.warn('Web Audio not supported', e);
            }
        }
    }

    render() {
        this.container.innerHTML = `
            <div style="padding:16px;background:linear-gradient(135deg,#fdf2e9,#fff);border-radius:8px;margin-bottom:16px;border:2px solid #F4A261;">
                <h3 style="margin-top:0;color:#BC6C25;">🔔 功能4: 虚拟敲钟体验 (公众互动版)</h3>
                <p style="color:#555;font-size:13px;">
                    选择不同的敲钟位置、木槌材质、敲击力度，聆听钟声的细微差异。
                    <b>拖动画钟的槌头敲击钟体 → 自动合成真实钟声！</b>
                    位置影响泛音结构，槌头硬度决定音色明暗，力度影响音量和衰减。
                </p>
                <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:10px;margin-bottom:10px;">
                    <div>
                        <label style="font-size:12px;color:#666;">🎐 选择钟体</label>
                        <select id="vs_bell" style="width:100%;padding:6px;">
                            ${this.bellOptions.map((b, i) => `<option value="${b.bell_id}" ${i === 0 ? 'selected' : ''}>${b.bell_name} (${b.expected_pitch} / ${b.expected_freq_hz.toFixed(0)}Hz / ${(b.weight_kg / 1000).toFixed(1)}吨)</option>`).join('')}
                        </select>
                    </div>
                    <div>
                        <label style="font-size:12px;color:#666;">📍 敲击位置 <span id="vs_pos_name" style="color:#E76F51;font-weight:bold;">钟口</span></label>
                        <select id="vs_pos" style="width:100%;padding:6px;">
                            <option value="lip">钟口 Lip (正鼓部 - 主音饱满)</option>
                            <option value="rim">唇边 Rim (侧鼓部 - 编钟双音!)</option>
                            <option value="waist" selected>钟腰 Waist (音色均衡)</option>
                            <option value="shoulder">钟肩 Shoulder (音色柔和)</option>
                            <option value="crown">钟顶 Crown (泛音空灵)</option>
                        </select>
                    </div>
                    <div>
                        <label style="font-size:12px;color:#666;">🔨 敲钟木槌</label>
                        <select id="vs_mallet" style="width:100%;padding:6px;">
                            <option value="soft">软质毡包槌 (温暖醇厚)</option>
                            <option value="medium" selected>中硬枣木槌 (经典编钟)</option>
                            <option value="hard">红木/牛角槌 (明亮清脆)</option>
                            <option value="metal">金属槌 (金石之声,慎击!)</option>
                        </select>
                    </div>
                    <div style="grid-column:span 2;">
                        <div style="display:flex;align-items:center;gap:12px;">
                            <label style="font-size:12px;color:#666;white-space:nowrap;">
                                💪 力度 <b id="vs_force_label">50%</b>
                                <span style="color:#888;font-weight:normal;">(轻抚/标准/重槌/全力)</span>
                            </label>
                            <input type="range" id="vs_force" min="10" max="100" value="50" style="flex:1;">
                            <button id="vs_strike_btn" style="background:#E63946;color:white;padding:8px 20px;border:none;border-radius:4px;cursor:pointer;font-weight:bold;font-size:14px;">
                                🔔 立即敲钟!
                            </button>
                            <button id="vs_tutorial_btn" style="background:#264653;color:white;padding:8px 16px;border:none;border-radius:4px;cursor:pointer;font-size:12px;">
                                📖 敲击教程
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div id="vs_interactive" style="background:white;border-radius:8px;padding:16px;margin-bottom:16px;box-shadow:0 2px 8px rgba(0,0,0,0.05);">
                <h4 style="margin-top:0;color:#BC6C25;">🎯 交互式敲击 (拖拽钟槌)</h4>
                <canvas id="vs_canvas" width="720" height="380" style="width:100%;max-width:720px;border:1px solid #ddd;border-radius:4px;background:linear-gradient(180deg,#e8f4f8 0%,#fdfbf7 100%);cursor:grab;"></canvas>
                <p style="font-size:12px;color:#666;margin:8px 0 0;">
                    🎮 操作: 从左侧拖动钟槌 → 向右滑动撞击钟体 → 速度决定力度 → 撞击位置决定音色 → 立刻听！
                </p>
            </div>

            <div id="vs_result" style="display:none;background:white;padding:16px;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,0.05);margin-bottom:16px;"></div>

            <div id="vs_history_area">
                <h4 style="color:#264653;">📜 敲击历史</h4>
                <div id="vs_history_list" style="display:grid;grid-template-columns:repeat(auto-fill,minmax(240px,1fr));gap:8px;"></div>
            </div>

            <div id="vs_tutorial" style="display:none;background:#FFF8E1;padding:16px;border-radius:8px;margin-top:16px;border:1px solid #F4A261;">
                <h4 style="margin-top:0;color:#BC6C25;">🎓 古代编钟敲击技法入门</h4>
                <div id="vs_tutorial_content" style="white-space:pre-line;font-size:13px;line-height:1.8;"></div>
            </div>
        `;

        // 事件
        document.getElementById('vs_pos').addEventListener('change', e => {
            this.selectedPos = e.target.value;
            const names = { lip: '钟口', rim: '唇边', waist: '钟腰', shoulder: '钟肩', crown: '钟顶' };
            document.getElementById('vs_pos_name').textContent = names[this.selectedPos] || '';
        });
        document.getElementById('vs_mallet').addEventListener('change', e => this.selectedMallet = e.target.value);
        document.getElementById('vs_force').addEventListener('input', e => {
            this.strikeForce = parseInt(e.target.value) / 100;
            document.getElementById('vs_force_label').textContent = e.target.value + '%';
        });
        document.getElementById('vs_bell').addEventListener('change', e => {
            this.currentBell = this.bellOptions.find(b => b.bell_id === e.target.value) || this.currentBell;
        });
        document.getElementById('vs_strike_btn').addEventListener('click', () => this.doStrike(this.strikeForce));
        document.getElementById('vs_tutorial_btn').addEventListener('click', () => this.showTutorial());
        this.setupDragInteraction();
        this.drawInteractiveScene();
    }

    setupDragInteraction() {
        const canvas = document.getElementById('vs_canvas');
        if (!canvas) return;
        let startX = 0, startT = 0, lastX = 0, lastT = 0;

        const getPos = (e) => {
            const r = canvas.getBoundingClientRect();
            const ex = e.touches ? e.touches[0].clientX : e.clientX;
            const ey = e.touches ? e.touches[0].clientY : e.clientY;
            return { x: (ex - r.left) * canvas.width / r.width, y: (ey - r.top) * canvas.height / r.height };
        };

        const onDown = (e) => {
            const p = getPos(e);
            if (p.x < 200) {
                this.dragging = true;
                startX = lastX = p.x;
                startT = lastT = performance.now();
                canvas.style.cursor = 'grabbing';
                e.preventDefault();
            }
        };
        const onMove = (e) => {
            if (!this.dragging) return;
            const p = getPos(e);
            this.malletX = Math.max(0, Math.min(520, p.x));
            this.malletY = p.y;
            lastX = p.x; lastT = performance.now();
            this.drawInteractiveScene();
            e.preventDefault();
        };
        const onUp = (e) => {
            if (!this.dragging) return;
            this.dragging = false;
            canvas.style.cursor = 'grab';
            const p = getPos(e.changedTouches ? { clientX: e.changedTouches[0].clientX, clientY: e.changedTouches[0].clientY } : e);
            const dt = Math.max(1, lastT - startT);
            const dx = p.x - startX;
            const velocity = dx / dt;
            const force = Math.max(0.1, Math.min(1.0, Math.abs(velocity) * 8.0));
            const posMap = [
                { yMin: 0, yMax: 80, key: 'crown', name: '钟顶' },
                { yMin: 80, yMax: 150, key: 'shoulder', name: '钟肩' },
                { yMin: 150, yMax: 230, key: 'waist', name: '钟腰' },
                { yMin: 230, yMax: 300, key: 'rim', name: '唇边' },
                { yMin: 300, yMax: 400, key: 'lip', name: '钟口' },
            ];
            const hitY = Math.max(40, Math.min(360, p.y));
            const pos = posMap.find(m => hitY >= m.yMin && hitY < m.yMax) || posMap[2];
            this.selectedPos = pos.key;
            document.getElementById('vs_pos').value = pos.key;
            document.getElementById('vs_pos_name').textContent = pos.name;
            this.strikeForce = force;
            document.getElementById('vs_force').value = Math.round(force * 100);
            document.getElementById('vs_force_label').textContent = Math.round(force * 100) + '%';
            if (p.x >= 480) {
                this.animateStrikeImpact();
                setTimeout(() => this.doStrike(force, pos.key), 80);
            }
            setTimeout(() => {
                this.malletX = 80;
                this.malletY = 190;
                this.drawInteractiveScene();
            }, 400);
        };
        canvas.addEventListener('mousedown', onDown);
        canvas.addEventListener('mousemove', onMove);
        canvas.addEventListener('mouseup', onUp);
        canvas.addEventListener('mouseleave', onUp);
        canvas.addEventListener('touchstart', onDown, { passive: false });
        canvas.addEventListener('touchmove', onMove, { passive: false });
        canvas.addEventListener('touchend', onUp);
        this.malletX = 80;
        this.malletY = 190;
    }

    animateStrikeImpact() {
        this.strikeFlash = 1.0;
        const tick = () => {
            this.strikeFlash = Math.max(0, (this.strikeFlash || 0) - 0.08);
            this.drawInteractiveScene();
            if (this.strikeFlash > 0) requestAnimationFrame(tick);
        };
        tick();
    }

    drawInteractiveScene() {
        const canvas = document.getElementById('vs_canvas');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        const W = canvas.width, H = canvas.height;
        ctx.clearRect(0, 0, W, H);
        const cx = 560, cy = 190;
        const bellProfile = [
            { x: 0, y: -150, w: 70 },
            { x: -20, y: -110, w: 85 },
            { x: -30, y: -60, w: 105 },
            { x: -40, y: -10, w: 125 },
            { x: -45, y: 40, w: 140 },
            { x: -50, y: 90, w: 155 },
            { x: -55, y: 130, w: 165 },
            { x: -20, y: 155, w: 175 },
            { x: 10, y: 165, w: 170 },
        ];
        const posZones = [
            { key: 'crown', yMin: cy - 150, yMax: cy - 80, color: '#FFE0B2', label: '钟顶 Crown' },
            { key: 'shoulder', yMin: cy - 80, yMax: cy - 10, color: '#FFCC80', label: '钟肩 Shoulder' },
            { key: 'waist', yMin: cy - 10, yMax: cy + 90, color: '#FFB74D', label: '钟腰 Waist' },
            { key: 'rim', yMin: cy + 90, yMax: cy + 140, color: '#FFA726', label: '唇边 Rim' },
            { key: 'lip', yMin: cy + 140, yMax: cy + 170, color: '#FB8C00', label: '钟口 Lip' },
        ];
        bellProfile.forEach((p, i) => {
            if (i === 0) return;
            const prev = bellProfile[i - 1];
            const zone = posZones.find(z => (cy + p.y) >= z.yMin && (cy + p.y) < z.yMax) || posZones[2];
            const grad = ctx.createLinearGradient(cx - 180, 0, cx + 20, 0);
            grad.addColorStop(0, '#5D4037');
            grad.addColorStop(0.3, zone.color);
            grad.addColorStop(0.6, '#8D6E63');
            grad.addColorStop(1, '#3E2723');
            ctx.fillStyle = grad;
            ctx.beginPath();
            ctx.moveTo(cx + prev.x - prev.w / 2, cy + prev.y);
            ctx.lineTo(cx + prev.x + prev.w / 2, cy + prev.y);
            ctx.lineTo(cx + p.x + p.w / 2, cy + p.y);
            ctx.lineTo(cx + p.x - p.w / 2, cy + p.y);
            ctx.closePath();
            ctx.fill();
            ctx.strokeStyle = '#3E2723';
            ctx.lineWidth = 0.8;
            ctx.stroke();
        });
        posZones.forEach(z => {
            const isSel = z.key === this.selectedPos;
            ctx.fillStyle = isSel ? 'rgba(230,57,70,0.25)' : 'rgba(38,70,83,0.05)';
            ctx.fillRect(cx - 180, z.yMin, 360, z.yMax - z.yMin);
            if (isSel) {
                ctx.strokeStyle = '#E63946';
                ctx.lineWidth = 3;
                ctx.strokeRect(cx - 180, z.yMin, 360, z.yMax - z.yMin);
            }
            ctx.fillStyle = isSel ? '#E63946' : '#264653';
            ctx.font = isSel ? 'bold 11px sans-serif' : '10px sans-serif';
            ctx.fillText(z.label, cx - 170, (z.yMin + z.yMax) / 2 + 4);
        });
        ctx.fillStyle = '#4E342E';
        ctx.fillRect(cx - 6, cy - 165, 12, 22);
        if (this.strikeFlash > 0) {
            const sf = this.strikeFlash;
            const grad = ctx.createRadialGradient(cx - 40, cy + 60, 0, cx - 40, cy + 60, 180 * sf);
            grad.addColorStop(0, `rgba(255,235,59,${0.8 * sf})`);
            grad.addColorStop(0.5, `rgba(255,152,0,${0.4 * sf})`);
            grad.addColorStop(1, 'rgba(255,87,34,0)');
            ctx.fillStyle = grad;
            ctx.fillRect(0, 0, W, H);
        }
        const mx = this.malletX || 80;
        const my = this.malletY || 190;
        ctx.save();
        ctx.translate(mx, my);
        const angle = (mx / 520) * 0.4 - 0.2;
        ctx.rotate(angle);
        const malletColors = {
            soft: { handle: '#6D4C41', head: '#8D6E63', wrap: '#EFEBE9' },
            medium: { handle: '#5D4037', head: '#A1887F', wrap: null },
            hard: { handle: '#4E342E', head: '#795548', wrap: '#B71C1C' },
            metal: { handle: '#424242', head: '#78909C', wrap: '#FFC107' },
        };
        const mc = malletColors[this.selectedMallet] || malletColors.medium;
        ctx.fillStyle = mc.handle;
        ctx.beginPath();
        ctx.roundRect(-80, -5, 90, 10, 4);
        ctx.fill();
        ctx.strokeStyle = '#3E2723';
        ctx.lineWidth = 1;
        ctx.stroke();
        ctx.fillStyle = mc.head;
        ctx.beginPath();
        ctx.ellipse(20, 0, 26, 20, 0, 0, Math.PI * 2);
        ctx.fill();
        ctx.strokeStyle = '#212121';
        ctx.stroke();
        if (mc.wrap) {
            ctx.strokeStyle = mc.wrap;
            ctx.lineWidth = 3;
            for (let a = 0; a < Math.PI * 2; a += 0.25) {
                ctx.beginPath();
                ctx.moveTo(20 + 10 * Math.cos(a), 14 * Math.sin(a));
                ctx.lineTo(20 + 22 * Math.cos(a + 0.1), 18 * Math.sin(a + 0.1));
                ctx.stroke();
            }
        }
        ctx.restore();
        if (this.dragging) {
            ctx.fillStyle = 'rgba(38,70,83,0.8)';
            ctx.fillRect(10, H - 32, 200, 22);
            ctx.fillStyle = 'white';
            ctx.font = 'bold 12px sans-serif';
            const fPct = Math.round(this.strikeForce * 100);
            ctx.fillText(`🎯 拖拽中 → 钟槌位置: (${Math.round(mx)},${Math.round(my)})  预估力度: ${fPct}%`, 18, H - 16);
        }
        ctx.fillStyle = '#666';
        ctx.font = '11px sans-serif';
        ctx.fillText('← 从这里按住拖动钟槌 → 撞击此区域 →', 200, 360);
        ctx.fillStyle = '#0077B6';
        ctx.fillText('⬅ 拖槌起点区', 40, 30);
        ctx.fillStyle = '#E63946';
        ctx.fillText('钟体敲击区 ➡', cx - 40, 30);
    }

    async doStrike(force, positionOverride) {
        if (!this.audioCtx) this.setupAudioContext();
        if (this.audioCtx && this.audioCtx.state === 'suspended') {
            try { await this.audioCtx.resume(); } catch (e) {}
        }
        const pos = positionOverride || this.selectedPos;
        const resultEl = document.getElementById('vs_result');
        resultEl.style.display = 'block';
        resultEl.innerHTML = '<div style="color:#666;padding:20px;text-align:center;">🔔 计算敲击响应 + 合成钟声...</div>';
        try {
            const payload = {
                bell_id: this.currentBell.bell_id,
                strike_position: pos,
                mallet_type: this.selectedMallet,
                strike_force: force,
                bell_height_m: this.currentBell.height_m,
                bell_diameter_m: this.currentBell.diameter_m,
                expected_freq_hz: this.currentBell.expected_freq_hz,
            };
            const res = await fetch(this.apiBase + '/experience/virtual-strike', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload),
            });
            const data = await res.json();
            if (!data.success) throw new Error(data.error || '敲击失败');
            const r = data.data;
            this.renderStrikeResult(r);
            this.synthesizeAudio(r.audio_synthesis_params);
            this.addHistoryEntry(r, pos, force);
        } catch (e) {
            resultEl.innerHTML = `<div style="color:#E63946;padding:20px;">❌ 敲击出错: ${e.message}<br><small>使用降级合成模式...</small></div>`;
            const fallback = this.buildFallbackSynth(force, pos);
            this.synthesizeAudio(fallback);
        }
    }

    buildFallbackSynth(force, pos) {
        const base = this.currentBell.expected_freq_hz || 256;
        const posBias = { lip: 0, rim: 4, waist: 2, shoulder: -2, crown: -5 }[pos] || 0;
        const ratios = [0.5, 1.0, 1.19 + posBias * 0.01, 1.5, 2.0, 2.5, 3.0, 4.0];
        const gains = [0.15, 1.0, 0.7 + posBias * 0.05, 0.35, 0.25, 0.15, 0.1, 0.06];
        const decays = [2.5 + force, 4.0 + force * 2, 3.5 + force * 1.5, 3.0, 2.5, 2.0, 1.5, 1.0];
        const malletBias = { soft: -1, medium: 0, hard: 1, metal: 2 }[this.selectedMallet] || 0;
        return {
            base_frequency_hz: base,
            master_gain: 0.2 + force * 0.5,
            stereo_pan: 0,
            partials: ratios.map((r, i) => ({
                freq_ratio: r,
                gain: gains[i] * (1 + malletBias * 0.15 * (i > 2 ? 1 : 0)),
                decay_s: decays[i],
                detune_cents: (Math.random() - 0.5) * 6,
            })),
        };
    }

    renderStrikeResult(r) {
        const el = document.getElementById('vs_result');
        const posNames = { lip: '钟口 Lip', rim: '唇边 Rim', waist: '钟腰 Waist', shoulder: '钟肩 Shoulder', crown: '钟顶 Crown' };
        const malletNames = { soft: '软质毡包槌', medium: '枣木槌', hard: '红木/牛角槌', metal: '金属槌' };
        const sp = r.strike_physics || {};
        const qm = r.quality_metrics || {};
        const partialsTable = (r.audio_synthesis_params?.partials || []).map((p, i) => `
            <tr style="${i === 1 ? 'background:#E3F2FD;font-weight:bold;' : ''}">
                <td style="padding:6px;border:1px solid #eee;text-align:center;">#${i + 1}</td>
                <td style="padding:6px;border:1px solid #eee;text-align:right;">${(r.audio_synthesis_params.base_frequency_hz * p.freq_ratio).toFixed(1)} Hz</td>
                <td style="padding:6px;border:1px solid #eee;text-align:center;">${p.freq_ratio.toFixed(3)}</td>
                <td style="padding:6px;border:1px solid #eee;text-align:right;">${(p.gain * 100).toFixed(0)}%</td>
                <td style="padding:6px;border:1px solid #eee;text-align:right;">${p.decay_s.toFixed(1)}s</td>
                <td style="padding:6px;border:1px solid #eee;text-align:right;">${p.detune_cents.toFixed(1)}音分</td>
            </tr>
        `).join('');
        el.innerHTML = `
            <div>
                <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:10px;margin-bottom:12px;">
                    <div style="background:linear-gradient(135deg,#E63946,#F4A261);color:white;padding:10px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">敲击参数</div>
                        <div style="font-size:14px;font-weight:bold;">${posNames[r.strike_position] || r.strike_position}</div>
                        <div style="font-size:11px;">${malletNames[r.mallet_type] || r.mallet_type} · 力度${Math.round((r.strike_force || 0.5) * 100)}%</div>
                    </div>
                    <div style="background:linear-gradient(135deg,#264653,#2A9D8F);color:white;padding:10px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">接触力学 (Hertz)</div>
                        <div style="font-size:14px;font-weight:bold;">峰值 ${(sp.peak_force_n || 0).toFixed(0)} N</div>
                        <div style="font-size:11px;">接触 ${(sp.contact_duration_ms || 0).toFixed(1)}ms · 能量 ${(sp.impact_energy_j || 0).toFixed(2)}J</div>
                    </div>
                    <div style="background:linear-gradient(135deg,#457B9D,#0077B6);color:white;padding:10px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">声学响应</div>
                        <div style="font-size:14px;font-weight:bold;">基频 ${(r.audio_synthesis_params?.base_frequency_hz || 0).toFixed(1)} Hz</div>
                        <div style="font-size:11px;">峰值SPL ${(qm.peak_spl_db || 0).toFixed(1)}dB · RT60 ${(qm.rt60_s || 0).toFixed(1)}s</div>
                    </div>
                    <div style="background:linear-gradient(135deg,#606C38,#BC6C25);color:white;padding:10px;border-radius:6px;">
                        <div style="font-size:11px;opacity:0.9;">音质评价</div>
                        <div style="font-size:14px;font-weight:bold;">综合 ${(qm.overall_score || 0).toFixed(0)}/100</div>
                        <div style="font-size:11px;">明亮 ${(qm.brightness || 0).toFixed(0)} · 温暖 ${(qm.warmth || 0).toFixed(0)} · 延音 ${(qm.sustain_quality || 0).toFixed(0)}</div>
                    </div>
                </div>
                <h4 style="color:#264653;margin:8px 0;">🎵 8阶分音结构 (Web Audio合成参数)</h4>
                <div style="overflow-x:auto;">
                    <table style="border-collapse:collapse;width:100%;font-size:12px;">
                        <thead><tr style="background:#264653;color:white;">
                            <th style="padding:8px;">阶</th>
                            <th style="padding:8px;">实际频率</th>
                            <th style="padding:8px;">频率比</th>
                            <th style="padding:8px;">能量</th>
                            <th style="padding:8px;">衰减</th>
                            <th style="padding:8px;">失谐</th>
                        </tr></thead>
                        <tbody>${partialsTable}</tbody>
                    </table>
                </div>
                ${r.positional_timbre_note ? `<div style="margin-top:10px;padding:10px;background:#FFF8E1;border-radius:4px;border-left:4px solid #BC6C25;font-size:13px;">💡 ${r.positional_timbre_note}</div>` : ''}
            </div>
        `;
    }

    synthesizeAudio(params) {
        if (!this.audioCtx) return;
        const ctx = this.audioCtx;
        const now = ctx.currentTime;
        const baseFreq = params.base_frequency_hz || 256;
        const master = ctx.createGain();
        master.gain.setValueAtTime(0, now);
        master.gain.linearRampToValueAtTime(params.master_gain || 0.5, now + 0.01);
        master.gain.exponentialRampToValueAtTime(0.001, now + 8);
        const panner = ctx.createStereoPanner();
        panner.pan.value = params.stereo_pan || 0;
        master.connect(panner).connect(ctx.destination);
        (params.partials || []).forEach((p, idx) => {
            const osc = ctx.createOscillator();
            osc.type = idx < 2 ? 'sine' : (idx < 5 ? 'triangle' : 'sine');
            const f = baseFreq * p.freq_ratio;
            osc.frequency.value = f;
            osc.detune.value = p.detune_cents || 0;
            const g = ctx.createGain();
            const attack = 0.002 + idx * 0.001;
            const decay = p.decay_s || 2;
            g.gain.setValueAtTime(0, now);
            g.gain.linearRampToValueAtTime(p.gain || 0.3, now + attack);
            g.gain.exponentialRampToValueAtTime(0.0001, now + attack + decay);
            osc.connect(g).connect(master);
            osc.start(now);
            osc.stop(now + attack + decay + 0.2);
        });
        if (window.__synthVisualizer) {
            try { window.__synthVisualizer(params); } catch (e) {}
        }
    }

    async showTutorial() {
        const el = document.getElementById('vs_tutorial');
        const content = document.getElementById('vs_tutorial_content');
        if (el.style.display !== 'none') {
            el.style.display = 'none';
            return;
        }
        el.style.display = 'block';
        content.textContent = '加载教程中...';
        try {
            const res = await fetch(this.apiBase + '/experience/strike-tutorial');
            const data = await res.json();
            if (data.success) {
                const t = data.data;
                content.innerHTML = `
                    <h5 style="margin:10px 0 4px;color:#8B4513;">🎯 学习目标</h5>
                    <p style="margin:4px 0;padding-left:12px;">${t.learning_objectives || ''}</p>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">✋ 握槌姿势</h5>
                    <p style="margin:4px 0;padding-left:12px;white-space:pre-line;">${t.grip_technique || ''}</p>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">💪 力度分级 (共4级)</h5>
                    <ul style="margin:4px 0;padding-left:32px;">
                        ${(t.force_levels || []).map(l => `<li><b>${l.name}</b> (${l.force_range}): ${l.description}</li>`).join('')}
                    </ul>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">📍 位置差异 (编钟双音原理)</h5>
                    <ul style="margin:4px 0;padding-left:32px;">
                        ${(t.position_guide || []).map(p => `<li><b>${p.name}</b>: ${p.description}</li>`).join('')}
                    </ul>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">🎼 编钟双音 (曾侯乙核心发现)</h5>
                    <p style="margin:4px 0;padding-left:12px;white-space:pre-line;">${t.bianzhong_dual_tone || ''}</p>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">🔨 木槌选择指南</h5>
                    <ul style="margin:4px 0;padding-left:32px;">
                        ${(t.mallet_guide || []).map(m => `<li><b>${m.name}</b>: ${m.description}</li>`).join('')}
                    </ul>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">⚠️ 保养禁忌</h5>
                    <p style="margin:4px 0;padding-left:12px;white-space:pre-line;color:#C62828;">${t.maintenance_taboo || ''}</p>
                    <h5 style="margin:10px 0 4px;color:#8B4513;">📜 练习曲目入门</h5>
                    <p style="margin:4px 0;padding-left:12px;white-space:pre-line;font-family:monospace;background:#FFF3E0;padding:8px;border-radius:4px;">${t.practice_exercises || ''}</p>
                `;
            }
        } catch (e) {
            content.innerHTML = `
                <b>教程加载失败，使用内置教程：</b>\n\n
                🎯 握槌: 右手握槌柄1/3处，手腕放松，小臂带动\n
                💪 力度: 轻拂(10%)→标准(40-60%)→重槌(80%)→全力(100%)\n
                📍 正鼓部=主音, 侧鼓部=上方小三度(曾侯乙双音!)\n
                🔨 软槌=温暖, 木槌=经典, 硬槌=明亮, 金属槌=金石之声
            `;
        }
    }

    addHistoryEntry(r, pos, force) {
        const now = new Date();
        const timeStr = now.toLocaleTimeString();
        const entry = { time: timeStr, pos, force, result: r };
        this.strikeHistory.unshift(entry);
        if (this.strikeHistory.length > 12) this.strikeHistory.pop();
        const listEl = document.getElementById('vs_history_list');
        if (!listEl) return;
        const posNames = { lip: '钟口', rim: '唇边', waist: '钟腰', shoulder: '钟肩', crown: '钟顶' };
        const forceColor = f => f < 0.3 ? '#43A047' : f < 0.6 ? '#FFB300' : f < 0.85 ? '#FB8C00' : '#E63946';
        listEl.innerHTML = this.strikeHistory.map((h, i) => {
            const qm = h.result.quality_metrics || {};
            return `
                <div style="background:white;padding:10px;border-radius:6px;border:1px solid #ddd;box-shadow:0 1px 3px rgba(0,0,0,0.05);font-size:12px;">
                    <div style="display:flex;justify-content:space-between;margin-bottom:6px;">
                        <b style="color:#264653;">#${this.strikeHistory.length - i} ${posNames[h.pos] || h.pos}</b>
                        <span style="color:#888;">${h.time}</span>
                    </div>
                    <div style="margin:4px 0;">
                        💪 力度: <b style="color:${forceColor(h.force)}">${Math.round(h.force * 100)}%</b>
                    </div>
                    <div style="color:#666;">🔊 SPL ${(qm.peak_spl_db || 0).toFixed(0)}dB · ⏱ RT60 ${(qm.rt60_s || 0).toFixed(1)}s</div>
                    <div style="margin-top:6px;">
                        <span style="display:inline-block;padding:2px 6px;background:#E3F2FD;color:#0D47A1;border-radius:3px;font-size:11px;">综合 ${(qm.overall_score || 0).toFixed(0)}</span>
                    </div>
                    <button onclick="window.__replayStrike(${i})" style="margin-top:6px;width:100%;padding:4px;background:#F4A261;color:white;border:none;border-radius:3px;cursor:pointer;font-size:11px;">
                        🔁 再听一次
                    </button>
                </div>
            `;
        }).join('');
        window.__replayStrike = (idx) => {
            const h = this.strikeHistory[idx];
            if (h && h.result?.audio_synthesis_params) {
                this.synthesizeAudio(h.result.audio_synthesis_params);
            }
        };
    }
}
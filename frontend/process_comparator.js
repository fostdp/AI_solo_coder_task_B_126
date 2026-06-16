const COLOR_PALETTE = [
    '#E63946', '#457B9D', '#2A9D8F', '#F4A261',
    '#E76F51', '#264653', '#606C38', '#BC6C25',
    '#DDA15E', '#60A5FA'
];

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
        for (let i = 0; i <= 5; i++) {
            const y = pad + (h - 2 * pad) * i / 5;
            svg += `<line x1="${pad}" y1="${y}" x2="${w - pad / 2}" y2="${y}" stroke="#eee"/>`;
            svg += `<text x="${pad - 8}" y="${y + 4}" font-size="10" fill="#666" text-anchor="end">${100 - i * 20}</text>`;
        }
        svg += `<text x="10" y="${h / 2}" font-size="11" fill="#666" transform="rotate(-90 10,${h / 2})">评分 (0-100)</text>`;
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

export { CastingMethodPanel };

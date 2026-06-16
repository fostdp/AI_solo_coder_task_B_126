const COLOR_PALETTE = [
    '#E63946', '#457B9D', '#2A9D8F', '#F4A261',
    '#E76F51', '#264653', '#606C38', '#BC6C25',
    '#DDA15E', '#60A5FA'
];

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
        for (let i = 0; i <= 5; i++) {
            const y = pad + (h - 2 * pad) * i / 5;
            const fv = maxF - (maxF - minF) * i / 5;
            svg += `<line x1="${pad}" y1="${y}" x2="${w - pad / 2}" y2="${y}" stroke="#eee"/>`;
            svg += `<text x="${pad - 8}" y="${y + 4}" font-size="10" fill="#666" text-anchor="end">${fv.toFixed(0)}Hz</text>`;
        }
        const modes = ['(2,0)', '(3,0)', '(4,0)', '(2,1)', '(5,0)', '(3,1)', '(6,0)', '(4,1)'];
        modes.forEach((m, i) => {
            const x = pad + (w - 2 * pad) * i / (modes.length - 1);
            svg += `<text x="${x}" y="${h - 20}" font-size="10" fill="#666" text-anchor="middle">模${m}</text>`;
            svg += `<circle cx="${x}" cy="${h - pad}" r="2" fill="#aaa"/>`;
        });
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
        for (let i = 0; i < N; i++) {
            const ang = -Math.PI / 2 + i * 2 * Math.PI / N;
            const x = cx + R * Math.cos(ang), y = cy + R * Math.sin(ang);
            const lx = cx + (R + 18) * Math.cos(ang), ly = cy + (R + 18) * Math.sin(ang);
            svg += `<line x1="${cx}" y1="${cy}" x2="${x}" y2="${y}" stroke="#ddd"/>`;
            svg += `<text x="${lx}" y="${ly + 4}" font-size="10" fill="#333" text-anchor="middle">${data.labels[i]}</text>`;
        }
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

export { AlloyComparisonPanel };

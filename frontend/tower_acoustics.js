const COLOR_PALETTE = [
    '#E63946', '#457B9D', '#2A9D8F', '#F4A261',
    '#E76F51', '#264653', '#606C38', '#BC6C25',
    '#DDA15E', '#60A5FA'
];

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
        for (let k = 1; k <= 4; k++) {
            ctx.strokeStyle = '#ddd';
            ctx.beginPath();
            ctx.arc(cx, cy, R * k / 4, 0, Math.PI * 2);
            ctx.stroke();
        }
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

export { TowerAcousticPanel };

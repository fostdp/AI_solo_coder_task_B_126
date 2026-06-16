const COLOR_PALETTE = [
    '#E63946', '#457B9D', '#2A9D8F', '#F4A261',
    '#E76F51', '#264653', '#606C38', '#BC6C25',
    '#DDA15E', '#60A5FA'
];

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

const Engine = Matter.Engine,
    Render = Matter.Render,
    Bodies = Matter.Bodies,
    Body = Matter.Body,
    Composite = Matter.Composite,
    Vector = Matter.Vector,
    Events = Matter.Events;

const container = document.getElementById('physics-container');
const W = 800;
const H = 500;

const engine = Engine.create({ 
    gravity: { x: 0, y: 0 },
    positionIterations: 10,
    velocityIterations: 10
});

const render = Render.create({
    element: container,
    engine: engine,
    options: {
        width: W,
        height: H,
        wireframes: false,
        background: '#ffffff'
    }
});
Render.run(render);

const POPULATION_SIZE = 100;
const TRIAL_FRAMES = 800;
const DNA_LENGTH = 62;
const MUTATION_RATE = 0.05;

const COLS = 16;
const ROWS = 10;
const CELL_W = Math.floor(W / COLS);
const CELL_H = Math.floor(H / ROWS);
const WALL_THICKNESS = 6;

let grid = [];
for (let r = 0; r < ROWS; r++) {
    for (let c = 0; c < COLS; c++) {
        grid.push({
            r, c,
            walls: [true, true, true, true],
            visited: false
        });
    }
}

function getCell(r, c) {
    if (r < 0 || c < 0 || r >= ROWS || c >= COLS) return null;
    return grid[r * COLS + c];
}

function generateMaze() {
    let stack = [];
    let current = getCell(0, 0);
    current.visited = true;
    stack.push(current);

    while (stack.length > 0) {
        let neighbors = [];
        let { r, c } = current;
        let top = getCell(r - 1, c);
        let right = getCell(r, c + 1);
        let bottom = getCell(r + 1, c);
        let left = getCell(r, c - 1);

        if (top && !top.visited) neighbors.push({ cell: top, dir: 0 });
        if (right && !right.visited) neighbors.push({ cell: right, dir: 1 });
        if (bottom && !bottom.visited) neighbors.push({ cell: bottom, dir: 2 });
        if (left && !left.visited) neighbors.push({ cell: left, dir: 3 });

        if (neighbors.length > 0) {
            let pick = neighbors[Math.floor(Math.random() * neighbors.length)];
            if (pick.dir === 0) { current.walls[0] = false; pick.cell.walls[2] = false; }
            if (pick.dir === 1) { current.walls[1] = false; pick.cell.walls[3] = false; }
            if (pick.dir === 2) { current.walls[2] = false; pick.cell.walls[0] = false; }
            if (pick.dir === 3) { current.walls[3] = false; pick.cell.walls[1] = false; }

            current = pick.cell;
            current.visited = true;
            stack.push(current);
        } else {
            current = stack.pop();
        }
    }
}

generateMaze();

function buildMazeWalls() {
    const wallColor = '#3a3a3c';
    let wallSet = new Set();

    for (let r = 0; r < ROWS; r++) {
        for (let c = 0; c < COLS; c++) {
            let cell = getCell(r, c);
            let x = c * CELL_W;
            let y = r * CELL_H;

            if (cell.walls[0]) {
                let key = `h_${r}_${c}`;
                if (!wallSet.has(key)) {
                    wallSet.add(key);
                    Composite.add(engine.world, Bodies.rectangle(
                        x + CELL_W / 2, y, CELL_W, WALL_THICKNESS,
                        { isStatic: true, isWall: true, render: { fillStyle: wallColor } }
                    ));
                }
            }
            if (cell.walls[1]) {
                if (!(r === ROWS - 1 && c === COLS - 1)) {
                    let key = `v_${r}_${c + 1}`;
                    if (!wallSet.has(key)) {
                        wallSet.add(key);
                        Composite.add(engine.world, Bodies.rectangle(
                            x + CELL_W, y + CELL_H / 2, WALL_THICKNESS, CELL_H,
                            { isStatic: true, isWall: true, render: { fillStyle: wallColor } }
                        ));
                    }
                }
            }
            if (cell.walls[2]) {
                let key = `h_${r + 1}_${c}`;
                if (!wallSet.has(key)) {
                    wallSet.add(key);
                    Composite.add(engine.world, Bodies.rectangle(
                        x + CELL_W / 2, y + CELL_H, CELL_W, WALL_THICKNESS,
                        { isStatic: true, isWall: true, render: { fillStyle: wallColor } }
                    ));
                }
            }
            if (cell.walls[3]) {
                if (!(r === 0 && c === 0)) {
                    let key = `v_${r}_${c}`;
                    if (!wallSet.has(key)) {
                        wallSet.add(key);
                        Composite.add(engine.world, Bodies.rectangle(
                            x, y + CELL_H / 2, WALL_THICKNESS, CELL_H,
                            { isStatic: true, isWall: true, render: { fillStyle: wallColor } }
                        ));
                    }
                }
            }
        }
    }
}

buildMazeWalls();

const startPos = { x: CELL_W / 2, y: CELL_H / 2 };
const goalPos = { x: (COLS - 0.5) * CELL_W, y: (ROWS - 0.5) * CELL_H };

Composite.add(engine.world, Bodies.circle(startPos.x, startPos.y, 10, {
    isSensor: true, isStatic: true, render: { fillStyle: '#0071e3', opacity: 0.15 }
}));

function solveMazeBFS() {
    let queue = [{ r: 0, c: 0, path: [{ r: 0, c: 0 }] }];
    let visited = new Set();
    visited.add('0,0');

    while (queue.length > 0) {
        let { r, c, path } = queue.shift();
        if (r === ROWS - 1 && c === COLS - 1) return path;

        let cell = getCell(r, c);
        let dirs = [
            { dr: -1, dc: 0, wall: 0 },
            { dr: 0, dc: 1, wall: 1 },
            { dr: 1, dc: 0, wall: 2 },
            { dr: 0, dc: -1, wall: 3 }
        ];
        for (let d of dirs) {
            if (!cell.walls[d.wall]) {
                let nr = r + d.dr, nc = c + d.dc;
                let key = `${nr},${nc}`;
                if (!visited.has(key)) {
                    visited.add(key);
                    queue.push({ r: nr, c: nc, path: [...path, { r: nr, c: nc }] });
                }
            }
        }
    }
    return [];
}

let solutionPath = solveMazeBFS();
let checkpoints = [];
if (solutionPath.length > 2) {
    let step = Math.floor(solutionPath.length / 4);
    for (let i = 1; i <= 3; i++) {
        let idx = Math.min(i * step, solutionPath.length - 1);
        let cp = solutionPath[idx];
        checkpoints.push({
            x: (cp.c + 0.5) * CELL_W,
            y: (cp.r + 0.5) * CELL_H
        });
    }
}
checkpoints.push({ x: goalPos.x, y: goalPos.y });

checkpoints.forEach((cp, i) => {
    let isGoal = i === checkpoints.length - 1;
    let color = isGoal ? '#34c759' : '#ff9f0a';
    Composite.add(engine.world, Bodies.circle(cp.x, cp.y, isGoal ? 14 : 10, {
        isSensor: true, isStatic: true, render: { fillStyle: color, opacity: 0.25 }
    }));
});

function lineIntersection(x1, y1, x2, y2, x3, y3, x4, y4) {
    let den = (x4 - x3) * (y2 - y1) - (y4 - y3) * (x2 - x1);
    if (den === 0) return null;
    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den;
    let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den;
    if (ua >= 0 && ua <= 1 && ub >= 0 && ub <= 1) {
        return {
            x: x1 + ua * (x2 - x1),
            y: y1 + ua * (y2 - y1),
            t: ua
        };
    }
    return null;
}

function getWallIntersection(x1, y1, x2, y2, wall) {
    let wx1 = wall.bounds.min.x;
    let wx2 = wall.bounds.max.x;
    let wy1 = wall.bounds.min.y;
    let wy2 = wall.bounds.max.y;

    let closest = null;
    let edges = [
        [wx1, wy1, wx2, wy1],
        [wx2, wy1, wx2, wy2],
        [wx1, wy2, wx2, wy2],
        [wx1, wy1, wx1, wy2]
    ];

    for (let edge of edges) {
        let intersect = lineIntersection(x1, y1, x2, y2, edge[0], edge[1], edge[2], edge[3]);
        if (intersect) {
            if (!closest || intersect.t < closest.t) {
                closest = intersect;
            }
        }
    }
    return closest;
}

function castRay(x1, y1, angle, maxDist, wallsList) {
    let x2 = x1 + Math.cos(angle) * maxDist;
    let y2 = y1 + Math.sin(angle) * maxDist;

    let closestT = 1.0;
    let closestPoint = { x: x2, y: y2 };

    for (let wall of wallsList) {
        let intersect = getWallIntersection(x1, y1, x2, y2, wall);
        if (intersect && intersect.t < closestT) {
            closestT = intersect.t;
            closestPoint = { x: intersect.x, y: intersect.y };
        }
    }

    return {
        point: closestPoint,
        fraction: closestT
    };
}

function createBrain(dna = null) {
    if (!dna) {
        dna = Array.from({ length: DNA_LENGTH }, () => (Math.random() - 0.5) * 2.0);
    }
    return {
        dna: dna,
        predict(inputs) {
            let hidden = [];
            let dnaIdx = 0;
            for (let h = 0; h < 6; h++) {
                let sum = 0;
                for (let i = 0; i < 7; i++) {
                    sum += inputs[i] * dna[dnaIdx++];
                }
                sum += dna[dnaIdx++];
                hidden.push(Math.tanh(sum));
            }
            let outputs = [];
            for (let o = 0; o < 2; o++) {
                let sum = 0;
                for (let h = 0; h < 6; h++) {
                    sum += hidden[h] * dna[dnaIdx++];
                }
                sum += dna[dnaIdx++];
                outputs.push(Math.tanh(sum));
            }
            return outputs;
        }
    };
}

let currentAgent = null;

function spawnAgent() {
    if (currentAgent) Composite.remove(engine.world, currentAgent);

    currentAgent = Bodies.circle(startPos.x, startPos.y, 8, {
        frictionAir: 0.08,
        restitution: 0.3,
        render: { fillStyle: '#0071e3' }
    });
    Body.setVelocity(currentAgent, { x: 0, y: 0 });
    currentAgent.heading = 0;
    currentAgent.brain = createBrain(allDNA[currentIndex]);

    Composite.add(engine.world, currentAgent);
}

function randomDNA() {
    return Array.from({ length: DNA_LENGTH }, () => (Math.random() - 0.5) * 2.0);
}

function mutateDNA(dna) {
    return dna.map(gene => {
        if (Math.random() < MUTATION_RATE) {
            return gene + (Math.random() - 0.5) * 0.5;
        }
        return gene;
    });
}

function crossoverDNA(a, b) {
    let child = [];
    let p1 = Math.floor(Math.random() * DNA_LENGTH);
    let p2 = Math.floor(Math.random() * DNA_LENGTH);
    if (p1 > p2) [p1, p2] = [p2, p1];
    for (let i = 0; i < DNA_LENGTH; i++) {
        child.push(i >= p1 && i <= p2 ? b[i] : a[i]);
    }
    return mutateDNA(child);
}

let allDNA = [];
let fitnesses = [];
let generation = 0;
let currentIndex = 0;
let frameCount = 0;
let isRunning = false;
let isFastForward = false;
let bestEverFitness = 0;
let bestDNA = null;
let currentAgentCheckpoints = 0;

function startGeneration() {
    if (allDNA.length === 0) {
        for (let i = 0; i < POPULATION_SIZE; i++) allDNA.push(randomDNA());
    }
    fitnesses = new Array(POPULATION_SIZE).fill(0);
    currentIndex = 0;
    frameCount = 0;
    currentAgentCheckpoints = 0;
    spawnAgent();
    updateUI();
}

function updateUI() {
    document.getElementById('valGen').textContent = generation;
    document.getElementById('valFitness').textContent = bestEverFitness.toFixed(0);
    document.getElementById('valCurrent').textContent = `${currentIndex + 1} / ${POPULATION_SIZE}`;
}

function tick() {
    if (!isRunning) {
        isLoopRunning = false;
        return;
    }
    isLoopRunning = true;

    let loops = isFastForward ? 30 : 1;
    for (let l = 0; l < loops; l++) {
        Engine.update(engine, 1000 / 60);

        let radius = 8;
        let x = currentAgent.position.x;
        let y = currentAgent.position.y;
        let cx = Math.max(radius, Math.min(W - radius, x));
        let cy = Math.max(radius, Math.min(H - radius, y));
        if (cx !== x || cy !== y) {
            Body.setPosition(currentAgent, { x: cx, y: cy });
            Body.setVelocity(currentAgent, { x: 0, y: 0 });
        }

        frameCount++;

        let vel = currentAgent.velocity;
        if (vel.x * vel.x + vel.y * vel.y > 0.01) {
            currentAgent.heading = Math.atan2(vel.y, vel.x);
        }
        let heading = currentAgent.heading || 0;

        let angles = [
            heading - Math.PI / 2,
            heading - Math.PI / 4,
            heading,
            heading + Math.PI / 4,
            heading + Math.PI / 2
        ];

        let wallsList = Composite.allBodies(engine.world).filter(b => b.isWall);
        let nearbyWalls = wallsList.filter(wall => {
            let dx = wall.position.x - x;
            let dy = wall.position.y - y;
            return dx * dx + dy * dy < 250 * 250;
        });

        let sensors = angles.map(angle => {
            let res = castRay(x, y, angle, 150, nearbyWalls);
            return res.fraction;
        });

        let targetCp = checkpoints[currentAgentCheckpoints];
        let dx = (targetCp.x - x) / 800;
        let dy = (targetCp.y - y) / 500;

        let inputs = [...sensors, dx, dy];
        let outputs = currentAgent.brain.predict(inputs);

        let forceX = outputs[0] * 0.002;
        let forceY = outputs[1] * 0.002;
        Body.applyForce(currentAgent, currentAgent.position, { x: forceX, y: forceY });

        if (currentAgentCheckpoints < checkpoints.length) {
            let targetCp = checkpoints[currentAgentCheckpoints];
            let dist = Vector.magnitude(Vector.sub(targetCp, currentAgent.position));
            if (dist < 25) {
                currentAgentCheckpoints++;
            }
        }

        let hitGoal = currentAgentCheckpoints >= checkpoints.length;

        if (frameCount >= TRIAL_FRAMES || hitGoal) {
            let fit = currentAgentCheckpoints * 3000;

            if (currentAgentCheckpoints < checkpoints.length) {
                let nextCp = checkpoints[currentAgentCheckpoints];
                let dist = Vector.magnitude(Vector.sub(nextCp, currentAgent.position));
                fit += Math.max(0, 1500 - dist);
            }

            if (hitGoal) {
                fit += 20000 + (TRIAL_FRAMES - frameCount) * 20;
            }

            if (fit < 0) fit = 0;
            fitnesses[currentIndex] = fit;

            if (fit > bestEverFitness) {
                bestEverFitness = fit;
                bestDNA = [...allDNA[currentIndex]];
            }

            currentIndex++;
            frameCount = 0;
            currentAgentCheckpoints = 0;

            if (currentIndex >= POPULATION_SIZE) {
                breed();
            } else {
                spawnAgent();
            }
            break;
        }
    }

    if (!isFastForward || currentIndex % 5 === 0) updateUI();
    requestAnimationFrame(tick);
}

function breed() {
    let ranked = allDNA.map((dna, i) => ({ dna, fit: fitnesses[i] }));
    ranked.sort((a, b) => b.fit - a.fit);

    let newDNA = [
        [...ranked[0].dna],
        [...ranked[1].dna]
    ];

    while (newDNA.length < POPULATION_SIZE) {
        let poolSize = Math.floor(POPULATION_SIZE * 0.3);
        let p1 = ranked[Math.floor(Math.random() * poolSize)].dna;
        let p2 = ranked[Math.floor(Math.random() * poolSize)].dna;
        newDNA.push(crossoverDNA(p1, p2));
    }

    allDNA = newDNA;
    generation++;
    renderDNABars();
    startGeneration();
}

function renderDNABars() {
    const barsEl = document.getElementById('dnaBars');
    document.getElementById('dnaGen').textContent = generation;
    barsEl.innerHTML = '';
    if (!bestDNA) return;

    for (let i = 0; i < bestDNA.length; i++) {
        let weight = bestDNA[i];
        let bar = document.createElement('div');
        bar.className = 'dna-bar';
        let h = Math.min(100, Math.max(5, Math.abs(weight) * 50));
        bar.style.height = `${h}%`;
        let color = weight >= 0 ? 'var(--accent)' : 'var(--warning)';
        bar.style.background = color;
        barsEl.appendChild(bar);
    }
}

const btnStart = document.getElementById('btnStart');
const btnFF = document.getElementById('btnFF');
let isLoopRunning = false;

btnFF.addEventListener('click', () => {
    isFastForward = !isFastForward;
    if (isFastForward) {
        btnFF.textContent = '⏩ Fast Forward (30x)';
        btnFF.classList.remove('btn-secondary');
        btnFF.classList.add('btn-active');
    } else {
        btnFF.textContent = 'Normal Speed';
        btnFF.classList.remove('btn-active');
        btnFF.classList.add('btn-secondary');
    }
});

btnStart.addEventListener('click', () => {
    if (isRunning) {
        isRunning = false;
        btnStart.textContent = 'Resume Evolution';
        btnStart.classList.remove('btn-secondary');
        btnStart.classList.add('btn-primary');
    } else {
        if (generation === 0 && currentIndex === 0) startGeneration();
        isRunning = true;
        btnStart.textContent = 'Pause Evolution';
        btnStart.classList.remove('btn-primary');
        btnStart.classList.add('btn-secondary');
        if (!isLoopRunning) {
            isLoopRunning = true;
            tick();
        }
    }
});

Events.on(render, 'afterRender', () => {
    if (!isRunning || !currentAgent) return;
    const ctx = render.context;

    let x = currentAgent.position.x;
    let y = currentAgent.position.y;
    let heading = currentAgent.heading || 0;

    let angles = [
        heading - Math.PI / 2,
        heading - Math.PI / 4,
        heading,
        heading + Math.PI / 4,
        heading + Math.PI / 2
    ];

    let wallsList = Composite.allBodies(engine.world).filter(b => b.isWall);
    let nearbyWalls = wallsList.filter(wall => {
        let dx = wall.position.x - x;
        let dy = wall.position.y - y;
        return dx * dx + dy * dy < 250 * 250;
    });

    ctx.save();
    angles.forEach(angle => {
        let rayResult = castRay(x, y, angle, 150, nearbyWalls);
        let rx = rayResult.point.x;
        let ry = rayResult.point.y;
        let fraction = rayResult.fraction;

        ctx.beginPath();
        ctx.moveTo(x, y);
        ctx.lineTo(rx, ry);

        let r = Math.floor((1 - fraction) * 255);
        let g = Math.floor(fraction * 180);
        ctx.strokeStyle = `rgba(${r}, ${g}, 0, 0.4)`;
        ctx.lineWidth = 1.5;
        ctx.stroke();

        if (fraction < 1.0) {
            ctx.beginPath();
            ctx.arc(rx, ry, 3, 0, 2 * Math.PI);
            ctx.fillStyle = `rgba(${r}, ${g}, 0, 0.8)`;
            ctx.fill();
        }
    });
    ctx.restore();
});

use std::io::{self, Write};
use rayon::prelude::*;
use serde_json;
use rand::RngExt;

const ROWS: usize = 10;
const COLS: usize = 16;
const CELL_W: f64 = 50.0;
const CELL_H: f64 = 50.0;
const WALL_THICKNESS: f64 = 6.0;
const W: f64 = 800.0;
const H: f64 = 500.0;

const POPULATION_SIZE: usize = 100;
const TRIAL_FRAMES: usize = 800;
const DNA_LENGTH: usize = 50;
const MUTATION_RATE: f64 = 0.05;

#[derive(Clone, Debug)]
struct Wall {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

#[derive(Clone)]
struct Cell {
    r: usize,
    c: usize,
    walls: [bool; 4],
    visited: bool,
}

struct SeededRng {
    seed: u32,
}

impl SeededRng {
    fn new(seed: u32) -> Self {
        Self { seed }
    }
    fn next_f64(&mut self) -> f64 {
        self.seed = self.seed.wrapping_add(0x6D2B79F5);
        let mut t = self.seed;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        let val = (t ^ (t >> 14)) as u32;
        val as f64 / 4294967296.0
    }
}

fn generate_maze(rng: &mut SeededRng) -> Vec<Cell> {
    let mut grid = Vec::new();
    for r in 0..ROWS {
        for c in 0..COLS {
            grid.push(Cell {
                r,
                c,
                walls: [true, true, true, true],
                visited: false,
            });
        }
    }

    let mut stack = Vec::new();
    let mut current_idx = 0;
    grid[current_idx].visited = true;
    stack.push(current_idx);

    while !stack.is_empty() {
        let current_r = grid[current_idx].r;
        let current_c = grid[current_idx].c;

        let mut neighbors = Vec::new();
        if current_r > 0 {
            let idx = (current_r - 1) * COLS + current_c;
            if !grid[idx].visited { neighbors.push((idx, 0)); }
        }
        if current_c < COLS - 1 {
            let idx = current_r * COLS + (current_c + 1);
            if !grid[idx].visited { neighbors.push((idx, 1)); }
        }
        if current_r < ROWS - 1 {
            let idx = (current_r + 1) * COLS + current_c;
            if !grid[idx].visited { neighbors.push((idx, 2)); }
        }
        if current_c > 0 {
            let idx = current_r * COLS + (current_c - 1);
            if !grid[idx].visited { neighbors.push((idx, 3)); }
        }

        if !neighbors.is_empty() {
            let pick_idx = (rng.next_f64() * neighbors.len() as f64) as usize;
            let (next_idx, dir) = neighbors[pick_idx];

            if dir == 0 {
                grid[current_idx].walls[0] = false;
                grid[next_idx].walls[2] = false;
            } else if dir == 1 {
                grid[current_idx].walls[1] = false;
                grid[next_idx].walls[3] = false;
            } else if dir == 2 {
                grid[current_idx].walls[2] = false;
                grid[next_idx].walls[0] = false;
            } else if dir == 3 {
                grid[current_idx].walls[3] = false;
                grid[next_idx].walls[1] = false;
            }

            current_idx = next_idx;
            grid[current_idx].visited = true;
            stack.push(current_idx);
        } else {
            current_idx = stack.pop().unwrap();
        }
    }
    grid
}

fn build_maze_walls(grid: &[Cell]) -> Vec<Wall> {
    let mut walls = Vec::new();
    let mut wall_set = std::collections::HashSet::new();

    for r in 0..ROWS {
        for c in 0..COLS {
            let cell = &grid[r * COLS + c];
            let x = c as f64 * CELL_W;
            let y = r as f64 * CELL_H;

            if cell.walls[0] {
                let key = format!("h_{}_{}", r, c);
                if wall_set.insert(key) {
                    walls.push(Wall {
                        min_x: x,
                        max_x: x + CELL_W,
                        min_y: y - WALL_THICKNESS / 2.0,
                        max_y: y + WALL_THICKNESS / 2.0,
                    });
                }
            }
            if cell.walls[1] {
                if !(r == ROWS - 1 && c == COLS - 1) {
                    let key = format!("v_{}_{}", r, c + 1);
                    if wall_set.insert(key) {
                        walls.push(Wall {
                            min_x: x + CELL_W - WALL_THICKNESS / 2.0,
                            max_x: x + CELL_W + WALL_THICKNESS / 2.0,
                            min_y: y,
                            max_y: y + CELL_H,
                        });
                    }
                }
            }
            if cell.walls[2] {
                let key = format!("h_{}_{}", r + 1, c);
                if wall_set.insert(key) {
                    walls.push(Wall {
                        min_x: x,
                        max_x: x + CELL_W,
                        min_y: y + CELL_H - WALL_THICKNESS / 2.0,
                        max_y: y + CELL_H + WALL_THICKNESS / 2.0,
                    });
                }
            }
            if cell.walls[3] {
                if !(r == 0 && c == 0) {
                    let key = format!("v_{}_{}", r, c);
                    if wall_set.insert(key) {
                        walls.push(Wall {
                            min_x: x - WALL_THICKNESS / 2.0,
                            max_x: x + WALL_THICKNESS / 2.0,
                            min_y: y,
                            max_y: y + CELL_H,
                        });
                    }
                }
            }
        }
    }
    walls
}

#[derive(Clone, Debug)]
struct PathNode {
    r: usize,
    c: usize,
    path: Vec<(usize, usize)>,
}

fn solve_maze_bfs(grid: &[Cell]) -> Vec<(usize, usize)> {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(PathNode {
        r: 0,
        c: 0,
        path: vec![(0, 0)],
    });
    let mut visited = std::collections::HashSet::new();
    visited.insert("0,0".to_string());

    while let Some(node) = queue.pop_front() {
        if node.r == ROWS - 1 && node.c == COLS - 1 {
            return node.path;
        }

        let cell = &grid[node.r * COLS + node.c];
        let dirs = [
            (-1, 0, 0),
            (0, 1, 1),
            (1, 0, 2),
            (0, -1, 3),
        ];

        for &(dr, dc, wall) in dirs.iter() {
            if !cell.walls[wall] {
                let nr = node.r as i32 + dr;
                let nc = node.c as i32 + dc;
                if nr >= 0 && nr < ROWS as i32 && nc >= 0 && nc < COLS as i32 {
                    let key = format!("{},{}", nr, nc);
                    if visited.insert(key) {
                        let mut next_path = node.path.clone();
                        next_path.push((nr as usize, nc as usize));
                        queue.push_back(PathNode {
                            r: nr as usize,
                            c: nc as usize,
                            path: next_path,
                        });
                    }
                }
            }
        }
    }
    Vec::new()
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn get_checkpoints(solution_path: &[(usize, usize)]) -> Vec<Point> {
    let goal_pos = Point {
        x: (COLS as f64 - 0.5) * CELL_W,
        y: (ROWS as f64 - 0.5) * CELL_H,
    };

    let mut checkpoints = Vec::new();
    if solution_path.len() > 2 {
        let step = solution_path.len() / 4;
        for i in 1..=3 {
            let idx = (i * step).min(solution_path.len() - 1);
            let cp = solution_path[idx];
            checkpoints.push(Point {
                x: (cp.1 as f64 + 0.5) * CELL_W,
                y: (cp.0 as f64 + 0.5) * CELL_H,
            });
        }
    }
    checkpoints.push(goal_pos);
    checkpoints
}

fn line_intersection(
    x1: f64, y1: f64, x2: f64, y2: f64,
    x3: f64, y3: f64, x4: f64, y4: f64,
) -> Option<f64> {
    let den = (x4 - x3) * (y2 - y1) - (y4 - y3) * (x2 - x1);
    if den.abs() < 1e-8 {
        return None;
    }
    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / den;
    let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / den;
    if ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0 {
        Some(ua)
    } else {
        None
    }
}

fn cast_ray(x1: f64, y1: f64, angle: f64, max_dist: f64, walls: &[Wall]) -> f64 {
    let x2 = x1 + angle.cos() * max_dist;
    let y2 = y1 + angle.sin() * max_dist;

    let mut closest_t = 1.0;

    for wall in walls {
        let wx1 = wall.min_x;
        let wx2 = wall.max_x;
        let wy1 = wall.min_y;
        let wy2 = wall.max_y;

        let edges = [
            (wx1, wy1, wx2, wy1),
            (wx2, wy1, wx2, wy2),
            (wx1, wy2, wx2, wy2),
            (wx1, wy1, wx1, wy2),
        ];

        for edge in edges.iter() {
            if let Some(t) = line_intersection(x1, y1, x2, y2, edge.0, edge.1, edge.2, edge.3) {
                if t < closest_t {
                    closest_t = t;
                }
            }
        }
    }
    closest_t
}

struct Brain {
    dna: Vec<f64>,
}

impl Brain {
    fn predict(&self, inputs: &[f64]) -> (f64, f64) {
        let mut hidden = Vec::with_capacity(6);
        let mut dna_idx = 0;
        for _ in 0..6 {
            let mut sum = 0.0;
            for i in 0..5 {
                sum += inputs[i] * self.dna[dna_idx];
                dna_idx += 1;
            }
            sum += self.dna[dna_idx];
            dna_idx += 1;
            hidden.push(sum.tanh());
        }

        let mut outputs = Vec::with_capacity(2);
        for _ in 0..2 {
            let mut sum = 0.0;
            for h in 0..6 {
                sum += hidden[h] * self.dna[dna_idx];
                dna_idx += 1;
            }
            sum += self.dna[dna_idx];
            dna_idx += 1;
            outputs.push(sum.tanh());
        }
        (outputs[0], outputs[1])
    }
}

fn evaluate_agent(dna: &[f64], checkpoints: &[Point], walls: &[Wall]) -> f64 {
    let brain = Brain { dna: dna.to_vec() };
    let mut pos_x = CELL_W / 2.0;
    let mut pos_y = CELL_H / 2.0;
    let mut vel_x = 0.0;
    let mut vel_y = 0.0;
    let mut heading = 0.0;
    let mut checkpoint_idx = 0;
    let mut frames = 0;

    let radius = 8.0;

    while frames < TRIAL_FRAMES {
        frames += 1;

        let angles = [
            heading - std::f64::consts::FRAC_PI_2,
            heading - std::f64::consts::FRAC_PI_4,
            heading,
            heading + std::f64::consts::FRAC_PI_4,
            heading + std::f64::consts::FRAC_PI_2,
        ];

        let mut sensors = [0.0; 5];
        for i in 0..5 {
            sensors[i] = cast_ray(pos_x, pos_y, angles[i], 150.0, walls);
        }

        let target = checkpoints[checkpoint_idx];

        let inputs = [
            sensors[0], sensors[1], sensors[2], sensors[3], sensors[4],
        ];

        let (fx, fy) = brain.predict(&inputs);

        vel_x = vel_x * 0.92 + fx * 0.002;
        vel_y = vel_y * 0.92 + fy * 0.002;

        pos_x += vel_x;
        pos_y += vel_y;

        pos_x = pos_x.clamp(radius, W - radius);
        pos_y = pos_y.clamp(radius, H - radius);

        for wall in walls {
            let cx = pos_x.clamp(wall.min_x, wall.max_x);
            let cy = pos_y.clamp(wall.min_y, wall.max_y);
            let w_dx = pos_x - cx;
            let w_dy = pos_y - cy;
            let dist = (w_dx * w_dx + w_dy * w_dy).sqrt();
            if dist < radius {
                let overlap = radius - dist;
                if dist > 0.0 {
                    pos_x += (w_dx / dist) * overlap;
                    pos_y += (w_dy / dist) * overlap;
                    let nx = w_dx / dist;
                    let ny = w_dy / dist;
                    let dot = vel_x * nx + vel_y * ny;
                    if dot < 0.0 {
                        vel_x -= dot * nx;
                        vel_y -= dot * ny;
                    }
                } else {
                    pos_y -= radius;
                    vel_y = 0.0;
                }
            }
        }

        if vel_x * vel_x + vel_y * vel_y > 0.01 {
            heading = vel_y.atan2(vel_x);
        }

        let dist_target = ((target.x - pos_x).powi(2) + (target.y - pos_y).powi(2)).sqrt();
        if dist_target < 25.0 {
            checkpoint_idx += 1;
            if checkpoint_idx >= checkpoints.len() {
                break;
            }
        }
    }

    let mut fit = checkpoint_idx as f64 * 3000.0;
    if checkpoint_idx < checkpoints.len() {
        let target = checkpoints[checkpoint_idx];
        let dist_target = ((target.x - pos_x).powi(2) + (target.y - pos_y).powi(2)).sqrt();
        fit += (1500.0 - dist_target).max(0.0);
    }
    if checkpoint_idx >= checkpoints.len() {
        fit += 20000.0 + (TRIAL_FRAMES - frames) as f64 * 20.0;
    }
    fit
}

fn mutate_dna(dna: &mut [f64]) {
    let mut rng = rand::rng();
    for gene in dna.iter_mut() {
        if rng.random::<f64>() < MUTATION_RATE {
            *gene += (rng.random::<f64>() - 0.5) * 0.5;
        }
    }
}

fn crossover_dna(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut rng = rand::rng();
    let mut child = vec![0.0; DNA_LENGTH];
    let p1 = (rng.random::<f64>() * DNA_LENGTH as f64) as usize;
    let p2 = (rng.random::<f64>() * DNA_LENGTH as f64) as usize;
    let (start, end) = if p1 > p2 { (p2, p1) } else { (p1, p2) };

    for i in 0..DNA_LENGTH {
        child[i] = if i >= start && i <= end { b[i] } else { a[i] };
    }
    mutate_dna(&mut child);
    child
}

fn train_brain(generations: usize) {
    println!("\nInitializing maze and checkpoints...");
    let mut maze_rng = SeededRng::new(42);
    let grid = generate_maze(&mut maze_rng);
    let walls = build_maze_walls(&grid);
    let solution = solve_maze_bfs(&grid);
    let checkpoints = get_checkpoints(&solution);
    println!("Maze generated! Solution path: {} cells. Checkpoints: {}.", solution.len(), checkpoints.len());

    println!("\nStarting training ({} generations)...", generations);

    let mut population: Vec<Vec<f64>> = (0..POPULATION_SIZE)
        .map(|_| {
            let mut rng = rand::rng();
            (0..DNA_LENGTH)
                .map(|_| (rng.random::<f64>() - 0.5) * 2.0)
                .collect()
        })
        .collect();

    let mut best_fitness_ever = 0.0;
    let mut best_dna_ever = population[0].clone();

    for generation_idx in 1..=generations {
        let fitnesses: Vec<f64> = population
            .par_iter()
            .map(|dna| evaluate_agent(dna, &checkpoints, &walls))
            .collect();

        let mut ranked: Vec<(usize, f64)> = fitnesses.iter().copied().enumerate().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let best_idx = ranked[0].0;
        let best_fit = ranked[0].1;

        if best_fit > best_fitness_ever {
            best_fitness_ever = best_fit;
            best_dna_ever = population[best_idx].clone();
        }

        let percent = (generation_idx as f64 / generations as f64 * 100.0) as usize;
        let bars = generation_idx * 20 / generations;
        let progress_bar: String = std::iter::repeat("=")
            .take(bars)
            .chain(std::iter::repeat(" ").take(20 - bars))
            .collect();

        print!(
            "\r[{}] Generation {}/{} ({}%) | Best Fitness: {:.0} (Record: {:.0})",
            progress_bar, generation_idx, generations, percent, best_fit, best_fitness_ever
        );
        io::stdout().flush().unwrap();

        let mut new_population = Vec::with_capacity(POPULATION_SIZE);
        new_population.push(population[ranked[0].0].clone());
        new_population.push(population[ranked[1].0].clone());

        let pool_size = (POPULATION_SIZE as f64 * 0.3) as usize;

        let mut rng = rand::rng();
        while new_population.len() < POPULATION_SIZE {
            let p1_idx = ranked[rng.random_range(0..pool_size)].0;
            let p2_idx = ranked[rng.random_range(0..pool_size)].0;
            let child = crossover_dna(&population[p1_idx], &population[p2_idx]);
            new_population.push(child);
        }

        population = new_population;
    }

    println!("\n\nTraining finished successfully!");
    println!("Saving best brain weights...");

    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("genesis_data");

    if let Ok(dna_json) = serde_json::to_string(&best_dna_ever) {
        let artifact_path = base.join("artifact.dna");
        if std::fs::write(&artifact_path, &dna_json).is_ok() {
            println!("- Saved raw weights to '{}'", artifact_path.display());
        }
        let js_content = format!("window.importedDNA = {};", dna_json);
        let vis_dir = base.join("visualizer");
        if std::fs::create_dir_all(&vis_dir).is_ok() {
            let dna_js_path = vis_dir.join("dna.js");
            if std::fs::write(&dna_js_path, js_content).is_ok() {
                println!("- Saved brain script to '{}'", dna_js_path.display());
            }
        }
    }
    println!("The trained brain is now ready to be loaded in the web browser.");
}

fn extract_visualizer() {
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("genesis_data");
        
    let vis_dir = base.join("visualizer");
    let _ = std::fs::create_dir_all(&vis_dir);
    let html = include_str!("../visualizer/index.html");
    let css = include_str!("../visualizer/style.css");
    let js = include_str!("../visualizer/app.js");

    let _ = std::fs::write(vis_dir.join("index.html"), html);
    let _ = std::fs::write(vis_dir.join("style.css"), css);
    let _ = std::fs::write(vis_dir.join("app.js"), js);
}

fn update_if_available() {
    println!("Checking for updates from GitHub...");
    let status = match self_update::backends::github::Update::configure()
        .repo_owner("don12335")
        .repo_name("Genesis")
        .bin_name("genesis")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()
    {
        Ok(updater) => updater.update(),
        Err(e) => Err(e),
    };

    match status {
        Ok(self_update::Status::UpToDate(v)) => {
            println!("You are running the latest version (v{}).", v);
        }
        Ok(self_update::Status::Updated(v)) => {
            println!("\nSUCCESS: Genesis updated to version v{}!", v);
            println!("Please close this window and restart the program.");
            std::process::exit(0);
        }
        Err(e) => {
            println!("Update check failed or skipped: {}", e);
        }
    }
}

fn main() {
    update_if_available();
    extract_visualizer();
    loop {
        println!("\n=========================================");
        println!("      GENESIS NEUROEVOLUTION TRAINER      ");
        println!("=========================================");
        println!("This tool runs simulations to train a neural network brain");
        println!("to navigate and escape a 16x10 maze.\n");
        println!("[1] Train Neural Network (High Speed CLI)");
        println!("[2] Open Web Visualizer (index.html)");
        println!("[3] Help / Instructions");
        println!("[4] Exit\n");
        print!("Enter choice (1-4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        let choice = input.trim();

        match choice {
            "1" => {
                println!("\n-----------------------------------------");
                println!("Select Training Intensity:");
                println!("[1] Quick Training (100 generations, ~3s)");
                println!("[2] Standard Training (500 generations, ~15s)");
                println!("[3] Extended Training (1000 generations, ~30s)");
                println!("[4] Custom generations");
                println!("-----------------------------------------");
                print!("Enter choice (1-4): ");
                io::stdout().flush().unwrap();

                let mut intensity_input = String::new();
                io::stdin().read_line(&mut intensity_input).ok();
                let intensity_choice = intensity_input.trim();

                let gens = match intensity_choice {
                    "1" => 100,
                    "2" => 500,
                    "3" => 1000,
                    "4" => {
                        print!("Enter number of generations: ");
                        io::stdout().flush().unwrap();
                        let mut custom_input = String::new();
                        io::stdin().read_line(&mut custom_input).ok();
                        custom_input.trim().parse::<usize>().unwrap_or(500)
                    }
                    _ => 500,
                };
                train_brain(gens);
            }
            "2" => {
                println!("\nOpening web visualizer in browser...");
                #[cfg(target_os = "windows")]
                {
                    let base = std::env::current_exe()
                        .ok()
                        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                        .unwrap_or_else(|| std::env::current_dir().unwrap())
                        .join("genesis_data");
                    let index_path = base.join("visualizer").join("index.html");
                    
                    if std::process::Command::new("cmd")
                        .args(&["/C", "start", "", index_path.to_str().unwrap()])
                        .spawn()
                        .is_ok()
                    {
                        println!("Browser launched successfully!");
                    } else {
                        println!("Error: Could not open browser automatically.");
                        println!("Please manually open 'visualizer/index.html' in your browser.");
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    println!("Please manually open 'visualizer/index.html' in your browser.");
                }
            }
            "3" => {
                println!("\n--- HELP & INSTRUCTIONS ---");
                println!("1. How training works:");
                println!("   The CLI trains a neural network brain (62 weights) to solve the maze.");
                println!("   Raycast sensors measure distances, and the weights are optimized using");
                println!("   genetic algorithms (natural selection).");
                println!("2. Loading the trained brain:");
                println!("   Once training finishes, a file 'visualizer/dna.js' is saved.");
                println!("   When you open 'visualizer/index.html', it will automatically detect");
                println!("   and load this pre-trained brain!");
                println!("3. Running without training:");
                println!("   If you open the web visualizer without training, it will start with");
                println!("   random brains and evolve directly in the browser.");
                println!("\nPress Enter to return to the main menu...");
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).ok();
            }
            "4" => {
                println!("\nExiting Genesis. Goodbye!");
                break;
            }
            _ => {
                println!("\nInvalid choice. Please select a number from 1 to 4.");
            }
        }
    }
}

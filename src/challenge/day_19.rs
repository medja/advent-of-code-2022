pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .map(|line| {
            let blueprint = Blueprint::new(line);
            blueprint.id * score_blueprint(&blueprint, 24)
        })
        .sum::<usize>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input[..3]
        .iter()
        .map(|line| score_blueprint(&Blueprint::new(line), 32))
        .product::<usize>())
}

#[derive(Default, Clone)]
struct Resource {
    count: u8,
    production: u8,
}

#[derive(Copy, Clone)]
struct Robot {
    mask: u8,
    negated_mask: u8,
}

impl Robot {
    const fn new(index: u8) -> Self {
        let mask = 1 << index;
        let negated_mask = !mask;
        Robot { mask, negated_mask }
    }

    const ORE: Self = Robot::new(0);
    const OBSIDIAN: Self = Robot::new(1);
    const CLAY: Self = Robot::new(2);
    const GEODE: Self = Robot::new(3);
}

#[derive(Copy, Clone)]
struct AllowedRobots(u8);

impl AllowedRobots {
    const ALL: AllowedRobots = AllowedRobots(
        Robot::ORE.mask | Robot::OBSIDIAN.mask | Robot::CLAY.mask | Robot::GEODE.mask,
    );

    fn contains(self, robot: &Robot) -> bool {
        self.0 & robot.mask == robot.mask
    }

    fn remove(&mut self, robot: &Robot) {
        self.0 &= robot.negated_mask
    }
}

impl Default for AllowedRobots {
    fn default() -> Self {
        AllowedRobots::ALL
    }
}

#[derive(Default, Clone)]
struct State {
    minute: u8,
    ore: Resource,
    clay: Resource,
    obsidian: Resource,
    geode: Resource,
    allowed_robots: AllowedRobots,
}

impl State {
    fn clone(&self) -> Self {
        let mut state = Clone::clone(self);
        state.allowed_robots = AllowedRobots::ALL;
        state
    }
}

struct Blueprint {
    id: usize,
    ore_robot_ore: u8,
    clay_robot_ore: u8,
    obsidian_robot_ore: u8,
    obsidian_robot_clay: u8,
    geode_robot_ore: u8,
    geode_robot_obsidian: u8,
    max_ore_cost: u8,
}

impl Blueprint {
    fn new(blueprint: &str) -> Self {
        let (id, blueprint) = blueprint[10..].split_once(':').unwrap();
        let (ore_robot_ore, blueprint) = blueprint[22..].split_once(' ').unwrap();
        let (clay_robot_ore, blueprint) = blueprint[27..].split_once(' ').unwrap();
        let (obsidian_robot_ore, blueprint) = blueprint[31..].split_once(' ').unwrap();
        let (obsidian_robot_clay, blueprint) = blueprint[8..].split_once(' ').unwrap();
        let (geode_robot_ore, blueprint) = blueprint[29..].split_once(' ').unwrap();
        let (geode_robot_obsidian, _) = blueprint[8..].split_once(' ').unwrap();

        let mut blueprint = Blueprint {
            id: id.parse().unwrap(),
            ore_robot_ore: ore_robot_ore.parse().unwrap(),
            clay_robot_ore: clay_robot_ore.parse().unwrap(),
            obsidian_robot_ore: obsidian_robot_ore.parse().unwrap(),
            obsidian_robot_clay: obsidian_robot_clay.parse().unwrap(),
            geode_robot_ore: geode_robot_ore.parse().unwrap(),
            geode_robot_obsidian: geode_robot_obsidian.parse().unwrap(),
            max_ore_cost: 0,
        };

        blueprint.max_ore_cost = blueprint
            .ore_robot_ore
            .max(blueprint.clay_robot_ore)
            .max(blueprint.obsidian_robot_ore)
            .max(blueprint.geode_robot_ore);

        blueprint
    }
}

fn score_blueprint(blueprint: &Blueprint, time: u8) -> usize {
    let mut state = State::default();
    state.minute = time;
    state.ore.production = 1;

    simulate(blueprint, state, 0) as usize
}

fn simulate(blueprint: &Blueprint, mut state: State, mut min_score: u8) -> u8 {
    if state.minute == 0 {
        return state.geode.count;
    }

    let can_build_geode_robot = can_build_geode_robot(blueprint, &state);
    let can_build_obsidian_robot = can_build_obsidian_robot(blueprint, &state);
    let can_build_clay_robot = can_build_clay_robot(blueprint, &state);
    let can_build_ore_robot = can_build_ore_robot(blueprint, &state);

    state.minute -= 1;
    state.ore.count += state.ore.production;
    state.clay.count += state.clay.production;
    state.obsidian.count += state.obsidian.production;
    state.geode.count += state.geode.production;

    if compute_max_potential(&state) < min_score as usize {
        return 0;
    }

    let mut max_score = 0;

    if can_build_geode_robot {
        let score = simulate_build_geode_robot(blueprint, state.clone(), min_score);
        max_score = max_score.max(score);
        min_score = min_score.max(score);
        state.allowed_robots.remove(&Robot::GEODE);
    }

    if can_build_obsidian_robot {
        let score = simulate_build_obsidian_robot(blueprint, state.clone(), min_score);
        max_score = max_score.max(score);
        min_score = min_score.max(score);
        state.allowed_robots.remove(&Robot::OBSIDIAN);
    }

    if can_build_clay_robot {
        let score = simulate_build_clay_robot(blueprint, state.clone(), min_score);
        max_score = max_score.max(score);
        min_score = min_score.max(score);
        state.allowed_robots.remove(&Robot::CLAY);
    }

    if can_build_ore_robot {
        let score = simulate_build_ore_robot(blueprint, state.clone(), min_score);
        max_score = max_score.max(score);
        min_score = min_score.max(score);
        state.allowed_robots.remove(&Robot::ORE);
    }

    let score = simulate(blueprint, state, min_score);
    max_score.max(score)
}

fn compute_max_potential(state: &State) -> usize {
    let minute = state.minute as usize;
    let count = state.geode.count as usize;
    let production = state.geode.production as usize;

    count + (minute * (minute - 1) / 2) + (production * (minute.saturating_sub(1) + 1))
}

fn simulate_build_ore_robot(blueprint: &Blueprint, mut state: State, min_score: u8) -> u8 {
    state.ore.count -= blueprint.ore_robot_ore;
    state.ore.production += 1;
    simulate(blueprint, state, min_score)
}

fn simulate_build_clay_robot(blueprint: &Blueprint, mut state: State, min_score: u8) -> u8 {
    state.ore.count -= blueprint.clay_robot_ore;
    state.clay.production += 1;
    simulate(blueprint, state, min_score)
}

fn simulate_build_obsidian_robot(blueprint: &Blueprint, mut state: State, min_score: u8) -> u8 {
    state.ore.count -= blueprint.obsidian_robot_ore;
    state.clay.count -= blueprint.obsidian_robot_clay;
    state.obsidian.production += 1;
    simulate(blueprint, state, min_score)
}

fn simulate_build_geode_robot(blueprint: &Blueprint, mut state: State, min_score: u8) -> u8 {
    state.ore.count -= blueprint.geode_robot_ore;
    state.obsidian.count -= blueprint.geode_robot_obsidian;
    state.geode.production += 1;
    simulate(blueprint, state, min_score)
}

fn can_build_ore_robot(blueprint: &Blueprint, state: &State) -> bool {
    state.allowed_robots.contains(&Robot::ORE)
        && blueprint.ore_robot_ore <= state.ore.count
        && state.ore.production < blueprint.max_ore_cost
}

fn can_build_clay_robot(blueprint: &Blueprint, state: &State) -> bool {
    state.allowed_robots.contains(&Robot::CLAY)
        && blueprint.clay_robot_ore <= state.ore.count
        && state.clay.production < blueprint.obsidian_robot_clay
}

fn can_build_obsidian_robot(blueprint: &Blueprint, state: &State) -> bool {
    state.allowed_robots.contains(&Robot::OBSIDIAN)
        && blueprint.obsidian_robot_ore <= state.ore.count
        && blueprint.obsidian_robot_clay <= state.clay.count
        && state.obsidian.production < blueprint.geode_robot_obsidian
}

fn can_build_geode_robot(blueprint: &Blueprint, state: &State) -> bool {
    state.allowed_robots.contains(&Robot::GEODE)
        && blueprint.geode_robot_ore <= state.ore.count
        && blueprint.geode_robot_obsidian <= state.obsidian.count
}

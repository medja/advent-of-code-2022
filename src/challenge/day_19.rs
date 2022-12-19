pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .map(|line| score_blueprint(line))
        .sum::<usize>())
}

#[derive(Default, Clone)]
struct Resource {
    count: u8,
    production: u8,
}

#[derive(Default, Clone)]
struct State {
    minute: u8,
    ore: Resource,
    clay: Resource,
    obsidian: Resource,
    geode: Resource,
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

fn score_blueprint(blueprint: &str) -> usize {
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

    let mut state = State::default();
    state.ore.production = 1;

    let x = simulate(&blueprint, state) as usize;
    println!(
        "id={}, score={}, result={}",
        blueprint.id,
        x,
        blueprint.id * x
    );
    blueprint.id * x
}

fn simulate(blueprint: &Blueprint, mut state: State) -> u8 {
    if state.minute == 24 {
        return state.geode.count;
    }

    let can_build_geode_robot = can_build_geode_robot(blueprint, &state);
    let can_build_obsidian_robot = can_build_obsidian_robot(blueprint, &state);
    let can_build_clay_robot = can_build_clay_robot(blueprint, &state);
    let can_build_ore_robot = can_build_ore_robot(blueprint, &state);

    state.minute += 1;
    state.ore.count += state.ore.production;
    state.clay.count += state.clay.production;
    state.obsidian.count += state.obsidian.production;
    state.geode.count += state.geode.production;

    let mut max_score = 0;

    if can_build_geode_robot {
        let score = simulate_build_geode_robot(blueprint, state.clone());

        if score > max_score {
            max_score = score;
        }
    }

    if can_build_obsidian_robot {
        let score = simulate_build_obsidian_robot(blueprint, state.clone());

        if score > max_score {
            max_score = score;
        }
    }

    if can_build_clay_robot {
        let score = simulate_build_clay_robot(blueprint, state.clone());

        if score > max_score {
            max_score = score;
        }
    }

    if can_build_ore_robot {
        let score = simulate_build_ore_robot(blueprint, state.clone());

        if score > max_score {
            max_score = score;
        }
    }

    let score = simulate(blueprint, state);

    if score > max_score {
        max_score = score;
    }

    max_score
}

fn simulate_build_ore_robot(blueprint: &Blueprint, mut state: State) -> u8 {
    state.ore.count -= blueprint.ore_robot_ore;
    state.ore.production += 1;
    simulate(blueprint, state)
}

fn simulate_build_clay_robot(blueprint: &Blueprint, mut state: State) -> u8 {
    state.ore.count -= blueprint.clay_robot_ore;
    state.clay.production += 1;
    simulate(blueprint, state)
}

fn simulate_build_obsidian_robot(blueprint: &Blueprint, mut state: State) -> u8 {
    state.ore.count -= blueprint.obsidian_robot_ore;
    state.clay.count -= blueprint.obsidian_robot_clay;
    state.obsidian.production += 1;
    simulate(blueprint, state)
}

fn simulate_build_geode_robot(blueprint: &Blueprint, mut state: State) -> u8 {
    state.ore.count -= blueprint.geode_robot_ore;
    state.obsidian.count -= blueprint.geode_robot_obsidian;
    state.geode.production += 1;
    simulate(blueprint, state)
}

fn can_build_ore_robot(blueprint: &Blueprint, state: &State) -> bool {
    blueprint.ore_robot_ore <= state.ore.count && state.ore.production < blueprint.max_ore_cost
}

fn can_build_clay_robot(blueprint: &Blueprint, state: &State) -> bool {
    blueprint.clay_robot_ore <= state.ore.count
        && state.clay.production < blueprint.obsidian_robot_clay
}

fn can_build_obsidian_robot(blueprint: &Blueprint, state: &State) -> bool {
    blueprint.obsidian_robot_ore <= state.ore.count
        && blueprint.obsidian_robot_clay <= state.clay.count
        && state.obsidian.production < blueprint.geode_robot_obsidian
}

fn can_build_geode_robot(blueprint: &Blueprint, state: &State) -> bool {
    blueprint.geode_robot_ore <= state.ore.count
        && blueprint.geode_robot_obsidian <= state.obsidian.count
}

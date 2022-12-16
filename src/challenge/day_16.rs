use std::cmp::Reverse;
use std::collections::HashMap;

const MAX_LINKS: usize = 5;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let graph = Graph::new(input);
    let solutions = find_solutions(graph.valves.len(), 30, graph.closed_valves, &graph);

    let max_pressure = solutions
        .into_iter()
        .map(|solution| solution.total_pressure)
        .max()
        .unwrap();

    Ok(max_pressure)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let graph = Graph::new(input);

    let mut solutions = find_solutions(graph.valves.len(), 26, graph.closed_valves, &graph);
    solutions.sort_by_key(|solution| Reverse(solution.total_pressure));

    let valves = solutions[0].valves;
    let lower_bound = solutions[1..]
        .iter()
        .position(|solution| !solution.valves.overlaps(valves))
        .unwrap();

    let mut max_pressure = solutions[0].total_pressure + solutions[lower_bound].total_pressure;

    for (index, player) in solutions[1..lower_bound - 1].iter().enumerate() {
        for elephant in solutions[index + 1..lower_bound].iter() {
            if player.valves.overlaps(elephant.valves) {
                continue;
            }

            let total_pressure = player.total_pressure + elephant.total_pressure;

            if total_pressure > max_pressure {
                max_pressure = total_pressure;
            }
        }
    }

    Ok(max_pressure)
}

fn find_solutions(
    id: usize,
    remaining_time: usize,
    closed_valves: ValveSet,
    graphs: &Graph,
) -> Vec<Solution> {
    let state = State {
        id,
        remaining_time,
        closed_valves,
        total_pressure: 0,
    };

    let mut solutions = Vec::with_capacity(220000);
    let mut states = Vec::with_capacity(140000);
    states.push(state);

    while let Some(state) = states.pop() {
        let length = states.len();

        for (id, valve) in graphs.valves.iter().enumerate() {
            if !state.closed_valves.contains(id) {
                continue;
            }

            let effort = graphs.distance(state.id, id) + 1;

            if effort > state.remaining_time {
                continue;
            }

            let remaining_time = state.remaining_time - effort;
            let closed_valves = state.closed_valves.remove(id);
            let total_pressure = state.total_pressure + valve.flow_rate * remaining_time;

            let state = State {
                id,
                remaining_time,
                closed_valves,
                total_pressure,
            };

            states.push(state);
        }

        if length == states.len() {
            let solution = Solution {
                valves: closed_valves.diff(state.closed_valves),
                total_pressure: state.total_pressure,
            };

            solutions.push(solution);
        }
    }

    solutions
}

struct Solution {
    valves: ValveSet,
    total_pressure: usize,
}

struct State {
    id: usize,
    remaining_time: usize,
    closed_valves: ValveSet,
    total_pressure: usize,
}

struct Graph {
    valves: Vec<Valve>,
    distances: Vec<usize>,
    closed_valves: ValveSet,
}

impl Graph {
    fn new(input: &[&str]) -> Self {
        let mut ids = HashMap::new();

        for line in input {
            ids.insert(&line[6..8], ids.len());
        }

        let start = *ids.get("AA").unwrap();

        let all_valves = input
            .iter()
            .map(|line| Valve::new(line, &ids))
            .collect::<Vec<_>>();

        let all_distances = compute_distances(&all_valves);

        // Clear valves with flow_rate = 0 (except the starting valve AA)
        let length = all_valves.len();
        let mut valves = Vec::new();
        let mut original_valve_lookup = Vec::new();

        for (id, valve) in all_valves.into_iter().enumerate() {
            if valve.flow_rate > 0 || id == start {
                valves.push(valve);
                original_valve_lookup.push(id);
            }
        }

        let start = original_valve_lookup
            .iter()
            .position(|id| *id == start)
            .unwrap();
        let last_index = valves.len() - 1;
        valves.swap(start, last_index);
        original_valve_lookup.swap(start, last_index);

        let mut distances = Vec::new();

        for &src in &original_valve_lookup {
            for &dest in &original_valve_lookup {
                distances.push(all_distances[src * length + dest]);
            }
        }

        // Remove starting valve AA
        valves.pop();

        let closed_valves = ValveSet::new((1 << valves.len()) - 1);

        Graph {
            valves,
            distances,
            closed_valves,
        }
    }

    fn distance(&self, source: usize, destination: usize) -> usize {
        // +1 to account for starting valve AA
        self.distances[source * (self.valves.len() + 1) + destination]
    }
}

fn compute_distances(valves: &[Valve]) -> Vec<usize> {
    let mut distances = vec![usize::MAX; valves.len() * valves.len()];

    for i in 0..valves.len() {
        distances[i * valves.len() + i] = 0;
    }

    for (source, valve) in valves.iter().enumerate() {
        for &destination in valve.links() {
            distances[source * valves.len() + destination] = 1;
        }
    }

    for valve in 0..valves.len() {
        for src in 0..valves.len() {
            for dest in 0..valves.len() {
                let index = src * valves.len() + dest;

                let distance = distances[src * valves.len() + valve]
                    .saturating_add(distances[valve * valves.len() + dest]);

                if distance < distances[index] {
                    distances[index] = distance;
                }
            }
        }
    }

    distances
}

struct Valve {
    flow_rate: usize,
    links: [usize; MAX_LINKS],
    link_count: usize,
}

impl Valve {
    fn new(input: &str, ids: &HashMap<&str, usize>) -> Self {
        let (flow_rate, input) = input[23..].split_once(';').unwrap();
        let flow_rate = flow_rate.parse().unwrap();

        let mut links = [0; MAX_LINKS];
        let mut link_count = 0;

        for link in input[23..].trim_start().split(", ") {
            links[link_count] = *ids.get(link).unwrap();
            link_count += 1;
        }

        Valve {
            flow_rate,
            links,
            link_count,
        }
    }

    fn links(&self) -> &[usize] {
        &self.links[..self.link_count]
    }
}

#[derive(Copy, Clone)]
struct ValveSet(u16);

impl ValveSet {
    fn new(mask: u16) -> Self {
        ValveSet(mask)
    }

    fn contains(self, valve: usize) -> bool {
        let mask = 1 << valve;
        self.0 & mask == mask
    }

    fn overlaps(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    fn diff(self, other: Self) -> Self {
        ValveSet(self.0 & !other.0)
    }

    fn remove(self, valve: usize) -> Self {
        ValveSet(self.0 & !(1 << valve))
    }
}

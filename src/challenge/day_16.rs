use std::collections::HashMap;

const MAX_LINKS: usize = 5;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let graph = Graph::new(input);

    let result = best_pressure(
        graph.valves.len(),
        30,
        graph.closed_valves,
        &graph,
    );

    Ok(result)
}

fn best_pressure(
    id: usize,
    remaining_time: usize,
    closed_valves: ValveSet,
    graphs: &Graph,
) -> usize {
    let mut best_score = 0;

    for (link, valve) in graphs.valves.iter().enumerate() {
        if !closed_valves.contains(link) {
            continue;
        }

        let effort = graphs.distance(id, link) + 1;

        if effort > remaining_time {
            continue;
        }

        let remaining_time = remaining_time - effort;
        let closed_valves = closed_valves.remove(link);

        let score = valve.flow_rate * remaining_time
            + best_pressure(link, remaining_time, closed_valves, graphs);

        if score > best_score {
            best_score = score;
        }
    }

    best_score
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

    fn remove(self, valve: usize) -> Self {
        ValveSet(self.0 & !(1 << valve))
    }
}

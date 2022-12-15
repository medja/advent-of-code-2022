use anyhow::{bail, Context};
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    const Y: isize = 2000000;

    let mut ranges = Vec::new();
    let mut beacons = Vec::new();

    for region in parse_regions(input)? {
        let range = match region.range_at_y(Y) {
            Some(range) => range,
            None => continue,
        };

        ranges.push(range);

        if region.beacon_y == Y {
            beacons.push(region.beacon_y);
        }
    }

    ranges.sort_by_key(|range| range.start);
    beacons.sort();
    beacons.dedup();

    let mut count = 0;
    let mut last_end = isize::MIN;

    for range in ranges {
        let start = range.start.max(last_end + 1);
        let end = range.end.max(last_end);

        if start <= end {
            count += end - start + 1;
        }

        last_end = end;
    }

    Ok(count as usize - beacons.len())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    const MAX: isize = 4000000;

    let regions = parse_regions(input)?;
    let mut ranges = Vec::new();

    for y in 0..MAX {
        ranges.clear();
        ranges.extend(regions.iter().filter_map(|region| region.range_at_y(y)));
        ranges.sort_by_key(|range| range.start);

        let mut last_end = 0;

        for range in &ranges {
            let x = last_end + 1;

            if x < range.start {
                return Ok(x * MAX + y);
            }

            last_end = last_end.max(range.end);

            if last_end >= MAX {
                break;
            }
        }

        if last_end < MAX {
            return Ok(MAX * MAX + y);
        }
    }

    bail!("Could not find beacon");
}

fn parse_regions(input: &[&str]) -> anyhow::Result<Vec<Region>> {
    input.iter().map(|line| line.parse()).collect()
}

struct Range {
    start: isize,
    end: isize,
}

struct Region {
    sensor_x: isize,
    sensor_y: isize,
    beacon_x: isize,
    beacon_y: isize,
}

impl Region {
    fn distance(&self) -> usize {
        self.sensor_x.abs_diff(self.beacon_x) + self.sensor_y.abs_diff(self.beacon_y)
    }

    fn range_at_y(&self, y: isize) -> Option<Range> {
        let distance = self.distance();
        let y_to_source_distance = self.sensor_y.abs_diff(y);

        if y_to_source_distance > distance {
            return None;
        }

        let delta = (distance - y_to_source_distance) as isize;

        Some(Range {
            start: self.sensor_x - delta,
            end: self.sensor_x + delta,
        })
    }
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (sensor_x, rest) = input[12..]
            .split_once(',')
            .with_context(|| format!("Could not find sensor x coordinate in: {}", input))?;
        let (sensor_y, rest) = rest[3..]
            .split_once(':')
            .with_context(|| format!("Could not find sensor y coordinate in: {}", input))?;
        let (beacon_x, rest) = rest[24..]
            .split_once(',')
            .with_context(|| format!("Could not find beacon x coordinate in: {}", input))?;
        let beacon_y = &rest[3..];

        let sensor_x = sensor_x.parse::<isize>()?;
        let sensor_y = sensor_y.parse::<isize>()?;
        let beacon_x = beacon_x.parse::<isize>()?;
        let beacon_y = beacon_y.parse::<isize>()?;

        Ok(Region {
            sensor_x,
            sensor_y,
            beacon_x,
            beacon_y,
        })
    }
}

use rayon::prelude::*;
use std::ops::Range;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Conversion {
    source: Range<u64>,
    destination: Range<u64>,
}
impl Conversion {
    fn convert(&self, location: u64) -> Option<u64> {
        let diff = location - self.source.start;
        if self.source.contains(&location) {
            return Some(self.destination.start + diff);
        }
        None
    }
}

#[derive(Debug)]
struct ConversionChart {
    conversions: Vec<Conversion>,
}
impl ConversionChart {
    fn convert(&self, seed: u64) -> u64 {
        let location = seed;
        for conversion in &self.conversions {
            let possible = conversion.convert(location);
            if let Some(final_location) = possible {
                return final_location;
            }
        }
        location
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    charts: Vec<ConversionChart>,
}
impl Almanac {
    fn seeds_to_ranges(&self) -> Vec<Range<u64>> {
        self.seeds
            .chunks(2)
            .map(|pair| pair[0]..pair[1] + pair[0])
            .collect()
    }
    fn min_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| self.seed_to_location(*s))
            .min()
            .unwrap()
        // .collect::<Vec<u64>>()
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        self.charts
            .iter()
            .fold(seed, |loc, chart| chart.convert(loc))
    }

    fn range_locations(&self) -> u64 {
        self.seeds_to_ranges()
            .par_iter()
            .map(|r| {
                dbg!(r);
                r.clone()
                    .map(|s| self.seed_to_location(s))
                    .min()
                    .unwrap_or(100)
            })
            .min()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = parse_input(input);
    Some(almanac.min_location())
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = parse_input(input);
    Some(almanac.range_locations())
}

fn parse_input(input: &str) -> Almanac {
    let mut seeds: Vec<u64> = Vec::new();
    let mut charts: Vec<ConversionChart> = Vec::new();

    for line in input.lines() {
        match line {
            l if l.contains("seeds") => {
                seeds = l
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();
            }
            l if l.contains("map") => {
                charts.push(ConversionChart {
                    conversions: Vec::new(),
                });
            }
            l if l.is_empty() => {}
            l => {
                let numbers: Vec<u64> = l
                    .trim()
                    .split(" ")
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();
                let source = numbers[1];
                let destination = numbers[0];
                let range_len = numbers[2];
                let chart = charts.last_mut().unwrap();

                chart.conversions.push(Conversion {
                    source: source..source + range_len,
                    destination: destination..destination + range_len,
                });
            }
        }
    }
    Almanac { seeds, charts }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}

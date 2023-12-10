use std::collections::HashMap;
use std::ops::Range;
use itertools::Itertools;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Converter {
    from: Material,
    ranges: Vec<RangeMap>
}

enum SplitOptions {
    NotContained(Range<i64>),
    PartiallyContained((Range<i64>, Range<i64>)),
    FullyContained(Range<i64>)
}
#[derive(Debug)]
struct InlineConvert {
    ranges: Vec<RangeMap>
}

impl InlineConvert {
    fn new(mut ranges: Vec<RangeMap>) -> Self {
        ranges.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        Self {
            ranges
        }
    }
    fn is_valid(&self) -> bool{
        let mut prev_end: Option<i64> = None;
        for r in &self.ranges {
            if prev_end.is_none(){
                prev_end = Some(r.range.end);
                continue
            }
            if prev_end.unwrap() != r.range.start {
                println!("No continuous at {prev_end:?}, {:?}", r);
                // return false
            }
            if r.range.contains(&(prev_end.unwrap()-1)) {
                println!("Overlapp at {prev_end:?}, {:?}", r);
            }
            prev_end = Some(r.range.end)
        }
        true
    }
    fn overlapping_ranges(&self, range: &Range<i64>) -> Vec<&RangeMap> {
        let out: Vec<_> = self.ranges.iter().filter(|range_map| {
            range_map.contains(&range.start)
                || range_map.contains(&range.end)
                || range.contains(&range_map.range.start)
                || range.contains(&range_map.range.end)
        }).collect_vec();
        out
    }
    fn convert(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        let overlapping_range_maps = self.overlapping_ranges(range);
        if overlapping_range_maps.is_empty() {
            return vec![range.clone()]
        }
        let mut new_ranges: Vec<Range<i64>> = Vec::new();
        let mut remainder: Range<i64> = range.clone();
        for lapping_range in overlapping_range_maps {
            let offset = lapping_range.offset;
            if lapping_range.range.contains(&remainder.start)
                && lapping_range.range.contains(&(remainder.end-1)) {
                new_ranges.push(remainder.start+offset..remainder.end+offset);
                remainder = 0..0;
                continue
            }
            if lapping_range.range.contains(&remainder.start){
                new_ranges.push(remainder.start+offset..lapping_range.range.end+offset);
                remainder = lapping_range.range.end..remainder.end;
                continue
            }
            if lapping_range.range.contains(&remainder.end) {
                new_ranges.push(lapping_range.range.start + offset..remainder.end + offset);
                remainder = remainder.start..lapping_range.range.start;
                continue
            }
            //     Non continuous range map...
            else{
                new_ranges.push(remainder.start..lapping_range.range.start);
                new_ranges.push(lapping_range.range.start..lapping_range.range.end);
                remainder = lapping_range.range.end..remainder.end;
                continue
            }
        }
        if remainder != (0..0) {
            new_ranges.push(remainder)
        };
        new_ranges
    }
}

#[derive(Debug)]
struct RangeMap {
    range: std::ops::Range<i64>,
    offset: i64,
}
impl RangeMap {
    fn contains(&self, input: &i64) -> bool {
        if self.range.contains(input) {
            return true
        }
       false
    }

    fn convert(&self, input: i64) -> i64 {
        let result = self.offset + (input);
        if result < 0 {
            println!("Result {result}");
        }
        result
    }
}
impl Converter {
    fn convert(&self, input: i64) -> i64 {
        let matched_range = self.ranges.iter().find(|range| range.contains(&input));

        if let Some(matched) = matched_range {
            return matched.convert(input)
        }
        input
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Debug)]
enum Material {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

fn parse_input(input: &str) -> (Vec<i64>, HashMap<Material, Converter>) {
    let (seed_str, remainder) = input.split_once("\n\n").unwrap();

    let (_, seed_str) = seed_str.split_once(": ").unwrap();

    let seeds: Vec<i64> = seed_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut return_map = HashMap::new();

    for converter_str in remainder.split("\n\n") {
        let (header, remainder) = converter_str.split_once('\n').unwrap();

        let mut material: Material = Material::Seed;
        if header == "soil-to-fertilizer map:" {material = Material::Soil}
        if header == "fertilizer-to-water map:" {material = Material::Fertilizer}
        if header == "water-to-light map:" {material = Material::Water}
        if header == "light-to-temperature map:" {material = Material::Light}
        if header == "temperature-to-humidity map:" {material = Material::Temperature}
        if header == "humidity-to-location map:" {material = Material::Humidity}

        let mut ranges: Vec<RangeMap> = Vec::new();

        for range_str in remainder.split('\n') {
            let (to, from, size): (i64, i64, i64) = range_str
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple().unwrap();

            ranges.push(RangeMap{
                range: from..from+size,
                offset: to - from
            })
        }
        let converter = Converter{
            ranges,
            from: material
        };

        return_map.insert(material, converter);

    }

    (seeds, return_map)
}

// fn range_split(r: &std::ops::Range<i64>, split: &i64) -> Option<((std::ops::Range<i64>, std::ops::Range<i64>))> {
//     if r.contains(split) {
//         return Some(((r.start..*split), (*split..r.end)))
//     }
//     None
//
// }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64
}

// impl SeedRange {
//     fn offset(&mut self, offset: i64) {
//         self.start += offset;
//         self.end += offset;
//     }
//
//     fn is_fully_contained(&self, compared: &std::ops::Range<i64>) -> bool {
//         compared.contains(&self.start) && compared.contains(&(self.end))
//     }
//
//     fn is_partially_contained(&self, compared: &std::ops::Range<i64>) -> bool {
//         compared.contains(&self.start) || compared.contains(&(self.end))
//     }
//
//     fn split(&self, split_point: i64) -> (SeedRange, SeedRange) {
//         (
//             SeedRange{
//             start: self.start,
//             end: split_point
//         },
//             SeedRange {
//             start: split_point+1,
//             end: self.end
//         })
//     }
// }

// enum SeedConverterResult {
//     NoMatch(SeedRange),
//     PartialMatch(SeedRange, SeedRange),
//     FullMatch(SeedRange)
// }
//

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, material_map) = parse_input(input);

    let mut seeds = seeds.clone();

    let conversion_path = [Material::Seed, Material::Soil, Material::Fertilizer, Material::Water, Material::Light, Material::Temperature, Material::Humidity];

    for material in conversion_path {
        let converter = material_map.get(&material).unwrap();
        let mut new_seeds = seeds.clone();
        for (idx, seed ) in seeds.into_iter().enumerate() {
            new_seeds[idx] = converter.convert(seed);
        }
        seeds = new_seeds;
    }


    Some(seeds.into_iter().min().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (seed_str, remainder) = input.split_once("\n\n").expect("input");
    let (_,seed_str) = seed_str.split_once(' ').expect("input");
    let seeds: Vec<i64> = seed_str
        .split_whitespace()
        .map(|x| x.parse().expect("Give me number!"))
        .collect_vec();
    let mut seeds = seeds
        .chunks(2)
        .map(|x| x[0]..x[0]+x[1])
        .collect_vec();
    let conversions: Vec<InlineConvert> = remainder.split("\n\n").map(|x| {
        let mut it  = x.lines();
        it.next();
        let ranges: Vec<RangeMap>= it.map(|line|{
            let mut it = line.split_whitespace();
            let dst: i64 = it.next().expect("No Number!").parse().expect("Still No Number!");
            let start: i64 = it.next().expect("No Number!").parse().expect("Still No Number!");
            let len: i64 = it.next().expect("No Number!").parse().expect("Still No Number!");
            RangeMap{
                range: start..start+len,
                offset: dst-start
            }
        }).collect();
        InlineConvert::new(ranges)
    }).collect();
    for converter in conversions{
        let mut new_seeds: Vec<Range<i64>> = Vec::new();
        for seed in &seeds{
            let mut temp = converter.convert(seed);
            new_seeds.append(& mut temp);
        }
        seeds = new_seeds;
    }
    let result = seeds.iter().map(|x| x.start).min();

    result

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

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(27992443));
    }
}

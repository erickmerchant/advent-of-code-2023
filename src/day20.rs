use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

trait Module {
    fn get_destinations(&self) -> Vec<String>;
    fn pulse(&mut self, high: bool, src: String) -> Vec<(bool, String)>;
    fn register(&mut self, _src: String) {}
}

#[derive(Debug, Default)]
struct FlipFlop {
    destinations: Vec<String>,
    on: bool,
}

impl Module for FlipFlop {
    fn get_destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn pulse(&mut self, high: bool, _src: String) -> Vec<(bool, String)> {
        if high {
            return vec![];
        }

        self.on = !self.on;

        self.destinations
            .iter()
            .map(|s| (self.on, s.to_string()))
            .collect()
    }
}

#[derive(Debug, Default)]
struct Conjunction {
    destinations: Vec<String>,
    inputs: HashMap<String, bool>,
}

impl Module for Conjunction {
    fn get_destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn pulse(&mut self, high: bool, src: String) -> Vec<(bool, String)> {
        self.inputs.insert(src, high);

        for high in self.inputs.values() {
            if !high {
                return self
                    .destinations
                    .iter()
                    .map(|k| (true, k.to_owned()))
                    .collect();
            }
        }

        self.destinations
            .iter()
            .map(|k| (false, k.to_owned()))
            .collect()
    }

    fn register(&mut self, src: String) {
        self.inputs.insert(src, false);
    }
}

#[derive(Debug, Default)]
struct Broadcaster {
    destinations: Vec<String>,
}

impl Module for Broadcaster {
    fn get_destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn pulse(&mut self, high: bool, _src: String) -> Vec<(bool, String)> {
        self.destinations
            .iter()
            .map(|k| (high, k.to_owned()))
            .collect()
    }
}

fn parse_modules(input: Vec<String>) -> HashMap<String, Box<dyn Module>> {
    static MODULE_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<type>[%&])?(?P<name>[a-z]+) -> (?P<destinations>.*)")
            .expect("should be a valid regex")
    });
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    for line in input {
        let captures = MODULE_REGEX
            .captures(line.as_str())
            .expect("should be able to capture");
        let destinations = captures["destinations"]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        modules.insert(
            captures["name"].to_string(),
            match captures.name("type").map(|m| m.as_str()) {
                Some("%") => Box::new(FlipFlop {
                    destinations,
                    ..Default::default()
                }),
                Some("&") => Box::new(Conjunction {
                    destinations,
                    ..Default::default()
                }),
                _ => Box::new(Broadcaster { destinations }),
            },
        );
    }

    let mut to_register = vec![];

    for (name, module) in modules.iter_mut() {
        for destination in module.get_destinations() {
            to_register.push((name.to_string(), destination));
        }
    }

    for (src, destination) in to_register {
        if let Some(module) = modules.get_mut(&destination) {
            module.register(src);
        }
    }

    modules
}

pub fn part1(input: Vec<String>) -> usize {
    let mut modules = parse_modules(input);
    let mut highs = 0;
    let mut lows = 0;

    for _i in 0..1000 {
        lows += 1;

        let broadcaster = modules.get("broadcaster").expect("should have broadcaster");
        let mut queue = broadcaster
            .get_destinations()
            .iter()
            .map(|s| (false, s.to_string(), "button".to_string()))
            .collect::<Vec<_>>();

        loop {
            if queue.is_empty() {
                break;
            }

            let mut new_queue = vec![];

            for (high, dest, src) in queue {
                if high {
                    highs += 1;
                } else {
                    lows += 1;
                }

                if let Some(module) = modules.get_mut(&dest) {
                    new_queue.extend(
                        module
                            .pulse(high, src)
                            .iter()
                            .map(|(h, n)| (h.to_owned(), n.to_owned(), dest.to_owned()))
                            .collect::<Vec<_>>(),
                    );
                }
            }

            queue = new_queue;
        }
    }

    highs * lows
}

pub fn part2(input: Vec<String>) -> usize {
    let mut modules = parse_modules(input);
    let mut results = HashMap::new();

    'o: for i in 1.. {
        let broadcaster = modules.get("broadcaster").expect("should have broadcaster");
        let mut queue = broadcaster
            .get_destinations()
            .iter()
            .map(|s| (false, s.to_string(), "button".to_string()))
            .collect::<Vec<_>>();
        let (penultimate, _) = modules
            .iter()
            .find(|(_, module)| module.get_destinations().contains(&"rx".to_string()))
            .expect("should have penultimate");
        let penultimate_src_len = modules
            .iter()
            .filter(|(_, module)| module.get_destinations().contains(&penultimate.to_string()))
            .collect::<Vec<_>>()
            .len();

        loop {
            if queue.is_empty() {
                break;
            }

            if let Some((_h, _dest, src)) = queue.iter().find(|(h, dest, _src)| *h && dest == "zh")
            {
                if !results.contains_key(src) {
                    results.insert(src.to_string(), i);
                }
            }

            if results.len() == penultimate_src_len {
                break 'o;
            }

            let mut new_queue = vec![];

            for (high, dest, src) in queue {
                if let Some(module) = modules.get_mut(&dest) {
                    new_queue.extend(
                        module
                            .pulse(high, src)
                            .iter()
                            .map(|(h, n)| (h.to_owned(), n.to_owned(), dest.to_owned()))
                            .collect::<Vec<_>>(),
                    );
                }
            }

            queue = new_queue;
        }
    }

    results.values().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        r"broadcaster -> a, b, c
          %a -> b
          %b -> c
          %c -> inv
          &inv -> a"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 32000000);
    }
}

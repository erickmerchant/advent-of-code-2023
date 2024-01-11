use core::panic;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> usize {
    let mut workflows = WorkflowMap::new();
    let mut items = Vec::<Item>::new();
    let mut is_workflow = true;

    for line in input {
        if line.is_empty() {
            is_workflow = false;
            continue;
        }

        if is_workflow {
            let workflow = Workflow::parse(line);

            workflows.insert(workflow.key.clone(), workflow);
        } else {
            items.push(Item::parse(&line));
        }
    }

    let result = items
        .par_iter()
        .map(|item| {
            let mut current_key = "in";
            let mut visited = HashSet::new();

            visited.insert(current_key);

            loop {
                let workflow = workflows
                    .get(current_key)
                    .expect("should be a valid workflow");
                let last_rule = &workflow.rules.last().expect("should have a final rule");

                let mut next_key = if last_rule.condition.is_none() {
                    last_rule.destination.as_str()
                } else {
                    panic!("should be a default rule")
                };

                for rule in &workflow.rules {
                    match rule.condition.clone() {
                        None => continue,
                        Some(Condition::GreaterThan(attribute, value)) => {
                            let matching = item.get(attribute.to_owned()) > value;

                            if matching {
                                next_key = rule.destination.as_str();
                                break;
                            }
                        }
                        Some(Condition::LessThan(attribute, value)) => {
                            let matching = item.get(attribute.to_owned()) < value;

                            if matching {
                                next_key = rule.destination.as_str();
                                break;
                            }
                        }
                    }
                }

                if visited.contains(&next_key) {
                    break;
                }

                if next_key == "R" {
                    return 0;
                }

                if next_key == "A" {
                    return item.sum();
                }

                visited.insert(next_key);

                current_key = next_key;
            }

            0
        })
        .sum::<u128>();

    result as usize
}

pub fn part2(input: Vec<String>) -> usize {
    let mut workflows = WorkflowMap::new();
    let mut destination_map = HashMap::new();
    let mut accepted_list = vec![];

    for line in input {
        if line.is_empty() {
            break;
        }

        let workflow = Workflow::parse(line);

        for (i, rule) in (0..).zip(workflow.clone().rules) {
            let desitination = rule.destination;

            if desitination == "A" {
                accepted_list.push((workflow.key.clone(), i));
            } else if desitination != "R" {
                destination_map.insert(desitination.clone(), (workflow.key.clone(), i));
            }
        }

        workflows.insert(workflow.key.clone(), workflow.clone());
    }

    let result = accepted_list
        .par_iter()
        .map(|(key, index)| {
            let mut range_map = RangeMap::default();
            let mut current_key = key.as_str();
            let mut current_index = *index;
            let mut visited = HashSet::new();

            visited.insert(current_key);

            loop {
                let workflow = workflows
                    .get(current_key)
                    .expect("should be a valid workflow");

                match workflow.rules[current_index].condition.clone() {
                    Some(Condition::GreaterThan(attribute, value)) => {
                        range_map.set_greater_than(&attribute, value + 1);
                    }
                    Some(Condition::LessThan(attribute, value)) => {
                        range_map.set_less_than(&attribute, value - 1);
                    }
                    _ => (),
                }

                for i in (0..current_index.to_owned()).rev() {
                    let rule = workflow.rules[i].clone();

                    match rule.condition {
                        Some(Condition::GreaterThan(attribute, value)) => {
                            range_map.set_less_than(&attribute, value);
                        }
                        Some(Condition::LessThan(attribute, value)) => {
                            range_map.set_greater_than(&attribute, value);
                        }
                        _ => (),
                    }
                }

                if workflow.key == "in" {
                    break;
                }

                let next = destination_map
                    .get(workflow.key.as_str())
                    .expect("should have a valid next");

                current_key = next.0.as_str();
                current_index = next.1;

                if visited.contains(&current_key) {
                    break;
                }

                visited.insert(current_key);
            }

            range_map.sum()
        })
        .sum::<u128>();

    result as usize
}

type WorkflowMap = HashMap<Key, Workflow>;

type Key = String;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Workflow {
    key: Key,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: String) -> Self {
        static WORKFLOW_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?P<key>[a-zA-Z]+)\{(?P<rules>.*?),(?P<default_destination>[a-zA-Z]+)\}")
                .expect("should be a valid regex")
        });

        let captures = WORKFLOW_REGEX
            .captures(line.as_str())
            .expect("should be able to capture");
        let mut rules = captures["rules"]
            .split(',')
            .map(Rule::parse_normal)
            .collect::<Vec<Rule>>();

        rules.push(Rule {
            condition: None,
            destination: captures["default_destination"].to_string(),
        });

        Self {
            key: captures["key"].to_string(),
            rules,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Condition {
    LessThan(Attribute, u128),
    GreaterThan(Attribute, u128),
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    destination: Key,
}

impl Rule {
    fn parse_normal(line: &str) -> Self {
        static RULE_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?P<attribute>[xmas])(?P<operator>[<>])(?P<value>\d+):(?P<destination>[a-zA-Z]+)",
            )
            .expect("should be a valid regex")
        });

        let captures = RULE_REGEX
            .captures(line)
            .expect("should be able to capture");
        let attribute = Attribute::parse(captures["attribute"].to_string());
        let operator = captures["operator"].to_string();
        let value = captures["value"]
            .parse::<u128>()
            .expect("should be a valid number");

        Self {
            destination: captures["destination"].to_string(),
            condition: match operator.as_str() {
                ">" => Some(Condition::GreaterThan(attribute, value)),
                "<" => Some(Condition::LessThan(attribute, value)),
                _ => panic!("should be a valid operator"),
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Attribute {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Attribute {
    fn parse(letter: String) -> Self {
        match letter.as_str() {
            "x" => Self::ExtremelyCool,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => panic!("should be a valid attribute"),
        }
    }
}

#[derive(Debug, Default)]
struct Item {
    x: u128,
    m: u128,
    a: u128,
    s: u128,
}

impl Item {
    fn get(&self, attribute: Attribute) -> u128 {
        match attribute {
            Attribute::ExtremelyCool => self.x,
            Attribute::Musical => self.m,
            Attribute::Aerodynamic => self.a,
            Attribute::Shiny => self.s,
        }
    }

    fn set(&mut self, attribute: Attribute, value: u128) {
        match attribute {
            Attribute::ExtremelyCool => self.x = value,
            Attribute::Musical => self.m = value,
            Attribute::Aerodynamic => self.a = value,
            Attribute::Shiny => self.s = value,
        }
    }

    fn sum(&self) -> u128 {
        self.x + self.m + self.a + self.s
    }

    fn parse(line: &str) -> Self {
        static ATTRIBUTE_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?P<attribute>[xmas])=(?P<value>\d+)").expect("should be a valid regex")
        });

        let mut item = Self::default();

        for (_, [attribute, value]) in ATTRIBUTE_REGEX.captures_iter(line).map(|c| c.extract()) {
            let attribute = Attribute::parse(attribute.to_string());
            let value = value.parse::<u128>().expect("should be a valid number");

            item.set(attribute, value);
        }

        item
    }
}

#[derive(Debug)]
struct RangeMap {
    x: (u128, u128),
    m: (u128, u128),
    a: (u128, u128),
    s: (u128, u128),
}

impl Default for RangeMap {
    fn default() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

impl RangeMap {
    fn get_mut(&mut self, attribute: &Attribute) -> &mut (u128, u128) {
        match attribute {
            Attribute::ExtremelyCool => &mut self.x,
            Attribute::Musical => &mut self.m,
            Attribute::Aerodynamic => &mut self.a,
            Attribute::Shiny => &mut self.s,
        }
    }

    fn set_greater_than(&mut self, attribute: &Attribute, value: u128) {
        let range = self.get_mut(attribute);

        if value > range.0 {
            range.0 = value
        }
    }

    fn set_less_than(&mut self, attribute: &Attribute, value: u128) {
        let range = self.get_mut(attribute);

        if value < range.1 {
            range.1 = value
        }
    }

    fn sum(&self) -> u128 {
        let mut result = 1;

        for range in [self.x, self.m, self.a, self.s] {
            if range.1 < range.0 {
                result *= 0;
            } else {
                result *= range.1 - range.0 + 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        r"px{a<2006:qkq,m>2090:A,rfg}
          pv{a>1716:R,A}
          lnx{m>1548:A,A}
          rfg{s<537:gd,x>2440:R,A}
          qs{s>3448:A,lnx}
          qkq{x<1416:A,crn}
          crn{x>2662:A,R}
          in{s<1351:px,qqz}
          qqz{s>2770:qs,m<1801:hdj,R}
          gd{a>3333:R,R}
          hdj{m>838:A,pv}

          {x=787,m=2655,a=1222,s=2876}
          {x=1679,m=44,a=2067,s=496}
          {x=2036,m=264,a=79,s=2244}
          {x=2461,m=1339,a=466,s=291}
          {x=2127,m=1623,a=2188,s=1013}"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 167409079868000);
    }
}

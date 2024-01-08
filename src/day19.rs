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

                let mut next_key = match last_rule {
                    Rule::Default(key) => key.as_str(),
                    _ => panic!("should be a default rule"),
                };

                for rule in &workflow.rules {
                    match rule {
                        Rule::Default(_) => continue,
                        Rule::Normal(attribute, operator, target_value, destination) => {
                            let value = item
                                .attributes
                                .get(attribute)
                                .expect("should have attribute");

                            let matching = match operator {
                                Operator::LessThan => value < target_value,
                                Operator::GreaterThan => value > target_value,
                            };

                            if matching {
                                next_key = destination.as_str();
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
                    return item.attributes.values().sum::<u128>();
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

        for (i, rule) in workflow.rules.iter().enumerate() {
            let desitination = match rule {
                Rule::Default(destination) => destination,
                Rule::Normal(_, _, _, destination) => destination,
            };

            if desitination == "A" {
                accepted_list.push((workflow.key.clone(), i));
            } else if desitination != "R" {
                destination_map.insert(desitination.clone(), (workflow.key.clone(), i));
            }
        }

        workflows.insert(workflow.key.clone(), workflow.clone());
    }

    let result = accepted_list
        .iter()
        .map(|(key, index)| {
            let mut range_map = RangeMap::new();

            range_map.insert(Attribute::ExtremelyCool, Range { min: 1, max: 4000 });
            range_map.insert(Attribute::Musical, Range { min: 1, max: 4000 });
            range_map.insert(Attribute::Aerodynamic, Range { min: 1, max: 4000 });
            range_map.insert(Attribute::Shiny, Range { min: 1, max: 4000 });

            let mut current_key = key.as_str();
            let mut current_index = *index;
            let mut visited = HashSet::new();

            visited.insert(current_key);

            loop {
                let workflow = workflows
                    .get(current_key)
                    .expect("should be a valid workflow");

                if let Rule::Normal(attribute, operator, value, _) =
                    workflow.rules[current_index].clone()
                {
                    let range = range_map.get_mut(&attribute).expect("should have entry");

                    match operator {
                        Operator::GreaterThan => {
                            if value + 1 > range.min {
                                range.min = value + 1
                            }
                        }
                        Operator::LessThan => {
                            if value - 1 < range.max {
                                range.max = value - 1
                            }
                        }
                    }
                };

                for i in (0..current_index.to_owned()).rev() {
                    let rule = workflow.rules[i].clone();

                    if let Rule::Normal(attribute, operator, value, _) = rule {
                        let range = range_map.get_mut(&attribute).expect("should have entry");

                        match operator {
                            Operator::GreaterThan => {
                                if value < range.max {
                                    range.max = value
                                }
                            }
                            Operator::LessThan => {
                                if value > range.min {
                                    range.min = value
                                }
                            }
                        }
                    };
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

            let mut result = 1;

            for range in range_map.values() {
                if range.max < range.min {
                    result *= 0;
                } else {
                    result *= range.max - range.min + 1;
                }
            }

            result
        })
        .sum::<u128>();

    result as usize
}

type WorkflowMap = HashMap<Key, Workflow>;

type Key = String;

type RangeMap = HashMap<Attribute, Range>;

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

        rules.push(Rule::Default(captures["default_destination"].to_string()));

        Self {
            key: captures["key"].to_string(),
            rules,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Rule {
    Normal(Attribute, Operator, u128, Key),
    Default(Key),
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
        let operator = Operator::parse(captures["operator"].to_string());
        let value = captures["value"]
            .parse::<u128>()
            .expect("should be a valid number");

        Self::Normal(
            attribute,
            operator,
            value,
            captures["destination"].to_string(),
        )
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl Operator {
    fn parse(letter: String) -> Self {
        match letter.as_str() {
            "<" => Self::LessThan,
            ">" => Self::GreaterThan,
            _ => panic!("should be a valid operator"),
        }
    }
}

#[derive(Debug)]
struct Item {
    attributes: HashMap<Attribute, u128>,
}

#[derive(Debug)]
struct Range {
    min: u128,
    max: u128,
}

impl Item {
    fn parse(line: &str) -> Self {
        static ATTRIBUTE_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?P<attribute>[xmas])=(?P<value>\d+)").expect("should be a valid regex")
        });

        let mut attributes = HashMap::new();

        for (_, [attribute, value]) in ATTRIBUTE_REGEX.captures_iter(line).map(|c| c.extract()) {
            let attribute = Attribute::parse(attribute.to_string());
            let value = value.parse::<u128>().expect("should be a valid number");

            attributes.insert(attribute, value);
        }

        Self { attributes }
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

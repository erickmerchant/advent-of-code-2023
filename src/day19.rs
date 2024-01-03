use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> usize {
    let mut workflows = vec![];
    let mut items = vec![];
    let mut is_workflow = true;

    for line in input {
        if line.is_empty() {
            is_workflow = false;
            continue;
        }

        if is_workflow {
            workflows.push(line);
        } else {
            items.push(line);
        }
    }

    println!("{:?}", workflows.len());
    println!("{:?}", items.len());

    0
}

pub fn part2(_input: Vec<String>) -> usize {
    0
}

type WorkflowMap = HashMap<Key, Workflow>;

#[derive(Default, PartialEq, Eq, Debug)]
struct Workflow {
    conditions: Vec<Condition>,
}

#[derive(PartialEq, Eq, Debug)]
struct Condition {
    attribute: Attribute,
    operator: Option<Operator>,
    destination: Key,
}

#[derive(PartialEq, Eq, Debug)]
enum Attribute {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(PartialEq, Eq, Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(PartialEq, Eq, Debug)]
struct Key {
    name: String,
}

struct Item {
    attributes: HashMap<Attribute, u128>,
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
        assert_eq!(part2(get_fixture()), 0);
    }
}

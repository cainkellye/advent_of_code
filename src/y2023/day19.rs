#![allow(unused)]
use std::collections::HashMap;

use super::*;

pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input19.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input19.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let (workflows, machine_parts) = parse_input(input_file);
    let workflow_in = workflows.get("in").unwrap();
    machine_parts
        .into_iter()
        .map(|part| grade_part(part, workflow_in, &workflows))
        .sum()
}

fn grade_part<'a>(
    part: MachinePart,
    mut next_workflow: &'a Workflow,
    workflows: &'a HashMap<String, Workflow>,
) -> usize {
    loop {
        let result = next_workflow.run(&part);
        match result {
            EvalResult::Workflow(name) => next_workflow = workflows.get(&name).unwrap(),
            EvalResult::Accepted => return part.rating(),
            EvalResult::Rejected => return 0,
            EvalResult::None => unreachable!(),
        }
    }
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

fn parse_input(input_file: &str) -> (HashMap<String, Workflow>, Vec<MachinePart>) {
    let mut input = iter_lines_from(input_file);
    let mut workflows = HashMap::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }
        let (k, v) = Workflow::parse(&line);
        workflows.insert(k, v);
    }
    let machine_parts = input.map(|line| line.as_str().into()).collect();
    (workflows, machine_parts)
}

struct MachinePart([usize; 4]);
impl From<&str> for MachinePart {
    fn from(value: &str) -> Self {
        MachinePart(
            value[1..value.len() - 1]
                .split(',')
                .map(|xx| xx.split_once('=').unwrap().1.parse::<usize>().unwrap())
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}
impl MachinePart {
    fn rating(&self) -> usize {
        self.0.iter().map(|val| *val as usize).sum()
    }
}

enum EvalResult {
    Workflow(String),
    Accepted,
    Rejected,
    None,
}

impl From<&str> for EvalResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => EvalResult::Accepted,
            "R" => EvalResult::Rejected,
            name => EvalResult::Workflow(name.into()),
        }
    }
}

enum Rule {
    Lt(usize, usize, String),
    Gt(usize, usize, String),
    NoOp(String),
}
impl Rule {
    fn evaluate(&self, part: &MachinePart) -> EvalResult {
        match self {
            Rule::Lt(id, value, result) => {
                if part.0[*id] < *value {
                    result.as_str().into()
                } else {
                    EvalResult::None
                }
            }
            Rule::Gt(id, value, result) => {
                if part.0[*id] > *value {
                    result.as_str().into()
                } else {
                    EvalResult::None
                }
            }
            Rule::NoOp(result) => result.as_str().into(),
        }
    }
}

const CAT: [u8; 4] = *b"xmas";

impl From<&str> for Rule {
    fn from(line: &str) -> Self {
        // a<2136:A
        if let Some((condition, workflow_name)) = line.split_once(':') {
            let id = CAT
                .iter()
                .position(|b| b == &condition.as_bytes()[0])
                .unwrap();
            let value = condition[2..].parse::<usize>().unwrap();
            if condition.as_bytes()[1] == b'<' {
                Rule::Lt(id, value, workflow_name.into())
            } else {
                Rule::Gt(id, value, workflow_name.into())
            }
        } else {
            Rule::NoOp(line.into())
        }
    }
}
struct Workflow {
    rules: Vec<Rule>,
}
impl Workflow {
    fn parse(line: &str) -> (String, Self) {
        // bfn{m<3889:A,a<2136:A,s>1544:R,A}
        let (name, rest) = line.split_once('{').unwrap();
        let mut workflow = Workflow { rules: vec![] };
        for rule in rest[..rest.len() - 1].split(',') {
            workflow.rules.push(rule.into());
        }
        (name.into(), workflow)
    }
    fn run(&self, part: &MachinePart) -> EvalResult {
        self.rules
            .iter()
            .map(|rule| rule.evaluate(part))
            .find(|eval| !matches!(eval, EvalResult::None))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test19.txt"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test19.txt"), 0);
    }
}

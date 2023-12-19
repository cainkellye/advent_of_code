use super::*;
use std::collections::HashMap;

pub(super) fn part2_internal(input_file: &str) -> isize {
    let workflows = parse_input(input_file);
    workflows["in"].count_accepted(
        MachinePartRange([(1, 4000), (1, 4000), (1, 4000), (1, 4000)]),
        &workflows,
    ) // 136146366355609
}

fn parse_input(input_file: &str) -> HashMap<String, Workflow> {
    let mut input = iter_lines_from(input_file);
    let mut workflows = HashMap::new();
    for line in input.by_ref() {
        if line.is_empty() {
            break;
        }
        let (k, v) = Workflow::parse(&line);
        workflows.insert(k, v);
    }
    workflows
}

#[derive(Clone, Copy, Default)]
struct MachinePartRange([(isize, isize); 4]);
impl MachinePartRange {
    fn product(&self) -> isize {
        self.0
            .iter()
            .map(|(start, end)| (end - start + 1).max(0))
            .product()
    }
}

enum Rule {
    Lt(usize, isize, String),
    Gt(usize, isize, String),
    NoOp(String),
}
impl Rule {
    fn count_accepted(
        &self,
        mut part: MachinePartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> (isize, MachinePartRange) {
        match self {
            &Rule::Lt(id, value, ref result) => {
                let (start, end) = part.0[id];
                let (matched_start, matched_end) = (start, end.min(value - 1));
                let mut matched = part;
                matched.0[id] = (matched_start, matched_end);
                part.0[id] = (start.max(value), end);
                if result == "A" {
                    (matched.product(), part)
                } else if result == "R" {
                    (0, part)
                } else {
                    (workflows[result].count_accepted(matched, workflows), part)
                }
            }
            &Rule::Gt(id, value, ref result) => {
                let (start, end) = part.0[id];
                let (matched_start, matched_end) = (start.max(value + 1), end);
                let mut matched = part;
                matched.0[id] = (matched_start, matched_end);
                part.0[id] = (start, end.min(value));
                if result == "A" {
                    (matched.product(), part)
                } else if result == "R" {
                    (0, part)
                } else {
                    (workflows[result].count_accepted(matched, workflows), part)
                }
            }
            &Rule::NoOp(ref result) => {
                if result == "A" {
                    (part.product(), Default::default())
                } else if result != "R" {
                    (
                        workflows[result].count_accepted(part, workflows),
                        Default::default(),
                    )
                } else {
                    (0, Default::default())
                }
            }
        }
    }
}
impl From<&str> for Rule {
    fn from(line: &str) -> Self {
        // a<2136:A
        if let Some((condition, workflow_name)) = line.split_once(':') {
            let id = b"xmas"
                .iter()
                .position(|b| b == &condition.as_bytes()[0])
                .unwrap();
            let value = condition[2..].parse::<isize>().unwrap();
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
    fn count_accepted(
        &self,
        mut part: MachinePartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> isize {
        let mut count = 0;
        for rule in &self.rules {
            let (c, p) = rule.count_accepted(part, workflows);
            count += c;
            part = p;
        }
        count
    }
}

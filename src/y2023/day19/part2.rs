use super::*;

pub(super) fn part2_internal(input_file: &str) -> isize {
    let workflows = parse_input(input_file);
    workflows["in"].count_accepted(
        MachinePartRange([(1, 4000), (1, 4000), (1, 4000), (1, 4000)]),
        &workflows,
    ) // ? < 494144000000236
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

#[derive(Clone, Copy)]
struct MachinePartRange([(isize, isize); 4]);
impl Default for MachinePartRange {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl MachinePartRange {
    fn product(&self) -> isize {
        self.0
            .iter()
            .map(|(start, end)| (end - start + 1).max(0))
            .product()
    }
    fn product_except(&self, id: usize) -> isize {
        self.0
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != id)
            .map(|(_, (start, end))| (end - start + 1).max(0))
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
            Rule::Lt(id, value, result) => {
                let (start, end) = part.0[*id];
                let (matched_start, matched_end) = (start, end.min(value - 1));
                if result == "A" {
                    ((matched_end - matched_start + 1).max(0) * part.product_except(*id), part)
                } else if result != "R" {
                    (workflows[result].count_accepted(part, workflows), MachinePartRange::default())
                } else {
                    part.0[*id] = (start.max(*value), end);
                    (0, part)
                }
            }
            Rule::Gt(id, value, result) => {
                let (start, end) = part.0[*id];
                let (matched_start, matched_end) = (start.max(value + 1), end);
                if result == "A" {
                    ((matched_end - matched_start + 1).max(0) * part.product_except(*id), part)
                } else if result != "R" {
                    (workflows[result].count_accepted(part, workflows), MachinePartRange::default())
                } else {
                    part.0[*id] = (start, end.min(*value));
                    (0, part)
                }
            }
            Rule::NoOp(result) => {
                if result == "A" {
                    (part.product(), MachinePartRange::default())
                } else if result != "R" {
                    (workflows[result].count_accepted(part, workflows), MachinePartRange::default())
                } else {
                    (0, MachinePartRange::default())
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

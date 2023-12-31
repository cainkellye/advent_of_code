use super::*;
use num::Integer;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input20.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input20.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let mut modules = parse_input(input_file);
    set_module_inputs(&mut modules);
    let mut message_bus = MessageQueue::new();
    for _ in 1..=1000 {
        message_bus.push_the_button(&mut modules);
    }
    println!(
        "low: {}, high: {}",
        message_bus.low_count, message_bus.high_count
    );
    message_bus.low_count * message_bus.high_count
}

fn part2_internal(input_file: &str) -> usize {
    let mut modules = parse_input(input_file);
    set_module_inputs(&mut modules);
    let mut message_queue = MessageQueue::new();

    // Get the conjunction that signals rx
    let module_before_rx = get_module_inputs(name_to_id("rx"), &modules)[0];

    // Get the conjunctions that signal the one before rx
    let modules_to_watch = get_module_inputs(module_before_rx, &modules);

    // Search for the first high message from the modules we watch
    let first_high_message = Rc::new(RefCell::new(vec![0; modules_to_watch.len()]));

    message_queue.high_callback = Box::new({
        let first_high_message = first_high_message.clone();
        move |id, pushes| {
            if let Some(idx) = modules_to_watch.iter().position(|&watched| id == watched) {
                // High message is being sent by a watched module
                // Store it if the first is still at 0:
                if first_high_message.borrow()[idx] == 0 {
                    first_high_message.borrow_mut()[idx] = pushes;
                }
            }
        }
    });
    loop {
        message_queue.push_the_button(&mut modules);
        let have_all_first_high = first_high_message.borrow().iter().all(|&push| push > 0);
        if have_all_first_high {
            return first_high_message
                .borrow()
                .iter()
                .cloned()
                .reduce(|a, b| a.lcm(&b))
                .unwrap();
        }
    }
}

type ModuleStore = HashMap<usize, Box<dyn Module>>;

fn get_module_inputs(id: usize, modules: &ModuleStore) -> Vec<usize> {
    modules
        .iter()
        .filter(|(_, module)| module.connections_contains(&id))
        .map(|(id, _)| id)
        .cloned()
        .collect_vec()
}

fn set_module_inputs(modules: &mut ModuleStore) {
    for id in modules.keys().copied().collect_vec() {
        let inputs = get_module_inputs(id, modules);
        modules.get_mut(&id).unwrap().set_inputs(inputs);
    }
}

fn parse_input(input_file: &str) -> ModuleStore {
    let mut modules: ModuleStore = HashMap::new();
    // %gt -> dk, pl
    // &hm -> kl, gh, tl, xx, zq
    for line in iter_lines_from(input_file) {
        let (module, connections) = line.split_once(" -> ").unwrap();
        let connections = connections.split(", ").map(name_to_id).collect_vec();
        if let Some(name) = module.strip_prefix('%') {
            let id = name_to_id(name);
            modules.insert(name_to_id(name), Box::new(FlipFlop::new(id, connections)));
        } else if let Some(name) = module.strip_prefix('&') {
            let id = name_to_id(name);
            modules.insert(
                name_to_id(name),
                Box::new(Conjunction::new(id, connections)),
            );
        } else if module == "broadcaster" {
            modules.insert(0, Box::new(Broadcaster::new(0, connections)));
        }
    }
    modules
}

/// Convert string module names to integer ids
fn name_to_id(name: &str) -> usize {
    let mut id = 0;
    for (exp, byte) in name.bytes().enumerate() {
        id += (byte as usize) * 256_usize.pow(exp as u32);
    }
    id
}

trait Module {
    fn receive(&mut self, message: Message, bus: &mut MessageQueue);
    fn set_inputs(&mut self, inputs: Vec<usize>);
    fn connections_contains(&self, id: &usize) -> bool;
}

struct FlipFlop {
    id: usize,
    connections: Vec<usize>,
    is_on: bool,
}
impl FlipFlop {
    fn new(id: usize, connections: Vec<usize>) -> Self {
        Self {
            id,
            connections,
            is_on: false,
        }
    }
}
impl Module for FlipFlop {
    fn receive(&mut self, message: Message, bus: &mut MessageQueue) {
        if !message.is_high {
            self.is_on ^= true;
            bus.send(self.id, self.is_on, &self.connections)
        }
    }
    fn set_inputs(&mut self, _: Vec<usize>) {}
    fn connections_contains(&self, id: &usize) -> bool {
        self.connections.contains(id)
    }
}

struct Conjunction {
    id: usize,
    connections: Vec<usize>,
    memory: HashMap<usize, bool>,
}
impl Conjunction {
    fn new(id: usize, connections: Vec<usize>) -> Self {
        Self {
            id,
            connections,
            memory: HashMap::new(),
        }
    }
}
impl Module for Conjunction {
    fn receive(&mut self, message: Message, bus: &mut MessageQueue) {
        self.memory
            .entry(message.sender)
            .and_modify(|mem| *mem = message.is_high);
        let all_high = self.memory.values().all(|v| v == &true);
        bus.send(self.id, !all_high, &self.connections)
    }
    fn set_inputs(&mut self, inputs: Vec<usize>) {
        inputs.into_iter().for_each(|input| {
            self.memory.insert(input, false);
        })
    }
    fn connections_contains(&self, id: &usize) -> bool {
        self.connections.contains(id)
    }
}

struct Broadcaster {
    id: usize,
    connections: Vec<usize>,
}
impl Broadcaster {
    fn new(id: usize, connections: Vec<usize>) -> Self {
        Self { id, connections }
    }
}
impl Module for Broadcaster {
    fn receive(&mut self, message: Message, bus: &mut MessageQueue) {
        bus.send(self.id, message.is_high, &self.connections);
    }
    fn set_inputs(&mut self, _: Vec<usize>) {}
    fn connections_contains(&self, id: &usize) -> bool {
        self.connections.contains(id)
    }
}

#[derive(Default)]
struct Message {
    sender: usize,
    target: usize,
    is_high: bool,
}

struct MessageQueue {
    queue: VecDeque<Message>,
    low_count: usize,
    high_count: usize,
    button_count: usize,
    high_callback: Box<dyn Fn(usize, usize)>,
}
impl MessageQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            low_count: 0,
            high_count: 0,
            button_count: 0,
            high_callback: Box::new(|_, _| ()),
        }
    }
    fn send(&mut self, sender: usize, is_high: bool, targets: &Vec<usize>) {
        for &target in targets {
            if is_high {
                self.high_count += 1;
                (self.high_callback)(sender, self.button_count);
            } else {
                self.low_count += 1;
            }
            self.queue.push_back(Message {
                sender,
                target,
                is_high,
            });
        }
    }
    fn push_the_button(&mut self, modules: &mut ModuleStore) -> Option<Message> {
        self.low_count += 1;
        self.button_count += 1;
        self.queue.push_back(Message {
            sender: 0,
            target: 0,
            is_high: false,
        });
        let mut output = None;
        while let Some(message) = self.queue.pop_front() {
            if let Some(module) = modules.get_mut(&message.target) {
                module.receive(message, self);
            } else {
                output = Some(message);
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test20.txt"), 4250 * 2750);
    }
}

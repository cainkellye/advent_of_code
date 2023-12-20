#![allow(unused)]
use std::collections::{HashMap, VecDeque};

use super::*;
pub fn part1() {
    println!("{:?}", part1_internal("res/2023/input20.txt"));
}
pub fn part2() {
    println!("{:?}", part2_internal("res/2023/input20.txt"));
}

fn part1_internal(input_file: &str) -> usize {
    let mut modules = parse_input(input_file);
    let mut message_bus = MessageBus::new();
    message_bus.initialize(&mut modules);
    assert_eq!(message_bus.low_count, 0);
    assert_eq!(message_bus.high_count, 0);
    for _ in 0..1000 {
        message_bus.push_the_button(&mut modules);
    }
    println!("low: {}, high: {}", message_bus.low_count, message_bus.high_count);
    message_bus.low_count * message_bus.high_count
}

fn part2_internal(input_file: &str) -> usize {
    parse_input(input_file);
    0
}

type ModuleStore = HashMap<usize, Box<dyn Module>>;

fn parse_input(input_file: &str) -> ModuleStore {
    let mut modules: ModuleStore = HashMap::new();
    // %gt -> dk, pl
    // &hm -> kl, gh, tl, xx, zq
    for line in iter_lines_from(input_file) {
        let (module, connections) = line.split_once(" -> ").unwrap();
        let connections = connections
            .split(", ")
            .map(name_to_id)
            .collect_vec();
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

fn name_to_id(name: &str) -> usize {
    let mut id = 0;
    for (exp, byte) in name.bytes().enumerate() {
        id += (byte as usize) * 256_usize.pow(exp as u32);
    }
    id
}

trait Module {
    fn receive(&mut self, message: Message, bus: &mut MessageBus);
    fn init(&mut self, message: Message, bus: &mut MessageBus);
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
    fn receive(&mut self, message: Message, bus: &mut MessageBus) {
        if !message.is_high {
            self.is_on ^= true;
            bus.send(self.id, self.is_on, &self.connections)
        }
    }
    fn init(&mut self, message: Message, bus: &mut MessageBus) {
        bus.send_init(self.id, &self.connections);
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
    fn receive(&mut self, message: Message, bus: &mut MessageBus) {
        self.memory
            .entry(message.sender)
            .and_modify(|mem| *mem = message.is_high);
        let all_high = self.memory.values().all(|v| v == &true);
        bus.send(self.id, !all_high, &self.connections)
    }
    fn init(&mut self, message: Message, bus: &mut MessageBus) {
        self.memory.insert(message.sender, false);
        bus.send_init(self.id, &self.connections);
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
    fn receive(&mut self, message: Message, bus: &mut MessageBus) {
        bus.send(self.id, message.is_high, &self.connections);
    }
    fn init(&mut self, message: Message, bus: &mut MessageBus) {
        bus.send_init(self.id, &self.connections);
    }
}

#[derive(Default)]
struct Message {
    sender: usize,
    target: usize,
    is_high: bool,
    init: bool,
}

struct MessageBus {
    queue: VecDeque<Message>,
    low_count: usize,
    high_count: usize,
}
impl MessageBus {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            low_count: 0,
            high_count: 0,
        }
    }
    fn send(&mut self, sender: usize, is_high: bool, targets: &Vec<usize>) {
        for &target in targets {
            if is_high {
                self.high_count += 1;
            } else {
                self.low_count += 1;
            }
            self.queue.push_back(Message {
                sender,
                target,
                is_high,
                init: false,
            });
        }
    }
    fn send_init(&mut self, sender: usize, targets: &Vec<usize>) {
        for &target in targets {
            self.queue.push_back(Message {
                sender,
                target,
                is_high: false,
                init: true,
            });
        }
    }
    fn reset(&mut self) {
        self.low_count = 0;
        self.high_count = 0;
    }
    fn initialize(&mut self, modules: &mut ModuleStore) {
        self.queue.push_back(Message {
            sender: 0,
            target: 0,
            is_high: false,
            init: true,
        });
        while let Some(message) = self.queue.pop_front() {
            if let Some(module) = modules.get_mut(&message.target) {
                module.init(message, self);
            }
        }
    }
    fn push_the_button(&mut self, modules: &mut ModuleStore) {
        self.low_count += 1;
        self.queue.push_back(Message {
            sender: 0,
            target: 0,
            is_high: false,
            init: false,
        });
        while let Some(message) = self.queue.pop_front() {
            if let Some(module) = modules.get_mut(&message.target) {
                module.receive(message, self);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_internal("res/2023/test20.txt"), 4250 * 2750);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_internal("res/2023/test20.txt"), 0);
    }
}

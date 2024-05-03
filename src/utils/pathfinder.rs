use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Debug, Clone, Copy)]
pub struct Step<Data: Eq + Hash + Copy, Weight: PartialOrd + Copy> {
    pub weight: Weight,
    pub data: Data,
}
pub trait Map<Data: Eq + Hash + Copy, Weight: PartialOrd + Copy> {
    fn get_valid_steps(&self, from: Step<Data, Weight>) -> Vec<Step<Data, Weight>>;
    fn get_cutoff() -> Weight;

    fn find_min_weight<Start>(
        &self,
        start: Start,
        is_terminal: impl Fn(Step<Data, Weight>) -> bool,
    ) -> Weight
    where
        Start: IntoIterator<Item = Step<Data, Weight>>,
    {
        let mut best_steps = HashMap::<Data, Weight>::new();
        let mut step_queue = VecDeque::from_iter(start);
        step_queue
            .make_contiguous()
            .sort_unstable_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

        let mut path_weight = Self::get_cutoff();
        while let Some(step) = step_queue.pop_front() {
            let weight = step.weight;
            if path_weight < weight {
                continue;
            }
            if let Some(best) = best_steps.get_mut(&step.data) {
                if weight < *best {
                    *best = weight;
                } else {
                    continue;
                }
            } else {
                best_steps.insert(step.data, weight);
            }
            if is_terminal(step) {
                path_weight = weight;
                continue;
            }
            let next_steps = Self::get_valid_steps(&self, step);
            for next in next_steps {
                let next_w = next.weight;
                let pos = step_queue
                    .binary_search_by(|&s| s.weight.partial_cmp(&next_w).unwrap())
                    .unwrap_or_else(|x| x);
                step_queue.insert(pos, next);
            }
        }
        path_weight
    }
}

use aoc_helpers::hash_utils::HashVec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add(usize),
    AddSelf,
    Multiply(usize),
    MultiplySelf,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    pub index: usize,
    pub starting_items: Vec<usize>,
    pub operation: Operation,
    pub test_condition: usize,
    pub true_case: usize,
    pub false_case: usize,
    pub items_processed: usize,
}

impl Monkey {
    pub fn take_turn_day_a(&self) -> HashVec<usize, usize> {
        let mut ret: HashVec<usize, usize> = HashVec::new();
        for item in self.starting_items.iter() {
            let worry_val = self.worrify_day_a(item);
            let throw_to = if (worry_val % self.test_condition) == 0 {
                self.true_case
            } else {
                self.false_case
            };
            ret.push(throw_to, worry_val)
        }
        ret
    }

    pub fn empty_monkey(&self) -> Monkey {
        Monkey {
            index: self.index,
            starting_items: Vec::new(),
            operation: self.operation.clone(),
            test_condition: self.test_condition,
            true_case: self.true_case,
            false_case: self.false_case,
            items_processed: self.items_processed + self.starting_items.len(),
        }
    }

    pub fn take_turn_day_b(&self, shared_modulo: usize) -> HashVec<usize, usize> {
        let mut ret: HashVec<usize, usize> = HashVec::new();
        for item in self.starting_items.iter() {
            let worry_val = self.worrify_day_b(item) % shared_modulo;
            let throw_to = if (worry_val % self.test_condition) == 0 {
                self.true_case
            } else {
                self.false_case
            };
            ret.push(throw_to, worry_val)
        }
        ret
    }
    fn worrify_day_b(&self, x: &usize) -> usize {
        match self.operation {
            Operation::Add(y) => x + y,
            Operation::AddSelf => x + x,
            Operation::Multiply(y) => x * y,
            Operation::MultiplySelf => x * x,
        }
    }

    fn worrify_day_a(&self, x: &usize) -> usize {
        let ret = match self.operation {
            Operation::Add(y) => x + y,
            Operation::AddSelf => x + x,
            Operation::Multiply(y) => x * y,
            Operation::MultiplySelf => x * x,
        };
        ret / 3
    }

    pub fn next_turn(&self, result: &HashVec<usize, usize>) -> Monkey {
        let mut starting_items = Vec::new();
        starting_items.extend(self.starting_items.clone());

        let our_result = result.get(&self.index);
        starting_items.extend(our_result);

        Monkey {
            index: self.index,
            starting_items,
            operation: self.operation.clone(),
            test_condition: self.test_condition,
            true_case: self.true_case,
            false_case: self.false_case,
            items_processed: self.items_processed,
        }
    }
}

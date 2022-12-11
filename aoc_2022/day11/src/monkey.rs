use std::collections::HashMap;

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
    pub fn take_turn(&self) -> (HashMap<usize, Vec<usize>>, Monkey) {
        let mut ret: HashMap<usize, Vec<usize>> = HashMap::new();
        for item in self.starting_items.iter() {
            let worry_val = self.worrify(item);
            println!("item: {:?}", item);
            println!("worry_val: {:?}", worry_val);
            let throw_to = if (worry_val % self.test_condition) == 0 {
                self.true_case
            } else {
                self.false_case
            };
            println!("throw_to: {:?}", throw_to);
            ret.entry(throw_to)
                .and_modify(|v| v.push(worry_val))
                .or_insert_with(|| vec![worry_val]);
        }
        (
            ret,
            Monkey {
                index: self.index,
                starting_items: Vec::new(),
                operation: self.operation.clone(),
                test_condition: self.test_condition,
                true_case: self.true_case,
                false_case: self.false_case,
                items_processed: self.items_processed + self.starting_items.len(),
            },
        )
    }

    fn worrify(&self, x: &usize) -> usize {
        let calculated = match self.operation {
            Operation::Add(y) => x + y,
            Operation::AddSelf => x + x,
            Operation::Multiply(y) => x * y,
            Operation::MultiplySelf => x * x,
        };
        calculated / 3
    }

    pub fn next_turn(&self, result: &HashMap<usize, Vec<usize>>) -> Monkey {
        let mut starting_items = Vec::new();
        starting_items.extend(self.starting_items.clone());
        let our_result = result.get(&self.index);
        if let Some(our_result) = our_result {
            starting_items.extend(our_result)
        }

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

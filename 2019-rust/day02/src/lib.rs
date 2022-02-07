fn get_values(input_str: &str) -> Vec<usize> {
    input_str
        .split(',')
        .map(|n| n.parse::<usize>())
        .flatten()
        .collect()
}

fn run_int_code(memory: &mut [usize]) {
    let mut i = 0;
    while memory[i] != 99 {
        let ret = match memory[i] {
            1 => memory[memory[i + 1]] + memory[memory[i + 2]],
            2 => memory[memory[i + 1]] * memory[memory[i + 2]],
            _ => unreachable!(),
        };
        let output_index = memory[i + 3];
        memory[output_index] = ret;
        i += 4;
    }
}

fn run_intcode_simulation(memory: &[usize], noun: usize, verb: usize) -> usize {
    let mut memory = memory.to_owned();
    memory[1] = noun;
    memory[2] = verb;
    run_int_code(&mut memory);
    memory[0]
}

pub fn calculate_day_a(input_str: &str) -> usize {
    let memory = get_values(input_str);
    run_intcode_simulation(&memory, 12, 2)
}

pub fn calculate_day_b(input_str: &str) -> usize {
    let target = 19690720;
    let memory = get_values(input_str);
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_intcode_simulation(&memory, noun, verb) == target {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Expected to find a result by this point");
}

#[cfg(test)]
mod test {
    use crate::get_values;
    use crate::run_int_code;

    #[test]
    fn test_run_intcode_simulation() {
        let mut memory = get_values("1,9,10,3,2,3,11,0,99,30,40,50");
        run_int_code(&mut memory);
        assert_eq!(3500, memory[0]);
    }
}

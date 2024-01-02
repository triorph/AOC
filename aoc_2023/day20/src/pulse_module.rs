use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PulseModuleType {
    Broadcaster,
    FlipFlop(Powered),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseModule {
    pub pulse_type: PulseModuleType,
    pub input: String,
    pub outputs: Vec<String>,
    pub last_pulse_received: Pulse,
    pub counts: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Powered {
    On,
    Off,
}

impl Powered {
    fn toggle(&self) -> Powered {
        match self {
            Powered::On => Powered::Off,
            Powered::Off => Powered::On,
        }
    }

    fn to_pulse_type(self) -> Pulse {
        match self {
            Powered::On => Pulse::Low,
            Powered::Off => Pulse::High,
        }
    }
}

impl PulseModule {
    pub fn new_flip_flop(input: String, outputs: Vec<String>) -> PulseModule {
        PulseModule {
            pulse_type: PulseModuleType::FlipFlop(Powered::Off),
            input,
            outputs,
            last_pulse_received: Pulse::Low,
            counts: (0, 0),
        }
    }

    pub fn new_conjunction(input: String, outputs: Vec<String>) -> PulseModule {
        PulseModule {
            pulse_type: PulseModuleType::Conjunction(HashMap::new()),
            input,
            outputs,
            last_pulse_received: Pulse::Low,
            counts: (0, 0),
        }
    }

    pub fn new_broadcaster(outputs: Vec<String>) -> PulseModule {
        PulseModule {
            pulse_type: PulseModuleType::Broadcaster,
            input: "broadcaster".to_string(),
            outputs,
            last_pulse_received: Pulse::Low,
            counts: (0, 0),
        }
    }

    pub fn receive_pulse(&mut self, source: &str, pulse: &Pulse) -> Option<Pulse> {
        self.last_pulse_received = *pulse;
        let ret = match &self.pulse_type.clone() {
            PulseModuleType::FlipFlop(powered) => {
                if pulse == &Pulse::Low {
                    self.pulse_type = PulseModuleType::FlipFlop(powered.toggle());
                    Some(powered.to_pulse_type())
                } else {
                    None
                }
            }
            PulseModuleType::Conjunction(inputs) => {
                let mut next_inputs = inputs.clone();
                next_inputs.insert(source.to_owned(), *pulse);
                self.pulse_type = PulseModuleType::Conjunction(next_inputs.clone());
                if next_inputs.values().all(|pulse| pulse == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            PulseModuleType::Broadcaster => Some(*pulse),
        };
        match ret {
            Some(Pulse::High) => self.counts.1 += 1,
            Some(Pulse::Low) => self.counts.0 += 1,
            _ => (),
        };
        ret
    }
}

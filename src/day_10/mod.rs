use std::{ops::Rem, str::FromStr};

pub struct CPU {
    pub register: isize,
    pub cycle: usize,
    pub instructions: Vec<Instruction>,
    pub crt: [char; 240],

    instruction_index: usize,
    loaded_instruction: Option<Instruction>,
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Noop,
    AddX(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, arg) = s.split_at(4);
        return match cmd {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::AddX(arg.trim().parse().unwrap())),
            _ => Err("Unknown instruction".to_string()),
        };
    }
}

impl CPU {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        return CPU {
            register: 1,
            cycle: 1,
            instructions,
            crt: ['.'; 240],

            instruction_index: 0,
            loaded_instruction: None,
        };
    }

    fn apply_instruction(&mut self) {
        if let Some(instruction) = self.loaded_instruction {
            match instruction {
                Instruction::Noop => panic!("should not load Noop instruction"),
                Instruction::AddX(value) => {
                    self.register += value;
                    self.loaded_instruction = None;
                }
            };
            self.instruction_index += 1;
            return;
        }

        let instruction = self.instructions[self.instruction_index];

        match instruction {
            Instruction::Noop => {
                self.instruction_index += 1;
            }
            Instruction::AddX(_) => {
                self.loaded_instruction = Some(instruction);
            }
        };
    }

    pub fn update_crt(&mut self) {
        let index = (self.cycle - 1).min(self.crt.len() - 1);
        let position = (index).rem(40);
        let sprite = (self.register - 1)..=(self.register + 1);
        self.crt[index] = if sprite.contains(&(position as isize)) {
            '#'
        } else {
            '.'
        };
    }

    pub fn tick(&mut self) {
        self.update_crt();
        self.cycle += 1;
        self.apply_instruction();
    }

    pub fn get_signal_strength(&self) -> isize {
        return self.register * self.cycle as isize;
    }

    pub fn draw(&self) {
        let screen = self
            .crt
            .chunks(40)
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", screen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        let instructions = vec![
            Instruction::Noop,
            Instruction::AddX(4),
            Instruction::Noop,
            Instruction::AddX(2),
        ];
        let mut cpu = CPU::new(instructions);

        assert_eq!(cpu.register, 1);
        assert_eq!(cpu.cycle, 1);

        // Noop
        cpu.tick();
        assert_eq!(cpu.register, 1);
        assert_eq!(cpu.cycle, 2);

        // AddX 4, 1st cycle
        cpu.tick();
        assert_eq!(cpu.register, 1);
        assert_eq!(cpu.cycle, 3);

        // AddX 4, 2nd cycle
        cpu.tick();
        assert_eq!(cpu.register, 5);
        assert_eq!(cpu.cycle, 4);
    }
}

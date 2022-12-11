use aoc_2022::utils::read_input;

#[derive(Debug)]
struct MachineState {
    value: isize,
}

#[derive(Debug)]
enum MachineOp {
    Noop,
    AddX(isize),
}

#[derive(Debug)]
struct Machine {
    state: MachineState,
}

impl Machine {
    fn apply(&mut self, op: &MachineOp) {
        self.state.value += match *op {
            MachineOp::Noop => 0,
            MachineOp::AddX(x) => x,
        };
    }
}

fn main() {
    let ops = read_input(10, true)
        .trim()
        .lines()
        .flat_map(|op| {
            let (cmd, arg) = op.split_at(4);
            return match cmd {
                "noop" => vec![MachineOp::Noop],
                "addx" => vec![
                    MachineOp::Noop,
                    MachineOp::AddX(arg.trim().parse().unwrap()),
                ],
                _ => panic!(),
            };
        })
        .collect::<Vec<MachineOp>>();

    let mut machine = Machine {
        state: MachineState { value: 1 },
    };

    let mut signal_strength = 0;

    for i in 0..ops.len() {
        let cycle = (i as isize) + 1isize;
        machine.apply(&ops[i]);
        if cycle == 20
            || cycle == 60
            || cycle == 100
            || cycle == 140
            || cycle == 180
            || cycle == 220
        {
            signal_strength += cycle * machine.state.value;
        }
    }

    println!("{:?}", signal_strength);
}

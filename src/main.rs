use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    Left,
    Right,
}

type State = usize;
type TapeChar = char;

type RuleMap = HashMap<(State, TapeChar), (State, TapeChar, Direction)>;

struct Machine {
    rules: RuleMap,
    tape: Vec<TapeChar>,
    tape_index: usize,
    current_state: State,
    goal: Vec<State>,
}

impl Machine {
    fn is_finished(&self) -> bool {
        self.goal.contains(&self.current_state)
    }

    fn next_state(&mut self) -> Result<(), String> {
        let input = self.tape[self.tape_index];
        let (next, output, dir) = self.rules.get(&(self.current_state, input)).ok_or(format!(
            "no rules for this state: (state, char)=(q{}, {})",
            self.current_state, self.tape[self.tape_index]
        ))?;
        self.tape[self.tape_index] = *output;
        self.current_state = *next;
        if *dir == Direction::Left {
            self.tape_index -= 1;
        } else {
            self.tape_index += 1;
        }

        Ok(())
    }

    fn run(&mut self) -> Result<(), String> {
        while !self.is_finished() {
            self.next_state()?;
            self.print();
        }
        Ok(())
    }

    fn print(&self) {
        for (i, c) in self.tape.iter().enumerate() {
            if i == self.tape_index {
                print!("|q{}|", self.current_state);
            }
            print!("{}", c);
        }
        if self.tape.len() == self.tape_index {
            print!("|q{}|", self.current_state);
        }
        println!("");
    }
}

fn make_map() -> RuleMap {
    let mut map = HashMap::new();
    use Direction::{Left, Right};
    map.insert((0, '0'), (1, 'X', Right));
    map.insert((0, 'Y'), (3, 'Y', Right));
    map.insert((1, '0'), (1, '0', Right));
    map.insert((1, 'Y'), (1, 'Y', Right));
    map.insert((1, '1'), (2, 'Y', Left));
    map.insert((2, '0'), (2, '0', Left));
    map.insert((2, 'Y'), (2, 'Y', Left));
    map.insert((2, 'X'), (0, 'X', Right));
    map.insert((3, 'Y'), (3, 'Y', Right));
    map.insert((3, 'B'), (4, 'B', Right));
    map
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run \"initial tape string\"");
        return;
    }

    let v = args[1].chars().collect();

    let mut m = Machine {
        rules: make_map(),
        tape: v,
        tape_index: 0,
        current_state: 0,
        goal: vec![4],
    };
    m.print();
    if let Err(e) = m.run() {
        eprintln!("{}", e);
    }
}

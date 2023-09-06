use std::error::Error;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use clap::ArgMatches;
use rand::Rng;


pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {

    let mut roll = Roll::new(
        matches.get_one::<u8>("keep"),
        matches.get_one::<u8>("number").unwrap(),
        matches.get_one::<u8>("sides").unwrap()
    );

    let (adv, dadv) = (
        matches.get_flag("advantage"),
        matches.get_flag("disadvantage"),
    );

    match (adv, dadv) {
        (true, _) => roll.with_advantage(),
        (_, true) => roll.with_disadvantage(),
        _         => roll.without_effect(),
    };

    roll.sum();

    Ok(())
}

struct Roll {
    result: Vec<u8>,
    number: u8,
    sides: u8,
}

impl Roll {
    
    fn new(keep: Option<&u8>, number: &u8, sides: &u8) -> Self {
        let result = match keep {
            Some(k) => (0..*k).collect(),
            _ => Vec::new()
        };
        
        Self { result: result, number: number.clone(), sides: sides.clone() }
    }

    fn sum(&self) {
        let sum: u8 = self.result.iter().sum();
        println!("Result keeping {} of {} d{}: {}",
                 self.result.len(), self.number, self.sides, sum);
    }

    fn with_advantage(&mut self) {
        let mut rolls = BinaryHeap::new();
        let mut rng = rand::thread_rng();

        for _ in 0..(self.number) {
            rolls.push(rng.gen_range(1..(self.sides + 1)));
        }

        for i in 0..(self.result.len()) {
            self.result[i] = rolls.pop().unwrap();
        }
    }
    
    fn with_disadvantage(&mut self) {
        let mut rolls = BinaryHeap::new();
        let mut rng = rand::thread_rng();

        for _ in 0..(self.number) {
            rolls.push(Reverse(rng.gen_range(1..(self.sides + 1))));
        }

        for i in 0..(self.result.len()) {
            if let Some(Reverse(x)) = rolls.pop() {
                self.result[i] = x;
            }
        }
    }

    fn without_effect(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..(self.number) {
            self.result.push(rng.gen_range(1..(self.sides + 1)));
        }
    }

}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
    
// }

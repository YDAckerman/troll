use clap::ArgMatches;
use rand::Rng;
use collections::BinaryHeap;


use std::collections::BinaryHeap;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {

    if let Some(number) = matches.get_one::<u8>("number") {
        println!("number: {number}");
    }

    if let Some(sides) = matches.get_one::<u8>("sides") {
        println!("sides: {sides}");
    }

    if let Some(keep) = matches.get_one::<u8>("keep") {
        println!("keep: {keep}");
    }

    let (adv, dadv) = (matches.get_flag("advantage"),
                       matches.get_flag("disadvantage"),
    );

    match (adv, dadv) {
        (true, _) => println!("advantage!"),
        (_, true) => println!("disadvantage!"),
        _ => println!("no effects!")
    };

    Ok(())
}

struct Roll {
    result: Vec<u8>,
}

impl Roll {
    
    fn new(keep: &u8) -> Self {
        let result: Vec<u8> = Vec::with_capacity(keep);
        Self { result }
    }

    fn sum(&self) {
        let sum: u8 = self.result.iter().sum();
        println!("{}", sum);
    }

    fn with_advantage(&self, number: &u8, sides: &u8) {
        
    }

}



let mut rolls: BinaryHeap<u8> = BinaryHeap::new();
let mut rng = rand::thread_rng();

for _ in 0..3 {
    rolls.push(rng.gen_range(1..21));
}
    
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    
}

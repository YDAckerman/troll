use std::error::Error;
use std::cmp::Reverse;
use clap::ArgMatches;
use rand::Rng;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {

    let dice = Dice::new(
        matches.get_one::<u8>("number").unwrap(),
        matches.get_one::<u8>("sides").unwrap()
    );

    let (adv, dadv) = (
        matches.get_flag("advantage"),
        matches.get_flag("disadvantage"),
    );

    let effects = match (adv, dadv) {
        (true, _) => Effects::Advantage,
        (_, true) => Effects::Disadvantage,
           _      => Effects::None,
    };

    dice.roll(&matches.get_one::<u8>("keep"), &effects);

    Ok(())
}

enum Effects {
    Advantage,
    Disadvantage,
    None,
}

struct Dice {
    number: u8,
    sides: u8,
}

impl Dice {
    
    fn new(number: &u8, sides: &u8) -> Self {
        Self { number: number.clone(), sides: sides.clone() }
    }

    fn roll(&self, keep: &Option<&u8>, effects: &Effects) {

        let mut rng = rand::thread_rng();
        let mut result = Vec::new();
        
        for _ in 0..(self.number) {
            result.push(rng.gen_range(1..(self.sides + 1)));
        }

        match effects {
            Effects::Advantage => result.sort_by_key(|w| Reverse(*w)),
            Effects::Disadvantage => result.sort(),
            Effects::None => (), 
        }

        let k = match keep {
            Some(x) => **x as usize,
            None    => result.len() as usize,
        };
        
        println!("Keeping {} of {} d{}: {}",
                 k, self.number, self.sides,
                 &result[..(k)].iter().sum::<u8>()
        );
        
    }

}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
    
// }

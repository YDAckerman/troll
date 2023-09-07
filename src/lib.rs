use std::error::Error;
use std::cmp::Reverse;
use clap::ArgMatches;
use rand::Rng;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {

    let mut dice = Dice::new(
        matches.get_one::<u8>("number").unwrap(),
        matches.get_one::<u8>("sides").unwrap(),
        &matches.get_one::<u8>("keep"),
    );

    let (adv, dadv) = (
        matches.get_flag("advantage"),
        matches.get_flag("disadvantage"),
    );

    let effects = match (adv, dadv) {
        (true, _) => Effect::Advantage,
        (_, true) => Effect::Disadvantage,
           _      => Effect::None,
    };

    let mut rng = rand::thread_rng();
    dice.roll(&mut rng);
    dice.apply_effects(&effects);
    dice.print_result();

    Ok(())
}

#[derive(Clone)]
enum Effect {
    Advantage,
    Disadvantage,
    None,
}

struct Dice {
    number: u8,
    sides: u8,
    keep: usize,
    roll_vals: Vec<u8>,
    effect: Effect,
}

impl Dice {
    
    fn new(number: &u8, sides: &u8, keep: &Option<&u8>) -> Self {

        let keep = match keep {
            Some(x) => **x as usize,
            None    => *number as usize,
        };
        
        Self { number: number.clone(),
               sides: sides.clone(),
               keep: keep.clone(),
               roll_vals: Vec::new(),
               effect: Effect::None,
        }
    }

    fn calculate_result(&self) -> u8 {
        self.roll_vals[..(self.keep)].iter().sum::<u8>()
    }
    
    fn print_result(&self) {

        let effect = match self.effect {
            Effect::Advantage => "advantage",
            Effect::Disadvantage => "disadvantage",
            Effect::None => "no effect",
        };
        
        println!("Rolling {} d{} and keeping {} with {}: \n {}",
                 self.number, self.sides, self.keep, effect,
                 self.calculate_result(),
        );
        
    }
    
    fn apply_effects(&mut self, effect: &Effect) {

        match effect {
            Effect::Advantage => self.roll_vals.sort_by_key(|w| Reverse(*w)),
            Effect::Disadvantage => self.roll_vals.sort(),
            _ => (), 
        }

        self.effect = effect.clone();
        
    }

    fn roll<R: Rng>(&mut self, rng: &mut R) {

        for _ in 0..(self.number) {
            self.roll_vals.push(rng.gen_range(1..(self.sides + 1)));
        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn one_d20_no_effects() {
        
        let number: u8 = 1;
        let sides: u8 = 20;
        let keep: Option<&u8> = None;
        let mut dice = Dice::new(&number, &sides, &keep);
        
        let effects = Effect::None;

        // set a seed
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        dice.roll(&mut rng);
        dice.apply_effects(&effects);

        assert_eq!(11, dice.calculate_result());
    }
}

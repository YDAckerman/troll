use std::error::Error;
use std::cmp::Reverse;
use clap::ArgMatches;
use rand::Rng;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {

    let number = matches.get_one::<u8>("number").unwrap();
    let sides = matches.get_one::<u8>("sides").unwrap();

    let mut rng = rand::thread_rng();
    let mut dice = Dice::roll(&number, &sides, &mut rng);
    
    let effect = Effect::new(matches.get_flag("advantage"),
                             matches.get_flag("disadvantage"));

    match effect {
        Effect::Advantage => dice.values.sort_by_key(|w| Reverse(*w)),
        Effect::Disadvantage => dice.values.sort(),
        _ => (), 
    }

    let keep = match matches.get_one::<u8>("keep") {
        Some(x) => *x as usize,
        None    => *number as usize,
    };

    let result = dice.values[..(keep)].iter().sum::<u8>();

    println!("Rolling {} d{} and keeping {} with {}: \n {}",
             number, sides, keep, effect.to_string(), result
    );

    Ok(())
}

enum Effect {
    Advantage,
    Disadvantage,
    None,
}

impl Effect {
    fn new(adv: bool, dadv: bool) -> Self {
        
        match (adv, dadv) {
            (true, _) => return Self::Advantage,
            (_, true) => return Self::Disadvantage,
            _      => return Self::None,
        };
        
    }

    fn to_string(&self) -> String {

        match self {
            Effect::Advantage => "advantage".to_string(),
            Effect::Disadvantage => "disadvantage".to_string(),
            Effect::None => "no effect".to_string(),
        }
        
    }
}

struct Dice {
    values: Vec<u8>,
}

impl Dice {

    fn roll<R: Rng>(number: &u8, sides: &u8, rng: &mut R) -> Self {
        let mut vals = Vec::new();
        for _ in 0..(*number) {
            vals.push(rng.gen_range(1..(*sides + 1)));
        }

        Self { values: vals }
    }
}

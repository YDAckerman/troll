use clap::error::ErrorKind;
use clap::{arg, value_parser, Command, ArgGroup, ArgAction};
// use std::process;

fn main() {

    let mut cmd = Command::new("Troll!")
        .version("1.0")
        .author("Yoni Ackerman <jonathan.d.ackerman@gmail.com>")
        .about("A commandline tool for rolling D&D dice")
        .arg(
            arg!([number] "Number of dice to roll")
                .required(true)
                .value_parser(value_parser!(u16).range(1..))
        )
        .arg(
            arg!([sides] "Number of sides on each die")
                .required(true)
                .value_parser(value_parser!(u16).range(1..))
        )
        .arg(
            arg!([keep] "Number of dice to keep")
                .required(false)
                .value_parser(value_parser!(u16).range(1..))
                .default_value("1")
        )
        .arg(
            arg!(-d --disadvantage "Roll with disadvantage")
                .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-a --advantage "Roll with advantage")
                .action(ArgAction::SetTrue)
        )
        .group(
            ArgGroup::new("effects")
                .required(false)
                .args(["disadvantage", "advantage"])
        );

    let matches = cmd.get_matches_mut();

    if let Some(keep) = matches.get_one::<u16>("keep") {
        if matches.get_one::<u16>("number").unwrap() < keep {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot keep more dice than were rolled",
            ).exit();
        }
    }

    if let Some(number) = matches.get_one::<u16>("number") {
        println!("number: {number}");
    }

    if let Some(sides) = matches.get_one::<u16>("sides") {
        println!("sides: {sides}");
    }

    if let Some(keep) = matches.get_one::<u16>("keep") {
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

    
}

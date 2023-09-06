use clap::error::ErrorKind;
use clap::{arg, value_parser, Command, ArgGroup, ArgAction};
use std::process;

fn main() {

    let mut cmd = Command::new("Troll!")
        .version("1.0")
        .author("Yoni Ackerman <jonathan.d.ackerman@gmail.com>")
        .about("A commandline tool for rolling D&D dice")
        .arg(
            arg!([number] "Number of dice to roll")
                .required(true)
                .value_parser(value_parser!(u8).range(1..))
        )
        .arg(
            arg!([sides] "Number of sides on each die")
                .required(true)
                .value_parser(value_parser!(u8).range(1..))
        )
        .arg(
            arg!([keep] "Number of dice to keep")
                .required(false)
                .value_parser(value_parser!(u8).range(1..))
                .requires("effects")
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
                .requires("keep")
        );

    let matches = cmd.get_matches_mut();

    if let Some(keep) = matches.get_one::<u8>("keep") {
        
        if matches.get_one::<u8>("number").unwrap() < keep {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Cannot keep more dice than were rolled",
            ).exit();
        }
        
    }

    if let Err(e) = troll::run(&matches) {
        println!("Application error: {e}");
        process::exit(1);
    }

    
}

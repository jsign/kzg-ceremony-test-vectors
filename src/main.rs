use std::{fs::File, io::Write};

use small_powers_of_tau::sdk::{
    contribution::{update_contribution, Contribution, ContributionJSON},
    NUM_CEREMONIES,
};

fn main() {
    let init_contribution_file = File::open("initialContribution.json").unwrap();

    println!("unmarshaling initial contribution file... ");
    let initial_contribution: ContributionJSON =
        serde_json::from_reader(init_contribution_file).unwrap();
    println!("done.");

    println!("parsing initial contribution...");
    let contributions: Contribution = (&initial_contribution).into();
    println!("done.");

    let secrets: [String; NUM_CEREMONIES] = [
        String::from("0x111100"),
        String::from("0x221100"),
        String::from("0x331100"),
        String::from("0x441100"),
    ];
    println!("fixed secrets for test vector: {:?}", secrets);

    println!("updating contribution... ");
    let (updated_contribution, _) = update_contribution(contributions, secrets).unwrap();
    println!("done.");

    let updated_contribution_json: ContributionJSON = (&updated_contribution).into();
    let mut updated_contribution_file = File::create("updatedContribution.json").unwrap();

    updated_contribution_file
        .write_all(
            serde_json::to_string_pretty(&updated_contribution_json)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
}

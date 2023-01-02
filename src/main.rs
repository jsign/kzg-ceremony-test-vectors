use std::{fs::File, io::Write};

use kzg_ceremony_crypto::*;

fn main() {
    // Load the spec `initiaContribution.json`.
    println!("unmarshaling initial contribution file... ");
    let init_contribution_file = File::open("initialContribution.json").unwrap();
    let mut initial_contribution: BatchContribution =
        serde_json::from_reader(init_contribution_file).unwrap();
    println!("done.");

    // Transform from big-endian hexadecimal sub-ceremony secrets, to `Tau`s.
    let fixed_secrets = vec!["0x111100", "0x221100", "0x331100", "0x441100"];
    println!("fixed secrets for test vector: {:?}", fixed_secrets);
    let taus: Vec<Tau> = fixed_secrets
        .iter()
        .map(|hex_secret| hex::decode(&hex_secret[2..]).unwrap())
        .map(|mut secret_bytes| {
            secret_bytes.reverse(); // `Tau` is a little-endian `[u8;32]`.
            let mut padded_fr: [u8; 32] = [0; 32];
            padded_fr[..secret_bytes.len()].copy_from_slice(&secret_bytes);
            Tau::new(F(padded_fr))
        })
        .collect();

    // Update initialContribution for the secrets.
    let identity = Identity::None;
    println!("updating contribution... ");
    initial_contribution
        .contributions
        .iter_mut()
        .zip(&taus)
        .enumerate()
        .try_for_each(|(i, (contribution, tau))| {
            contribution
                .add_tau::<DefaultEngine>(tau, &identity)
                .map_err(|e| CeremoniesError::InvalidCeremony(i, e))
        })
        .unwrap();
    println!("done.");

    // Save the new batch contribution to `updatedContribution.json`.
    let mut updated_contribution_file = File::create("updatedContribution.json").unwrap();
    updated_contribution_file
        .write_all(
            serde_json::to_string_pretty(&initial_contribution)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
}

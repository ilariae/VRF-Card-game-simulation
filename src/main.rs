use std::{
    clone,
    io::{self, Read, Write},
    ops::Shl,
};

use ::schnorrkel;
use rand::{rngs::OsRng, RngCore};
use schnorrkel::{context::SigningContext, signing_context, Keypair};
use sha2::{Digest, Sha256, Sha512};

const CARD_GAME_SIGNING_CONTEXT: &str = "Card Game";
const NUM_PLAYERS: u8 = 3;

#[derive(Debug, Clone)]
struct Player {
    id: u8,
    key_pair: Keypair,
    selected_card: Option<u32>,
    commitment: Option<Vec<u8>>,
}

fn create_player(player_number: u8) -> Player {
    return Player {
        id: player_number,
        key_pair: schnorrkel::Keypair::generate(),
        selected_card: None,
        commitment: None,
    };
}

fn generate_random_seed() -> OsRng {
    let mut rng = OsRng;
    let mut random_input = [0u8; 32]; // initializes 32 byte array filled with zeros
    rng.fill_bytes(&mut random_input); // fills with random bytes -> random 32 bytes higher entropy
    rng
}
// update selected_card and generate proof / commitments
fn generate_random_for_player(player: &mut Player) {
    // generate random value as VRF input
    let mut rng = OsRng;
    let mut random_input = [0u8; 32]; // initializes 32 byte array filled with zeros
    rng.fill_bytes(&mut random_input); // fills with random bytes -> random 32 bytes higher entropy

    // hash the vrf output to create a commitment
    let mut hasher = Sha256::new();
    // create new sha256 hasher instance

    hasher.update(&random_input); // feeds random input to hasher
    let commitment = hasher.finalize();

    // update players selected card and commitment
    player.commitment = Some(commitment.to_vec());

    // sign the input using VRF
    let vrf_result = player
        .key_pair
        .vrf_sign(signing_context(CARD_GAME_SIGNING_CONTEXT.as_bytes()).bytes(&random_input));

    // vrf_sign reduces result containing vrf output and proof
    let vrf_output = vrf_result.0; // extracts vrf output from result [0]
    let vrf_proof = vrf_result.1; // extracts vrf proof from reslt [1]

    // calculate card value mod 52
    // returns the VRF output as a byte array and takes first 8 bytes of this array
    // converts the 8-byte slice into a u64 integer
    // result modulo 52 to ensure it is within the range of card values
    let card_value =
        (u64::from_le_bytes(vrf_output.as_output_bytes()[..8].try_into().unwrap()) % 52) + 1;
    player.selected_card = Some(card_value as u32);
}

fn verify_commitments(player_commitments: Vec<Player>) {
    // TODO: verify the proof /commitments of the players
    todo!()
}

fn select_winner(players: Vec<Player>) {
    // TODO: verify the proof /commitments of the players
    todo!()
}

fn main() {
    // GAME VARIABLES
    let mut continue_playing = true;

    while continue_playing {
        let mut players = Vec::new();
        let game_seed = println!("Game will start with {} number of players", NUM_PLAYERS);

        for i in 0..NUM_PLAYERS {
            // 1) create player key pair

            let mut player = create_player(i as u8);
            println!("P{} has joined the game", i,);
            // 2) each player generates a random values and publishes the commitment

            players.push(player);
        }

        for i in 0..NUM_PLAYERS {
            let mut player = &mut players[i as usize];
            generate_random_for_player(&mut player);
            println!(
                "P{} committed value {:?}",
                i,
                hex::encode(player.clone().commitment.unwrap())
            );
        }

        // 3) system reveal the commitment

        // 4) people publish their original random values

        // 5) system can verify their

        // 6) check who's the winner

        // 7) loop
        println!("Do you want to continue the simulation ? (Y/N)");

        let mut input = String::new();
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => continue_playing = true,
            "n" => continue_playing = false,
            _ => println!("Invalid input. Please enter 'Y' or 'N'."),
        }
    }
}

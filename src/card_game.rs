use std::{
    io::{self, Read, Write},
    ops::Shl,
};

use super::message_board::PublicMessageBoard;
use bitcoin::hashes::hex::ToHex;
use rand::{rngs::OsRng, RngCore};
use rand::{rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng};
use schnorrkel::{self, vrf::VRFProof};
use schnorrkel::{context::SigningContext, signing_context, Keypair};
use sha2::{Digest, Sha256, Sha512};
use sp_core::hexdisplay::AsBytesRef;

const CARD_GAME_SIGNING_CONTEXT: &str = "Card Game";
const NUM_PLAYERS: u8 = 3;

/// Now, we will use our message board to play a game of rock paper scissors!
///
/// This enum tracks the game state. The game will always go in the following order:
///
/// 1. Not Started
/// 2. Generate Player Public Keys
/// 3. Generate random numbers then commit
/// 4.
#[derive(Debug, Clone)]
struct Player {
    id: u8, // this will be used to index commitments
    key_pair: Keypair,
}
pub struct Reveal((u64, VRFProof));
struct Game {
    message_board: PublicMessageBoard,
    commitments: Vec<Vec<u8>>,
    reveals: Vec<Reveal>,
    players: Vec<Player>,
}

fn create_player(player_number: u8) -> Player {
    return Player {
        id: player_number,
        key_pair: schnorrkel::Keypair::generate(),
    };
}

fn generate_random_seed() -> OsRng {
    let mut rng = OsRng;
    let mut random_input = [0u8; 32]; // initializes 32 byte array filled with zeros
    rng.fill_bytes(&mut random_input); // fills with random bytes -> random 32 bytes higher entropy
    rng
}

fn generate_commit_for_player(player: Player) -> Vec<u8> {
    let rng = generate_random_seed();
    // hash the vrf output to create a commitment
    let mut hasher = Sha256::new();
    // create new sha256 hasher instance
    let mut random_input = [0u8; 32]; // initializes 32 byte array filled with zeros

    hasher.update(&random_input); // feeds random input to hasher
    let commitment = hasher.finalize();
    return commitment.to_vec();
    // update players selected card and commitment
}

// fn verify_commitments(player_commitments: Vec<Player>) -> Vec<u32> {
//     let mut verified_numbers = vec![];

//     for player in player_commitments {
//         let ctx = player.sig_ctx.clone().expect("Signature context not found");
//         let data = player.selected_card.expect("Player selection not found");
//         let signature = player.commitment.expect("Player commitment not found");

//         let data_as_str = data.to_string();

//         let verfied_users_number = match player
//             .key_pair
//             .verify(ctx.bytes(data_as_str.as_bytes()), &signature)
//         {
//             Ok(_) => player.selected_card.unwrap(),
//             Err(whatever) => {
//                 println!("Player {} has cheated", player.id);
//                 0
//             }
//         };
//         verified_numbers.push(verfied_users_number)
//     }

//     verified_numbers
// }
fn select_winner(game: &Game) {
    // TODO: verify the proof /commitments of the players
    todo!()
}

/*
Each player will generate their commit
*/
fn generate_commit_for_players(mut game: &Game) {
    for i in 0..game.players.len() {
        let mut player = &game.players[i];
        let commit = generate_commit_for_player(player.clone());
        game.commitments.push(commit);
    }
}

/*
Each player will commit their result to the public board
*/

fn publish_commit_to_board_for_players(mut game: &Game) {
    for i in 0..game.players.len() {
        let mut player = &game.players[i];
        let commit_data = &game.commitments[i];
        let result = &game
            .message_board
            .post_commitment(commit_data.as_bytes_ref().to_hex());
        // TODO: each player will commit and check the other commits of the people previously
    }
}

fn get_vrf_input(game: &Game) -> [u8; 32] {
    let mut sum: [u8; 32] = [0; 32];

    // Iterate over the outer vector
    for inner_vec in &game.commitments {
        // Iterate over the elements of the inner vector
        for (i, &byte) in inner_vec.iter().enumerate() {
            // Sum the bytes into the fixed-size array, wrapping around on overflow
            sum[i % 32] = sum[i % 32].wrapping_add(byte);
        }
    }
    sum
}

fn reveal_results_for_players(mut game: Game) {
    for i in 0..game.players.len() {
        // Iterate over the elements of the inner vector

        let mut player = &game.players[i];
        let commit = &game.commitments[i];
        game.message_board
            .post_reveal(commit.as_bytes_ref().to_hex());
    }
}

fn generate_pseudorandom_output_for_players(game: &Game) {
    let shared_vrf_input = get_vrf_input(game);

    for i in 0..game.players.len() {
        // Iterate over the elements of the inner vector

        let mut player = &game.players[i];
        let reveal = generate_pseudorandom_output_for_player(player.clone(), shared_vrf_input);
        game.reveals.push(reveal);
    }
}
// update selected_card and generate proof / commitments
fn generate_pseudorandom_output_for_player(player: Player, shared_vrf_input: [u8; 32]) -> Reveal {
    // sign the input using VRF
    let vrf_result = player
        .key_pair
        .vrf_sign(signing_context(CARD_GAME_SIGNING_CONTEXT.as_bytes()).bytes(&shared_vrf_input));

    // vrf_sign reduces result containing vrf output and proof
    let vrf_output = vrf_result.0; // extracts vrf output from result [0]
    let vrf_proof = vrf_result.1; // extracts vrf proof from result [1]

    // calculate card value mod 52
    // returns the VRF output as a byte array and takes first 8 bytes of this array
    // converts the 8-byte slice into a u64 integer
    // result modulo 52 to ensure it is within the range of card values
    let card_value =
        (u64::from_le_bytes(vrf_output.as_output_bytes()[..8].try_into().unwrap()) % 52) + 1;
    let signature = vrf_output;

    Reveal((card_value, vrf_proof))
}

fn init_game() -> Game {
    let mut players = Vec::new();
    let rng_seed = 2023;
    let mut pmb = PublicMessageBoard::new(rng_seed);
    for i in 0..NUM_PLAYERS {
        // 1) create player key pair

        let mut player = create_player(i as u8);
        println!("P{} has joined the game", i,);

        players.push(player);
    }

    return Game {
        players: players,
        commitments: Vec::new(),
        message_board: pmb,
        reveals: Vec::new(),
    };
}

fn verify_proofs_for_players(mut game: &Game) {
    // TODO
}

fn main() {
    // GAME VARIABLES
    let mut continue_playing = true;

    while continue_playing {
        let game_seed = println!("Game will start with {} number of players", NUM_PLAYERS);

        let game = init_game();

        // Step 1: Every player will generate a number

        generate_commit_for_players(&game);

        // Step 2: Each player will publish their commit to the board

        publish_commit_to_board_for_players(&game);

        // Step 3: Each player will read the other players commits then sum them including their own to generate

        generate_pseudorandom_output_for_players(&game);

        // Step 4: Each player then can verify their proofs against eachtoher

        verify_proofs_for_players(&game);

        // Step 5: Select winner

        select_winner(&game);

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

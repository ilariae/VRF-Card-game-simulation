use std::io::{self, Write};

use ::schnorrkel;
use schnorrkel::Keypair;

struct Player {
    id: u8,
    key_pair: Keypair,
    selected_card: Option<u32>,
    commitment: Option<u32>,
}

fn create_player(player_number: u8) -> Player {
    // TODO: generate a private / public key pair

    return Player {
        id: player_number,
        key_pair: schnorrkel::Keypair::generate(),
        selected_card: None,
        commitment: None,
    };
}

// update selected_card and generate proof / commitments
fn generate_random_for_player(player: &mut Player) {
    // generate random value as VRF input 
    let mut rng = OsRng;
    let mut random_input = [0u8; 32]; // initializes 32 byte array filled with zeros
    rng.fill_bytes(&mut random_input); // fills with random bytes -> random 32 bytes higher entropy 

    // sign the input using VRF
    let vrf_result = player.key_pair.vrf_sign(&random_input).unwrap(); // key_pair generates vrf signature on random input
    // vrf_sign reduces result containing vrf output and proof 
    let vrf_output = vrf_result.0.to_output(); // extracts vrf output from result [0]
    let vrf_proof = vrf_result.1; // extracts vrf proof from reslt [1]

    // calculate card value mod 52 
    // returns the VRF output as a byte array and takes first 8 bytes of this array
    // converts the 8-byte slice into a u64 integer
    // result modulo 52 to ensure it is within the range of card values
    let card_value = (u64::from_le_bytes(vrf_output.to_bytes()[..8].try_into().unwrap()) % 52) + 1;

    // hash the vrf output to create a commitment 
    let mut hasher = Sha256::new(); // create new sha256 hasher instance
    hasher.update(&random_input); // feeds random input to hasher
    let commitment = hasher.finalize(); 

    // update players selected card and commitment 
    player.selected_card = Some(card_value as u32);
    player.commitment = Some(commitment.to_vec());

    
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
    let NUM_PLAYERS = 3;
    let mut continue_playing = true;
    while continue_playing {
        let mut players = Vec::new();
        println!("Game will start with {} number of players", NUM_PLAYERS);

        for i in 0..NUM_PLAYERS {
            // 1) create player key pair

            let player = create_player(i as u8);
            println!(
                "P{} has joined the game with public_key: {:?}",
                i, player.key_pair.public
            );
            // 2) each player generates a random values and publishes the commitment

            generate_random_for_player(&player);
            players.push(player);
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

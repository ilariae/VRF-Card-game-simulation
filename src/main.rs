use ::schnorrkel;
use schnorrkel::Keypair;

struct GameState {}

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

fn generate_random_for_player(player: &Player) {
    // TODO: update selected_card and generate proof / commitments
    todo!()
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

    while true {
        let mut players = Vec::new();

        for i in 0..NUM_PLAYERS {
            // 1) create player key pair

            let player = create_player(i as u8);
            // 2) each player generates a random values and publishes the commitment

            generate_random_for_player(&player);
            players.push(player);
        }

        // 3) system reveal the commitment

        // 4) people publish their original random values

        // 5) system can verify their

        // 6) check who's the winner

        // 7) loop
    }

    let pair = schnorrkel::Keypair::generate();
    println!("Hello, world!");
}

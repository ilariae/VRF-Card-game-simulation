use ::schnorrkel;
use schnorrkel::Keypair;

struct GameState {}

struct Player {
    key_pair: Keypair,
    selected_card: u32,
    comittment: u32,
}

fn create_player(player_number: u8) -> Player {
    // TODO: generate a private / public key pair
    todo!()
}

fn generate_random_for_player(player: &Player) {
    // TODO: update selected_card and generate proof / committment

    todo!()
}

fn verify_committments(player_committments: Vec<Player>) {
    // TODO: verify the proof /committments of the players
    todo!()
}

fn select_winner(players: Vec<Player>) {
    // TODO: verify the proof /committments of the players
    todo!()
}

fn main() {
    let num_players = 3;

    while true {

        // 1) create player key pair

        // 2) each player generates a random values and publishes the comittment

        // 3) system reveal the committment

        // 4) people publish their original random values

        // 5) system can verify their

        // 6) check who's the winner

        // 7) loop
    }

    let pair = schnorrkel::Keypair::generate();
    println!("Hello, world!");
}

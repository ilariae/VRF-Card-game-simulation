use std::io::{self, Write};

use ::schnorrkel;
use schnorrkel::{context::SigningContext, Keypair, Signature};

struct Player {
  id: u8,
  key_pair: Keypair,
  selected_card: Option<u32>,
  commitment: Option<Signature>,
  sig_ctx: Option<SigningContext>,
}

fn create_player(player_number: u8) -> Player {
  // TODO: generate a private / public key pair

  return Player {
    id: player_number,
    key_pair: schnorrkel::Keypair::generate(),
    selected_card: None,
    commitment: None,
    sig_ctx: None,
  };
}

fn generate_random_for_player(player: &Player) {
  // TODO: update selected_card and generate proof / commitments
  todo!()
}

fn verify_commitments(player_commitments: Vec<Player>) -> Vec<u32> {
  let mut verified_numbers = vec![];

  for player in player_commitments {
    let ctx = player.sig_ctx.clone().expect("Signature context not found");
    let data = player.selected_card.expect("Player selection not found");
    let signature = player.commitment.expect("Player commitment not found");

    let data_as_str = data.to_string();

    let verfied_users_number = match player
      .key_pair
      .verify(ctx.bytes(data_as_str.as_bytes()), &signature)
    {
      Ok(_) => player.selected_card.unwrap(),
      Err(whatever) => {
        println!("Player {} has cheated", player.id);
        0
      }
    };
    verified_numbers.push(verfied_users_number)
  }

  verified_numbers
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

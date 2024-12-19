use std::io::stdin;

use crate::cards::DEFAULT_SEQUENCE;

pub fn handle_play_card(
    current_hand: &mut Vec<String>,
    flipped_card: &String,
    current_game_state: &str
) -> String {
    if current_hand.len() > 3 {
        panic!("Mão inválida")
    }

    // Determine the manilha
    let flipped_index = DEFAULT_SEQUENCE
        .iter()
        .position(|&card| flipped_card.starts_with(card))
        .expect("Flipped card must be in the sequence");

    // Get the next card in the sequence (with wrap-around)
    let manilha_index = (flipped_index + 1) % DEFAULT_SEQUENCE.len();
    let manilha = DEFAULT_SEQUENCE[manilha_index];

    let mut index: i32 = 0;

    for card in current_hand.iter() {
        index += 1;

        if card.starts_with(manilha) {
            println!("{}. {} - Manilha", index, card)
        } else {
            println!("{}. {}", index, card);
        }
    }

    let mut user_choice = String::new();

    println!("Escolha uma carta para jogar");
    stdin().read_line(&mut user_choice).expect("Failed to read input");

    match user_choice.trim().parse::<usize>() {
        Ok(choice) if choice > 0 && choice <= current_hand.len() => {
            let chosen_card = current_hand.remove(choice - 1);

            println!("Você jogou: {}", chosen_card);

            return chosen_card;
        }
        _ => {
            println!("Escolha inválida, tente novamente");

            handle_play_card(current_hand, flipped_card, current_game_state)
        }
    }
}

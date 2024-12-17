mod bot;
mod cards;
mod player;
mod round;

use cards::get_shuffled_cards;
use round::{best_two_out_of_three, MatchResult};

fn main() {
    let mut bot_score: i16 = 0;
    let mut player_score: i16 = 0;
    let mut current_game_state = "default";

    loop {
        if bot_score >= 12 || player_score >= 12 {
            println!("Jogo acabou");

            break;
        }

        let mut cards = get_shuffled_cards();

        // Distribuição de cartas para cada 'jogador'
        let mut rival_bot_cards = cards.drain(..3).collect::<Vec<String>>();
        let mut player_cards = cards.drain(..3).collect::<Vec<String>>();

        // Vira para determinar a manilha
        let flipped_card = cards.drain(..1).next().unwrap(); // Take the first card

        println!("Vira: {}", flipped_card);

        let rounds_result = best_two_out_of_three(
            &mut rival_bot_cards,
            &mut player_cards,
            &flipped_card,
            &mut current_game_state,
        );

        let points_to_gain = match current_game_state {
            "truco" => 3,
            "6" => 6,
            "9" => 9,
            "12" => 12,
            "default" => 1,
            _ => panic!("fodase, {}", current_game_state),
        };

        match rounds_result {
            MatchResult::BotWon => {
                bot_score += points_to_gain;
            }
            MatchResult::PlayerWon => {
                player_score += points_to_gain;
            }
            MatchResult::Tie => {
                panic!("Deu merda")
            }
        }
    }
}

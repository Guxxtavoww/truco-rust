mod bot;
mod cards;
mod player;
mod round;

use bot::play_bot_card;
use player::handle_play_card;
use round::{play_round, RoundResult};
use cards::{get_highest_card, get_shuffled_cards, DEFAULT_SEQUENCE};

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
        
        let mut player1_wins = 0;
        let mut player2_wins = 0;

        for round in 1..=3 {
            let bot_played_card =
                play_bot_card(&mut rival_bot_cards, &DEFAULT_SEQUENCE, &flipped_card);

            let player_played_card = handle_play_card(
                &mut player_cards,
                &flipped_card,
                &DEFAULT_SEQUENCE,
                &current_game_state,
            );

            let current_hand = vec![bot_played_card, player_played_card.clone()];

            let winner_card = get_highest_card(&current_hand, &flipped_card);

            println!("Carta vencedora: {}", winner_card);

            let player_card_copy = player_played_card.clone();

            if winner_card == player_card_copy {
                println!("Você ganhou!");

                player_score += if current_game_state != "default" {
                    3
                } else {
                    1
                }
            }
        }
    }
}

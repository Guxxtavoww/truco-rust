mod bot;
mod cards;
mod player;

use bot::play_bot_card;
use cards::{get_shuffled_cards, DEFAULT_SEQUENCE};
use player::handle_play_card;

fn main() {
    let mut bot_score: i16 = 0;
    let mut player_score: i16 = 0;

    loop {
        if bot_score >= 12 || player_score >= 12 {
            println!("Jogo acabou");

            break;
        }

        let mut cards = get_shuffled_cards();

        // Distribuição de cartas para cada 'jogador'
        let mut rival_bot_1_cards = cards.drain(..3).collect::<Vec<String>>();
        let mut partner_bot_cards = cards.drain(..3).collect::<Vec<String>>();
        let mut rival_bot_2_cards = cards.drain(..3).collect::<Vec<String>>();
        let mut player_cards = cards.drain(..3).collect::<Vec<String>>();

        // Vira para determinar a manilha
        let flipped_card = cards.drain(..1).next().unwrap(); // Take the first card

        let bot_1_played_card = play_bot_card(&mut rival_bot_1_cards, &DEFAULT_SEQUENCE, &flipped_card);
        let parther_played_card = play_bot_card(&mut partner_bot_cards, &DEFAULT_SEQUENCE, &flipped_card);
        let bot_2_played_card = play_bot_card(&mut rival_bot_2_cards, &DEFAULT_SEQUENCE, &flipped_card);
        let player_played_card = handle_play_card(&mut player_cards, &flipped_card, &DEFAULT_SEQUENCE);

        let current_hand = vec![bot_1_played_card, parther_played_card, bot_2_played_card, player_played_card];

        
    }
}

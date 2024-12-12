mod cards;

use cards::get_shuffled_cards;

fn main() {
    let mut bot_score: i16 = 0;
    let mut player_score: i16 = 0;

    loop {
        if bot_score >= 12 || player_score >= 12 {
            println!("Jogo acabou");

            break;
        }

        let mut cards = get_shuffled_cards();

        let rival_bot_1_cards = cards.drain(..3).collect::<Vec<String>>();
        let partner_bot_cards = cards.drain(..3).collect::<Vec<String>>();
        let rival_bot_2_cards = cards.drain(..3).collect::<Vec<String>>();
        let player_cards = cards.drain(..3).collect::<Vec<String>>();

        let flipped_card = cards.drain(..1).collect::<Vec<String>>();
    }
}

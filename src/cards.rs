use rand::{thread_rng, seq::SliceRandom};

pub(crate) const DEFAULT_SEQUENCE: [&str; 10] = ["4", "5", "6", "7", "Q", "J", "K", "A", "2", "3"];

pub(crate) const CARDS_SUITS: [&str; 4] = ["♦️", "♠️", "♥️", "♣️"];

pub fn get_shuffled_cards() -> Vec<String> {
    let mut truco_cards: Vec<String> = DEFAULT_SEQUENCE
        .iter()
        .flat_map(|&value| {
            CARDS_SUITS
                .iter()
                .map(move |&suit| format!("{}{}", value, suit))
        })
        .collect();

    let mut rng = thread_rng();
    truco_cards.shuffle(&mut rng);

    return truco_cards;
}

pub fn get_highest_card(cards: &Vec<String>, flipped_card: &str) {
    
}

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

pub fn get_highest_card(cards: &Vec<String>, flipped_card: &str) -> String {
    let flipped_index = DEFAULT_SEQUENCE
        .iter()
        .position(|&seq_card| flipped_card.starts_with(seq_card))
        .expect("Flipped card must be in the sequence");

    let manilha_index = (flipped_index + 1) % DEFAULT_SEQUENCE.len();
    let manilha = DEFAULT_SEQUENCE[manilha_index];

    // Find the highest card based on DEFAULT_SEQUENCE and manilha
    let highest_card = cards
        .iter()
        .max_by_key(|card| {
            if card.starts_with(manilha) {
                // Manilha cards get the highest priority
                usize::MAX
            } else {
                // For non-manilha cards, use their position in the sequence
             DEFAULT_SEQUENCE
                    .iter()
                    .position(|&seq_card| card.starts_with(seq_card))
                    .unwrap_or(0)
            }
        })
        .expect("Bot hand cannot be empty");

    return highest_card.to_string()
}

pub fn play_bot_card(
    bot_hand: &mut Vec<String>,
    card_sequence: &[&str; 10],
    flipped_card: &String,
) -> String {
    // Determine the manilha (card after the flipped card in the sequence)
    let flipped_index = card_sequence
        .iter()
        .position(|&seq_card| flipped_card.starts_with(seq_card))
        .expect("Flipped card must be in the sequence");

    let manilha_index = (flipped_index + 1) % card_sequence.len();
    let manilha = card_sequence[manilha_index];

    // Find the highest card based on card_sequence and manilha
    let highest_card = bot_hand
        .iter()
        .max_by_key(|card| {
            if card.starts_with(manilha) {
                // Manilha cards get the highest priority
                usize::MAX
            } else {
                // For non-manilha cards, use their position in the sequence
                card_sequence
                    .iter()
                    .position(|&seq_card| card.starts_with(seq_card))
                    .unwrap_or(0)
            }
        })
        .expect("Bot hand cannot be empty");

    // Clone the highest card to return it
    let highest_card_value = highest_card.clone();

    // Remove the highest card from the bot's hand
    let index_to_remove = bot_hand
        .iter()
        .position(|card| card == highest_card)
        .expect("Highest card should be in the hand");
    bot_hand.remove(index_to_remove);

    println!("Bot jogou: {}", highest_card_value);

    // Return the highest card as a String
    highest_card_value
}

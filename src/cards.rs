use rand::{seq::SliceRandom, thread_rng};

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

fn suit_priority(suit: &str) -> usize {
    match suit {
        "♦️" => 1,
        "♠️" => 2,
        "♥️" => 3,
        "♣️" => 4,
        _ => 0, // Caso não seja reconhecido, usa prioridade mínima
    }
}

// Função para calcular a pontuação de cada carta
fn get_card_score(card: &String, manilha: &str) -> (bool, usize, usize) {
    let is_manilha = card.starts_with(manilha);

    // Posição da carta na sequência padrão
    let sequence_score = DEFAULT_SEQUENCE
        .iter()
        .position(|&seq_card| card.starts_with(seq_card))
        .unwrap_or(0);

    // Pegando o último caractere (naipe) de forma segura
    let suit = card.chars().last().unwrap_or(' ');
    let suit_score = suit_priority(&suit.to_string());

    (is_manilha, sequence_score, suit_score)
}

pub fn get_highest_card(cards: &Vec<String>, flipped_card: &str) -> String {
    // Índice da carta virada na sequência padrão
    let flipped_index = DEFAULT_SEQUENCE
        .iter()
        .position(|&seq_card| flipped_card.starts_with(seq_card))
        .expect("Flipped card must be in the sequence");

    // Determinar a manilha
    let manilha_index = (flipped_index + 1) % DEFAULT_SEQUENCE.len();
    let manilha = DEFAULT_SEQUENCE[manilha_index];

    // Determinar a carta mais alta
    let highest_card = cards
        .iter()
        .max_by_key(|card| get_card_score(card, manilha))
        .expect("Bot hand cannot be empty");

    return highest_card.to_string();
}

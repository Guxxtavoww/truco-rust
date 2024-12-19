use crate::{
    bot::play_bot_card,
    cards::get_highest_card,
    player::handle_play_card,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MatchResult {
    PlayerWon,
    BotWon,
    Tie,
}

pub fn best_two_out_of_three(
    bot_hand: &mut Vec<String>,
    player_hand: &mut Vec<String>,
    flipped_card: &String,
    current_game_state: &str,
) -> MatchResult {
    let mut bot_wins = 0;
    let mut player_wins = 0;

    let mut round_results: Vec<MatchResult> = Vec::new(); // Armazena o resultado de cada rodada

    for round in 1..=3 {
        println!("--- Rodada {} ---", round);

        let bot_card = play_bot_card(bot_hand, flipped_card);

        let player_played_card = handle_play_card(
            player_hand,
            flipped_card,
            &mut current_game_state.to_string(),
        );

        let current_hand = vec![bot_card.clone(), player_played_card.clone()];
        let winner_card = get_highest_card(&current_hand, &flipped_card);

        if winner_card == player_played_card {
            player_wins += 1;
            println!("Você ganhou este round!");
            round_results.push(MatchResult::PlayerWon);
        } else if winner_card == bot_card {
            bot_wins += 1;
            println!("Bot ganhou este round!");
            round_results.push(MatchResult::BotWon);
        } else {
            println!("Este round empatou!");
            round_results.push(MatchResult::Tie);
        }

        if player_wins == 2 {
            println!("Você venceu a partida (melhor de 3)!");
            return MatchResult::PlayerWon;
        } else if bot_wins == 2 {
            println!("Bot venceu a partida (melhor de 3)!");
            return MatchResult::BotWon;
        }
    }

    // Caso empate, resolver com base nos critérios
    if player_wins == bot_wins {
        println!("Partida empatada! Aplicando critério de desempate...");

        // Critério: Se a primeira rodada empatou, o vencedor da próxima rodada decide
        if let Some(MatchResult::Tie) = round_results.get(0) {
            if let Some(next_winner) = round_results.iter().find(|&&r| r != MatchResult::Tie) {
                match next_winner {
                    MatchResult::PlayerWon => {
                        println!("A primeira rodada empatou, mas você venceu a segunda rodada.");
                        return MatchResult::PlayerWon;
                    }
                    MatchResult::BotWon => {
                        println!("A primeira rodada empatou, mas o bot venceu a segunda rodada.");
                        return MatchResult::BotWon;
                    }
                    _ => {}
                }
            }
        } else if let Some(first_round_winner) = round_results.get(0) {
            // Se a primeira rodada teve vencedor, ele decide o desempate
            match first_round_winner {
                MatchResult::PlayerWon => {
                    println!("Você venceu a primeira rodada e desempata a partida.");
                    return MatchResult::PlayerWon;
                }
                MatchResult::BotWon => {
                    println!("O bot venceu a primeira rodada e desempata a partida.");
                    return MatchResult::BotWon;
                }
                _ => {}
            }
        }
    }

    // Fallback (segurança, caso nenhum critério se aplique)
    if player_wins > bot_wins {
        println!("Você venceu a partida (melhor de 3)!");
        MatchResult::PlayerWon
    } else {
        println!("Bot venceu a partida (melhor de 3)!");
        MatchResult::BotWon
    }
}

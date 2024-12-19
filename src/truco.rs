use std::io::stdin;

pub fn ask_truco(current_game_state: &mut String) -> bool {
    println!("Você quer pedir Truco? (s/n)");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");

    if input.trim().eq_ignore_ascii_case("s") {
        match current_game_state.as_str() {
            "default" => *current_game_state = "truco".to_string(),
            "truco" => *current_game_state = "6".to_string(),
            "6" => *current_game_state = "9".to_string(),
            "9" => *current_game_state = "12".to_string(),
            _ => {
                println!("Você já chegou ao máximo de escalas!");
                return false;
            }
        }

        println!("Você pediu: {}", current_game_state);

        true
    } else {
        false
    }
}

pub fn respond_to_truco(current_game_state: &str) -> bool {
    println!(
        "Seu oponente pediu: {}. Você aceita, rejeita ou escala? (a/r/e)",
        current_game_state
    );

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().to_lowercase().as_str() {
        "a" => {
            println!("Você aceitou o pedido de {}!", current_game_state);
            true
        }
        "r" => {
            println!("Você rejeitou o pedido de {}!", current_game_state);
            false
        }
        "e" => {
            println!("Você escalou o pedido!");
            true
        }
        _ => {
            println!("Entrada inválida. Tente novamente.");
            respond_to_truco(current_game_state)
        }
    }
}

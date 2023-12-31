use std::{collections::{HashMap, VecDeque}, fs::File, io::Read};

use crate::Commands;
use crate::lexer;


pub fn tok_to_commands(tokens: Vec<String>) -> Vec<(Commands, Vec<String>)> {
    let mut commands: Vec<(Commands, Vec<String>)> = Vec::new();
    let mut unique_nb: u64 = 0;
    let mut mess_nb: u64 = 0;
    let mut states: VecDeque<(Commands, u64)> = VecDeque::new();
    let mut in_func_def: bool = false;
    let mut if_nb_stack: Vec<u64> = Vec::new();

    for token in tokens {
        if token == "end" {
            match states.pop_back() {
                Some((Commands::If, nb)) => {
                    commands.push((Commands::EndIf, [nb.to_string()].to_vec()));
                    if_nb_stack.pop();
                },
                Some((Commands::While, nb)) => {
                    commands.push((Commands::EndWhile, [nb.to_string()].to_vec()));
                },
                Some((Commands::Func, nb)) => {
                    commands.push((Commands::EndFunc, [nb.to_string()].to_vec()));
                },
                _ => {
                    println!("Error : end");
                }
            }
        }
        else if token.chars().all(char::is_numeric) {
            commands.push((Commands::Push, vec![token]));
        }
        else if token == "." {
            commands.push((Commands::Dump, vec![]));
        }
        else if token == "+" {
            commands.push((Commands::Add, vec![]));
        }
        else if token == "-" {
            commands.push((Commands::Sub, vec![]));
        }
        else if token == "dup" {
            commands.push((Commands::Dup, vec![]));
        }
        else if token == "if" {
            commands.push((Commands::If, vec![unique_nb.to_string()]));
            states.push_back((Commands::If, unique_nb));
            if_nb_stack.push(unique_nb);
            unique_nb += 1;
        }
        else if token == "while" {
            commands.push((Commands::While, vec![unique_nb.to_string()]));
            states.push_back((Commands::While, unique_nb));
            unique_nb += 1;
        }
        else if token == "else" {
            let last = if_nb_stack.pop().unwrap().to_string();
            if_nb_stack.push(last.clone().parse::<u64>().unwrap());
            commands.push((Commands::Else, vec![last]));
        }
        else if token == "func" {
            in_func_def = true;
        }
        else if token == ">" {
            commands.push((Commands::G, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "<" {
            commands.push((Commands::L, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "=" {
            commands.push((Commands::E, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "!=" {
            commands.push((Commands::Ne, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == ">=" {
            commands.push((Commands::Ge, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if token == "<=" {
            commands.push((Commands::Le, vec![unique_nb.to_string()]));
            unique_nb += 1;
        }
        else if is_string(&token) {
            commands.push((Commands::PrintStringConst, [token, format!("{}", mess_nb)].to_vec()));
            mess_nb += 1;
        }
        else if token.starts_with("syscall") {
            // on récupère le nombre après syscall
            let nb = token.chars().skip(7).collect::<String>();
            // on vérifie que c'est bien un nombre
            if nb.chars().all(char::is_numeric) {
                // on convertit le nombre en u64
                let nb = nb.parse::<u64>().unwrap();
                // on ajoute la commande syscall
                commands.push((Commands::Syscall, [nb.to_string()].to_vec()));
            }
            else {
                println!("Error : syscall invoqued without a number");
            }
        }
        else if token == "*" {
            commands.push((Commands::Mul, vec![]));
        }
        else if token == "mem" {
            commands.push((Commands::Mem, vec![]));
        }
        else if token.starts_with("read") {
            let nb = token.chars().skip(4).collect::<String>();
            // on vérifie que c'est bien un nombre
            if nb.chars().all(char::is_numeric) {
                // on convertit le nombre en u64
                let nb = nb.parse::<u64>().unwrap();
                commands.push((Commands::Read, [nb.to_string()].to_vec()));
            }
            else {
                println!("Error : syscall invoqued without a number");
            }
        }
        else if token.starts_with("write") {
            let nb = token.chars().skip(5).collect::<String>();
            // on vérifie que c'est bien un nombre
            if nb.chars().all(char::is_numeric) {
                // on convertit le nombre en u64
                let nb = nb.parse::<u64>().unwrap();
                commands.push((Commands::Write, [nb.to_string()].to_vec()));
            }
            else {
                println!("Error : syscall invoqued without a number");
            }
        }
        else if token == "swap" {
            commands.push((Commands::Swap, vec![]));
        }
        else if token == "drop" {
            commands.push((Commands::Drop, vec![]));
        }
        else if token == "over" {
            commands.push((Commands::Over, vec![]));
        }
        else if token == "rot" {
            commands.push((Commands::Rot, vec![]));
        }
        else if token == "divmod" {
            commands.push((Commands::Div, vec![]));
        }
        else if token == "true" {
            commands.push((Commands::True, vec![]));
        }
        else if token == "false" {
            commands.push((Commands::False, vec![]));
        }
        else {
            if in_func_def {
                in_func_def = false;
                commands.push((Commands::Func, vec![unique_nb.to_string(), token]));
                states.push_back((Commands::Func, unique_nb));
                unique_nb += 1;
            }
            else {
                commands.push((Commands::Unknown, vec![token]));
            }
        }
    }
    commands
}

pub fn parse_macros(tokens: Vec<String>, mut macros: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut new_tokens: Vec<String> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    let mut end_counter = 0;
    let mut in_macro: bool = false;

    for token in tokens.clone() {
        if token == "macro" {
            in_macro = true;
        }
        else if token == "if" || token == "while" || token == "func" {
            end_counter += 1;
        }
        else if token == "end" {
            if end_counter == 0 {
                in_macro = false;
                macros.insert(current[1].clone(), current[2..].to_vec());
                current.clear();
            }
            else {
                end_counter -= 1;
            }
        }
        if in_macro {
            current.push(token.clone());
        }
    }

    for token in tokens {
        if macros.contains_key(&token) {
            new_tokens.append(&mut macros[&token].clone());
        }
        else {
            new_tokens.push(token);
        }
    }

    #[cfg(debug_assertions)]
    println!("{:?}", macros);

    let mut true_tokens: Vec<String> = Vec::new();

    // we remove the macro definitions
    let mut in_macro: bool = false;
    let mut end_counter: i32 = 0;
    for token in new_tokens.clone() {
        if token == "macro" {
            in_macro = true;
        }
        if !in_macro {
            true_tokens.push(token.clone());
        }
        if token == "if" || token == "while" || token == "func" {
            end_counter += 1;
        }
        else if token == "end" {
            if end_counter == 0 {
                in_macro = false;
            }
            else {
                end_counter -= 1;
            }
        }
    }

    let mut not_used: bool = false;
    for macro_name in macros.keys() {
        if true_tokens.contains(macro_name) {
            not_used = true;
        }
    }

    if not_used {
        true_tokens = parse_macros(true_tokens, macros);
    }

    true_tokens
}

pub fn parse_includes(tokens: Vec<String>) -> Vec<String> {
    let mut new_tokens: Vec<String> = Vec::new();
    let mut in_include: bool = false;

    for token in tokens {
        if token.starts_with("include") {
            in_include = true;
        }
        if !in_include {
            new_tokens.push(token.clone());
        }
        else if !token.starts_with("include") {
            let mut file = File::open(token).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let new_content = parse_includes(lexer::file_to_tok(&content));
            for tok in new_content.clone() {
                new_tokens.push(tok);
            }
            in_include = false;
       }
    }

    new_tokens
}

fn is_string(token: &str) -> bool {
    if let Some(first_char) = token.chars().next() {
        if let Some(last_char) = token.chars().rev().next() {
            return first_char == '"' && last_char == '"';
        }
    }
    false
}

pub fn cut_string(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

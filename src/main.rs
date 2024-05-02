extern crate rand;
use rand::seq::SliceRandom;

const NUM_PLAYERS: usize = 4;
const NUM_GAMES: usize = 10000;

fn simulate_game(num_players: usize) -> ([usize; NUM_PLAYERS], [usize; NUM_PLAYERS]) {
    let mut rng = rand::thread_rng();
    let mut deck: Vec<usize> = (2..=14).cycle().take(num_players * 13).collect();
    deck.shuffle(&mut rng);

    let mut sips: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
    let mut aces: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
    for round in 0..13 {
        for player in 0..num_players {
            let card = deck[round * num_players + player];
            sips[player] += card;
            if card == 14 {
                aces[player] += 1;
            }
        }
    }
    (sips, aces)
}

fn monte_carlo_simulation(num_players: usize, num_simulations: usize) -> ([usize; NUM_PLAYERS], [usize; NUM_PLAYERS]) {
    let mut total_sips: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
    let mut total_aces: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
    for _ in 0..num_simulations {
        let (sips, aces) = simulate_game(num_players);
        for player in 0..num_players {
            total_sips[player] += sips[player];
            total_aces[player] += aces[player];
        }
    }
    (total_sips, total_aces)
}

fn to_base14(mut num: usize) -> String {
    let symbols = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D'];
    let mut result = String::new();
    while num > 0 {
        let remainder = num % 14;
        num /= 14;
        result.insert(0, symbols[remainder]);
    }
    if result.is_empty() {
        result.push('0');
    }
    result
}

fn main() {
    let (total_sips, total_aces) = monte_carlo_simulation(NUM_PLAYERS, NUM_GAMES);
    for (player, (&sips, &aces)) in total_sips.iter().zip(total_aces.iter()).enumerate() {
        let base14_sips = to_base14(sips);
        println!("Player {}: Total sips = {}, Total aces = {}", player + 1, base14_sips, aces);
    }
}
use rand::Rng;
pub fn craps_win_1_roll(runs: usize) -> (i32, i32, i32) {
    let (mut wins, mut losses, mut undecided) = (0, 0, 0);
    let mut rng = rand::thread_rng();
    for _ in 0..runs {
        let (dice1, dice2) = (rng.gen_range(1..=6), rng.gen_range(1..=6));
        let sum = dice1 + dice2;
        match sum {
            7 | 11 => wins += 1,
            2 | 3 | 12 => losses += 1,
            _ => undecided += 1,
        }
    }
    (wins, losses, undecided)
}

pub fn craps_second_turn(runs: usize) -> (i32, i32, i32) {
    let (mut undecided_wins, mut undecided_losses, mut still_undecided) = (0, 0, 0);
    let mut rng = rand::thread_rng();

    for _ in 0..=runs {
        let (dice1, dice2) = (rng.gen_range(1..=6), rng.gen_range(1..=6));
        let sum = dice1 + dice2;

        if matches!(sum, 4 | 5 | 6 | 8 | 9 | 10) {
            let point = sum;

            let (dice1_next, dice2_next) = (rng.gen_range(1..=6), rng.gen_range(1..=6));
            let second_sum = dice1_next + dice2_next;

            match second_sum {
                x if x == point => undecided_wins += 1,
                7 => undecided_losses += 1,
                _ => still_undecided += 1,
            }
        }
    }

    (undecided_wins, undecided_losses, still_undecided)
}

pub fn craps_third_turn(runs: usize) -> (f64, f64, f64) {
    let (mut und2_win, mut und2_loss, mut und2_still) = (0, 0, 0);
    let mut rng = rand::thread_rng();

    for _ in 0..runs {
        let (dice1, dice2) = (rng.gen_range(1..=6), rng.gen_range(1..=6));
        let sum = dice1 + dice2;

        if matches!(sum, 4 | 5 | 6 | 8 | 9 | 10) {
            let point = sum;

            let (dice1_next, dice2_next) = (rng.gen_range(1..=6), rng.gen_range(1..=6));
            let second_sum = dice1_next + dice2_next;

            if second_sum != point && second_sum != 7 {
                let (dice1_2next, dice2_2next) = (rng.gen_range(1..=6), rng.gen_range(1..=6));
                let third_sum = dice1_2next + dice2_2next;

                match third_sum {
                    x if x == point => und2_win += 1,
                    7 => und2_loss += 1,
                    _ => und2_still += 1,
                }
            }
        }
    }
    (und2_win as f64, und2_loss as f64, und2_still as f64)
}

fn main() {
    let trials = 25_000_000;
    let (wins, losses, undecided) = craps_win_1_roll(trials);
    let win_percentage: f64 = wins as f64 / trials as f64;
    let loss_percentage: f64 = losses as f64 / trials as f64;
    let undecided_percentage: f64 = undecided as f64 / trials as f64;
    println!("Simulated with {} trials.", trials);
    println!("After One Roll: ");
    println!("Win Percentage: {}", win_percentage);
    println!("Loss Percentage: {}", loss_percentage);
    println!("Undecided Percentage: {}", undecided_percentage);
    println!("<------ End of Part 1 ------>");
    let (un_win, un_loss, still_un) = craps_second_turn(trials);
    let (un_win_pct, un_loss_pct, still_un_pct) = (
        un_win as f64 / trials as f64,
        un_loss as f64 / trials as f64,
        still_un as f64 / trials as f64
    );
    println!("After Two Rolls: ");
    println!("Undecided -> Win: {}", un_win_pct);
    println!("Undecided -> Loss: {}", un_loss_pct);
    println!("Undecided -> Undecided: {}", still_un_pct);
    println!("<------ End of Part 2 ------>");
    let (un_un_win, un_un_loss, un_un_un) = craps_third_turn(trials);
    let (un_un_wpct, un_un_lpct, un_un_unpct) = (
        un_un_win / trials as f64,
        un_un_loss / trials as f64,
        un_un_un / trials as f64
    );
    println!("After Three Rolls: ");
    println!("Undecided -> Undecided -> Win: {}", un_un_wpct);
    println!("Undecided -> Undecided -> Loss: {}", un_un_lpct);
    println!("Undecided -> Undecided -> Undecided: {}", un_un_unpct);
}

const TABLE_SIZE: usize = 2000;

fn fill_coins(table: Vec<u8>, max_coins: usize, current_coins: Vec<usize>, printout: Option<usize>) -> (usize, Vec<usize>) {
    if current_coins.len() == max_coins {
        for (i, t) in table.iter().enumerate() {
            if *t > max_coins as u8 {
                return (i - 1, current_coins);
            }
        }
        return (table.len(), current_coins);
    } else {
        let mut new_coin = current_coins[current_coins.len() - 1] + 1;
        let mut best_value = (0, vec![]);
        loop {
            let mut new_table = table.clone();
            for (i, t) in table.iter().enumerate() {
                if *t <= max_coins as u8 {
                    for offset in 1..=max_coins as u8 - t {
                        let index = i + new_coin * offset as usize;
                        if index < table.len() {
                            let new_t = t + offset as u8;
                            if new_t < new_table[index] {
                                new_table[index] = new_t;
                            }
                        }
                    }
                }
            }
            let mut new_coins = current_coins.clone();
            new_coins.push(new_coin);
            let value = fill_coins(new_table, max_coins, new_coins, printout);
            if value.0 > best_value.0 {
                best_value = value;
            }
            if new_coin >= table.len() {
                break;
            }
            if table[new_coin] > max_coins as u8 {
                break;
            }
            new_coin += 1;
        }
        if let Some(p) = printout {
            if p >= current_coins.len() {
                println!("{:?} {:?}", current_coins, best_value);
            }
        }
        return best_value;
    }
}

fn find_best_coins(max_coins: usize, printout: Option<usize>) -> (usize, Vec<usize>) {
    let mut table = vec![255; TABLE_SIZE];
    for i in 0..=max_coins {
        table[i] = i as u8;
    }
    fill_coins(table, max_coins, vec![1], printout)
}

fn main() {
    let num_coins = std::env::args().nth(1).unwrap().parse().unwrap();
    let printout = std::env::args().nth(2).map(|s| s.parse().unwrap());
    let result = find_best_coins(num_coins, printout);
    println!("{:?}", result);
}

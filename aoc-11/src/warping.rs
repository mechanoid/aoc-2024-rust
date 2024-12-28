use std::time::Instant;

type Stone = u64;

pub fn parse(blink_stones: &str) -> Vec<Stone> {
    blink_stones
        .trim()
        .split(" ")
        .map(|stone| stone.parse::<Stone>().unwrap())
        .collect::<Vec<Stone>>()
}

fn warp(blink_stones: &mut Vec<Stone>, stone: Stone) {
    match stone {
        0 => blink_stones.push(1),
        i if i.to_string().len() % 2 == 0 => {
            let i = i.to_string();
            let (a, b) = i.split_at(i.len() / 2);

            blink_stones.push(a.parse::<Stone>().unwrap());
            blink_stones.push(b.parse::<Stone>().unwrap());
        }
        _ => blink_stones.push(stone * 2024),
    }
}

fn blink(blink_stones: Vec<Stone>) -> Vec<Stone> {
    let mut blinked = vec![];

    for stone in blink_stones {
        warp(&mut blinked, stone);
    }

    return blinked;
}

pub fn blink_n_times(blink_stones: &Vec<Stone>, n: u8) -> Vec<Stone> {
    println!("\n\nstarting to blink!\n");
    let mut blinked = blink_stones.clone();

    for i in 0..n {
        let now = Instant::now();
        blinked = blink(blinked);
        let elapsed_time = now.elapsed();
        println!(
            "Blinked for {} times: we have {} stones now! ({}s)",
            i + 1,
            blinked.len(),
            elapsed_time.as_secs()
        );
    }

    println!("stop blinking!!!\n\n");
    return blinked;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use super::{blink, blink_n_times};

    #[test]
    fn test_blink() {
        let stones: Vec<u64> = vec![0, 1, 10, 99, 999];
        let result = blink(stones);
        assert_eq!(result, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_blink_n_times() {
        let stones: Vec<u64> = vec![125, 17];

        let result = blink_n_times(&stones, 1);
        assert_eq!(result, vec![253000, 1, 7]);

        let result = blink_n_times(&stones, 2);
        assert_eq!(result, vec![253, 0, 2024, 14168]);

        let result = blink_n_times(&stones, 3);
        assert_eq!(result, vec![512072, 1, 20, 24, 28676032]);

        let result = blink_n_times(&stones, 4);
        assert_eq!(result, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);

        let result = blink_n_times(&stones, 5);
        assert_eq!(
            result,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );

        let result = blink_n_times(&stones, 6);
        assert_eq!(
            result,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }
}
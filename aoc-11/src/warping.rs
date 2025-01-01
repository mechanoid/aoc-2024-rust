use memoize::memoize;

pub type StoneValue = u64;
pub type StoneCount = u64;

pub fn parse(blink_stones: &str) -> Vec<StoneValue> {
    blink_stones
        .trim()
        .split(" ")
        .map(|stone| stone.parse::<StoneValue>().unwrap())
        .collect::<Vec<StoneValue>>()
}

#[memoize]
fn number_of_stones_after_blink(stone: StoneValue, mutations: u8) -> StoneCount {
    if mutations == 0 {
        return 1;
    }

    if stone == 0 {
        return number_of_stones_after_blink(1, mutations - 1);
    } else if stone.to_string().len() % 2 == 0 {
        let i = stone.to_string();
        let (a, b) = i.split_at(i.len() / 2);
        return number_of_stones_after_blink(a.parse::<StoneValue>().unwrap(), mutations - 1)
            + number_of_stones_after_blink(b.parse::<StoneValue>().unwrap(), mutations - 1);
    }

    return number_of_stones_after_blink(stone * 2024, mutations - 1);
}

pub fn stone_count_for_n_blinks(blink_stones: &Vec<StoneValue>, n: u8) -> StoneCount {
    let mut stone_count: StoneCount = 0;

    for stone in blink_stones {
        let stones_for_stone = number_of_stones_after_blink(*stone, n);
        stone_count += stones_for_stone;
    }

    return stone_count;
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use super::{stone_count_for_n_blinks, StoneValue};

    #[test]
    fn test_stone_count_for_n_blinks() {
        let stones: Vec<StoneValue> = vec![125, 17];
        let result = stone_count_for_n_blinks(&stones, 6);
        assert_eq!(result, 22);

        let stones: Vec<StoneValue> = vec![125, 17];
        let result = stone_count_for_n_blinks(&stones, 25);
        assert_eq!(result, 55312);
    }
}

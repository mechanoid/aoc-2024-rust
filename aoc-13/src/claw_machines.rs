use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClawMachine {
    prize: Coord,
    a_button: Coord,
    b_button: Coord,
}

impl ClawMachine {
    fn solve(self) -> Option<i64> {
        let ClawMachine {
            prize,
            a_button,
            b_button,
        } = self;

        let Coord { x: ax, y: ay } = a_button;
        let Coord { x: bx, y: by } = b_button;
        let Coord { x: px, y: py } = prize;

        // linear system
        // ax * x + bx * y - px = 0
        // ay * x + by * y - py = 0

        // 1. solving first equation to y
        // ax * x + bx * y - px = 0
        // ax * x + bx * y = px
        // bx * y = px - ax*x
        // y = (px - ax*x)/bx

        // 1. solving second equation to y
        // ay * x + by * y - py = 0
        // ay * x + by * y = py
        // by * y = py - ay*x
        // y = (py - ay*x)/by

        // y = y
        // (px - ax*x)/bx = (py - ay*x)/by
        // (px - ax*x)*by = (py - ay*x)*bx
        // px*by - ax*by*x = py*bx - ay*bx*x
        // ay*bx*x - ax*by*x = py*bx - px*by
        // (ay*bx - ax*by) * x = py*bx - px*by
        // x = (py*bx - px*by) / (ay*bx - ax*by)

        // number of times the a button has to be clicked
        let x = (py * bx - px * by) / (ay * bx - ax * by);

        // number of times the b button has to be clicked
        let y1 = (px - ax * x) / bx;
        let y2 = (py - ay * x) / by;

        if y1 == y2 {
            let matched_px = ax * x + bx * y1 - px == 0;
            let matched_py = ay * x + by * y1 - py == 0;

            if matched_px && matched_py {
                return Some(3 * x + y1);
            }
        }

        return None;
    }
}

pub fn min_tokens_for_max_prices(machines: &Vec<ClawMachine>) -> i64 {
    let mut sum = 0;

    for machine in machines {
        if let Some(tokens) = machine.solve() {
            sum += tokens;
        };
    }

    return sum;
}

// ########### Parsing ######################################################################################## //

fn parse_line(config: &str, template: &Regex) -> Result<Coord, String> {
    if let Some(res) = template.captures(config) {
        let x = *&res[1].parse::<i64>().unwrap();
        let y = *&res[2].parse::<i64>().unwrap();

        return Ok(Coord { x, y });
    }

    return Err(format!("failed to parse config {}", config));
}

fn parse_claw_machine(
    config: &str,
    button_template: &Regex,
    prize_template: &Regex,
) -> Result<ClawMachine, String> {
    let lines: Vec<&str> = config.trim().split("\n").map(|line| line.trim()).collect();

    if let [a_button_line, b_button_line, prize_line] = *lines.as_slice() {
        let a_button = parse_line(a_button_line, button_template).unwrap();
        let b_button = parse_line(b_button_line, button_template).unwrap();
        let mut prize = parse_line(prize_line, prize_template).unwrap();
        prize.x += 10000000000000;
        prize.y += 10000000000000;

        return Ok(ClawMachine {
            a_button,
            b_button,
            prize,
        });
    };

    return Err(format!("failed to parse claw machine config {}", config));
}

pub fn parse(machines: &str) -> Vec<ClawMachine> {
    let button_template = Regex::new(r"^Button [AB]: X\+(\d+?), Y\+(\d+?)$").unwrap();
    let prize_template = Regex::new(r"^Prize: X=(\d+?), Y=(\d+?)$").unwrap();

    return machines
        .trim()
        .split("\n\n")
        .map(|machine| parse_claw_machine(machine, &button_template, &prize_template).unwrap())
        .collect();
}

// ###############################################################################################################
// ###############################################################################################################
// # TESTS                                                                                                       #
// ###############################################################################################################
// ###############################################################################################################

#[cfg(test)]
mod tests {
    use crate::claw_machines::{min_tokens_for_max_prices, ClawMachine, Coord};

    use super::parse;

    #[test]
    fn test_solve() {
        let machines = parse(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );

        let machine = &machines[0];
        let tokens = machine.solve();
        assert_eq!(tokens, Some(280));

        let machine = &machines[1];
        let tokens = machine.solve();
        assert_eq!(tokens, None);

        let machine = &machines[2];
        let tokens = machine.solve();
        assert_eq!(tokens, Some(200));

        let machine = &machines[3];
        let tokens = machine.solve();
        assert_eq!(tokens, None);
    }

    #[test]
    fn test_parse() {
        let machines = parse(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );

        assert_eq!(machines.len(), 4);
        assert_eq!(
            machines[0],
            ClawMachine {
                a_button: Coord { x: 94, y: 34 },
                b_button: Coord { x: 22, y: 67 },
                prize: Coord { x: 8400, y: 5400 }
            }
        );
    }

    #[test]
    fn test_min_tokens_for_max_prices() {
        let machines = parse(
            "Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279",
        );

        let tokens = min_tokens_for_max_prices(&machines);

        assert_eq!(tokens, 480);
    }
}

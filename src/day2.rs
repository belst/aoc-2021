
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

pub fn generate(input: &str) -> Vec<Command> {
    input.lines()
        .filter_map(|l| l.rsplit_once(" "))
        .map(|t| match t {
            ("forward", n) => Command::Forward(n.parse().unwrap()),
            ("down", n) => Command::Down(n.parse().unwrap()),
            ("up", n) => Command::Up(n.parse().unwrap()),
            _ => panic!("Invalid Input {:?}", t),
        })
        .collect()
}

pub fn part1(input: &[Command]) -> usize {
    let (h, d) = input.iter().fold((0usize, 0), |(h, d), c| match c {
        Command::Forward(n) => (h + n, d),
        Command::Down(n) => (h, d + n),
        Command::Up(n) => (h, d - n),
    });
    h * d
}

pub fn part2(input: &[Command]) -> usize {
    let (h, d, _a) = input.iter().fold((0usize, 0, 0usize), |(h, d, a), c| match c {
        Command::Forward(n) => (h + n, d + n * a, a),
        Command::Down(n) => (h, d, a + n),
        Command::Up(n) => (h, d, a - n),
    });
    h * d
}

#[cfg(test)]
mod tests {

    const TESTINPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn testp1() {
        let input = super::generate(TESTINPUT);
        assert_eq!(super::part1(&input), 150);
    }

    #[test]
    fn testp2() {
        let input = super::generate(TESTINPUT);
        assert_eq!(super::part2(&input), 900);
    }
    
}



pub fn generate(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(l, r)| l < r)
        .count()
}

pub fn part2(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .zip(input[1..].windows(3).map(|w| w.iter().sum()))
        .filter(|(l, r)| l < r)
        .count()
}

#[test]
fn testp1() {
    const INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263";

    let list = generate(INPUT);
    assert_eq!(part1(&list), 7);
}

#[test]
fn testp2() {
    const INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263";

    let list = generate(INPUT);
    assert_eq!(part2(&list), 5);
}

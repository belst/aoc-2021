use std::collections::HashSet;

// Transpose numbers
pub fn generate(input: &str) -> HashSet<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect())
        .collect()
}

fn invert_and_trim(input: usize, length: usize) -> usize {
    let out = !input;
    out & ((1 << length) - 1)
}

fn bits_to_usize(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc + (b as usize) * (1 << i))
}

pub fn part1(input: &HashSet<Vec<bool>>) -> usize {
    let n = input.iter().next().unwrap().len();
    let mut gamma_rate_bits = vec![false; n];
    let mut epsilon_rate_bits = vec![true; n];
    for i in 0..n {
        let mut bytes = vec![false; input.len()];
        let mut count_ones = 0;
        for j in 0..input.len() {
            if input.iter().nth(j).unwrap()[i] {
                count_ones += 1;
            }
            bytes[j] = input.iter().nth(j).unwrap()[i];
        }
        if count_ones * 2 > input.len() {
            gamma_rate_bits[i] = true;
            epsilon_rate_bits[i] = false;
        }
    }
    let gamma_rate = bits_to_usize(&gamma_rate_bits);
    let epsilon_rate = invert_and_trim(gamma_rate, n);
    gamma_rate * epsilon_rate
}

pub fn part2(input: &HashSet<Vec<bool>>) -> usize {
    let n = input.iter().next().unwrap().len();
    let mut oxygen_rate_options = input.clone();
    let mut co2scrubber_options = input.clone();

    for i in 0..n {
        let oxy_ones = oxygen_rate_options.iter().filter(|v| v[i]).count();
        let scrub_ones = co2scrubber_options.iter().filter(|v| v[i]).count();
        
        // if ones are majority, keep ones, otherwise keep nulls
        if oxygen_rate_options.len() > 1 {
            if oxy_ones * 2 >= oxygen_rate_options.len() {
                oxygen_rate_options.retain(|v| v[i]);
            } else {
                oxygen_rate_options.retain(|v| !v[i]);
            }
        }
        
        // if ones are less, keep ones, otherwise keep nulls
        if co2scrubber_options.len() > 1 {
            if scrub_ones * 2 < co2scrubber_options.len() {
                co2scrubber_options.retain(|v| v[i]);
            } else {
                co2scrubber_options.retain(|v| !v[i]);
            }    
        }
    }

    let oxygen_rate = bits_to_usize(oxygen_rate_options.iter().next().unwrap());
    let co2scrubber = bits_to_usize(co2scrubber_options.iter().next().unwrap());

    oxygen_rate * co2scrubber
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day3::bits_to_usize;

    const INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_generator() {
        let input = super::generate(INPUT);
        let expected: HashSet<Vec<_>> = 
            vec![
                vec![0 == 1, 0 == 1, 1 == 1, 0 == 1, 0 == 1],
                vec![1 == 1, 1 == 1, 1 == 1, 1 == 1, 0 == 1],
                vec![1 == 1, 0 == 1, 1 == 1, 1 == 1, 0 == 1],
                vec![1 == 1, 0 == 1, 1 == 1, 1 == 1, 1 == 1],
                vec![1 == 1, 0 == 1, 1 == 1, 0 == 1, 1 == 1],
                vec![0 == 1, 1 == 1, 1 == 1, 1 == 1, 1 == 1],
                vec![0 == 1, 0 == 1, 1 == 1, 1 == 1, 1 == 1],
                vec![1 == 1, 1 == 1, 1 == 1, 0 == 1, 0 == 1],
                vec![1 == 1, 0 == 1, 0 == 1, 0 == 1, 0 == 1],
                vec![1 == 1, 1 == 1, 0 == 1, 0 == 1, 1 == 1],
                vec![0 == 1, 0 == 1, 0 == 1, 1 == 1, 0 == 1],
                vec![0 == 1, 1 == 1, 0 == 1, 1 == 1, 0 == 1]
            ].into_iter().collect();
        
        assert_eq!(input, expected);
    }

    #[test]
    fn test_reverse_and_trim() {
        let n = 0b10110;
        let out = 0b01001;
        assert_eq!(super::invert_and_trim(n, 5), out);
        assert_eq!(super::invert_and_trim(out, 5), n);
    }

    #[test]
    fn test_bits_to_usize() {
        assert_eq!(
            bits_to_usize(&vec![true, false, true, true, false]),
            0b10110
        );
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::generate(INPUT)), 198);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::generate(INPUT)), 230);
    }
}

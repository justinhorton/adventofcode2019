const INPUT: &str = include_str!("../day16.txt");
const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn main() {
    println!("Day 16-1: {}", part1(INPUT));
    println!("Day 16-2: {}", part2(INPUT));
}

fn part1(input: &str) -> String {
    let result = fft_full(input.trim());
    result[..8].to_string()
}

fn fft_full(input: &str) -> String {
    let mut output = fft(input);
    for _i in 1..100 {
        output = fft(&output);
    }
    output
}

fn fft(input: &str) -> String {
    let mut output = String::new();
    for element_num in 1..=input.len() {
        let element = fft_element(input.to_string(), element_num as i32);
        output.push(std::char::from_digit(element as u32, 10).unwrap());
    }
    output
}

fn fft_element(input: String, element_num: i32) -> i32 {
    let first_non_zero: usize = (element_num) as usize;
    let relevant = &input[first_non_zero - 1..];

    let mut sum = 0;
    let mut p_i = 1; // index into pattern
    let mut i = 0;
    for digit in relevant
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|it| it as i32)
    {
        sum += digit * BASE_PATTERN[p_i];

        i += 1;
        if i % element_num == 0 {
            p_i = (p_i + 1) % 4;
        }
    }

    i32::abs(sum % 10)
}

fn part2(input: &str) -> String {
    let input_repeated = input.trim().repeat(10000);
    // 7 digit offset at start
    let offset = (&input_repeated[..7]).parse::<usize>().unwrap();

    // The original input has 650 characters, making our repeated input 6,500,000 characters. With
    // the given encoding, the n-th (1-indexed) 'phase' has 0 factors for (n-1) digits before it. It
    // then repeats 1 factors for n more digits.
    //
    // The offset is 5,976,277, so our our first 'phase' is the 5,976,278th phase (1-indexed) or the
    // one which starts its 1 factors on character 5,976,277 (0-indexed). Since the total length of
    // the input is only 6,500,000, the only other factor that will appear in this first and the
    // following 7 phases of interest is 1
    //
    // This simplifies the problem to a sum & modulo across the digits to the right of our offset.
    // Starting from the end of the input (call it index n-1), we have the following at each of the
    // 100 iterations of the FFT:
    //
    // digit[n-1] = digit[n-1]
    // digit[n-2] = (digit[n-2] + digit[n-1]) % 10
    // digit[n-3] = (digit[n-3] + digit[n-2]) % 10
    // ... until the start of the offset
    let mut digits: Vec<u32> = (&input_repeated[offset..])
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for _phase in 0..100 {
        let mut i = digits.len() - 2;
        loop {
            digits[i] = (digits[i] + digits[i + 1]) % 10;

            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    // take the first 8 digits of the result
    (&digits[0..8])
        .into_iter()
        .map(|&it| std::char::from_digit(it, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), "34694616");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), "17069048");
    }

    #[test]
    fn test_part2_ex1() {
        let input = "03036732577212944063491565474664";
        let result = part2(input);
        assert_eq!(result, "84462026");
    }

    #[test]
    fn test_part2_ex2() {
        let input = "02935109699940807407585447034323";
        let result = part2(input);
        assert_eq!(result, "78725270");
    }
}

use std::collections::VecDeque;
use std::ops::RangeInclusive;

const INPUT_RANGE: RangeInclusive<i32> = 158126..=624574;

fn main() {
    println!(
        "Day 4-1: {}",
        pwds_meeting_criteria(|p| meets_criteria(p, two_adjacent_digits))
    );
    println!(
        "Day 4-2: {}",
        pwds_meeting_criteria(|p| meets_criteria(p, two_adjacent_digits_not_larger_group))
    );
}

fn pwds_meeting_criteria(crit_fn: fn(i32) -> bool) -> i32 {
    let mut matching_pwds = 0;
    for pwd in INPUT_RANGE {
        if crit_fn(pwd) {
            matching_pwds += 1;
        }
    }
    matching_pwds
}

/*
    It is a six-digit number. --> range
    The value is within the range given in your puzzle input. --> range

    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

    Pt. 2: the two adjacent matching digits are not part of a larger group of matching digits.
*/
fn meets_criteria(num: i32, adj_digits_criterion: fn(&VecDeque<i32>) -> bool) -> bool {
    let digits = digits(num);
    adj_digits_criterion(&digits) && non_decreasing_digits(&digits)
}

fn digits(num: i32) -> VecDeque<i32> {
    let mut digits: VecDeque<i32> = VecDeque::new();
    let mut value = num;
    while value > 0 {
        digits.push_front(value % 10);
        value = value / 10;
    }
    digits
}

fn two_adjacent_digits(digits: &VecDeque<i32>) -> bool {
    for i in 0..5 {
        let digit = digits.get(i);
        if digit == digits.get(i + 1) {
            return true;
        }
    }
    false
}

// passes: 111122
// fails:  123444
fn two_adjacent_digits_not_larger_group(digits: &VecDeque<i32>) -> bool {
    let mut iter = digits.iter();
    let mut cur_digit = iter.next();
    while let Some(_) = cur_digit {
        let (adj_digits_count, next_digit) = consume_matching_digits(&mut iter, cur_digit);

        // exactly 2 of the same digit is a match
        if adj_digits_count == 2 {
            return true;
        }
        cur_digit = next_digit;
    }
    false
}

fn consume_matching_digits<'a>(
    iter: &mut dyn Iterator<Item = &'a i32>,
    cur_digit: Option<&i32>,
) -> (i32, Option<&'a i32>) {
    match cur_digit {
        None => (0, None),
        Some(_) => {
            // consume matching digits, incrementing count
            let mut adj_digits_count = 1;
            let mut next_digit: Option<&i32> = iter.next();
            while cur_digit == next_digit {
                adj_digits_count += 1;
                next_digit = iter.next();
            }

            (adj_digits_count, next_digit)
        }
    }
}

fn non_decreasing_digits(digits: &VecDeque<i32>) -> bool {
    for i in 1..6 {
        if digits.get(i) < digits.get(i - 1) {
            return false;
        }
    }
    true
}

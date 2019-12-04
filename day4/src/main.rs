use std::ops::RangeInclusive;
use std::collections::VecDeque;

const INPUT_RANGE: RangeInclusive<i32> = 158126..=624574;

fn main() {
    println!("Day 4-1: {}", pwds_meeting_criteria(|p| meets_criteria(p, true)));
    println!("Day 4-2: {}", pwds_meeting_criteria(|p| meets_criteria(p, false)));
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
fn meets_criteria(num: i32, as_part_of_larger_group_ok: bool) -> bool {
    let digits = digits(num);
    let adj = if as_part_of_larger_group_ok {
        two_adjacent_digits(&digits)
    } else {
        two_adjacent_digits_not_larger_group(&digits)
    };
    adj && non_decreasing_digits(&digits)
}

fn digits(num: i32) -> VecDeque<i32> {
    let mut digits: VecDeque<i32> = VecDeque::new();
    let mut value = num;
    while value > 0  {
        digits.push_front(value % 10);
        value = value / 10;
    }
    digits
}

fn two_adjacent_digits(digits: &VecDeque<i32>) -> bool {
    for i in 0..5 {
        let digit = digits.get(i);
        if digit == digits.get(i + 1) {
            return true
        }
    }
    false
}

// passes: 111122
// fails:  123444
fn two_adjacent_digits_not_larger_group(digits: &VecDeque<i32>) -> bool {
    let mut iter = digits.iter();

    let mut cur_digit = iter.next();
    loop {
        let mut next_digit: Option<&i32> = None;
        let mut adj_digits_count = 1;

        // consume the matching digits, incrementing the count
        loop {
            next_digit = iter.next();

            if cur_digit == next_digit {
                adj_digits_count += 1;
            } else {
                break;
            }
        }

        // if we counted exactly 2 of the same digit, we have a match
        if adj_digits_count == 2 {
            return true;
        }

        cur_digit = next_digit;
        if cur_digit.is_none() {
            break;
        }
    }

    false
}

fn non_decreasing_digits(digits: &VecDeque<i32>) -> bool {
    for i in 1..6 {
        if digits.get(i) < digits.get(i-1) {
            return false
        }
    }
    true
}

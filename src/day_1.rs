pub fn solve(input: &str) -> (Option<i32>, Option<usize>) {
    print!("Day {}", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    let mut part_2 = None;
    let mut floor = 0;

    for (index, direction) in input.chars().enumerate() {
        match direction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        };

        if floor < 0 && part_2.is_none() {
            part_2 = Some(index + 1);
        }
    }

    print!(" - Part 1: {}.", floor);

    if part_2.is_some() {
        println!(" Part 2: {}.", part_2.unwrap());
    }

    (Some(floor), part_2)
}

#[cfg(test)]
#[test]
#[ignore]
fn part_1_test_1() {
    assert_eq!(solve("(())").0.unwrap(), 0);
}

#[test]
#[ignore]
fn part_1_test_2() {
    assert_eq!(solve("()()").0.unwrap(), 0);
}

#[test]
#[ignore]
fn part_1_test_3() {
    assert_eq!(solve("(((").0.unwrap(), 3);
}

#[test]
#[ignore]
fn part_1_test_4() {
    assert_eq!(solve("(()(()(").0.unwrap(), 3);
}

#[test]
#[ignore]
fn part_1_test_5() {
    assert_eq!(solve("))(((((").0.unwrap(), 3);
}

#[test]
#[ignore]
fn part_1_test_6() {
    assert_eq!(solve("())").0.unwrap(), -1);
}

#[test]
#[ignore]
fn part_1_test_7() {
    assert_eq!(solve("))(").0.unwrap(), -1);
}

#[test]
#[ignore]
fn part_1_test_8() {
    assert_eq!(solve(")))").0.unwrap(), -3);
}

#[test]
#[ignore]
fn part_1_test_9() {
    assert_eq!(solve(")())())").0.unwrap(), -3);
}

#[test]
#[ignore]
fn part_2_test_1() {
    assert_eq!(solve(")").1.unwrap(), 1);
}

#[test]
#[ignore]
fn part_2_test_2() {
    assert_eq!(solve("()())").1.unwrap(), 5);
}

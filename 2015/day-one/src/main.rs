fn solve(string: &str) -> i64 {
    let mut floor = 0;

    for ch in string.split("") {
        if ch == "(" {
            floor += 1;
        } else if ch == ")"{
            floor -= 1;
        }
    }


    floor
}

fn main() {
    let input = include_str!("../input.txt");
    let result = solve(&input);
    dbg!(result);
}

#[test]
fn test_one() {
    let result = solve("(())");
    let result2 = solve("()()");

    assert_eq!(result, 0);
    assert_eq!(result2, 0);
}

#[test]
fn test_two() {
    let result = solve("))(((((");
    assert_eq!(result, 3);
}

/*
Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

) causes him to enter the basement at character position 1.
()()) causes him to enter the basement at character position 5.
What is the position of the character that causes Santa to first enter the basement?
*/

fn solve(string: &str) -> i64 {
    let mut floor = 0;
    let mut char_position = 0;
    for ch in string.split("") {
        if ch == "(" {
            floor += 1;
        } else if ch == ")"{
            floor -= 1;
        }

        if floor == -1 {
            break
        }
        char_position += 1;
    }
    char_position
}

fn main() {
    let input = include_str!("../input.txt");
    let result = solve(&input);
    dbg!(result);
}

#[test]
fn test_one() {
    let result = solve("()())");
    assert_eq!(result, 5);
}


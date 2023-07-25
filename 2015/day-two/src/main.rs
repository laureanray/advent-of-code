fn get_area_rqd(l: u64, w: u64, h: u64) -> u64 {
    let vector = vec![l * w, l * h, w * h];
    let smallest = vector.iter().min().unwrap();

    (2 * l * w) + (2 * l * h) + (2 * w * h) + smallest
}

fn parse_str(input: &str) -> (u64, u64, u64) {
    let vector: Vec<&str>= input.split('x').collect();
    let l: u64 = vector[0].parse().unwrap();
    let w: u64 = vector[1].parse().unwrap();
    let h: u64 = vector[2].parse().unwrap();
    (l, w, h)
}

fn get_ribbon_rqd(l: u64, w: u64, h: u64) -> u64 {
    let mut vector = vec![l, w, h];
    vector.sort();
    let smallest_face = (vector[0] * 2) + (vector[1] * 2);
    let volume = l * w * h;
    smallest_face + volume
}

fn main() {
    let input = include_str!("../input.txt");
    let mut res = 0;
    for line in input.split("\n") {
        if line.len() > 0 {
            let (l, w, h) = parse_str(line);
            res = res + get_ribbon_rqd(l, w, h);
        }
    }

    dbg!(res);
}

#[test]
fn test_get_ribbon_rqd() {
    assert_eq!(get_ribbon_rqd(2, 3, 4), 34);
}

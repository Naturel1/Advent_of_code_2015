pub fn first_star() -> i32{
    let input = include_str!("day_1.txt");
    let mut floor: i32 = 0;
    for c in input.chars(){
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

pub fn bonus_star() -> i32{
    let input = include_str!("day_1.txt");
    let mut floor: i32 = 0;
    let mut counter: i32 = 0;
    for c in input.chars(){
        counter += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return counter;
        }
    }
    -1
}
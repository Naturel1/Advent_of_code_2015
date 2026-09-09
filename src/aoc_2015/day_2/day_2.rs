pub struct Box {
    pub l: i32,
    pub w: i32,
    pub h: i32,
}

impl Box {
    pub fn from_str(line: &str) -> Self {
        let mut parts = line.split('x')
                .map(|x| x.parse::<i32>()
                .expect("Invalid input"));

        Self {
            l: parts.next().expect("Missing l"),
            w: parts.next().expect("Missing w"),
            h: parts.next().expect("Missing h"),
        }
    }
    pub fn smallest_side(&self) -> i32 {
        (self.l*self.w).min((self.w*self.h).min(self.h*self.l))
    }

    pub fn sorted_vec(&self) -> Vec<i32> {
        let mut result: Vec<i32> = vec![self.l, self.w, self.h];
        result.sort();
        result
    }
}

pub fn first_star() -> i32{
    let input: Vec<Box> = include_str!("day_2.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(Box::from_str).collect();
    let mut result: i32 = 0;
    for x in input {
        result += (2*x.l*x.w) + (2*x.w*x.h) + (2*x.h*x.l);
        result += x.smallest_side();
    }
    result
}

pub fn bonus_star() -> i32{
    let input: Vec<Box> = include_str!("day_2.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(Box::from_str).collect();
    let mut result: i32 = 0;
    for x in input {
        let sorted_sizes = x.sorted_vec();
        result += (sorted_sizes[0] * 2) + (sorted_sizes[1] * 2);
        result += x.h * x.w * x.l;
    }
    result
}
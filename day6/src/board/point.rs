pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x + 1, y: y + 1 }
    }
    pub fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<&String> for Point {
    fn from(input: &String) -> Point {
        let split = input.split(", ").collect::<Vec<&str>>();
        Point::new(
            split[0].parse::<i32>().unwrap(),
            split[1].parse::<i32>().unwrap(),
        )
    }
}

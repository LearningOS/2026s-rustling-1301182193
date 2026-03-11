// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

impl Clone for Point {
    fn clone(&self) -> Point{
        Point {
            x : self.x,
            y : self.y,
        }
    }
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    let x: Option<Point> = y.clone();
    match x {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

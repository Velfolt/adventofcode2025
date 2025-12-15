use std::ops::{Add, Mul, Sub};

use itertools::Itertools;

pub trait PrintGrid {
    fn print_grid(&self, width: usize);
}

impl PrintGrid for Vec<char> {
    fn print_grid(&self, width: usize) {
        for chunk in self.chunks(width) {
            let str: String = chunk.iter().collect();
            println!("{}", str);
        }
        println!("");
    }
}

impl PrintGrid for Vec<u32> {
    fn print_grid(&self, width: usize) {
        for chunk in self.chunks(width) {
            println!("{:?}", chunk.iter().map(|x| format!("{}", x)).join(""));
        }
        println!("");
    }
}

pub trait PosWithinBounds {
    fn within_bounds(self, width: usize) -> bool;
}

impl PosWithinBounds for (i64, i64) {
    fn within_bounds(self, width: usize) -> bool {
        self.0 >= 0 && self.0 < width as i64 && self.1 >= 0 && self.1 < width as i64
    }
}

impl PosToIndex for (i64, i64) {
    fn to_index(self, width: usize) -> usize {
        (self.0 + self.1 * width as i64) as usize
    }
}

pub trait PosToIndex {
    fn to_index(self, width: usize) -> usize;
}

impl IndexToPos for usize {
    fn to_pos(self, width: usize) -> (i64, i64) {
        (self as i64 % width as i64, self as i64 / width as i64)
    }
}

pub trait IndexToPos {
    fn to_pos(self, width: usize) -> (i64, i64);
}

pub trait Distance {
    fn distance(self) -> usize;
}

impl Distance for ((i64, i64), (i64, i64)) {
    fn distance(self) -> usize {
        let (a, b) = self;
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
    }
}

impl Distance for ((i64, i64, i64), (i64, i64, i64)) {
    fn distance(self) -> usize {
        let (a, b) = self;

        ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)).isqrt() as usize
    }
}

pub trait Directions {
    fn directions(&self) -> [(i64, i64); 4];
    fn all_directions(&self) -> [(i64, i64); 8];
}

impl Directions for (i64, i64) {
    fn directions(&self) -> [(i64, i64); 4] {
        [
            (self.0 - 1, self.1),
            (self.0 + 1, self.1),
            (self.0, self.1 + 1),
            (self.0, self.1 - 1),
        ]
    }

    fn all_directions(&self) -> [(i64, i64); 8] {
        [
            (self.0 - 1, self.1),
            (self.0 - 1, self.1 - 1),
            (self.0, self.1 - 1),
            (self.0 + 1, self.1 - 1),
            (self.0 + 1, self.1),
            (self.0 + 1, self.1 + 1),
            (self.0, self.1 + 1),
            (self.0 - 1, self.1 + 1),
        ]
    }
}

pub struct Point(pub (i64, i64));

impl Sub for Point {
    type Output = (i64, i64);

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0.0 - rhs.0.0, self.0.1 - rhs.0.1)
    }
}

impl Add for Point {
    type Output = (i64, i64);

    fn add(self, rhs: Self) -> Self::Output {
        (self.0.0 + rhs.0.0, self.0.1 + rhs.0.1)
    }
}

impl Mul<i64> for Point {
    type Output = (i64, i64);

    fn mul(self, rhs: i64) -> Self::Output {
        (self.0.0 * rhs, self.0.1 * rhs)
    }
}


pub fn plot(
    p1: (i64, i64),
    p2: (i64, i64),
    mut plot: impl FnMut((i64, i64)) -> Option<()>,
) -> Option<()> {
    if (p2.1 - p1.1).abs() < (p2.0 - p1.0).abs() {
        if p1.0 > p2.0 {
            plot_line_low(p2, p1, plot)
        } else {
            plot_line_low(p1, p2, plot)
        }
    } else {
        if p1.1 > p2.1 {
            plot_line_high(p2, p1, plot)
        } else {
            plot_line_high(p1, p2, plot)
        }
    }

}

fn plot_line_low(
    p1: (i64, i64),
    p2: (i64, i64),
    mut plot: impl FnMut((i64, i64)) -> Option<()>,
) -> Option<()> {
    let dx = p2.0 - p1.0;
    let mut dy = p2.1 - p1.1;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }

    let mut D = (2 * dy) - dx;
    let mut y = p1.1;

    for x in p1.0..=p2.0 {
        if let None = plot((x, y)) {
            return None;
        }

        if D > 0 {
            y = y + yi;
            D = D + (2 * (dy - dx));
        } else {
            D = D + 2 * dy
        }
    }

    Some(())
}

fn plot_line_high(
    p1: (i64, i64),
    p2: (i64, i64),
    mut plot: impl FnMut((i64, i64)) -> Option<()>,
) -> Option<()> {
    let mut dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }

    let mut D = (2 * dx) - dy;
    let mut x = p1.0;

    for y in p1.1..=p2.1 {
        if let None = plot((x, y)) {
            return None;
        }

        if D > 0 {
            x = x + xi;
            D = D + (2 * (dx - dy));
        } else {
            D = D + 2 * dx
        }
    }

    Some(())
}

pub enum LineIntersection {
    Parallel,
    Intersects,
    None,
}

pub trait LineIntersect {
    fn line_intersect(&self, other_line: &((i64, i64), (i64, i64))) -> LineIntersection;
}

impl LineIntersect for ((i64, i64), (i64, i64)) {
    fn line_intersect(&self, other_line: &((i64, i64), (i64, i64))) -> LineIntersection {
        let ((x1, y1), (x2, y2)) = self;
        let ((x3, y3), (x4, y4)) = other_line;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if den == 0 {
            return LineIntersection::Parallel;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64 / den as f64;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f64 / den as f64;

        let intersects = t > 0.0 && t < 1.0 && u > 0.0 && u < 1.;

        if intersects {
            LineIntersection::Intersects
        } else {
            LineIntersection::None
        }
    }
}

pub trait WindingNumber {
    /// The winding number goes through all edges in a single
    /// direction. For every edge it determines if the edge goes
    /// upwards or downwards.
    ///
    /// If the result is non-zero it is outside of the polygon.
    fn winding_number(&self, edges: &Vec<(&(i64, i64), &(i64, i64))>) -> i64;
}

impl WindingNumber for (i64, i64) {
    fn winding_number(&self, edges: &Vec<(&(i64, i64), &(i64, i64))>) -> i64 {
        let line = (*self, (self.0 + 100000000000, self.1));
        edges
            .iter()
            .map(|(a, b)| (if a.1 <= b.1 { -1 } else { 1 }, a, b))
            //.filter(|(sign, a, b)| a.0 >= self.0 && b.0 >= self.0)
            .filter_map(|(sign, a, b)| match line.line_intersect(&(**a, **b)) {
                LineIntersection::Parallel => {
                    if self.0 == a.0 && self.0 == b.0 {
                        Some(sign)
                    } else {
                        None
                    }
                }
                LineIntersection::Intersects => Some(sign),
                LineIntersection::None => None,
            })
            .sum()

    }
}

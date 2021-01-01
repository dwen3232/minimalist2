use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, Copy)]
pub struct Row {
    x1: i32,
    x2: i32,
    y: i32,
}

impl Row {
    pub fn new(x1: i32, x2: i32, y: i32) -> Self {
        Row{x1, x2, y}
    }

    pub fn full_image(width: u32, height: u32) -> Vec<Self> {
        let (width, height) = (width as i32, height as i32);
        let mut vec = Vec::new();
        let (x1, x2) = (0, width-1);
        for y in 0..height {
           vec.push(Self::new(x1, x2, y));
        }
        vec
    }
}

impl From<Row> for (i32, i32, i32) {
    fn from(row: Row) -> Self {
        (row.x1, row.x2, row.y)
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x1, self.x2, self.y)
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}
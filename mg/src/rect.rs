#[derive(PartialEq, Clone, Debug)]
pub struct Rect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize
}

impl Rect {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Rect {
        Rect { x1: x1, y1: y1, x2: x2, y2: y2 }
    }

    // check if Rect self intersects Rect other
    pub fn intersects(&self, other: &Rect) -> bool {
        (self.x1.saturating_sub(3) < other.x2 &&
            self.x2.saturating_add(4) > other.x1 &&
         self.y1.saturating_sub(3) < other.y2 &&
            self.y2.saturating_add(4) > other.y1)
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    // this assumes the rectanges do not intersect.
    pub fn distance(&self, other: &Rect) -> usize {
        let left = other.x2 < self.x1;
        let right = self.x2 < other.x1;
        let bottom = other.y2 < self.y1;
        let top = self.y2 < other.y1;

        let mut dist =
            |x1: usize, y1: usize, x2: usize, y2: usize| {
                (((x2.saturating_sub(x1)).pow(2) +
                  (y2.saturating_sub(y1)).pow(2))as f64).sqrt() as usize
            };

        if top && left {
            return dist(self.x1, other.y2, other.x2, other.y1);
        } else if left && bottom {
            return dist(self.x1, self.y1, other.x2, other.y2);
        } else if bottom && right {
            return dist(self.x2, self.y1, other.x1, other.y2);
        } else if right && top {
            return dist(self.x2, self.y2, other.y1, other.y2);
        } else if left {
            return self.x1 - other.x2;
        } else if right {
            return other.x1 - self.x2;
        } else if bottom {
            return self.y1 - other.y2;
        } else if top {
            return other.y1 - self.y2;
        } else {
            // nada, it intersects
            return 0;
        }
    }
}

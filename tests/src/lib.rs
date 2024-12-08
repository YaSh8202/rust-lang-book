pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectange) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectange::new(8, 7);
        let smaller = Rectange::new(5, 1);

        assert!(larger.can_hold(&smaller));
    }



    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectange::new(8, 7);
        let smaller = Rectange::new(5, 1);

        assert!(!smaller.can_hold(&larger));
    }
    

}

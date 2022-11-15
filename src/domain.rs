use std::fmt;
use std::fmt::Formatter;
use crate::game_loop::GameResult;

pub mod boneyard;
pub mod sets;
pub mod table;
pub mod tiles;
pub mod player_rack;
pub mod initial_meld;

// FYI, doing this instead of mod.rs is the 'preferred' convention

#[derive(Debug)]
pub struct RummikubError;

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone)]
pub struct ScoreValue {
    total: u16
}


impl ScoreValue {
    pub fn test() {
        let left = ScoreValue{total: 9};

    }
}

impl fmt::Display for ScoreValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.total)
    }
}

impl std::ops::Add<ScoreValue> for ScoreValue {
    type Output = ScoreValue;
    fn add(self, rhs: ScoreValue) -> Self::Output {
        let new_val = self.total + rhs.total;
        ScoreValue{total: new_val}
    }
}

impl std::ops::Mul<u8> for ScoreValue {
    type Output = ScoreValue;
    fn mul(self, rhs: u8) -> Self::Output {
        let total = self.total * rhs as u16;
        ScoreValue{total}
    }
}

impl std::ops::AddAssign for ScoreValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {total: self.total + rhs.total};
    }    
}

#[cfg(test)]
pub mod quicktests {
    use crate::domain::ScoreValue;

    #[test]
    fn quick_test_of_score_syntactic_sugar() {
        let left = ScoreValue{total: 9};
        let right = ScoreValue{total: 20};
        let mut sum = left + right;
        println!("sum: {:?}", &sum);
        sum += left;
        println!("plus eq {:?}", sum);
    }
}
use std::{ops::Deref, sync::Arc};

pub trait DiceTrait: 'static + Send + Sync {
    fn roll(&self) -> RollResult;
}

#[derive(Clone)]
pub struct Dice {
    inner: Arc<dyn DiceTrait>,
}

impl Default for Dice {
    fn default() -> Self {
        RegularDice.into()
    }
}

impl<T: DiceTrait> From<Arc<T>> for Dice {
    fn from(inner: Arc<T>) -> Self {
        Self { inner }
    }
}

impl<T: DiceTrait> From<T> for Dice {
    fn from(inner: T) -> Self {
        Arc::new(inner).into()
    }
}

impl Deref for Dice {
    type Target = dyn DiceTrait;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

pub struct RegularDice;

impl DiceTrait for RegularDice {
    fn roll(&self) -> RollResult {
        RollResult::new(rand::random::<u8>() % 6 + 1).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct RollResult(u8);

impl RollResult {
    pub fn new(value: u8) -> Option<Self> {
        if value > 0 && value < 7 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn iter() -> impl Iterator<Item = RollResult> {
        (1..=6).map(|v| Self(v))
    }
}

impl Deref for RollResult {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Calculate the score of a roll. If combination is not valid, will return unused dice.
/// Will return `Err(vec![])` for empty rolls.
pub fn get_score(mut roll: Vec<RollResult>) -> Result<u32, Vec<RollResult>> {
    if roll.is_empty() {
        return Err(roll);
    }

    let mut score = 0;

    // Handle all triplets
    let mut prev_len = 0;
    while prev_len != roll.len() {
        prev_len = roll.len();

        for expected in RollResult::iter() {
            if let Some(rest) = find_combination(roll.clone(), expected, 3) {
                score += if *expected == 1 {
                    1000
                } else {
                    *expected as u32 * 100
                };

                roll = rest;
                break;
            }
        }
    }

    // Handle all single 1s and 5s
    let mut prev_len = 0;
    while prev_len != roll.len() {
        prev_len = roll.len();

        if let Some(rest) = find_combination(roll.clone(), RollResult(1), 1) {
            score += 100;
            roll = rest;
        }

        if let Some(rest) = find_combination(roll.clone(), RollResult(5), 1) {
            score += 50;
            roll = rest;
        }
    }

    if roll.is_empty() {
        Ok(score)
    } else {
        Err(roll)
    }
}

/// Try to find a combination in the roll, if found will return rest of the roll
fn find_combination(
    mut roll: Vec<RollResult>,
    target_result: RollResult,
    amount: usize,
) -> Option<Vec<RollResult>> {
    let mut found = 0;

    roll.retain(|r| {
        if *r == target_result && found < amount {
            found += 1;
            false
        } else {
            true
        }
    });

    if found >= amount {
        Some(roll)
    } else {
        None
    }
}

#[test]
fn test_get_score() {
    fn f(v: &[u8]) -> Result<u32, Vec<u8>> {
        get_score(v.into_iter().cloned().map(RollResult).collect())
            .map_err(|v| v.into_iter().map(|r| *r).collect())
    }

    assert_eq!(f(&[]), Err(vec![]));
    assert_eq!(f(&[1, 1, 1, 5, 5, 5]), Ok(1500));
    assert_eq!(f(&[1, 1, 1, 5, 5, 6]), Err(vec![6]));
    assert_eq!(f(&[1]), Ok(100));
    assert_eq!(f(&[1, 1]), Ok(200));
    assert_eq!(f(&[1, 1, 1]), Ok(1000));
    assert_eq!(f(&[1, 1, 1, 1]), Ok(1100));
    assert_eq!(f(&[1, 1, 1, 1, 1]), Ok(1200));
    assert_eq!(f(&[1, 1, 1, 1, 1, 1]), Ok(2000));
    assert_eq!(f(&[4]), Err(vec![4]));
    assert_eq!(f(&[4, 4]), Err(vec![4, 4]));
    assert_eq!(f(&[4, 4, 4]), Ok(400));
    assert_eq!(f(&[4, 4, 4, 4]), Err(vec![4]));
    assert_eq!(f(&[4, 4, 4, 5]), Ok(450));
}

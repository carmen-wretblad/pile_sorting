pub const MAX_NBR_OF_PILES: usize = 10;
pub const MIN_NBR_OF_PILES: usize = 3; //unsure if this should be 3 or 4
pub const MAX_NBR_OF_CARDS: usize = 100;
pub const MIN_NBR_OF_CARDS: usize = 3; // Never set lower than 2

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    const fn config_check() {
        assert!(
            MIN_NBR_OF_CARDS > 2,
            "Internal logic of Board requires 2 cards to determine if a board is solved. New creation requires 3 however "
        );
        assert!(
            MIN_NBR_OF_PILES > 2,
            "There is no solution if there is not at least 3 piles"
        );
        assert!(MAX_NBR_OF_CARDS >= MIN_NBR_OF_CARDS);
        assert!(MAX_NBR_OF_PILES >= MIN_NBR_OF_CARDS);
    }
}

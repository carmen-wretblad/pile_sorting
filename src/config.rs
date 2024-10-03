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
        assert!(MAX_NBR_OF_CARDS >= MIN_NBR_OF_CARDS);
    }
}

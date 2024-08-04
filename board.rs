struct Move {}
struct Board {
    piles: Vec<u8>,
    position_translator: Vec<u8>,
    past_move: Move,
    size: u8,
    solution_pile_pos: Option<u8>,
}

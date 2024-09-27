#[derive(Clone, Debug)]
pub struct Translator {
    abs_to_rel_translator: Vec<usize>,
}
impl Translator {
    pub fn new(nbr_piles: usize) -> Self {
        Self {
            abs_to_rel_translator: (0..nbr_piles).collect(),
        }
    }
    pub fn relative_piles(&self, piles: &[Vec<u8>]) -> Vec<u8> {
        let start: u8 = 200;
        let end: u8 = 222;
        let mut piles_in_rel_order = Vec::new();
        for i in 0..piles.len() {
            piles_in_rel_order.push(start);
            piles_in_rel_order.append(&mut piles[self.into_abs(i)].clone());
            piles_in_rel_order.push(end);
        }
        piles_in_rel_order
    }

    pub fn update_indexes(&mut self, piles: &[Vec<u8>]) {
        let mut non_empty_piles = Vec::<usize>::new();
        let mut empty_piles = Vec::<usize>::new();
        for (i, el) in piles.iter().enumerate() {
            match el.is_empty() {
                true => empty_piles.push(i),
                false => non_empty_piles.push(i),
            }
        }
        non_empty_piles.sort_by(|a, b| piles[*b][0].cmp(&piles[*a][0]));
        let mut counter = 0;
        for pile in &non_empty_piles {
            self.abs_to_rel_translator[*pile] = counter;
            counter += 1;
        }
        for pile in empty_piles {
            self.abs_to_rel_translator[pile] = counter;
            counter += 1;
        }
        // order rel based on highest card
    }
    pub fn into_abs(&self, rel: usize) -> usize {
        self.abs_to_rel_translator
            .iter()
            .position(|x| *x == rel)
            .expect("All values should be present")
    }
    pub fn into_rel(&self, abs: usize) -> usize {
        self.abs_to_rel_translator[abs]
    }
    pub fn into_abs_move(&self, rel: [usize; 2]) -> [usize; 2] {
        [
            self.abs_to_rel_translator
                .iter()
                .position(|x| *x == rel[0])
                .expect("All values should be present"),
            self.abs_to_rel_translator
                .iter()
                .position(|x| *x == rel[1])
                .expect("All values should be present"),
        ]
    }
    pub fn into_rel_move(&self, abs: [usize; 2]) -> [usize; 2] {
        [
            self.abs_to_rel_translator[abs[0]],
            self.abs_to_rel_translator[abs[1]],
        ]
    }
    pub fn into_rel_vector(&self, abs_vec: &Vec<[usize; 2]>) -> Vec<[usize; 2]> {
        abs_vec
            .iter()
            .map(|abs_move| self.into_rel_move(*abs_move))
            .collect()
    }
}

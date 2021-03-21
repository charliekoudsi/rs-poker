pub struct Isomorph {
    card1: u8,
    card2: u8,
    suited: bool,
}

impl Isomorph {
    pub fn new(card1: u8, card2: u8, suited: bool) -> Self {
        Self {
            card1,
            card2,
            suited,
        }
    }

    pub fn gen_combos(&self) -> Vec<(u8, u8)> {
        if self.card1 == self.card2 {
            return Self::gen_pairs(self.card1 * 4);
        }
        if self.suited {
            if self.card1 < self.card2 {
                return Self::gen_suited(self.card1 * 4, self.card2 * 4);
            }
            return Self::gen_suited(self.card2 * 4, self.card1 * 4);
        }

        if self.card1 < self.card2 {
            return Self::gen_offsuit(self.card1 * 4, self.card2 * 4);
        }
        return Self::gen_offsuit(self.card2 * 4, self.card1 * 4);
    }

    fn gen_pairs(value: u8) -> Vec<(u8, u8)> {
        let mut combos: Vec<(u8, u8)> = Vec::with_capacity(6);
        combos.push((value, value + 1));
        combos.push((value, value + 2));
        combos.push((value, value + 3));
        combos.push((value + 1, value + 2));
        combos.push((value + 1, value + 3));
        combos.push((value + 2, value + 3));
        return combos;
    }

    fn gen_suited(value1: u8, value2: u8) -> Vec<(u8, u8)> {
        let mut combos: Vec<(u8, u8)> = Vec::with_capacity(4);
        combos.push((value1, value2));
        combos.push((value1 + 1, value2 + 1));
        combos.push((value1 + 2, value2 + 2));
        combos.push((value1 + 3, value2 + 3));
        return combos;
    }

    fn gen_offsuit(value1: u8, value2: u8) -> Vec<(u8, u8)> {
        let mut combos: Vec<(u8, u8)> = Vec::with_capacity(12);
        combos.push((value1, value2 + 1));
        combos.push((value1, value2 + 2));
        combos.push((value1, value2 + 3));
        combos.push((value1 + 1, value2));
        combos.push((value1 + 1, value2 + 2));
        combos.push((value1 + 1, value2 + 3));
        combos.push((value1 + 2, value2));
        combos.push((value1 + 2, value2 + 1));
        combos.push((value1 + 2, value2 + 3));
        combos.push((value1 + 3, value2));
        combos.push((value1 + 3, value2 + 1));
        combos.push((value1 + 3, value2 + 2));

        return combos;
    }
}

pub fn gen_ranges(hands: &[Isomorph], flop: &[u8; 3]) -> Vec<(u8, u8)> {
    let mut range = Vec::new();
    for hand in hands {
        let combos = hand.gen_combos();
        for combo in combos {
            if combo.0 != flop[0]
                && combo.0 != flop[1]
                && combo.0 != flop[2]
                && combo.1 != flop[0]
                && combo.1 != flop[1]
                && combo.1 != flop[2]
            {
                range.push(combo);
            }
        }
    }
    return range;
}

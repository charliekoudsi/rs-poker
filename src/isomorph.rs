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

    fn char_to_card(card: char) -> Option<u8> {
        match card {
            '2' => Some(0),
            '3' => Some(1),
            '4' => Some(2),
            '5' => Some(3),
            '6' => Some(4),
            '7' => Some(5),
            '8' => Some(6),
            '9' => Some(7),
            'T' => Some(8),
            'J' => Some(9),
            'Q' => Some(10),
            'K' => Some(11),
            'A' => Some(12),
            _ => None,
        }
    }

    fn char_to_suited(suit: char) -> Option<bool> {
        match suit {
            's' => Some(true),
            'o' => Some(false),
            _ => None,
        }
    }

    pub fn new_from_str(hand: &str) -> Option<Self> {
        let chars: Vec<char> = hand.chars().collect();
        if chars.len() == 2 {
            if chars[0] == chars[1] {
                let card = Isomorph::char_to_card(chars[0])?;
                return Some(Isomorph::new(card, card, false));
            }
            return None;
        } else if chars.len() == 3 {
            let suited = Isomorph::char_to_suited(chars[2])?;
            let card1 = Isomorph::char_to_card(chars[0])?;
            let card2 = Isomorph::char_to_card(chars[1])?;
            return Some(Isomorph::new(card1, card2, suited));
        }
        return None;
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

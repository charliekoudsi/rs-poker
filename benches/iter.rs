#![feature(test)]
extern crate furry_fiesta;
extern crate test;
extern crate rand;

use furry_fiesta::core::{Deck, Hand};
use rand::{thread_rng, sample};

#[bench]
fn iter_in_deck(b: &mut test::Bencher) {
    b.iter(|| {
        let d = Deck::default();
        d.into_iter().count()
    });
}


#[bench]
fn iter_hand(b: &mut test::Bencher) {
    let mut rng = thread_rng();
    let d = Deck::default();
    let cards = sample(&mut rng, &d[..], 7).iter().map(|c| (*c).clone()).collect();
    let hand = Hand::new_with_cards(cards);

    b.iter(|| hand.into_iter().count())
}
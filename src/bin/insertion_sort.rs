use std::usize;

/*
Insertion Sort
@author: Sankar boro
*/

static CARDS: [usize; 6] = [5, 2, 56, 6, 1, 3];

struct Cards {
    cards: [usize; 6],
    pointer: usize,
    selected_card: usize,
}

impl Cards {
    
    fn set_pointer(&mut self, index: usize) {
        self.pointer = index - 1;
    }

    fn get_pointer(&self) -> usize {
        self.pointer
    }

    fn change_compare_card(&mut self) {
        self.pointer -= 1;
    }

    fn selected_card(&mut self) -> usize {
        self.selected_card
    }

    fn set_selected_card(&mut self, index: usize) {
        self.selected_card = self.cards[index];
    }

    fn compare_card(&mut self) -> usize {
        self.cards[self.pointer]
    }

    fn swap_card(&mut self) {
        self.cards[self.pointer + 1] = self.cards[self.pointer];
        self.cards[self.pointer] = self.selected_card;
    }
}

fn functional() {
    let mut cards = CARDS;

    for j in 1..cards.len() {
        let sc = cards[j];
        let mut pointer = j - 1;

        while pointer > 0 && cards[pointer] > cards[pointer + 1] {
            cards[pointer + 1] = cards[pointer];
            cards[pointer] = sc;
            pointer -= 1;
        }

        if pointer == 0 && cards[pointer] > cards[pointer + 1] {
            cards[pointer + 1] = cards[pointer];
            cards[pointer] = sc;
        }
    }
    
    println!("{:?}", cards);
}

fn object_oriented() {

    let mut cards = Cards {
        cards: CARDS,
        selected_card: 0,
        pointer: 0,
    };

    for index in 1..cards.cards.len() {
        cards.set_selected_card(index);
        cards.set_pointer(index);

        while cards.get_pointer() > 0 && cards.compare_card() > cards.selected_card() {
            cards.swap_card();
            cards.change_compare_card();
        }

        if cards.get_pointer() == 0 && cards.compare_card() > cards.selected_card() {
            cards.swap_card();
        }
    }

    println!("{:?}", cards.cards);
}

fn main() {
    object_oriented();
    functional();
}
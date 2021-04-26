use std::usize;

/*
Insertion Sort
@author: Sankar boro
*/

static CARDS: [usize; 7] = [20,100,34,55,76,5,100];

struct Cards {
    cards: [usize; 7],
    pointer: usize,
    selected_card: usize,
    compare_card: usize,
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
        self.compare_card = self.cards[self.pointer];
    }

    fn selected_card(&mut self) -> usize {
        self.selected_card = self.cards[self.pointer + 1];
        self.selected_card
    }

    fn compare_card(&mut self) -> usize {
        self.compare_card = self.cards[self.pointer];
        self.compare_card
    }

    fn swap_card(&mut self) {
        self.cards[self.pointer + 1] = self.compare_card;
        self.cards[self.pointer] = self.selected_card;
    }
}

fn main() {
    let mut cards = Cards {
        cards: CARDS,
        selected_card: 0,
        compare_card: 0,
        pointer: 0,
    };

    for index in 1..cards.cards.len() {
        cards.set_pointer(index);
        let selected_card = cards.selected_card();
        let compare_card = cards.compare_card();

        while cards.get_pointer() > 0 && compare_card > selected_card {
            cards.swap_card();
            cards.change_compare_card();
        }

        if cards.get_pointer() == 0 && compare_card > selected_card {
            cards.swap_card();
        }
    }

    println!("Cards: {:?}", cards.cards);
}

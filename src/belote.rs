use rand::seq::SliceRandom;
use rand::thread_rng;

const MAX_CARDS: usize = 32;
const N_PLAYERS: usize = 4;
const NB_CARDS: usize = MAX_CARDS / N_PLAYERS;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<u8>,
}

impl Deck {
    pub fn new_shuffled_deck() -> Deck {
        let mut cards: Vec<u8> = (0..MAX_CARDS as u8).collect();
        cards.shuffle(&mut thread_rng());

        Deck { cards }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub deck: Deck,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new_shuffled_deck(),
        }
    }

    pub fn get_cards_players(self, player_id: usize) -> Result<Vec<u8>, &'static str> {
        let result: Result<Vec<u8>, &'static str>;

        if player_id > 3 {
            result = Err("Player id should be between 0 and 3");
        } else {
            let mut card_players: Vec<u8> = (0..NB_CARDS as u8).collect();
            
            let i = NB_CARDS;
            card_players.copy_from_slice(&self.deck.cards[i * player_id..i * (player_id + 1)]);

            result = Ok(card_players);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let d = Deck::new_shuffled_deck();

        println!("{:?}", d.cards);
    }
}

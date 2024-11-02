#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

use rand::seq::SliceRandom;
fn main() {
    let mut deck: Vec<Card> = Vec::new(); // 空のVecを用意
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // 山札を作成
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }

    // 山札をシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    // println!("deck: {:#?}", deck);

    // 手札用のVecを用意
    let mut hand: Vec<Card> = Vec::new();
    // 山札から手札を5枚引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札を表示
    println!("----HAND----");
    for card in hand {
        println!("{:?} {:}", card.suit, card.rank);
    }
}

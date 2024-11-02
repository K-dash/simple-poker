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
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }

    // --------- 手札交換 ---------
    println!("入れ替えたいカードの番号を入力してください（例： 1 2 3）");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // 選ばれたカードを山札から引いたカードで置き換える
    let numbers: Vec<usize> = input
        .split_whitespace() // 空白区切りで分割（ "1 2 3" => ["1", "2", "3"] ）
        .map(|x| x.parse().unwrap()) // 文字列を数値に変換 ( "1" => 1 )
        .collect::<Vec<usize>>(); // Vecに変換

    // 与えられた数字の箇所を山札から取り出し、手札の該当箇所を置き換える
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    println!("----HAND----");
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }
}

use std::error::Error;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Strength {
    FiveOf,
    FourOf,
    FullHouse,
    ThreeOf,
    TwoPair,
    OnePair,
    HighCard,
    A,
    K,
    Q, 
    T, 
    Nine, 
    Eight, 
    Seven,
    Six, 
    Five,
    Four,
    Three,
    Two,
    J
}

impl Strength {
    fn from_hand_p1(hand: &str) -> Option<Self> {
        let mut cards: HashMap<Strength, i32> = HashMap::new();

        for i in hand.chars() {
            let card = Strength::from_char(i).unwrap();
            
            if cards.contains_key(&card) {
                *cards.get_mut(&card).unwrap() += 1;
            } else {
                cards.insert(Strength::from_char(i).unwrap(), 1);
            }
        }
        
        let mut pairs = 0;
        let mut threes = 0;
        
        for (_s, i) in cards.iter() {
            if *i == 2 {
                pairs += 1;
            } else if *i == 3 {
                threes += 1;
            } else if *i == 4 {
                return Some(Self::FourOf);
            } else if *i == 5 {
                return Some(Self::FiveOf);
            }
        }

        if threes == 1 && pairs == 1 {
            return Some(Self::FullHouse);
        } else if pairs == 2 {
            return Some(Self::TwoPair);
        } else if pairs == 1 {
            return Some(Self::OnePair);
        } else if threes == 1 {
            return Some(Self::ThreeOf);
        } else {
            return Some(Self::HighCard);
        }
    }

    fn from_hand_p2(hand: &str) -> Option<Self> {
        let mut cards: HashMap<Strength, i32> = HashMap::new();
        let mut jokers = 0;

        for i in hand.chars() {
            let card = Strength::from_char(i).unwrap();
            
            if card == Strength::J {
                jokers += 1;
                continue;
            }

            if cards.contains_key(&card) {
                *cards.get_mut(&card).unwrap() += 1;
            } else {
                cards.insert(Strength::from_char(i).unwrap(), 1);
            }
        } 

        // print!("hand: {hand}; jokers: {jokers}");
        if jokers == 5 {
            cards.insert(Strength::J, 5);
        } else {
            *cards.values_mut().max().unwrap() += jokers;
        }

        let mut pairs = 0;
        let mut threes = 0;
        
        for (_s, i) in cards.iter() {
            if *i == 2 {
                pairs += 1;
            } else if *i == 3 {
                threes += 1;
            } else if *i == 4 {
                return Some(Self::FourOf);
            } else if *i >= 5 {
                return Some(Self::FiveOf);
            }
        }

        if threes == 1 && pairs == 1 {
            return Some(Self::FullHouse);
        } else if threes == 1 {
            return Some(Self::ThreeOf);
        } else if pairs == 2 {
            return Some(Self::TwoPair);
        } else if pairs == 1 {
            return Some(Self::OnePair);
        } else {
            return Some(Self::HighCard);
        }
    }

    fn from_char(char: char) -> Option<Self> {
        match char {
            'A'  =>  Some(Strength::A),
            'K'  =>  Some(Strength::K), 
            'Q'  =>  Some(Strength::Q), 
            'J'  =>  Some(Strength::J), 
            'T'  =>  Some(Strength::T), 
            '9'  =>  Some(Strength::Nine), 
            '8'  =>  Some(Strength::Eight), 
            '7'  =>  Some(Strength::Seven),
            '6'  =>  Some(Strength::Six), 
            '5'  =>  Some(Strength::Five),
            '4'  =>  Some(Strength::Four),
            '3'  =>  Some(Strength::Three),
            '2'  =>  Some(Strength::Two),
            _    =>  None
        }
    }
}

#[derive(Debug)]
struct Hand {
    str: Strength,
    cards: Vec<Strength>,
    bid: i32
}


fn part1(contents: &str) -> Result<()> {
    // Parse and sort after strength lowest comes first
    let mut hands: Vec<Hand> = vec![];
    for l in contents.lines() {
        let h = l.split(" ").nth(0).unwrap();
        let b = l.split(" ").nth(1).unwrap();

        hands.push(
            Hand  {
                str: Strength::from_hand_p1(h).unwrap(),
                bid: b.parse::<i32>().unwrap(),
                cards: h.chars().map(|c| { Strength::from_char(c).unwrap() }).collect::<Vec<Strength>>(),
            }
        )
    }

    hands.sort_by(|a,b|  {
        if a.str == b.str {
            let mut acards = a.cards.iter();
            let mut bcards = b.cards.iter();
            return loop {
                let ac = acards.next().unwrap();
                let bc = bcards.next().unwrap();

                if ac != bc  {
                    break ac.cmp(bc);
                }
            }
        } else {
            a.str.cmp(&b.str)
        }
    });
    hands.reverse();
    
    let val = hands
        .iter()
        .enumerate()
        .fold(0, |s, (i, j) | { s + ((i as i32 + 1) * j.bid) });

    println!("{}", val);

    Ok(())
}


fn part2(contents: &str) -> Result<()> {
    // Parse and sort after strength lowest comes first
    let mut hands: Vec<Hand> = vec![];
    for l in contents.lines() {
        let h = l.split(" ").nth(0).unwrap();
        let b = l.split(" ").nth(1).unwrap();

        hands.push(
            Hand  {
                str: Strength::from_hand_p2(h).unwrap(),
                bid: b.parse::<i32>().unwrap(),
                cards: h.chars().map(|c| { Strength::from_char(c).unwrap() }).collect::<Vec<Strength>>(),
            }
        );
    }

    hands.sort_by(|a,b|  {
        if a.str == b.str {
            let mut acards = a.cards.iter();
            let mut bcards = b.cards.iter();
            return loop {
                let ac = acards.next().unwrap();
                let bc = bcards.next().unwrap();

                if ac != bc  {
                    break ac.cmp(bc);
                }
            }
        } else {
            a.str.cmp(&b.str)
        }
    });
    hands.reverse();
    

    let val = hands
        .iter()
        .enumerate()
        .fold(0, |s, (i, j) | {
            let nex = s + ((i as i32 + 1) * j.bid);
            println!("{j:?}: {nex}");
            nex
        });

    println!("{}", val);

    Ok(())
}


fn main() -> Result<()>{
    let file = std::env::args().nth(1).expect("No File Specified");
    let contents = std::fs::read_to_string(file)?;
    //part1(&contents);
    part2(&contents);
    Ok(())
}


use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err("Invalid string length".to_string());
        }

        let chars: Vec<char> = s.chars().collect();
        let rank = match chars[0] {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => return Err("Invalid rank".to_string()),
        };

        let suit = match chars[1] {
            's' => Suit::Spades,
            'h' => Suit::Hearts,
            'd' => Suit::Diamonds,
            'c' => Suit::Clubs,
            _ => return Err("Invalid suit".to_string()),
        };

        Ok(Card { rank, suit })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandCategory {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BestHand {
    pub category: HandCategory,
    pub tie_breaker: Vec<Rank>,
    pub cards: Vec<Card>,
}

impl PartialOrd for BestHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BestHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.category.cmp(&other.category)
            .then_with(|| self.tie_breaker.cmp(&other.tie_breaker))
    }
}

pub fn analyze_5_cards(cards: &[Card]) -> BestHand {
    if cards.len() != 5 {
        panic!("analyze_5_cards requires exactly 5 cards");
    }

    let mut rank_counts = std::collections::HashMap::new();
    let mut suit_counts = std::collections::HashMap::new();
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
        *suit_counts.entry(card.suit).or_insert(0) += 1;
    }

    let is_flush = suit_counts.values().any(|&c| c >= 5);
    
    let mut rank_stats: Vec<(usize, Rank)> = rank_counts.into_iter().map(|(r, c)| (c, r)).collect();
    rank_stats.sort_unstable_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));

    let mut unique_ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    unique_ranks.sort_unstable_by(|a, b| b.cmp(a));
    unique_ranks.dedup();

    let mut is_straight = false;
    let mut straight_high_rank = Rank::Two;

    if unique_ranks.len() == 5 {
        if unique_ranks[0] as u8 - unique_ranks[4] as u8 == 4 {
            is_straight = true;
            straight_high_rank = unique_ranks[0];
        } else if unique_ranks == vec![Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
            is_straight = true;
            straight_high_rank = Rank::Five;
        }
    }

    let category;
    let tie_breaker;

    if is_flush && is_straight {
        category = HandCategory::StraightFlush;
        tie_breaker = vec![straight_high_rank];
    } else if rank_stats[0].0 == 4 {
        category = HandCategory::FourOfAKind;
        tie_breaker = vec![rank_stats[0].1, rank_stats[1].1];
    } else if rank_stats[0].0 == 3 && rank_stats[1].0 == 2 {
        category = HandCategory::FullHouse;
        tie_breaker = vec![rank_stats[0].1, rank_stats[1].1];
    } else if is_flush {
        category = HandCategory::Flush;
        tie_breaker = unique_ranks.clone();
    } else if is_straight {
        category = HandCategory::Straight;
        tie_breaker = vec![straight_high_rank];
    } else if rank_stats[0].0 == 3 {
        category = HandCategory::ThreeOfAKind;
        tie_breaker = vec![rank_stats[0].1, rank_stats[1].1, rank_stats[2].1];
    } else if rank_stats[0].0 == 2 && rank_stats[1].0 == 2 {
        category = HandCategory::TwoPair;
        tie_breaker = vec![rank_stats[0].1, rank_stats[1].1, rank_stats[2].1];
    } else if rank_stats[0].0 == 2 {
        category = HandCategory::Pair;
        tie_breaker = rank_stats.iter().map(|s| s.1).collect();
    } else {
        category = HandCategory::HighCard;
        tie_breaker = unique_ranks.clone();
    }

    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_unstable_by(|a, b| {
        let count_a = cards.iter().filter(|c| c.rank == a.rank).count();
        let count_b = cards.iter().filter(|c| c.rank == b.rank).count();
        count_b.cmp(&count_a)
            .then_with(|| b.rank.cmp(&a.rank))
            .then_with(|| b.suit.cmp(&a.suit)) 
    });
    
    if is_straight && straight_high_rank == Rank::Five {
        if let Some(pos) = sorted_cards.iter().position(|c| c.rank == Rank::Ace) {
            let ace = sorted_cards.remove(pos);
            sorted_cards.push(ace);
        }
    }

    BestHand {
        category,
        tie_breaker,
        cards: sorted_cards,
    }
}

pub fn get_combinations(cards: &[Card]) -> Vec<Vec<Card>> {
    let mut result = Vec::new();
    let n = cards.len();
    if n < 5 { return result; }
    
    let mut indices: Vec<usize> = (0..5).collect();
    loop {
        let combo: Vec<Card> = indices.iter().map(|&i| cards[i]).collect();
        result.push(combo);
        
        let mut i = 5;
        while i > 0 && indices[i - 1] == i - 1 + n - 5 {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        indices[i - 1] += 1;
        for j in i..5 {
            indices[j] = indices[j - 1] + 1;
        }
    }
    result
}

pub fn evaluate_7_cards(cards: &[Card]) -> BestHand {
    if cards.len() != 7 {
        panic!("evaluate_7_cards requires exactly 7 cards");
    }
    let combos = get_combinations(cards);
    combos.into_iter()
        .map(|combo| analyze_5_cards(&combo))
        .max()
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hole_cards: [Card; 2],
}

pub fn determine_winners(community_cards: &[Card; 5], players: &[Player]) -> Vec<(String, BestHand)> {
    let mut results: Vec<(String, BestHand)> = players.iter().map(|p| {
        let mut available_cards = community_cards.to_vec();
        available_cards.extend_from_slice(&p.hole_cards);
        
        let best_hand = evaluate_7_cards(&available_cards);
        (p.name.clone(), best_hand)
    }).collect();
    
    if results.is_empty() { return vec![]; }
    
    results.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    
    let best_val = results[0].1.clone();
    results.into_iter().filter(|(_, hand)| hand.cmp(&best_val) == std::cmp::Ordering::Equal).collect()
}

fn main() {
    println!("Texas Hold'em Poker Hand Evaluator");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let card = Card::from_str("As").unwrap();
        assert_eq!(card.rank, Rank::Ace);
        assert_eq!(card.suit, Suit::Spades);
    }

    #[test]
    fn test_analyze_5_cards_categories() {
        let high_card = analyze_5_cards(&[
            Card::from_str("2s").unwrap(), Card::from_str("4d").unwrap(),
            Card::from_str("6h").unwrap(), Card::from_str("9c").unwrap(),
            Card::from_str("Ks").unwrap(),
        ]);
        assert_eq!(high_card.category, HandCategory::HighCard);

        let pair = analyze_5_cards(&[
            Card::from_str("2s").unwrap(), Card::from_str("2d").unwrap(),
            Card::from_str("6h").unwrap(), Card::from_str("9c").unwrap(),
            Card::from_str("Ks").unwrap(),
        ]);
        assert_eq!(pair.category, HandCategory::Pair);

        let two_pair = analyze_5_cards(&[
            Card::from_str("2s").unwrap(), Card::from_str("2d").unwrap(),
            Card::from_str("6h").unwrap(), Card::from_str("6c").unwrap(),
            Card::from_str("Ks").unwrap(),
        ]);
        assert_eq!(two_pair.category, HandCategory::TwoPair);

        let three_kind = analyze_5_cards(&[
            Card::from_str("2s").unwrap(), Card::from_str("2d").unwrap(),
            Card::from_str("2h").unwrap(), Card::from_str("6c").unwrap(),
            Card::from_str("Ks").unwrap(),
        ]);
        assert_eq!(three_kind.category, HandCategory::ThreeOfAKind);

        let straight = analyze_5_cards(&[
            Card::from_str("7s").unwrap(), Card::from_str("8d").unwrap(),
            Card::from_str("9h").unwrap(), Card::from_str("Tc").unwrap(),
            Card::from_str("Js").unwrap(),
        ]);
        assert_eq!(straight.category, HandCategory::Straight);

        let straight_a5 = analyze_5_cards(&[
            Card::from_str("As").unwrap(), Card::from_str("2d").unwrap(),
            Card::from_str("3h").unwrap(), Card::from_str("4c").unwrap(),
            Card::from_str("5s").unwrap(),
        ]);
        assert_eq!(straight_a5.category, HandCategory::Straight);

        let flush = analyze_5_cards(&[
            Card::from_str("2s").unwrap(), Card::from_str("4s").unwrap(),
            Card::from_str("7s").unwrap(), Card::from_str("9s").unwrap(),
            Card::from_str("Ks").unwrap(),
        ]);
        assert_eq!(flush.category, HandCategory::Flush);

        let full_house = analyze_5_cards(&[
            Card::from_str("7s").unwrap(), Card::from_str("7d").unwrap(),
            Card::from_str("7h").unwrap(), Card::from_str("9c").unwrap(),
            Card::from_str("9s").unwrap(),
        ]);
        assert_eq!(full_house.category, HandCategory::FullHouse);

        let four_kind = analyze_5_cards(&[
            Card::from_str("As").unwrap(), Card::from_str("Ad").unwrap(),
            Card::from_str("Ah").unwrap(), Card::from_str("Ac").unwrap(),
            Card::from_str("Ks").unwrap(),
        ]);
        assert_eq!(four_kind.category, HandCategory::FourOfAKind);

        let straight_flush = analyze_5_cards(&[
            Card::from_str("7s").unwrap(), Card::from_str("8s").unwrap(),
            Card::from_str("9s").unwrap(), Card::from_str("Ts").unwrap(),
            Card::from_str("Js").unwrap(),
        ]);
        assert_eq!(straight_flush.category, HandCategory::StraightFlush);
    }

    #[test]
    fn test_evaluate_7_cards() {
        let cards = vec![
            Card::from_str("2s").unwrap(), Card::from_str("3s").unwrap(),
            Card::from_str("4s").unwrap(), Card::from_str("5s").unwrap(),
            Card::from_str("6s").unwrap(), Card::from_str("7s").unwrap(),
            Card::from_str("8s").unwrap(),
        ];
        let best_hand = evaluate_7_cards(&cards);
        assert_eq!(best_hand.category, HandCategory::StraightFlush);
        assert_eq!(best_hand.tie_breaker[0], Rank::Eight);
    }

    #[test]
    fn test_determine_winners() {
        let community = [
            Card::from_str("As").unwrap(), Card::from_str("Ks").unwrap(),
            Card::from_str("Qs").unwrap(), Card::from_str("Js").unwrap(),
            Card::from_str("Ts").unwrap(), // Royal flush on board!
        ];

        let p1 = Player { name: "Alice".to_string(), hole_cards: [Card::from_str("2c").unwrap(), Card::from_str("3c").unwrap()] };
        let p2 = Player { name: "Bob".to_string(),   hole_cards: [Card::from_str("4d").unwrap(), Card::from_str("5d").unwrap()] };

        let winners = determine_winners(&community, &[p1.clone(), p2.clone()]);
        assert_eq!(winners.len(), 2); // Split pot, everyone plays the board

        let community2 = [
            Card::from_str("2s").unwrap(), Card::from_str("3s").unwrap(),
            Card::from_str("4s").unwrap(), Card::from_str("5c").unwrap(),
            Card::from_str("6d").unwrap(), 
        ];
        let p3 = Player { name: "Charlie".to_string(), hole_cards: [Card::from_str("7s").unwrap(), Card::from_str("8s").unwrap()] }; // Higher straight 4-8
        let p4 = Player { name: "Dave".to_string(),    hole_cards: [Card::from_str("Ac").unwrap(), Card::from_str("9d").unwrap()] }; // Low straight A-5
        
        let winners2 = determine_winners(&community2, &[p3, p4]);
        assert_eq!(winners2.len(), 1);
        assert_eq!(winners2[0].0, "Charlie");
    }
}

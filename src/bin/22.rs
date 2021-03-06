use std::collections::*;
use std::hash::{Hash, Hasher};

static PLAYER1: [u8; 25] = [45,10,43,46,25,36,16,38,30,15,26,34,9,2,44,1,4,40,5,24,49,3,41,19,13];
static PLAYER2: [u8; 25] = [28,50,37,20,6,42,32,47,39,22,14,7,21,17,27,8,48,11,23,12,18,35,29,33,31];

fn hash(d1: &VecDeque<u8>, d2: &VecDeque<u8>) -> u64 {
  let mut hasher = hash_map::DefaultHasher::new();
  d1.hash(&mut hasher);
  d2.hash(&mut hasher);
  hasher.finish()
}

fn score(deck: &VecDeque<u8>) -> usize {
  deck.iter()
    .rev()
    .enumerate()
    .map(|(i,&n)| (i+1) * n as usize)
    .sum()
}

fn part_one() -> usize {
  let mut p1 = PLAYER1.iter().copied().collect::<VecDeque<_>>();
  let mut p2 = PLAYER2.iter().copied().collect::<VecDeque<_>>();
  while !p1.is_empty() && !p2.is_empty() {
    let (c1,c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
    if c1 > c2 {
      p1.extend(&[c1,c2]);
    } else {
      p2.extend(&[c2,c1]);
    }
  }
  score(if p2.is_empty() {&p1} else {&p2})
}

fn game(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> usize {
  let mut seen = HashSet::new();
  while !p1.is_empty() && !p2.is_empty() {
    if !seen.insert(hash(&p1, &p2)) {
      return 1;
    }

    let c1 = p1.pop_front().unwrap() as usize;
    let c2 = p2.pop_front().unwrap() as usize;
    let winner = if c1 <= p1.len() && c2 <= p2.len() {
      let mut p1 = p1.iter().take(c1).copied().collect();
      let mut p2 = p2.iter().take(c2).copied().collect();
      game(&mut p1, &mut p2)
    } else {
      if c1 > c2 {1} else {2}
    };

    if winner == 1 {
      p1.extend(&[c1 as u8, c2 as u8]);
    } else {
      p2.extend(&[c2 as u8, c1 as u8]);
    }
  }
  if p2.is_empty() {1} else {2}
}

fn part_two() -> usize {
  let mut p1 = PLAYER1.iter().copied().collect();
  let mut p2 = PLAYER2.iter().copied().collect();
  let winner = game(&mut p1, &mut p2);
  score(if winner == 1 {&p1} else {&p2})
}

aoc2020::main! {
  (part_one(), part_two())
}

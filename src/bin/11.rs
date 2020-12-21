use itertools::Itertools;

static INPUT: [&str; 98] = ["LLLLL.LLLLLLL.LLLLLL.L.LLLL..LLLL.LLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLL.L.LLLLLLLLLLLLLL.LLLLLL","LLLLLLLLLLLLLLL.LLLL..LLLLLLLLLLLL.LLLLLLLL.L..LLLLLLLLLLLL.LLLL.LLL.LLLLL.LLL.LLLLLLLLLLL","LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLL.LLL.LLLLLLLL.LLLLL.LL.LLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLL.LLL.LLLLLLLL.LLLLLL","LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLLLL","LLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLL.L.LLLLLL.LLLLLL..LLLLLL..LLLLLLLLLLLLLL.LLLLLL",".L..L.L.L.LL..LL.........L....L.L..LL.LLLL.L..L.LLLL.L.L..L.......LL...LL..LL.L..L.L.LL..L","LLL.L.LLL.LLLLL..LLL.LLLLLL.LL.L.L.LLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLLLLLLL..LLLLL","LL.L..LLLLLLLLLLLLLL.LLLLLL.LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.LLLL.LLLLLLLLL.LLLLLLLLLLLLLLL","LLL.LLL.LLLL.LL.LLLL.LLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLL","LLLLLLLLL.LLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LL.LLL","LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LL.LLLLLL..LLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLL..LLLLLLL.LLL.LL.LLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LL.LLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLL.LLLLLLLLL..LLLLLLLL.LLLLLL.L.LLLLLLLLLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLLLLLL.LLL.LLLLL.LLLLLLLL.LLL.LLLL.LLLLLL.LLLLLL.LLLLLLL.LLLLL.LLLLLLLLL","...L..L..........L...L..LLL..L....LL......L......L.L.L...........L....L.L.LL..LL.L....LL..","LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","LLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLLLLLLLLL..LLLLL..LLLLLLL.LLLLLLLL..LLLLL.LLLLLLLL.LLLLLLLLLLL.LL.LLLLLL","LLLLL.LL.LLL.LL..LLLL.LLLLL.LL.LL..LLL.LLLL.LLLLLLL..LLLLLLLL.LLLLLL.LLLL..LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLL.L.LLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLLLLL.LLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLL..LLLLLLLL.LLLLLL","LLLLLLLLLLL.LLL.LLLL.LLLLLL.L.LLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLL","LL...L.LLL.....L....LL.....L...L.L....L.L.....L.LL...L..L.....L.L..LLL.L.LL.LLLL..........","LLLLL.LLLLLLLLL.LLLL.LLLLL.LLLLLLLLLLL.LLL..LLLL.LLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL..LLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLL..LLLLLLL.LLLLLL","LLLLLLLLLLLLLLL.LLL..LLLLLL.LLLLL.LL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLLL","LLLLL.LLL.LLLLL.LLLLLLLLLLLLLLLLLL.LLLLLLL..LLLLL.LLLLLLLLL.LLLLLLLL.LLLLL..LL.LLLL.LLLLLL","L..LL.L.....L...LL.....L.L........L..LL...L.....L....L..L.L...L...LL.......L.L....L.......","LLLLL..LLLLLLLL.LLLL.LLLLLL.LLLLLLLL.LLLL.LLLLLLL.LLLLLLLLLLLLLLLLLL..LLLL.LLLLLLLL.LLLLLL","LLLLLLLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLL.LLLL.LLL.LLLL.L.LLLLLLLLLLLLLLLLL.L.LLL.LLLLLL","LLLLL.LLLLLLLLL.LL.LLLLLLLLLLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL..LLLLL","LLLLLLLLLLLLLLL.LLL..LL.LLLLLLLLLLLLLLLLLLL.LLL.LLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLL.","LLLLL.LLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLL.L.LLLLLL.LLLLLLLLLLLLLLL.LL.LLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLL","LLLLLLLLLLLLLLL..LLL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLLL","LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLL.LL",".L.LLLLL.............L.LL.L.LLL.L.LL....L...L...L.....L..L...LLLL..L..L.......L.L.LLL.LLLL","LLLLL.LLLLLLLLLLLLLLLLLLLLL..LLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLL.LLL.LL.LLLLL.LLLLL.LLLLLL.LLLLLLLL","LLLLLLLLLL.LLL.LLLLL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLL.LLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLL.LLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLL..LLLLL.LLLLLLLLL","LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLL..LLLLLL.LLLLLLLL.LLLLL.LLLLLLLL.LLLLLL","LLLL..LLLLLLLLL..LLL.LLLLLL.L.LLLLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.L..LL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLLLL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLL..LLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLLLLLLLLL.LLL.LLLLLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLL..LLLLLL","LLLLL.LLLLL.LLL..LLL.LLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.LLL.LL",".L.......LL...L...L.LL.LL......L..L...L...L...L....LL..L...L..LLL.....L.....L.....LLL.LL..","LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLL..LLLLL.LL.LLLLL.LLLLLLL..L.LLLL","LLLLL.LLLLLLLLLLLLLLLLLLLLL.L.LLLLLLLLLLL.L.LLL.LLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLL","LLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLL.L.LLLLL.LL.LLLLL.LLLLLL","LLLL..LLLLLLLLL.LLLLLLLLLLL.LL.LLL.LLLLLLLL.LLLLLLLLLLLL.LL.LLLLLLLL.LLLLL.LLLLLLLL.LLLLLL",".......L......L....L....L......L...L.L..L...L.LL.....L......L.......LL.........L.L.L...LLL","L.LLL.LLLLLLLLL.LLLL.LLLLL..LLLLLL.LLLLLLLLLLLLLL.LL.LLLLLL.LLLLLL.L.LLLLL.LLLLL.LLL.LLLLL","LLLLL.LLLLLLLLLLL.LL.LLLLLL..LLLL..LLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLLL","LLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLL.LLLLLLLLLL.LLLLLL","LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLL.LLLL.LLL.LLLL.LLL.L..LLL.LLL.LLLL.LLLLLLL.LLLLLLLLLLLLL","LLLL.LLLLLLLLLL.L.LL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLLL.LLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLLLL.LLL..LLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLL.LLL.LLLLLLLLLLLLLL.LLLLLL",".LLLLLLLLLLLLLL.LLLLLLLLLLLLL.LLLL.LLLLLLLL.L.LLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLL.LLL..LLLLLL","LLLLLLLLLLLLLLL.LLLL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","L...L.........L....L......LL..LL.....LL.L..LLL....L.L...L..LLL.......LL.L...L..L...LL...L.","LLLLL.LLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLL.LLLLLLLLLL.LLLLLLLLL.LLLLL","LLLLL.L.LLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLL..LL.LLLLLLLLLLL","LLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLL.LLLLLLLL.L.LLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LL.LLL","LLLLL.LLLLLLLLLLLLLL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLLLLLLL..LLLLL",".L.L.L..L.L...LLLLLL.....LL...L..L..L......LLLL.L.LL.....LLLL.LL.L.....L.L...L....L...L...","LLLL..LLLLLL.LL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLLLL.LLLLLLLLL","LLLLL.LLLLLLLLL.LLLLLLLLLLL..LLLLL.L.L.LLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLL.LLLLLL.LLL.LLLLLLLLLLLLLL.LLLL..LLLLL","LLLLLL.LLLLLL.LLLLLLLLLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LL.LL..LLLLLLL.LLLLLL","LLLLL.LLLL..LLLLL.LL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLL.LLL.LLLLLL","....L........L..L...........L.L............L.....LL..L.L.L.........LLLLLL.LL..L..L.L..L...","LL.LLLLLLLLLLLL.LLLLLLLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLL.L..LLLL..LLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLLLLLLL.L.LLLLLLLL.LLLLLLLL.LLL.LL.LLLLLLLL..LLLL.LLLL.LLLLLLLLLL","LLLL.LLLLLLLLLL.LLLL.LLLLLL.LLLLLLLLLLLLL.L.LLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLL","LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLL.LLL.LLLLLLLL.LLLLLLLLLLL.LLL.LLLLL.LLLLLLL..LLLLLL","LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLL..LL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLLLL.LLLLLL.LLLLLL..LLLLLLL.LLLLL.LL.LLLLLL.LLLLLLLL.LLLLLLLLLLLLLL.LLLLLL","LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL...LLLLLL.LLLL.L.LLLLL.LL.LLLLL.LLLLLLLL.L.LLLL","LLLLLLLLLLLLLLL.LLLL.L.LLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLL.L.LLLLLL.LLLLLL","L.LLL......LLL....L.......L..L..L.....LL......L.LLLL...L.....L.L............L.LL....LL....","LLLLLLLLLLLLLLL.LLLLLLLLLLL.LLL.LL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLLLL.LLLLLL","L..LL.LLLLLLLLL.LLLLLLLLLLLLLLLLLL.LLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLLL","LLLLL.LLLLLLLLL.LLLLLLLLLLL.LLLLL..LL.LLLLL.LLLLLLLL.LLLLLL.LLLLLLLL.LLLLL.LLL.LLLL.LLLLLL","LLLLL.LL.LLLLLL.LLLL.LLLLLL.LL.LLL.LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLLL","LLLLLLLLLLLLLLL.LLLLLLL..LL.LLLL.L.LLLLLLLL.L.LLLLL..LLLLLL.LLLLLLL..LLLLLLLLLLLLLL.LLLLLL","LLL...LLLLLLLLL.LLLL.LLLLL..LLLLLLL.LLLLLLL...LLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLL.LL","LLLLLLLLLLLLLLL.LLLL.LLLLLL.LLLL.LLLLLLLLLL.LLLLLLL.LLL.LLLLLLLLLL.L.LLLLLLLLLLLLLL.L.LLLL","...L.L.L.L.....LL...L...LL.L........L...LL.L...........L..L.L.L.L..L...LL.LL.L.L.L....L..L","LLL.L.LLLLLLLLL.LLLL.L.LLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLL..LLLL.LLLLL.LL.LLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLL.LLLLLLLLLL.LLLLLLLLLLLLLLL","LLLLL.LLLLLLLLL.LLLL.LLLLLL.LLLLLL.LLLLLLLL.LLLLLLLL.LLLLLLLLLLLLLLL.LLLL..LLLLLLLL.LLLLLL","LLLLL.LLLLLLLLLLLLLL.L.LLLL.LLLL.L.LLLLLLLL.LLLLLLLL.LLL.LL.LLLLLLLL.LLLLL.LLLLLLLL.LLLLLL","LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLL.LLLLLLLL.LLLLLL"];
static DIRS: [(i64,i64); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

fn should_swap_p1(map: &[Vec<char>], i: usize, j: usize) -> bool {
  let mut neighbours = DIRS.iter()
    .map(|&(dy,dx)| (i as i64 + dy, j as i64 + dx))
    .filter_map(|(y,x)| map.get(y as usize).and_then(|v| v.get(x as usize)));
  match map[i][j] {
    'L' => neighbours.all(|&c| c != '#'),
    '#' => neighbours.filter(|&&c| c == '#').count() >= 4,
    _ => unreachable!()
  }
}

fn find_neighbour(
  map: &[Vec<char>],
  (dy, dx): (i64, i64),
  (i,j): (usize,usize),
) -> Option<char> {
  let (mut i, mut j) = (i as i64, j as i64);
  loop {
    i += dy;
    j += dx;
    match map.get(i as usize).and_then(|row| row.get(j as usize)) {
      Some('.') => {},
      Some(&c)  => return Some(c),
      None      => break,
    }
  }
  None
}

fn should_swap_p2(map: &[Vec<char>], i: usize, j: usize) -> bool {
  let mut neighbours = DIRS.iter().filter_map(|&dir| find_neighbour(&map, dir, (i,j)));
  match map[i][j] {
    'L' => neighbours.all(|c| c != '#'),
    '#' => neighbours.filter(|&c| c == '#').count() >= 5,
    _ => unreachable!()
  }
}

fn run_simulation<F: Fn(&[Vec<char>], usize, usize) -> bool>(should_swap: F) -> usize {
  let mut map = INPUT.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
  let mut to_swap = Vec::new();
  loop {
    to_swap.clear();
    for (i,j) in (0..map.len()).cartesian_product(0..map[0].len()) {
      if map[i][j] != '.' && should_swap(&map, i, j) {
        to_swap.push((i,j));
      }
    }
    for &(i,j) in &to_swap {
      map[i][j] = if map[i][j] == 'L' {'#'} else {'L'};
    }
    if to_swap.is_empty() { break; }
  }
  map.iter().flatten().filter(|&&c| c == '#').count()
}

aoc2020::main! {
  let part_one = run_simulation(should_swap_p1);
  let part_two = run_simulation(should_swap_p2);
  (part_one, part_two)
}

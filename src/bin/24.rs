use std::collections::*;

static INPUT: [&[u8]; 394] = [b"swswwneswswswswseswewwswswsewseswneesw",b"nwenwnwsenwnwsewswnenw",b"swswwswswswewswswnewswsewwswswwsw",b"nwswswswswwneswsweswenwswswwsewswswswsw",b"sewnwnwwwnwenwnwsenwnwwwnw",b"sesesesweswseeseswnwswwswwseseneswnw",b"wsenweneswseseseeeeseeneeseeeese",b"neenwneneseneeneeeneseswnwnweesenene",b"swneswsewwwwwwswwswwswneswswwsw",b"swwwswswswweswswnewswwswsewnwswwsw",b"eswnweneeenwnwnenweeseeeweswswee",b"wneneseswswwseseswswnwseeeswseseswsw",b"wnwesenwswnesewnwneeswenwnwnenwnwwnw",b"swswswswswswswseswesenwseswwswweswnwne",b"nwwenwswnwnwnwwnwnwnwenenwnwsenwnwnenw",b"nwwnenwwnwnwenwwnwwwwsewnwwsew",b"wsenwwswwswesweeseneswnwswne",b"wsenwnewnwnwseenwnwsenewsesewewne",b"nwsesenwwnwsenwnwnwnwnewswenwnenwnwnwne",b"eesenwnewesenwneswseeseseeesweew",b"seseswsenenwswswswseswswswswseseeswwsw",b"nenwnenwnwenwswnwnwnwne",b"nwswsenwseewswsenwseseesweswnesenw",b"wnwwswenwsewwnwwwnwnww",b"seswnwsenwnwseswseseswseseeseseseenwse",b"esewswwwswwnewswwwwswwwwnww",b"eneseeeeeneewseneeneenweeee",b"wneswwswwswswnwwwswswswneswsewswswe",b"swswneswswsweswswswwwnwsweswswswswswsw",b"nesenenwnwnesewnwnenwswnesenwnwnenwnwne",b"neneswnweneeenwnwwnwseseeeneewswsew",b"nenwseneeseneneswneeeeeeneenwenw",b"neeswswnwseswswsesenwswswswswneseneswse",b"wswewswwswwsewnwwneswwswswsenew",b"nwnenwseswneseswswswswnwsesesweneenw",b"sesweseseseseseswnwseseseeseeesenwse",b"wneneswneenwwewnesenenweneneew",b"swswseseeneseswswswsenwswswsesesenwsesw",b"seewwwwwswwneswswswswnewwwnwnwsw",b"swwwsewwwnenewwnenwnwsewneseswe",b"neneseswsesenwseswwswsweswseswswseswswsw",b"swwnwwswswswwwswneswwwwwswewe",b"enewewneeseeeeeeeseneweesee",b"swnwnewnewswswswseswnweswswnwsene",b"swenwneeeneeesenwseesewneenwwneee",b"wnwnweswwnwwwnwswewwnwseswneew",b"swseeeeseeenewweesesenweese",b"nwsenenwnwnwsenwwsenw",b"eesewwwseeeneeseneeseeenweese",b"eeneeswneswneesweenenwneneneneenew",b"senwenenwswswseseneeswswswwwwsenwwenw",b"wwwnwnewwwenwwnwnwsewnweswsewse",b"nwnwnwnwnwnwnwnenwswnwnwsenwnenw",b"neseeenewnewsenenew",b"senwnwnwwnwnwnwswnewnwnenwwnwnwnwsenw",b"seswseeseseswnewseneswseswwnwseneswne",b"seseseseseneneseswseew",b"nwneenenwsweneeeweswseswenenenese",b"wnwnwnwnwwnwnwnwewswswnenwnwwnwww",b"wwnwswwneswswwnewwwsewseeenww",b"wnwenwsenwnenwnwnenesww",b"senewneneneneneseneswnweneneneneneee",b"sewnwsenwnwnwsenenwwnwwewwnwwnww",b"eseneswwwswswswswswswenwswsweswswsesw",b"nenewswweseeneeseeweeeeneswnwsw",b"swswnwwwsewseswswswnwwswseswwneswsw",b"nwnewnesenwnesenwwnwnwnewenwnewsenesw",b"seeswnwenwseseewseeeseneeswseesenwsw",b"swswseswswswsweseswswsenwswswnwswswseese",b"wswswwnwnwwswswseewswwweneswnwwse",b"eswseswswswwwwswnwwnwewnwwneww",b"swswsesenwneweseenweeseeswnwnweswse",b"wwwswneswwnewwswswswwwewwnwsew",b"wneswnenenewnenenenenenweswnweenew",b"enwnwnwnwnwnewnwnwnwnwenwnwnwnwwnw",b"eeeesenweeeeeesewewenwee",b"seseseeseeseseenwnwsesesesesenweswse",b"sesewnweenwnesenwnwswnwnenenwswwnesenwnw",b"swnwseswwswwwwwnenwswswsenwswewne",b"weseswsewswswseseneseseneesesesewse",b"senwwnewnwnewsenewseswswwwwswseww",b"swnwneneswswwneesenenwenwwneswneneeee",b"wswnwswnwneswswwsweswseneswsweewsenw",b"seneseseneswseweesenwseseenesewesese",b"seeesweeenwenweseseseeeeseee",b"seseseeseeenwnwseseseseswsesesewsesese",b"swnenwswenwnwnwenwnweswswweneswsenwne",b"swswnenwnenweneneseeewnenwnwswnwswnenw",b"swseseseseseseesesenenwseswseseseseseswnwsw",b"eneseeseseeseseeseseewse",b"swnenwswwswseswswswswswswswswswwswsew",b"eswnenewswswnweeeeeeeeeeseee",b"swwneswsenwnwsewnwnenewnwnwsenw",b"nweneeesweeneneneneswneeneenenee",b"eeeeseneeswew",b"nwnwnwnwnewnwnenwnwnwnenwnenwe",b"newswneswswsewnwswswswwwswswswseswsw",b"nweswswswseswseswseseswswnw",b"sesenesesesewseeseseweeesesewsew",b"swseseswswswswswneswseswsw",b"neseseswseseswsewwseneseswsenwewsesw",b"eenwwwwswswswswnenwseswswseswswww",b"seseseswseseneseeseseseee",b"esweesenweeesenenweesesweeee",b"eswseeewseeeeeeeesewewwenw",b"nwewwsewwwwsenwwnwswnewwwnwnw",b"swsewwwwswswswswnwwwwswsweesww",b"wsewswsenwneeswwwwenwwswweswswwne",b"eseswseneesesenwseesesewesesesene",b"neeeswnwseeswnweesenenwenenewene",b"seneneseeneeeneeeeenwneeenenew",b"eneenwenweneneneneneeneseswwse",b"neeesenwswswwwswnwsewnwswwwswswsw",b"swnwswswswseswswneseswswswsesenewswwswne",b"neneneeneneneneswnesenwswneneneneeneene",b"nwswnenwnwnwnwnwnwsenewnwnenwnenwnwnwnw",b"weeeeeeeeneeeesweeeeeew",b"nwnwnenwnwnwnwnwwnwswesenwnwnwnwnwnwnwnwse",b"nenwwwnwnwnwnwnwnwneswnwnwnwnenwnesee",b"wswnewneneewesewweseeswnewnwwsew",b"nwneseswnewneneneneneeneneswnenwwnenene",b"senwesenenewnwnenwnwnwnwnenwwnwnw",b"senesesweneeewneseswsewe",b"swwwwnewswwswwswswesewwswswswnw",b"weenwnwnwseneneswseenwnwnwnwnenwswnenene",b"seswnwesweeeeenweesee",b"wswwwwwwswewswsewnwswwnewwww",b"wwwwwnwsewnesewwwwnenwsewsenwwe",b"seswnewsenwwnwwesw",b"seenwsesesenwswsenwsenweswswswswnwswenw",b"seswnenwneneswsenwswwwesw",b"wnwwwwwwewnwwsw",b"swsenwsenwnwnenwwenwwswseseewsenww",b"enwswnwnwnwnwnwnwnwsewnwswsenenwneenew",b"neneseneewnenwneneenewnenenenwnenenene",b"nwnwwnewnwnwnwnwseeswswsenesenwwnwne",b"swneeswswnwswsenwnwswseneseeeswswswne",b"swweswneswswneswsesewnw",b"swwswenwneswseseswsesesesesenwseswnese",b"wnwnwseweenenwnwsenwswneenwnesenene",b"swswseseswswsewswswseesesesw",b"neseeeewwnwnenwseeeeswee",b"swnwneeswwneswswsewswewswsewswwwww",b"nenwesesesesesewwneswsweseweesesee",b"nesenesewnwwnwnwsenewnwwnenenwenwsene",b"swenwenwneeeswsweeseneeneneswe",b"swswsewneswswwswswwsww",b"nwnwnwnwnwnwnwnwsenenwnwnw",b"nwnwewnwwnwnwnwnwnwswnwenwnwnw",b"senwswnwwnesenwswneswnwwwsenwsenenee",b"neswseswenenwwnwnwnenwwnwenwnwnenwne",b"nenwnenenenenwnenenenesenwnenene",b"neesenenewnenesenenewneneeneneenenene",b"nwwnenwnwneenenenenenwnwnenenewseenene",b"senenewsenwnwnwnwnenwnenenwnwnewnenwnwnwse",b"ewnwwnwwwwsew",b"swseswwwswswwswneswenewswwswswswswswsw",b"sweneeeseeenwneseswseswseeneeswese",b"nwwnenwnwnenwnwnwnwnenwnwnwsenwnwswnwneswe",b"eeeeeeeweneeweeseeneeee",b"swenweseswenwnwswswnwnenese",b"nwwnwneeswsenwwsenwnenwenwwnwwnwnw",b"nenewseswwnenenewneseneesenenwnenenene",b"nenenwnenenenwnesenwnesenenenene",b"nenenenenewneneneneneseneneneswnenwnewse",b"wseseseseswsesesesesenwswenwseesenwne",b"eenesewneneseenenenesenewneeewne",b"seseswwseeswnwseswseweswswswswneswse",b"neeneneneneseswneneewneneneenenewene",b"eesenweesweeesesee",b"wseswwnwwwnwwnwneenenwnwswnwsenwnewe",b"wwnwewswwweswwwswswwwsweww",b"esweewweenwseswesweneeenwene",b"eewneneneewneneeeneneneneenee",b"seneseseseeseswsenwwseseseeseseseewse",b"eeneeneesenewneneneee",b"swwswenwswswswswswswsweswswswnwswswsw",b"wnwswweewwwnwwswwwwenwwnw",b"nweneseewwwwseenwenenwnwenwswwnw",b"eeesweseeweeeenwseneeewesenw",b"eenewneeeeseewseeeeeeee",b"ewseswneeeseneeenewenene",b"senwnwnwnenenwnenwnenwnwnwnwnw",b"nwnwnwnenwnwswnwwnwnwnwswnwnwnwne",b"nwnenwneswnwnewswnwseenenenwnenenenese",b"nwswnwnwnwwenwesenwswwnwnweneseswese",b"nwnwwwwsewweneswswwwsewnwnwenenw",b"esenwesesesweseseswsenwseseseseseenwsesw",b"eeeneeeseeseeeesewsweneewswe",b"ewswsweeeneeenweeneeewneseneee",b"seseseseseswsesesesesesenwsenwswswswsenw",b"neenwwsweeneswsesesesesewwseneesw",b"eseswweeseeneswnweeeeeseneese",b"swswnwswswnwseswswswnwswsweswsweswneesw",b"seseenwwnwneneweeseswseseswseswsenwse",b"seneseswswswswswswswseseseseswsw",b"eeewseseneseseseseseeeeseese",b"enwnwnwnwswnwnwnwnwnwnwnwwnwwnw",b"seseseswnwseseeseseeeeesese",b"swneswseswswwswswwneswswswswwswswsw",b"wwwsenwwwnwswnwwwswewnwwwnwe",b"seseswneneseswswneseswswsenwsesesesesesew",b"senwswswswsweeswnwse",b"senwnwenwnwneswwnweswnwnenweeswnwnww",b"neeswesewesewnwnwnewswseswseseswne",b"nwwwnwwsewnenwnwwnwnwnwsewwnwwnw",b"wwswnwwswneswwwwwwewwwwsww",b"swwseseseeseswwnewseneneswseswsw",b"eewsewseeseeneseseeneeseseeee",b"swswseneseswwseseseswne",b"swswnweswswswnwswswswswswswswswesw",b"seneseswsewseseswseswseseseswsesene",b"swsesesesesesewsesesewseeseseseenwse",b"swswnweneeeeeneenweeswenweeese",b"seeeewnweswneeeneeeeeenwneeee",b"nenwnenenwnwswesesesesewneneswwwewse",b"swswswnwnwsweswwwsweswwswswswswswswsw",b"wnwenwnenwenenwseswsesewswnw",b"neswswseswswseseswnwseswswswswswswswswswnw",b"neeseesesewseswnwesweswnewneewnwe",b"seseseewnweeseneseeseseseesesesese",b"nwneneneeswneneweeneneneesesenwne",b"neneneneneneseswnewneneseneneneneenewne",b"wwnwwewnwnenwnewwswwnwsewwnw",b"eeseeseswnenweseesenweeswesesesw",b"wswsweswswwnwswswswsw",b"eswseneneswseswswsenwswwwneswwwswse",b"enwseseseseseeswsewesenewwse",b"nenenewswseseenenenwneswwnewswneeneenw",b"swwnwneseeesweswswseeneenwnweswne",b"eeseeseenweesesesesese",b"swwenwsweneswsesweswswswswswswswnwswswnw",b"swwesenenewnwneneeswnwneeswnwswnenwe",b"swswswsesesweswswswswnwswsw",b"wnewwwsewswseeewnwwswwnewewnew",b"neeseneseswenwseseseeswswsewsesenwenw",b"swnenesewswswneswswwswswswwswswneseswsww",b"seswneneeeneneneswwneseswnenwneswnenenw",b"senweswwswswwwwwwwwnewwwswsew",b"wnwwwwsenwwnewenwwwwnwwnwwew",b"neneeseswswnenewseneneenenewnenwneneww",b"senenwnwseseneswnwswnwwwewwswnwneeww",b"sesesesesenwseseesesesesewswseseswese",b"sesesesesewneswseswswswswseswswnwseswswe",b"senwnwneneneenenenwwneswnwnenenenenwnw",b"wnwnwwwnenwwwsenw",b"seesewwweeseeeseeeseseeseee",b"swswswswseswseswsweseseswswwswewswnwne",b"eseeeeswseseenwseesesesweesesenw",b"nwnwswnwenwwewnwnwnewnwnwnenwnwswnw",b"seswnenwwenwnwnwnwnwnwnwnwnwnwnenwnwnw",b"wwseswwwswewswnwwnewneswwenwnw",b"seseswseeswswsenwsesewswneswseswseseswsw",b"swswswswnwswseseseneswseseswswswswnwneswsw",b"wwewnwwwsweseswwwwwwwwwnese",b"swswnwneweswswswswswneswswewswsw",b"wnenwewewwwnwnwnewnwsewseeswwne",b"neswsenenweenwweseeeswseeewsew",b"esesenwswsesesesenenwsewsesenwsesesee",b"swsesenwwnweeswnesewwseenw",b"eeneeneneenwneneeesesweneeeeew",b"nweseneneneswenesewnwwswswnenwnwsese",b"swnwswewwwneseswwsweswwswswswesw",b"swseseswseswseseswseneswswneswwwsesese",b"wwwwswnwnwsewwsenwnwnwswnwenwnwnwnenw",b"wsweswswwnewsweneswswswswwswswwnw",b"neeswnweneneneenwnwewwswnwnwwnwnenw",b"nwenwnwnwnwnenwnenewse",b"swewwwneeswneneseneseneneeenenwneenw",b"eenenenesenwneeseneneneswsenenewnenwne",b"wsweseeswneeweneneneeneeeeneee",b"nwnwnwsenenwnwnwnwnwswnwnwnw",b"neneenwnwneswwnenwseswesenwenewnwne",b"eswneswswwswneswswswswnwsw",b"nweeswsweseeeeeeseenweeeeenwe",b"weneswsweswswswswseswwswswseswswsww",b"senwswswswswseswsewweseswsenwswswnenesw",b"sesenwsesesesesesesesese",b"swnenweenwneeeeeseenwwswswe",b"neenenwsenwweswnenwnwnenwnenwwnenenw",b"sewseseswseneneseeseswsenwwnesesesene",b"nwwswwnwwwwswwwsewwewnewew",b"nesewwwseseewnwseeswwsesenesenene",b"weeneeseseeesweewe",b"swswswswseseseswseneeneseseswnwsw",b"neseseseweseseseseenwsesesesesesesese",b"seswswsweswswswswswwsweswswswswswnwsw",b"wwwwsewsewnewwwwnwwwwww",b"nwneswnwnenwnenwnwnenwnwnenenenw",b"seenesesewsweeneneneneenenenwnewe",b"nwwnwnwnwswwnwnwnwnwenwnwnw",b"newseneseewnwnwseneneneneeneneneswnew",b"neeeeseeeneswneswweneswnenewneene",b"nwswswnwenenwswnwnenenenenwnenwnenenwnw",b"eseeeeseeeseweeeeeeenw",b"swsenwwswnweneswewsenenesesenweese",b"seeeeseseewswsenweseneseeseeesenw",b"neseswwseseneweseseseseswsesewsesenesese",b"newesenwnwewnwnwnwwewseseswwwwnw",b"seswneswneenwswneswswwenwseswwswswwsw",b"nenwnwnenenwnesenwne",b"sewseeeseeeseee",b"esweweenweeeeseweeneneeesw",b"ewswnenwswnwsenwwwwseewewseswww",b"wwneneneseseswsesenwe",b"wswneswwswewewsenwenwnewweseswsw",b"wswwwwwnwnwnewww",b"senwnwnwnwnwnwwnwnw",b"senenenwnenwnenwnewnwnwnenwsenwnenenenwsw",b"nenwsenwnwnwnwswnwsenwnwnenwnwnwnwseenwesw",b"seseswwneseeswnwswswneeswsewswswnew",b"swwewwwseneenewswwswswsenwswswsw",b"nwnwsenweeneneneeeneweeswseneswne",b"swswnewwswswenweseswewsweswswewsw",b"wseswnwswseswnwnwswswnenweeswswneswse",b"wnenwswnwnenwenenwnwnwnwnwnwnesenenwnwne",b"eswsweswneweswenenwswnwwnwnwsenwnese",b"neeeneeeeneesweeneeweenee",b"swwneswesewswwswwnwneswseneswsenwsw",b"ewswswwswwwwwswwswseswwnewswe",b"nwsewsesweewseseseseseswseswsesesesese",b"enwnwnwswnenwwnwewnwnewswnwenwseswsw",b"wwsewwneswwwsenwwnewsewswnwnew",b"eeeneeswswnwneeseneseeneeewene",b"seseswwseeswsesewneeseseswsesenesenwsesw",b"wswswswswswwseswnwswswneswsw",b"wneswswwnwnwenewwwwwwnwswnwesenwne",b"swnwnwnwnwnwnwenwnw",b"swswnenenenenenenenenewwseneneneneenenene",b"nwwwwnwwenwnwnwwwwwweswwnww",b"swnwnwnwnwnwnwnwnwnwnwnwnwne",b"seseseseeseeesewsene",b"enwnwwnwnwnwnwnwnwnwnenwnenwnwnw",b"wswnewwwwswwweswwwwwswwew",b"sesewswswseswseseneswse",b"sesenweweeeseeneeeseesesweee",b"swswswswswswswswwweswswesenwnweenwse",b"swswswnwswwswseswseenenesweswwswwsw",b"seseeswnweseseswswsenwnwnwseesenwnwww",b"wnwnwwwwwsenewwnwwnwnwsene",b"swesenwseseswnwseseswnwswsweseseseswswsese",b"senwwenwnwnenewnwsenenwnenweewswse",b"eeenenwenesenewneweneeeeesenwe",b"senwwneseseseseseseseseesesesesesesese",b"ewnwswnwswenwswnesewseseneseswswsesw",b"nesenenweeeeeswwnenwnesweeneenene",b"neeswwewnweenwewneswsesenwwseee",b"nesenwseesenwnwswnwsenwnwswenwnwnenwnenw",b"senenenwnenenewsewnenwnenwnwnwneenewsene",b"wwwswswwnewwwswswswsenewwwww",b"neseseenweenesewswsewwseswseseseesw",b"seseseseseneseseswsesenesesesesesenewwse",b"swseswswswswswswwseeswseswswsw",b"swnewnewsesewneswwswwnw",b"seesesesenwseseswseseseseneeseswseee",b"neneneneneneseneenwwnenenenenenenwwne",b"seseewenwenwseeseseeeeseeneeneesw",b"neswsweeseeseneeesesenwswewnwe",b"seneswnwwnwwwwnwnenesewenwnwwwnwww",b"newnwnewswwwwwwwswwswwswwwsew",b"swseswwnwwwwwswsewswswenwwsw",b"wnenwnwewsenwnewsewwswnenwsewwswnw",b"nenwneswnwnesenenwnewsenenwneswnenwnwenw",b"wwswnwnwnwwwwwwnew",b"swswswnesesenwseswsewweneneswnesewse",b"swwsewwwwwwswwneswnenwwwnewsw",b"nwnwsenenenenenenenwnenenesenenwnwenwsw",b"sesesenweswswseseseswswswswswnwseswese",b"wsewwnwwwewnewwwsewwwwww",b"swseswswwnwesenwswswsenee",b"sesweseesewsewswswseseseswneseseswsesw",b"nwsenenenwnwnwwnwnwnwnwenwnenwnwnenwnwsw",b"nwwnwsesewwewneseswwwnw",b"eeseseseeseseeswenwseeeseeeenwsw",b"nwwnewwwsewwsew",b"wwnewwwwswswwneswswwwneswwwse",b"wswswswswswswswnwswwswswseneswneseswwsw",b"nenwnwwseeweswswswswnwnwwewneeswswse",b"nwnwnwenwsenweswwnenwnwsewnwwwnwnwnwne",b"eseswwnwneenwsewswsenweswwneswnenw",b"wswswswnwswneswswwswsenew",b"swwsesewnwwnwwswswwswwwswsww",b"wwswesewswswswswnenwswswswnewwwsw",b"swwwswseswswwswneswswnesweswneswswswsw",b"swnwnwswnwnenwwwwswnwwnwnwenwnwne",b"seeseseesesesenwswsesenwswsesesesesese",b"seeeseseseseseeseesenweswwseewsese",b"enenenenenewneseseswnenwswnenenenwnenene",b"swwweswesweswnwnweneenweseswene",b"senenwnenwsweweenweseneswseneswwnw",b"nesewneneswwneseneesenenenwnenwnwswse",b"swnesesewnenwwwnwnwenesewwweseew",b"nenenesesenwnwnenewnenwsenenwnwnwnenwne",b"nwseseeeseesenwewseeeseseseenwsw"];
static DIRS: [(i64,i64); 6] = [(1,0),(0,1),(-1,1),(-1,0),(0,-1),(1,-1)];

fn part_two(mut black: HashSet<(i64,i64)>) -> usize {
  for _ in 0..100 {
    let mut neighbours = HashMap::new();
    for &(q,r) in black.iter() {
      for &(dq,dr) in &DIRS {
        *neighbours.entry((q+dq,r+dr)).or_insert(0) += 1;
      }
    }
    black = neighbours.iter()
      .filter(|&(&t, &n)| n == 2 || (n == 1 && black.contains(&t)))
      .map(|(&t,_)| t)
      .collect();
  }
  black.len()
}

fn part_one() -> HashSet<(i64,i64)> {
  let mut black = HashSet::new();
  for s in &INPUT {
    let (mut q, mut r, mut i) = (0,0,0);
    while i < s.len() {
      let ((dq, dr), di) = match (s[i], s.get(i+1).unwrap_or(&0)) {
        (b'e', _)    => (DIRS[0], 1),
        (b's', b'e') => (DIRS[1], 2),
        (b's', b'w') => (DIRS[2], 2),
        (b'w', _)    => (DIRS[3], 1),
        (b'n', b'w') => (DIRS[4], 2),
        (b'n', b'e') => (DIRS[5], 2),
        _ => unreachable!(),
      };
      q += dq;
      r += dr;
      i += di;
    }
    if !black.remove(&(q,r)) {
      black.insert((q,r));
    }
  }
  black
}

aoc2020::main! {
  let black = part_one();
  (black.len(), part_two(black))
}

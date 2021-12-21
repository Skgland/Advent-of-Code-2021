use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Player {
    name: u16,
    position: u16,
    points: u16,
}

impl Player {
    pub fn advance_by(&mut self, roll: u16) {
        self.position = (self.position + roll - 1) % 10 + 1;
        self.points += self.position;
    }
}

fn parse_input(input: &str) -> (Player, Player) {
    let mut lines = input.lines();

    let player1 = lines.next().unwrap().splitn(5, ' ').collect::<Vec<_>>();
    let player2 = lines.next().unwrap().splitn(5, ' ').collect::<Vec<_>>();
    #[allow(clippy::match_ref_pats)]
    let (p1_idx, s1_idx) = match player1.as_slice() {
        &[_, p_idx, _, _, s_idx] => (p_idx, s_idx),
        _ => panic!(),
    };
    #[allow(clippy::match_ref_pats)]
    let (p2_idx, s2_idx) = match player2.as_slice() {
        &[_, p_idx, _, _, s_idx] => (p_idx, s_idx),
        _ => panic!(),
    };

    (
        Player {
            name: p1_idx.parse().unwrap(),
            position: s1_idx.parse().unwrap(),
            points: 0,
        },
        Player {
            name: p2_idx.parse().unwrap(),
            position: s2_idx.parse().unwrap(),
            points: 0,
        },
    )
}

pub fn part1(input: &str) -> u32 {
    let mut dice = (1..=100).cycle();
    let (mut player_a, mut player_b) = parse_input(input);

    let mut rolls = 0;

    'game: loop {
        for player in [&mut player_a, &mut player_b] {
            let roll = (&mut dice).take(3).sum();
            rolls += 3;
            player.advance_by(roll);
            /*
            println!(
                "Player {} rolled {} and now has {} points!",
                player.name, roll, player.points
            );
            */
            if player.points >= 1000 {
                break 'game;
            }
        }
    }

    player_a.points.min(player_b.points) as u32 * rolls
}

pub fn split_the_timeline(
    a: &Player,
    b: &Player,
    cache: &mut HashMap<(Player, Player), (u64, u64)>,
) -> (u64, u64) {
    if let Some(cache_result) = cache.get(&(a.clone(), b.clone())) {
        *cache_result
    } else {
        let mut wins_a = 0;
        let mut wins_b = 0;
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                for d3 in 1..=3 {
                    let mut a = a.clone();
                    let b = b.clone();

                    a.advance_by(d1 + d2 + d3);
                    if a.points >= 21 {
                        wins_a += 1;
                    } else {
                        let (win_b, win_a) = split_the_timeline(&b, &a, cache);
                        wins_a += win_a;
                        wins_b += win_b;
                    }
                }
            }
        }
        cache.insert((a.clone(), b.clone()), (wins_a, wins_b));
        (wins_a, wins_b)
    }
}

pub fn part2(input: &str) -> u64 {
    let (a, b) = parse_input(input);
    let mut cache: HashMap<(Player, Player), (u64, u64)> = HashMap::new();
    let (wins_a, wins_b) = split_the_timeline(&a, &b, &mut cache);
    wins_a.max(wins_b)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day21.example.txt"));
    assert_eq!(part1(input), 745 * 993);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day21.txt"));
    assert_eq!(part1(input), 734820);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day21.example.txt");
    assert_eq!(part2(input), 444356092776315);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day21.txt"));
    assert_eq!(part2(input), 193170338541590);
}

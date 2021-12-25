use crate::day23::Crab::*;
use crate::day23::HallwaySpot::*;
use crate::day23::Move::{FromRoom, ToRoom};

#[derive(Debug)]
pub struct Input<const N: usize> {
    pub rooms: [[Crab; N]; 4],
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum HallwaySpot {
    Left(usize),
    AB,
    BC,
    CD,
    Right(usize),
}

impl HallwaySpot {
    pub fn get_spot<'a, const N: usize>(&self, state: &'a State<N>) -> &'a Option<Crab> {
        match self {
            HallwaySpot::Left(idx) => &state.left[*idx],
            HallwaySpot::AB => &state.a_b,
            HallwaySpot::BC => &state.b_c,
            HallwaySpot::CD => &state.c_d,
            HallwaySpot::Right(idx) => &state.right[*idx],
        }
    }

    pub fn get_spot_mut<'a, const N: usize>(
        &self,
        state: &'a mut State<N>,
    ) -> &'a mut Option<Crab> {
        match self {
            HallwaySpot::Left(idx) => &mut state.left[*idx],
            HallwaySpot::AB => &mut state.a_b,
            HallwaySpot::BC => &mut state.b_c,
            HallwaySpot::CD => &mut state.c_d,
            HallwaySpot::Right(idx) => &mut state.right[*idx],
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct RoomSpot(Crab, usize);

impl RoomSpot {
    pub fn get_spot<'a, const N: usize>(&self, state: &'a State<N>) -> &'a Option<Crab> {
        let room = self.0.dest_room_idx();

        &state.rooms[room][self.1]
    }

    pub fn get_spot_mut<'a, const N: usize>(
        &self,
        state: &'a mut State<N>,
    ) -> &'a mut Option<Crab> {
        let room = self.0.dest_room_idx();

        &mut state.rooms[room][self.1]
    }
}

#[derive(Debug)]
pub enum Move {
    FromRoom(RoomSpot, HallwaySpot),
    ToRoom(HallwaySpot, RoomSpot),
}

impl Move {
    pub fn get_source<'a, const N: usize>(&self, state: &'a State<N>) -> &'a Option<Crab> {
        match self {
            Move::FromRoom(src, _) => src.get_spot(state),
            Move::ToRoom(src, _) => src.get_spot(state),
        }
    }
    pub fn get_source_mut<'a, const N: usize>(
        &self,
        state: &'a mut State<N>,
    ) -> &'a mut Option<Crab> {
        match self {
            Move::FromRoom(src, _) => src.get_spot_mut(state),
            Move::ToRoom(src, _) => src.get_spot_mut(state),
        }
    }

    pub fn get_target<'a, const N: usize>(&self, state: &'a State<N>) -> &'a Option<Crab> {
        match self {
            Move::FromRoom(_, target) => target.get_spot(state),
            Move::ToRoom(_, target) => target.get_spot(state),
        }
    }

    pub fn get_target_mut<'a, const N: usize>(
        &self,
        state: &'a mut State<N>,
    ) -> &'a mut Option<Crab> {
        match self {
            Move::FromRoom(_, target) => target.get_spot_mut(state),
            Move::ToRoom(_, target) => target.get_spot_mut(state),
        }
    }

    pub fn steps(&self) -> usize {
        let (Move::FromRoom(room, hallway) | Move::ToRoom(hallway, room)) = self;

        let mut distance = match (room, hallway) {
            (RoomSpot(Amber, _), HallwaySpot::Left(_))
            | (RoomSpot(Amber, _), AB)
            | (RoomSpot(Bronze, _), AB)
            | (RoomSpot(Bronze, _), BC)
            | (RoomSpot(Copper, _), BC)
            | (RoomSpot(Copper, _), CD)
            | (RoomSpot(Desert, _), CD)
            | (RoomSpot(Desert, _), HallwaySpot::Right(_)) => 2,
            (RoomSpot(Bronze, _), HallwaySpot::Left(_)) | (RoomSpot(Amber, _), BC) => 4,
            (RoomSpot(Copper, _), AB) | (RoomSpot(Bronze, _), CD) => 4,
            (RoomSpot(Desert, _), BC) | (RoomSpot(Copper, _), HallwaySpot::Right(_)) => 4,
            (RoomSpot(Amber, _), CD) | (RoomSpot(Copper, _), HallwaySpot::Left(_)) => 6,
            (RoomSpot(Desert, _), AB) | (RoomSpot(Bronze, _), HallwaySpot::Right(_)) => 6,
            (RoomSpot(Desert, _), HallwaySpot::Left(_))
            | (RoomSpot(Amber, _), HallwaySpot::Right(_)) => 8,
        };

        distance += room.1;

        if let HallwaySpot::Left(idx) | HallwaySpot::Right(idx) = hallway {
            distance += idx;
        }

        distance
    }

    pub fn passes_through<'a, const N: usize>(&self, state: &'a State<N>) -> Vec<&'a Option<Crab>> {
        let mut passes_through = vec![];

        let (Move::FromRoom(room, hallway) | Move::ToRoom(hallway, room)) = self;

        for idx in 0..room.1 {
            passes_through.push(RoomSpot(room.0.clone(), idx).get_spot(state))
        }

        if let HallwaySpot::Left(idx) = hallway {
            for idx in 0..*idx {
                passes_through.push(HallwaySpot::Left(idx).get_spot(state))
            }
        } else if let HallwaySpot::Right(idx) = hallway {
            for idx in 0..*idx {
                passes_through.push(HallwaySpot::Right(idx).get_spot(state))
            }
        }

        match (room, hallway) {
            (RoomSpot(Amber, _), HallwaySpot::Left(_))
            | (RoomSpot(Amber, _), AB)
            | (RoomSpot(Bronze, _), AB)
            | (RoomSpot(Bronze, _), BC)
            | (RoomSpot(Copper, _), BC)
            | (RoomSpot(Copper, _), CD)
            | (RoomSpot(Desert, _), CD)
            | (RoomSpot(Desert, _), HallwaySpot::Right(_)) => {}
            (RoomSpot(Bronze, _), HallwaySpot::Left(_)) | (RoomSpot(Amber, _), BC) => {
                passes_through.push(HallwaySpot::AB.get_spot(state))
            }
            (RoomSpot(Copper, _), AB) | (RoomSpot(Bronze, _), CD) => {
                passes_through.push(HallwaySpot::BC.get_spot(state));
            }
            (RoomSpot(Desert, _), BC) | (RoomSpot(Copper, _), HallwaySpot::Right(_)) => {
                passes_through.push(HallwaySpot::CD.get_spot(state))
            }
            (RoomSpot(Amber, _), CD) | (RoomSpot(Copper, _), HallwaySpot::Left(_)) => {
                passes_through.push(HallwaySpot::AB.get_spot(state));
                passes_through.push(HallwaySpot::BC.get_spot(state))
            }
            (RoomSpot(Desert, _), AB) | (RoomSpot(Bronze, _), HallwaySpot::Right(_)) => {
                passes_through.push(HallwaySpot::BC.get_spot(state));
                passes_through.push(HallwaySpot::CD.get_spot(state));
            }
            (RoomSpot(Desert, _), HallwaySpot::Left(_))
            | (RoomSpot(Amber, _), HallwaySpot::Right(_)) => {
                passes_through.push(HallwaySpot::AB.get_spot(state));
                passes_through.push(HallwaySpot::BC.get_spot(state));
                passes_through.push(HallwaySpot::CD.get_spot(state))
            }
        }
        passes_through
    }
}

#[derive(Clone, Debug)]
pub struct State<const N: usize> {
    rooms: [[Option<Crab>; N]; 4],
    left: [Option<Crab>; 2],
    right: [Option<Crab>; 2],
    a_b: Option<Crab>,
    b_c: Option<Crab>,
    c_d: Option<Crab>,
}

impl<const N: usize> State<N> {
    ///```
    /// use aoc2021::day23::Crab::*;
    /// use aoc2021::day23::{Input, State};
    /// let state: State<2> = Input {rooms:[
    ///    [Amber , Amber ],
    ///    [Bronze, Bronze],
    ///    [Copper, Copper],
    ///    [Desert, Desert],
    /// ]}.into();
    ///
    /// assert!(state.is_final());
    /// let state: State<4> = Input {rooms:[
    ///    [Amber , Amber , Amber , Amber ],
    ///    [Bronze, Bronze, Bronze, Bronze],
    ///    [Copper, Copper, Copper, Copper],
    ///    [Desert, Desert, Desert, Desert],
    /// ]}.into();
    /// assert!(state.is_final());
    ///```

    pub fn is_final(&self) -> bool {
        self.rooms.iter().enumerate().all(|(idx, room)| {
            room.iter().all(|elem| {
                if let Some(c) = elem {
                    c.dest_room_idx() == idx
                } else {
                    false
                }
            })
        })
    }

    pub fn apply_move(&mut self, movement: &Move) -> usize {
        let crab = movement.get_source_mut(self).take().unwrap();
        let move_cost = movement.steps() * crab.step_cost();
        *movement.get_target_mut(self) = Some(crab);
        move_cost
    }

    pub fn is_valid_move(&self, movement: &Move) -> bool {
        if let (Some(source), None) = (movement.get_source(self), movement.get_target(self)) {
            let empty_path = movement
                .passes_through(self)
                .iter()
                .all(|spot| spot.is_none());
            empty_path
                && match movement {
                    FromRoom(room_spot, _) => {
                        // only move out when we are in the wrong room or there is a crab below us which is wrong here
                        let wrong_room = &room_spot.0 != source;
                        let wrong_below = ((room_spot.1 + 1)..N).any(|room_idx| {
                            RoomSpot(room_spot.0.clone(), room_idx)
                                .get_spot(self)
                                .as_ref()
                                .map(|elem| elem != source)
                                .unwrap_or(false)
                        });
                        wrong_below || wrong_room
                    }
                    ToRoom(_, room_spot) => {
                        &room_spot.0 == source
                            && ((room_spot.1 + 1)..N).all(|room_idx| {
                                RoomSpot(room_spot.0.clone(), room_idx)
                                    .get_spot(self)
                                    .as_ref()
                                    .map(|elem| elem == source)
                                    .unwrap_or(true)
                            })
                    }
                }
        } else {
            false
        }
    }
}

impl<const N: usize> From<Input<N>> for State<N> {
    fn from(input: Input<N>) -> Self {
        State {
            left: [None, None],
            a_b: None,
            b_c: None,
            c_d: None,
            right: [None, None],
            rooms: input.rooms.map(|room| room.map(Some)),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Crab {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Crab {
    pub fn dest_room_idx(&self) -> usize {
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }

    pub fn step_cost(&self) -> usize {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}

fn parse_input(input: &str) -> Input<2> {
    let mut lines = input.lines();
    let head = lines.nth(2).unwrap().trim_matches('#');
    let tail = lines.next().unwrap().trim().trim_matches('#');

    let rooms = [head, tail].map(|elem| {
        elem.splitn(4, '#')
            .map(|elem| match elem {
                "A" => Some(Amber),
                "B" => Some(Bronze),
                "C" => Some(Copper),
                "D" => Some(Desert),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
    });

    if let [Some(tops), Some(bottoms)] = rooms {
        let rooms = tops
            .into_iter()
            .zip(bottoms.into_iter())
            .map(|(a, b)| [a, b])
            .collect::<Vec<_>>();
        let rooms = rooms.try_into().unwrap();
        Input { rooms }
    } else {
        panic!("{:?}", rooms)
    }
}

pub fn possible_moves<const N: usize>(state: &State<N>) -> Vec<Move> {
    const CRABS: [Crab; 4] = [Amber, Bronze, Copper, Desert];
    const HALL_SPORTS: [HallwaySpot; 7] = [AB, BC, CD, Left(0), Left(1), Right(0), Right(1)];

    HALL_SPORTS
        .into_iter()
        .flat_map(move |hallway_spot| {
            CRABS.into_iter().flat_map(move |crab| {
                let hallway_spot = hallway_spot.clone();
                (0..N)
                    .into_iter()
                    .map(move |room_idx| (RoomSpot(crab.clone(), room_idx), hallway_spot.clone()))
            })
        })
        .flat_map(|(room_spot, hallway_spot)| {
            [
                FromRoom(room_spot.clone(), hallway_spot.clone()),
                ToRoom(hallway_spot, room_spot),
            ]
        })
        .filter(|movement| state.is_valid_move(movement))
        .collect()
}

pub fn simulate<const N: usize>(
    state: &State<N>,
    depth: usize,
    state_cost: usize,
    minimum: &mut Option<usize>,
) {
    for movement in possible_moves(state) {
        let mut new_state = state.clone();
        let move_cost = new_state.apply_move(&movement);

        let current_cost = move_cost + state_cost;

        if minimum.map_or(false, |min| min < current_cost) {
            continue;
        }

        if new_state.is_final() {
            let min = minimum.take();
            *minimum = Some(min.map_or(current_cost, |cur| cur.min(current_cost)));
            // no other moves can be more efficient than getting to the final state
            break;
        } else {
            simulate(&new_state, depth + 1, current_cost, minimum);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let init_state = State::from(input);
    let mut min = None;
    simulate(&init_state, 1, 0, &mut min);
    min.unwrap()
}

pub fn part2(input: &str) -> usize {
    let Input {
        rooms: [[a_0, a_1], [b_0, b_1], [c_0, c_1], [d_0, d_1]],
    } = parse_input(input);

    let input = Input::<4> {
        rooms: [
            [a_0, Desert, Desert, a_1],
            [b_0, Copper, Bronze, b_1],
            [c_0, Bronze, Amber, c_1],
            [d_0, Amber, Copper, d_1],
        ],
    };

    let init_state = State::from(input);

    let mut min = None;
    simulate(&init_state, 1, 0, &mut min);
    min.unwrap()
}

#[test]
#[ignore]
fn part1_example() {
    let input = include_str!(concat!("../input/day23.example.txt"));
    assert_eq!(part1(input), 12521);
}

#[test]
#[ignore]
fn part1_full() {
    let input = include_str!(concat!("../input/day23.txt"));
    assert_eq!(part1(input), 18195);
}

#[test]
#[ignore]
fn part2_example() {
    let input = include_str!("../input/day23.example.txt");
    assert_eq!(part2(input), 44169);
}

#[test]
#[ignore]
fn part2_full() {
    // #############
    // #...........#
    // ###A#C#B#A###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #D#D#B#C#
    //   #########

    let input = include_str!(concat!("../input/day23.txt"));
    assert_eq!(part2(input), 50265);
}

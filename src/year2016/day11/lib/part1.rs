use itertools::Itertools;
use pathfinding::prelude::astar;

pub type FloorNumber = u8;

pub type NumberOfMicrochip = u8;
pub type NumberOfGenerator = u8;
pub type Floor = (NumberOfMicrochip, NumberOfGenerator);

pub type Floors = [Floor; 4];

pub type Elevator = u8;

pub fn part1(floors: Floors) -> String {
    let number_of_microchips_or_generators =
        floors.clone().into_iter().map(|(m, g)| m + g).sum::<u8>() / 2;
    let result = astar(
        &(0u8, floors),
        |(elevator, node_floors)| {
            let mut going_up = vec![];
            let mut going_down = vec![];

            let (microchips, generators) = node_floors[*elevator as usize];
            let combinations = vec![(2, 0), (1, 0), (1, 1), (0, 1), (0, 2)]
                .into_iter()
                .filter(|(m, g)| *m <= microchips && *g <= generators)
                .collect_vec();

            if *elevator > 0 {
                going_down = moving_to_floor(*elevator, *elevator - 1, node_floors, &combinations);
            }

            if *elevator < 3 {
                going_up = moving_to_floor(*elevator, *elevator + 1, node_floors, &combinations);
            }

            vec![going_up, going_down].concat()
        },
        |(_elevator, node_floors)| {
            node_floors
                .iter()
                .enumerate()
                .map(|(index, (m, g))| (3 - index as u8) * (m + g) / 2)
                .sum::<u8>()
        },
        |(_elevator, node_floors)| {
            node_floors[3].0 == number_of_microchips_or_generators
                && node_floors[3].1 == number_of_microchips_or_generators
        },
    )
    .unwrap();
    result.1.to_string()
}

fn moving_to_floor(
    current_floor: FloorNumber,
    next_floor: FloorNumber,
    floors: &Floors,
    combinations: &Vec<(NumberOfMicrochip, NumberOfGenerator)>,
) -> Vec<((Elevator, [Floor; 4]), u8)> {
    let (microchips, generators) = floors[current_floor as usize];
    let (next_floor_microchips, next_floor_generators) = floors[next_floor as usize];

    combinations
        .clone()
        .into_iter()
        .filter(|(m, g)| {
            let total_generators = g + next_floor_generators;
            let total_microchips = m + next_floor_microchips;
            total_generators == 0 || total_generators >= total_microchips
        })
        .map(|(m, g)| {
            let total_microchips = m + next_floor_microchips;
            let total_generators = g + next_floor_generators;
            let mut next_floors = floors.clone();
            next_floors[next_floor as usize] = (total_microchips, total_generators);
            next_floors[current_floor as usize] = (microchips - m, generators - g);
            ((next_floor, next_floors), 1)
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use crate::year2016::day11::lib::part1::part1;

    #[test]
    fn part1_test() {
        let floors = [(2, 0), (0, 1), (0, 1), (0, 0)];
        assert_eq!(part1(floors), "11");
    }
}

use crate::GardenEnum::{GardenStruct, PartOf};
use adv_code::lib::grid_utils::{Coordinates, GridCoordinates};
use adv_code::year2024::day12::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use grid::Grid;
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const INPUT_FILE: &str = "input/2024/12/inputs.txt";

type Id = usize;
type RegionId = char;
type Area = isize;
type Perimeter = isize;
type GardenMap = HashMap<GridCoordinates, GardenEnum>;

fn main() -> Result<()> {
    start_day(2024, 12, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1375574);

    Ok(())
}

fn part1(file_input_path: &str) -> isize {
    let grid = parse_input(&file_input_path);

    let mut garden_map = HashMap::new();
    group_gardens_for_all(&mut garden_map, &grid);

    garden_map
        .values()
        .into_iter()
        .map(|g| g.get_root_garden_struct())
        .unique_by(|g| g.id())
        .map(|garden| garden.area() * garden.perimeter())
        .sum()
}

#[derive(Debug, Clone)]
enum GardenEnum {
    PartOf(Arc<Mutex<GardenEnum>>),
    GardenStruct {
        id: Id,
        region_id: RegionId,
        area: Area,
        perimeter: Perimeter,
    },
}

impl GardenEnum {
    pub fn id(&self) -> Id {
        match self {
            PartOf(m) => m.lock().unwrap().id(),
            GardenStruct { id, .. } => *id,
        }
    }

    pub fn area(&self) -> Area {
        match self {
            PartOf(m) => m.lock().unwrap().area(),
            GardenStruct { area, .. } => *area,
        }
    }

    pub fn perimeter(&self) -> Perimeter {
        match self {
            PartOf(m) => m.lock().unwrap().perimeter(),
            GardenStruct { perimeter, .. } => *perimeter,
        }
    }

    pub fn add_perimeter(&mut self, perimeter_value: Perimeter) {
        match self {
            PartOf(m) => m.lock().unwrap().add_perimeter(perimeter_value),
            GardenStruct { perimeter, .. } => *perimeter += perimeter_value,
        }
    }

    pub fn add_area(&mut self, area_value: Area) {
        match self {
            PartOf(m) => m.lock().unwrap().add_area(area_value),
            GardenStruct { area, .. } => *area += area_value,
        }
    }

    pub fn region_id(&self) -> RegionId {
        match self {
            PartOf(m) => m.lock().unwrap().region_id(),
            GardenStruct { region_id, .. } => *region_id,
        }
    }

    pub fn set_root_garden(&self, garden: &Arc<Mutex<GardenEnum>>) {
        match self {
            PartOf(pointer) => {
                let mut garden_enum = pointer.lock().unwrap();
                match *garden_enum {
                    PartOf(_) => garden_enum.set_root_garden(garden),
                    GardenStruct { .. } => {
                        *garden_enum = PartOf(garden.clone());
                    }
                }
            }
            GardenStruct { .. } => panic!("too deep in the tree! Can't merge there."),
        }
    }

    pub fn get_root_garden_struct(&self) -> Self {
        match self {
            PartOf(m) => m.lock().unwrap().get_root_garden_struct(),
            GardenStruct { .. } => self.clone(),
        }
    }
}

fn group_gardens_for_all(gardens: &mut GardenMap, grid: &Grid<RegionId>) {
    let mut id_counter = 0;
    grid.indexed_iter()
        .map(|(c, region)| (GridCoordinates::from(c), region))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(grid_coordinates, region_id)| {
            group_gardens_for_coordinates(grid_coordinates, *region_id, gardens, &mut id_counter)
        });
}

fn group_gardens_for_coordinates(
    from: GridCoordinates,
    region_id: RegionId,
    gardens: &mut GardenMap,
    id_counter: &mut Id,
) {
    if gardens.get(&from).is_some() {
        return;
    }

    let adjoining_matching_garden_coordinates: Vec<GridCoordinates> = from
        .cardinals()
        .into_iter()
        .filter(|c| {
            gardens
                .get(&c)
                .filter(|garden| garden.region_id() == region_id)
                .is_some()
        })
        .collect();

    let number_of_connecting_gardens: isize = adjoining_matching_garden_coordinates
        .len()
        .try_into()
        .unwrap();
    match number_of_connecting_gardens {
        0 => {
            let new_garden = GardenStruct {
                id: *id_counter,
                region_id,
                area: 1,
                perimeter: 4,
            };
            gardens.insert(from, PartOf(Arc::new(Mutex::new(new_garden))));
            *id_counter += 1;
        }
        1 | 2 | 3 | 4 => {
            let connecting_garden: &mut GardenEnum = &mut gardens
                .get_mut(&adjoining_matching_garden_coordinates[0])
                .unwrap()
                .clone();
            connecting_garden.add_area(1);
            connecting_garden.add_perimeter(4 - 2 * number_of_connecting_gardens);

            gardens.insert(
                from,
                PartOf(Arc::new(Mutex::new(connecting_garden.clone()))),
            );

            adjoining_matching_garden_coordinates[1..]
                .into_iter()
                .map(|c| gardens.get(&c).unwrap())
                .for_each(|garden| {
                    if garden.id() != connecting_garden.id() {
                        connecting_garden.add_area(garden.area());
                        connecting_garden.add_perimeter(garden.perimeter());

                        garden.set_root_garden(&Arc::new(Mutex::new(connecting_garden.clone())));
                    }
                });
        }
        _ => panic!("Shouldn't happen"),
    };
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day12::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/12/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 1930);
    }

    #[test]
    fn test_parser() {
        let grid = parse_input(TEST_INPUT_FILE);
        println!("{}", grid_to_str(&grid));
    }
}

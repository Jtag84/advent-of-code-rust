use crate::lib::grid_utils::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};
use crate::lib::grid_utils::{Coordinates, GridCoordinates};
use crate::year2024::day12::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day12::lib::part1::part1;
pub use crate::year2024::day12::lib::part2::part2;
use crate::year2024::day12::lib::GardenEnum::{GardenStruct, PartOf};
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use grid::Grid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_12_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 12),
    parser: parse_input,
    part1,
    expected_part1: "1375574",
    part2,
    expected_part2: "830566",
};

aoc_solver!(YEAR_2024_DAY_12_SOLUTION);

pub type Id = usize;
pub type RegionId = char;
pub type Area = isize;
pub type Perimeter = isize;
pub type CornerCount = usize;
pub type GardenMap = HashMap<GridCoordinates, GardenEnum>;

#[derive(Debug, Clone)]
pub enum GardenEnum {
    PartOf(Arc<Mutex<GardenEnum>>),
    GardenStruct {
        id: Id,
        region_id: RegionId,
        area: Area,
        perimeter: Perimeter,
        corner_count: CornerCount,
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

    pub fn corner_count(&self) -> CornerCount {
        match self {
            PartOf(m) => m.lock().unwrap().corner_count(),
            GardenStruct { corner_count, .. } => *corner_count,
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

    pub fn add_corners(&mut self, corner_count_value: CornerCount) {
        match self {
            PartOf(m) => m.lock().unwrap().add_corners(corner_count_value),
            GardenStruct { corner_count, .. } => *corner_count += corner_count_value,
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

pub fn group_gardens_for_all(gardens: &mut GardenMap, grid: &Grid<RegionId>) {
    let mut id_counter = 0;
    grid.indexed_iter()
        .map(|(c, region)| (GridCoordinates::from(c), region))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(grid_coordinates, region_id)| {
            group_gardens_for_coordinates(
                grid_coordinates,
                *region_id,
                gardens,
                &mut id_counter,
                count_corners(grid_coordinates, grid),
            )
        });
}

fn group_gardens_for_coordinates(
    from: GridCoordinates,
    region_id: RegionId,
    gardens: &mut GardenMap,
    id_counter: &mut Id,
    corner_count: CornerCount,
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
                corner_count: corner_count,
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
            connecting_garden.add_corners(corner_count);

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
                        connecting_garden.add_corners(garden.corner_count());

                        garden.set_root_garden(&Arc::new(Mutex::new(connecting_garden.clone())));
                    }
                });
        }
        _ => panic!("Shouldn't happen"),
    };
}

fn count_corners(coordinates: GridCoordinates, grid: &Grid<char>) -> usize {
    let current_region = coordinates.get_from_grid(&grid).unwrap();

    let same_as_west = coordinates.is_direction_equal_to_in_grid(&grid, West, current_region);
    let same_as_north_west =
        coordinates.is_direction_equal_to_in_grid(&grid, NorthWest, current_region);
    let same_as_north = coordinates.is_direction_equal_to_in_grid(&grid, North, current_region);
    let same_as_north_east =
        coordinates.is_direction_equal_to_in_grid(&grid, NorthEast, current_region);
    let same_as_east = coordinates.is_direction_equal_to_in_grid(&grid, East, current_region);
    let same_as_south_east =
        coordinates.is_direction_equal_to_in_grid(&grid, SouthEast, current_region);
    let same_as_south = coordinates.is_direction_equal_to_in_grid(&grid, South, current_region);
    let same_as_south_west =
        coordinates.is_direction_equal_to_in_grid(&grid, SouthWest, current_region);

    vec![
        !same_as_west && !same_as_north,
        !same_as_north && !same_as_east,
        !same_as_east && !same_as_south,
        !same_as_west && !same_as_south,
        same_as_north && same_as_east && !same_as_north_east,
        same_as_south && same_as_east && !same_as_south_east,
        same_as_south && same_as_west && !same_as_south_west,
        same_as_north && same_as_west && !same_as_north_west,
    ]
    .into_iter()
    .filter(|c| *c)
    .count()
}

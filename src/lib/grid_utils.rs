use std::fmt::Display;
use std::ops::{Add, Sub};
use tabled::Table;

use grid::Grid;
use itertools::Itertools;
use strum_macros::EnumIter;
use tabled::settings::object::Rows;
use tabled::settings::{Remove, Style};

pub trait Coordinates: Sized {
    fn move_to(&self, direction: Direction) -> Option<Self> {
        self.move_to_n(direction, 1)
    }
    fn move_to_n(&self, direction: Direction, n: isize) -> Option<Self> {
        match direction {
            Direction::North => self.north_n(n),
            Direction::NorthEast => self.north_east_n(n),
            Direction::East => self.east_n(n),
            Direction::SouthEast => self.south_east_n(n),
            Direction::South => self.south_n(n),
            Direction::SouthWest => self.south_west_n(n),
            Direction::West => self.west_n(n),
            Direction::NorthWest => self.north_west_n(n),
        }
    }
    fn north(&self) -> Option<Self> {
        self.north_n(1)
    }
    fn north_n(&self, n: isize) -> Option<Self>;
    fn north_east_n(&self, n: isize) -> Option<Self> {
        self.north_n(n)?.east_n(n)
    }
    fn north_east(&self) -> Option<Self> {
        self.north_east_n(1)
    }
    fn east(&self) -> Option<Self> {
        self.east_n(1)
    }
    fn east_n(&self, n: isize) -> Option<Self>;
    fn south_east_n(&self, n: isize) -> Option<Self> {
        self.south_n(n)?.east_n(n)
    }
    fn south_east(&self) -> Option<Self> {
        self.south_east_n(1)
    }
    fn south_n(&self, n: isize) -> Option<Self>;
    fn south(&self) -> Option<Self> {
        self.south_n(1)
    }
    fn south_west(&self) -> Option<Self> {
        self.south_west_n(1)
    }
    fn south_west_n(&self, n: isize) -> Option<Self> {
        self.south_n(n)?.west_n(n)
    }
    fn west(&self) -> Option<Self> {
        self.west_n(1)
    }
    fn west_n(&self, n: isize) -> Option<Self>;
    fn north_west(&self) -> Option<Self> {
        self.north_west_n(1)
    }
    fn north_west_n(&self, n: isize) -> Option<Self> {
        self.north_n(n)?.west_n(n)
    }
}

pub type Row = isize;
pub type Column = isize;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridCoordinates(pub Row, pub Column);

impl GridCoordinates {
    pub fn row(&self) -> Row {
        self.0
    }
    pub fn column(&self) -> Column {
        self.1
    }
}

impl Into<(Row, Column)> for GridCoordinates {
    fn into(self) -> (Row, Column) {
        (self.row(), self.column())
    }
}

impl From<(Row, Column)> for GridCoordinates {
    fn from((row, column): (Row, Column)) -> Self {
        GridCoordinates(row, column)
    }
}

impl Into<(usize, usize)> for GridCoordinates {
    fn into(self) -> (usize, usize) {
        (
            self.row().try_into().unwrap(),
            self.column().try_into().unwrap(),
        )
    }
}

impl From<(usize, usize)> for GridCoordinates {
    fn from((row, column): (usize, usize)) -> Self {
        GridCoordinates(row.try_into().unwrap(), column.try_into().unwrap())
    }
}

impl Add for GridCoordinates {
    type Output = GridCoordinates;

    fn add(self, other: GridCoordinates) -> Self {
        GridCoordinates(self.row() + other.row(), self.column() + other.column())
    }
}

impl Sub for GridCoordinates {
    type Output = GridCoordinates;
    fn sub(self, other: GridCoordinates) -> Self {
        GridCoordinates(self.row() - other.row(), self.column() - other.column())
    }
}

impl Coordinates for GridCoordinates {
    fn north_n(&self, n: isize) -> Option<Self> {
        if self.row() < n {
            None
        } else {
            Some(GridCoordinates(self.row() - n, self.column()))
        }
    }

    fn east_n(&self, n: isize) -> Option<Self> {
        Some(GridCoordinates(self.row(), self.column() + n))
    }

    fn south_n(&self, n: isize) -> Option<Self> {
        Some(GridCoordinates(self.row() + n, self.column()))
    }

    fn west_n(&self, n: isize) -> Option<Self> {
        if self.column() < n {
            None
        } else {
            Some(GridCoordinates(self.row(), self.column() - n))
        }
    }
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn rotate_clockwise_90(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
}

pub fn grid_to_str<T: Display>(grid: &Grid<T>) -> String {
    let a: Vec<String> = grid
        .iter_rows()
        .map(|row| row.map(|e| e.to_string()).join(""))
        .collect();

    Table::new(a)
        .with(Style::sharp().remove_horizontals())
        .with(Remove::row(Rows::first()))
        .to_string()
}

pub fn set_grid_element<T>(
    grid: &mut Grid<T>,
    guard_coordinates: &GridCoordinates,
    grid_element: T,
) {
    *(grid
        .get_mut(guard_coordinates.row(), guard_coordinates.column())
        .unwrap()) = grid_element;
}

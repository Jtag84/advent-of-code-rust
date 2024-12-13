use std::fmt::Display;
use tabled::Table;

use grid::Grid;
use itertools::Itertools;
use strum_macros::EnumIter;
use tabled::settings::object::Rows;
use tabled::settings::{Remove, Style};

pub trait Coordinates : Sized {

    fn move_to(&self, direction: Direction) -> Option<Self> {
        self.move_to_n(direction, 1)
    }
    fn move_to_n(&self, direction: Direction, n:usize) -> Option<Self> {
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
    fn north_n(&self, n: usize) -> Option<Self>;
    fn north_east_n(&self, n:usize) -> Option<Self> {
        self.north_n(n)?.east_n(n)
    }
    fn north_east(&self) -> Option<Self> {
        self.north_east_n(1)
    }
    fn east(&self) -> Option<Self> {
        self.east_n(1)
    }
    fn east_n(&self, n: usize) -> Option<Self>;
    fn south_east_n(&self, n: usize) -> Option<Self>{
        self.south_n(n)?.east_n(n)
    }
    fn south_east(&self) -> Option<Self>{
        self.south_east_n(1)
    }
    fn south_n(&self, n:usize) -> Option<Self>;
    fn south(&self) -> Option<Self> {
        self.south_n(1)
    }
    fn south_west(&self) -> Option<Self> {
        self.south_west_n(1)
    }
    fn south_west_n(&self, n:usize) -> Option<Self> {
        self.south_n(n)?.west_n(n)
    }
    fn west(&self) -> Option<Self> {
        self.west_n(1)
    }
    fn west_n(&self, n:usize) -> Option<Self>;
    fn north_west(&self) -> Option<Self> {
        self.north_west_n(1)
    }
    fn north_west_n(&self, n:usize) -> Option<Self> {
        self.north_n(n)?.west_n(n)
    }
}

type Row = usize;
type Column = usize;
#[derive(Clone, Copy)]
pub struct GridCoordinates(pub Row, pub Column);

impl GridCoordinates {
    pub fn row(&self) -> Row {self.0}
    pub fn column(&self) -> Column {self.1}
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

impl Coordinates for GridCoordinates
{
    fn north_n(&self, n:usize) -> Option<Self> {
       Some(GridCoordinates(self.row() + n, self.column()))
    }

    fn east_n(&self, n:usize) -> Option<Self> {
        Some(GridCoordinates(self.row(), self.column() + n))
    }

    fn south_n(&self, n:usize) -> Option<Self> {
        if self.row() < n {
            None
        } else {
            Some(GridCoordinates(self.row() - n, self.column()))
        }
    }

    fn west_n(&self, n:usize) -> Option<Self> {
        if self.column() < n {
            None
        } else {
            Some(GridCoordinates(self.row(), self.column() - n))
        }
    }
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
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

pub fn grid_to_str<T: Display>(grid: &Grid<T>) -> String {
    let a: Vec<String> = grid.iter_rows().map(|row| row.map(|e| e.to_string()).join("")).collect();

    Table::new(a)
        .with(Style::sharp().remove_horizontals())
        .with(Remove::row(Rows::first()))
        .to_string()
}
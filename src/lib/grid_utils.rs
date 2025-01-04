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

    fn cardinals(&self) -> Vec<Self> {
        vec![self.north(), self.east(), self.south(), self.west()]
            .into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }

    fn manhattan_distance(&self, other: Self) -> usize;
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

    pub fn get_grid_element<'a, T>(&self, grid: &'a Grid<T>) -> Option<&'a T> {
        grid.get(self.row(), self.column())
    }

    pub fn is_direction_equal_to_in_grid<T: Clone + PartialEq>(
        &self,
        grid: &Grid<T>,
        direction: Direction,
        equal_to: T,
    ) -> bool {
        self.move_to(direction)
            .map(|c| c.is_equal_to_in_grid(grid, equal_to))
            .unwrap_or(false)
    }

    pub fn is_equal_to_in_grid<T: Clone + PartialEq>(&self, grid: &Grid<T>, equal_to: T) -> bool {
        self.get_from_grid(&grid)
            .map_or_else(|| false, |v| v == equal_to)
    }

    pub fn get_from_grid<T: Clone>(&self, grid: &Grid<T>) -> Option<T> {
        grid.get(self.row(), self.column()).cloned()
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

    fn manhattan_distance(&self, GridCoordinates(other_row, other_column): Self) -> usize {
        (self.row() - other_row).abs() as usize + (other_column - other_column).abs() as usize
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
    pub fn opposite(&self) -> Direction {
        self.rotate_clockwise_90().rotate_clockwise_90()
    }

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

    pub fn rotate_counter_clockwise_90(&self) -> Direction {
        self.rotate_clockwise_90()
            .rotate_clockwise_90()
            .rotate_clockwise_90()
    }
}

pub type X = isize;
pub type Y = isize;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XYCoordinates(pub X, pub Y);

impl XYCoordinates {
    pub fn x(&self) -> X {
        self.0
    }

    pub fn y(&self) -> Y {
        self.1
    }

    pub fn mirror_around_middle_x(&self, x_width: isize) -> Self {
        XYCoordinates(x_width - 1 - self.x(), self.y())
    }
}

impl Into<(X, Y)> for XYCoordinates {
    fn into(self) -> (X, Y) {
        (self.x(), self.y())
    }
}

impl From<(X, Y)> for XYCoordinates {
    fn from((x, y): (X, Y)) -> Self {
        XYCoordinates(x, y)
    }
}

impl From<XYCoordinates> for GridCoordinates {
    fn from(XYCoordinates(x, y): XYCoordinates) -> Self {
        GridCoordinates(y, x)
    }
}

impl Into<XYCoordinates> for GridCoordinates {
    fn into(self) -> XYCoordinates {
        XYCoordinates(self.column(), self.row())
    }
}

impl Add for XYCoordinates {
    type Output = XYCoordinates;

    fn add(self, other: XYCoordinates) -> Self {
        XYCoordinates(self.x() + other.x(), self.y() + other.y())
    }
}

impl Sub for XYCoordinates {
    type Output = XYCoordinates;
    fn sub(self, other: XYCoordinates) -> Self {
        XYCoordinates(self.x() - other.x(), self.y() - other.y())
    }
}

impl Coordinates for XYCoordinates {
    fn north_n(&self, n: isize) -> Option<Self> {
        GridCoordinates::from(*self)
            .north_n(n)
            .map(GridCoordinates::into)
    }

    fn east_n(&self, n: isize) -> Option<Self> {
        GridCoordinates::from(*self)
            .east_n(n)
            .map(GridCoordinates::into)
    }

    fn south_n(&self, n: isize) -> Option<Self> {
        GridCoordinates::from(*self)
            .south_n(n)
            .map(GridCoordinates::into)
    }

    fn west_n(&self, n: isize) -> Option<Self> {
        GridCoordinates::from(*self)
            .west_n(n)
            .map(GridCoordinates::into)
    }

    fn manhattan_distance(&self, other: Self) -> usize {
        GridCoordinates::from(*self).manhattan_distance(GridCoordinates::from(other))
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

pub fn set_grid_element<T>(grid: &mut Grid<T>, coordinates: &GridCoordinates, grid_element: T) {
    *(grid
        .get_mut(coordinates.row(), coordinates.column())
        .unwrap()) = grid_element;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position<C: Coordinates + Copy>(pub C, pub Direction);

impl<C: Coordinates + Copy> Position<C> {
    pub fn coordinates(&self) -> C {
        self.0
    }

    pub fn direction(&self) -> Direction {
        self.1
    }

    pub fn move1(&self) -> Option<Position<C>> {
        self.move_n(1)
    }

    pub fn move_n(&self, n: isize) -> Option<Position<C>> {
        self.coordinates()
            .move_to_n(self.direction(), n)
            .map(|coordinates| Position(coordinates, self.direction()))
    }

    pub fn rotate_clockwise_90(&self) -> Position<C> {
        Position(self.coordinates(), self.direction().rotate_clockwise_90())
    }

    pub fn rotate_counter_clockwise_90(&self) -> Position<C> {
        Position(
            self.coordinates(),
            self.direction().rotate_counter_clockwise_90(),
        )
    }
}

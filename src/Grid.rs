use arrayvec::ArrayVec;
use std::fmt::{write, Display, Formatter};

pub type SignedCoord = i32;

/** grid with strictly positive coordinates
*/
pub struct Grid<T: Display + Clone> {
    array: ArrayVec<T, { 10000000 }>,
    pub width: usize,
    pub height: usize,
    offset_x: SignedCoord,
    offset_y: SignedCoord,
}

impl<T: Display + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            array: ArrayVec::new(),
            width,
            height,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn new_w_offset(
        width: usize,
        height: usize,
        offset_x: SignedCoord,
        offset_y: SignedCoord,
    ) -> Grid<T> {
        Grid {
            array: ArrayVec::new(),
            width,
            height,
            offset_x,
            offset_y,
        }
    }

    /** creates a grid from coords.
    list: list of coords (x,y)
    padding adds 1 radius taxicab to grid.
    init: value to initialise to grid
    */
    pub fn new_dynamic(list: Vec<(SignedCoord, SignedCoord)>, padding: u8, init: T) -> Grid<T> {
        assert!(!list.is_empty());

        //find normalise values and width and height
        let mut max_x = SignedCoord::MIN;
        let mut max_y = SignedCoord::MIN;
        let mut min_x = SignedCoord::MAX;
        let mut min_y = SignedCoord::MAX;

        //find normalisation values
        for coordinate in list {
            let x = coordinate.0;
            let y = coordinate.1;

            if x > max_x {
                max_x = x;
            }
            if x < min_x {
                min_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            if y < min_y {
                min_y = y;
            }
        }

        //+1 because it counts the elements, yet array and coords start at 0
        //padding *2 because you have to add width and height to both sides
        let width: usize = max_x.abs_diff(min_x) as usize + (padding * 2) as usize + 1;
        let height = max_y.abs_diff(min_y) as usize + (padding * 2) as usize + 1;

        //moves the grid so it's nestled in +/+ zone of the grid, then adds padding
        let offset_x = padding as SignedCoord - min_x;
        let offset_y = padding as SignedCoord - min_y;

        let mut g = Grid {
            array: ArrayVec::new(),
            width,
            height,
            offset_x,
            offset_y,
        };
        g
    }

    pub fn getc(&self, coord: Coordinate) -> &T {
        let index = self.create_index_from(coord);
        &self.array[index]
    }

    pub fn setc(&mut self, coord: Coordinate, value: T) {
        let index = self.create_index_from(coord);
        self.array[index] = value
    }

    /** in case you are working with coordinates that start at a high value or are in the negative
     * you can pass them into this function to translate them into their normalised variants.
     * normalised variants are strictly positive and are basically a 2 dimensional offset
     */
    pub fn normalise_to_grid(&self, coordinate: (SignedCoord, SignedCoord)) -> Coordinate {
        let x = coordinate.0 + self.offset_x;
        let y = (coordinate.1 + self.offset_y);
        //todo maybe i have to check and do panic? not sure how safe cast works entirely
        Coordinate {
            x: x as usize,
            y: y as usize,
        }
    }

    /** in case you are working with coordinates that start at a high value or are in the negative
     * you can pass them into this function to translate them back to their original values.
     */
    pub fn normalise_away_from_grid(&self, coordinate: Coordinate) -> (SignedCoord, SignedCoord) {
        let x: SignedCoord = coordinate.x as SignedCoord - self.offset_x;
        let y: SignedCoord = coordinate.y as SignedCoord - self.offset_y;
        (x, y)
    }

    pub fn create_coordinate(&self, index: usize) -> Coordinate {
        let y = (index / self.width);
        let x = (index % self.width);
        Coordinate { x, y }
    }

    pub fn create_index_from(&self, coordinate: Coordinate) -> usize {
        (coordinate.x + coordinate.y * self.width) as usize
    }

    pub fn in_grid_neighbours(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let mut answer = Vec::new();
        if coordinate.x > 0 {
            answer.push(Coordinate {
                x: (coordinate.x - 1),
                y: coordinate.y,
            });
        }
        if coordinate.x < (self.width - 1) {
            answer.push(Coordinate {
                x: (coordinate.x + 1),
                y: coordinate.y,
            });
        }
        if coordinate.y > 0 {
            answer.push(Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
            });
        }
        if coordinate.y < (self.height - 1) {
            answer.push(Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
            });
        }
        answer
    }

    pub fn print(&self) {
        for index in 0..self.array.len() {
            if index % self.width == 0 {
                println!();
            }
            print!("{}", self.array[index]);
        }
        println!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Coordinate {
    pub fn taxicab_distance(&self, coord: Coordinate) -> usize {
        self.x.abs_diff(coord.x) + self.y.abs_diff(coord.y)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BinaryState(pub bool);

impl Display for BinaryState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match bool::from(*self) {
            true => {
                write!(f, "#")
            }
            false => {
                write!(f, ".")
            }
        }
    }
}

impl From<BinaryState> for bool {
    fn from(i: BinaryState) -> bool {
        true
    }
}

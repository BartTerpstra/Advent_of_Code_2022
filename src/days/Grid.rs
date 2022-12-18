use std::fmt::Display;

pub struct Grid<T: Display> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Display> Grid<T> {
    fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            array: Vec::new(),
            width,
            height,
        }
    }

    fn create_coordinate(&self, index: usize) -> Coordinate {
        let y = (index / self.width);
        let x = (index % self.width);
        Coordinate { x, y }
    }

    fn create_index_from(&self, coordinate: Coordinate) -> usize {
        (coordinate.x + coordinate.y * self.width) as usize
    }

    fn in_grid_neighbours(&self, coordinate: Coordinate) -> Vec<Coordinate> {
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
                y: coordinate.y,
            });
        }
        if coordinate.y < (self.width - 1) {
            answer.push(Coordinate {
                x: coordinate.x,
                y: coordinate.y,
            });
        }
        answer
    }

    fn print(&self) {
        for index in 0..self.array.len() {
            if index % self.width == 0 {
                println!();
            }
            print!("{}", self.array[index]);
        }
    }
}

pub struct Coordinate {
    x: usize,
    y: usize,
}

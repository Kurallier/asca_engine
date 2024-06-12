use sdl2::rect::Point;

#[derive(Debug, Clone, Copy)]
//Acutal Automata object, holds the position as a vec2, the state, and the amount of alive
//neighbors
pub struct ConAutomata {
    pub sdl_point: Point,
    cell_state: bool,
    pub alive_neighbors: u8,
}

impl ConAutomata {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: Clone + PartialEq + PartialOrd + Into<i32>,
    {
        ConAutomata {
            sdl_point: Point::new(x.into(), y.into()),
            cell_state: false,
            alive_neighbors: 0,
        }
    }

    pub fn set_automata_state(&mut self, alive: bool) {
        self.cell_state = alive;
    }

    pub fn get_automata_state(&self) -> bool {
        self.cell_state
    }

    //Returns the amount of alive neighbors aound the cell
    pub fn get_alive_neighbors(self) -> u8 {
        self.alive_neighbors
    }

    //Sets the amount of alive neighbors
    pub fn set_alive_neigbors(&mut self, alive_neighbors: u8) {
        self.alive_neighbors = alive_neighbors;
    }

    pub fn get_xy(self) -> (i32, i32) {
        let v = (self.sdl_point.x(), self.sdl_point.y());
        v
    }

    pub fn get_x(self) -> i32 {
        self.sdl_point.x()
    }

    pub fn get_y(self) -> i32 {
        self.sdl_point.y()
    }

    pub fn get_point(self) -> Point {
        self.sdl_point
    }

    pub fn println_alive_neigbors(&self) {
        println!("{}", self.alive_neighbors);
    }
}

use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
    video::Window,
};

mod con_automata;

use array2d::Array2D;
use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

use self::con_automata::ConAutomata;

#[derive(Debug, Clone)]
//This is the object that contains a 2d matrix of Automata objects
pub struct ConAutomataBoard(Array2D<ConAutomata>);

impl ConAutomataBoard {
    //This function takes 2 parameters:
    //board_w corresponds to the row;
    //board_h corresponds to collums
    pub fn init<T>(board_w: T, board_h: T) -> Self
    where
        T: Copy + PartialEq + PartialOrd + Into<usize>,
    {
        //Weird ass lambda function
        let mut r_count: usize = 0;
        let r_max: T = board_w;

        let mut c_count: usize = 0;
        let c_max: T = board_h;

        let increment = || {
            if r_count >= r_max.into() {
                r_count = 0
            }
            if c_count >= c_max.into() {
                c_count = 0;
                r_count += 1
            }
            let r_tmp = r_count;
            let c_tmp = c_count;

            c_count += 1;

            return ConAutomata::new(r_tmp as i32, c_tmp as i32);
        };
        //Wierd ass lambda function

        ConAutomataBoard(Array2D::filled_by_row_major(
            increment,
            board_w.into(),
            board_h.into(),
        ))
        .clone()
    }

    pub fn get_all_cells(&self) -> Vec<&ConAutomata> {
        let elements_iter = self.0.elements_row_major_iter();

        let cells = elements_iter.collect();
        cells
    }

    /*
    pub fn get_all_points(&self) -> Vec<&Point> {
        let elements_iter = self.0.elements_row_major_iter();
        let mut cells: Vec<&Point> = Vec::new();

        for i in elements_iter {
            cells.push(&i.get_point());
        }
        cells
    }
    */

    //Seeds the board with alive_chance representing the percentage (0.0 - 1.0)
    //of the cell  being alive
    pub fn seed_board(&mut self, alive_chance: f64) {
        let chance = Bernoulli::new(alive_chance).unwrap();
        //let mut rng = rand::thread_rng();

        let arr_size_r = self.0.num_rows();
        let arr_size_c = self.0.num_columns();

        for i in 0..arr_size_r {
            for j in 0..arr_size_c {
                let cell = self.0.get_mut(i, j);
                match cell {
                    Some(cell) => {
                        //let r_bool: bool = rng.gen_bool(alive_chance);
                        let v = chance.sample(&mut rand::thread_rng());
                        //let r_bool: bool = rng.gen_bool(p)
                        cell.set_automata_state(v);
                        //cell.set_automata_state(r_bool);
                    }

                    None => panic!(),
                }
            }
        }
    }

    //TODO: Refactor code if needed
    pub fn sdl2_draw_cells(&self, sdl2_draw: &mut Canvas<Window>) {
        sdl2_draw.set_draw_color(Color::MAGENTA);

        let conway_entities = self.get_all_cells();

        for i in conway_entities {
            let cell_state = i.get_automata_state();
            if cell_state == true {
                sdl2_draw.draw_point(i.get_point()).unwrap();
            }
        }

        println!("Drawn cells");
        /*
                sdl2_draw.set_draw_color(Color::GREEN);


                let points = self
                    .0
                    .elements_row_major_iter()
                    .filter(|&&x| x.get_automata_state() == true);

                let mut vec_points: Vec<Point> = Vec::new();

                points.for_each(|x| vec_points.push(x.get_point()));

                match sdl2_draw.draw_points(vec_points.as_slice()) {
                    Ok(_) => (),
                    Err(t) => panic!("{}", t),
                };
        */
    }

    pub fn get_cells_point(&mut self) -> Vec<Point> {
        let points: Vec<Point> = self
            .0
            .elements_row_major_iter()
            .map(|&cell| cell.get_point()).collect();
        points
    }

    //TODO: Refactor code
    pub fn calculate_surr_life(&mut self) {
        let board_w = self.0.num_rows();
        let board_h = self.0.num_columns();

        //Iterate through all the cells
        for curr_cell_x in 0..board_w {
            for curr_cell_y in 0..board_h {
                let mut alive_neighbors = 0;

                //Check around the current cell
                for check_x in (curr_cell_x as isize - 1)..=(curr_cell_x as isize + 1) {
                    for check_y in (curr_cell_y as isize - 1)..=(curr_cell_y as isize + 1) {
                        //If the current actual posistion is out of bounds
                        if (curr_cell_x == check_x as usize) && (curr_cell_y == check_y as usize)
                            || (check_x < 0 || check_y < 0)
                            || (check_x > (board_w - 1) as isize
                                || check_y > (board_h - 1) as isize)
                        {
                            continue;
                        } else {
                            //Get the cell that is being checked
                            //The check_x and check_y shouldn't ever be a negative number
                            let check_cell = self.0.get(
                                check_x.try_into().expect("Failed to become usize"),
                                check_y.try_into().expect("Failed to become usize"),
                            );
                            match check_cell {
                                //Match the Array2d .get() return
                                Some(cell) => {
                                    //If current cell is alive, += 1
                                    let state = cell.get_automata_state();
                                    if state == true {
                                        alive_neighbors += 1;
                                    } else {
                                        alive_neighbors += 0;
                                    }
                                }

                                //If the None type is returned, that means that we somehow
                                //wanted to go beyond the array
                                //This **SHOULDN'T** ever happen
                                None => {
                                    println!(
                                        "Current Cell (X: {}, Y: {}) \n
                                            Check (X: {}, Y: {}) \n
                                            Act_check (X: {}, Y:{}) \n
                                            Array dimensions (Rows: {}, Coll: {})",
                                        curr_cell_x,
                                        curr_cell_y,
                                        check_x,
                                        check_y,
                                        check_x,
                                        check_y,
                                        board_w,
                                        board_h
                                    );
                                }
                            }
                        }
                    }
                }

                //Grab the source cell
                let curr_cell = self
                    .0
                    .get_mut(curr_cell_x, curr_cell_y)
                    .expect("Failed to get current cell: CALC_LIFE");
                //Set the alive neighbors
                curr_cell.set_alive_neigbors(alive_neighbors);
            }
        }
    }

    pub fn apply_con_rules(&mut self) {
        //If cell is alive, and has less than 2 live negibors
        //it dies
        //
        //If cell is alive, and has 2 or 3 live neighbors
        //it lives
        //
        //If cell is alive, and has more than 3 live neigbors
        //it dies
        //
        //If cell is dead, and has exactly 3 neigbors
        //it lives
        let board_w = self.0.num_rows();
        let board_h = self.0.num_columns();

        //Iterate through all the cells
        for i in 0..board_w {
            for j in 0..board_h {
                let cell = self.0.get_mut(i, j).expect("APPLY_CON_RULES");

                let aliv_neigbors = cell.get_alive_neighbors();
                let state = cell.get_automata_state();

                if state == true && aliv_neigbors < 2 {
                    cell.set_automata_state(false);
                } else if state == true && aliv_neigbors == (2 | 3) {
                    cell.set_automata_state(true);
                } else if state == true && aliv_neigbors > 3 {
                    cell.set_automata_state(false);
                } else if state == false && aliv_neigbors == 3 {
                    cell.set_automata_state(true);
                } else {
                    cell.set_automata_state(state);
                }
                //Just in case that there's any bleed over from the last gen
                cell.set_alive_neigbors(0);
            }
        }
    }
}

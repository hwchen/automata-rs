// Conway's game of life
//got to figure out "map" in rust.
extern crate ncurses;
extern crate rand;

use ncurses as n;

struct Game {
    grid: Vec<Vec<bool>> 
}

impl Game {
    
    fn new(x: usize, y: usize) -> Game {
        let mut new_grid = vec![];
        for _ in 0.. x {
            let mut row: Vec<bool> = vec![];
            for _ in 0.. y {
                //add random
                let r = rand::random::<usize>() % 2usize;
                match r {
                    0 => row.push(false),
                    1 => row.push(true),
                    _ => row.push(false)
                }
            }
            new_grid.push(row)
        }
        Game {grid: new_grid}
    }

    fn string(&self) -> String {
        //map! (there's got to be an easier way
        //to map on two vectors?

        //map to string symbol
        let mut print_grid = vec![];
        for i in 0.. self.grid.len() {
            for j in 0.. self.grid[i].len() {
                if self.grid[i][j] == false {
                    print_grid.push("   ");     
                } 
                else {
                    print_grid.push(" * ");
                }
            }
            print_grid.push("\n");
        }
        print_grid.concat()
    }

    //since I have to build the whole grid on each refresh, I
    //can't have a separate add_alive().
    fn refresh_grid(&self, alives: Vec<(usize, usize)>) -> Game {
        //initialize a new grid to false (could just call self.new()?)
        let mut new_grid = vec![];
        for i in 0.. self.grid.len() {
            let mut row: Vec<bool> = vec![];
            for _ in 0.. self.grid[i].len() {
                row.push(false);
            }
            new_grid.push(row)
        }
        //add all the alives (don't need to worry about those marked
        //for dead
        for xy in alives.iter(){
            match *xy {
                (x,y) => new_grid[x][y] = true
            }
        }
        Game {grid: new_grid}
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        //top left
        if (x > 0) && (y > 0) && (self.grid[x-1][y-1] == true) {
            sum += 1;
        }
        //top
        if (x > 0) && (self.grid[x-1][y] == true) {
            sum += 1;
        }
        //top right
        if (x > 0) && (y < self.grid[x].len()-1) && (self.grid[x-1][y+1] == true) {
            sum += 1;
        }
        //below left
        if (x < self.grid.len()-1) && (y > 0) && (self.grid[x+1][y-1] == true) {
            sum += 1;
        }
        //below
        if (x < self.grid.len()-1) && (self.grid[x+1][y] == true) {
            sum += 1;
        }
        //below right
        if (x < self.grid.len()-1) && (y < self.grid[x].len()-1) && (self.grid[x+1][y+1] == true) {
            sum += 1;
        }
        //left
        if (y > 0) && (self.grid[x][y-1] == true) {
            sum += 1;
        }
        //right
        if (y < self.grid[x].len()-1) && (self.grid[x][y+1] == true) {
            sum += 1;
        }
        sum
    }

    fn calc_next_alive(&self) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = vec![];
        for x in 0.. self.grid.len(){
            for y in 0.. self.grid[x].len() {
                let neighbors = self.count_neighbors(x,y);
                match neighbors {
                    2 if self.grid[x][y] == true => res.push((x,y)),
                    3                            => res.push((x,y)),
                    _                            => continue
                }
            }
        }
        res
    }
}


fn main() {
    
    let mut game = Game::new(20, 20);
    n::initscr();
    loop{
        n::printw("Conway's Game of Life\n");
        n::printw("by Walther Chen\n\n\n");
        n::printw(&game.string()[..]);
        n::refresh();
        
        let alive = game.calc_next_alive();
        game = game.refresh_grid(alive);
        n::napms(1000);
        n::clear();
    }
    // doesn't reach. figure out how to break loop in-program
    //n::endwin();

    


//    tests for neighbor count. change when I figure out [#test]
//    let neighbors = game.count_neighbors(1,1);
//    println!("(1,1) has {} neighbors", neighbors);

//    let neighbors = game.count_neighbors(0,0);
//    println!("(0,0) has {} neighbors", neighbors);
    
//    let neighbors = game.count_neighbors(0,1);
//    println!("(0,1) has {} neighbors", neighbors);

//    let neighbors = game.count_neighbors(1,4);
//    println!("(1,4) has {} neighbors", neighbors);

//    let neighbors = game.count_neighbors(19,0);
//    println!("(19,0) has {} neighbors", neighbors);
    
//    let neighbors = game.count_neighbors(0,9);
//    println!("(0,9) has {} neighbors", neighbors);

//    let neighbors = game.count_neighbors(19,9);
//    println!("(19,9) has {} neighbors", neighbors);
}





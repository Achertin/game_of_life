use rand::prelude::*;
use bmp::Image;

const DIMENSIONS: usize = 128;
const SQUARE: usize = DIMENSIONS * DIMENSIONS; 

fn generate(starting_cells: usize) -> [bool; SQUARE] {
    let mut world = [false; SQUARE];
    let mut rng = rand::rng();
    
    for _ in 0..starting_cells {
        world[rng.random_range(0..SQUARE)] = true;
    }

    world
}

fn save(world: &[bool; SQUARE], img: &mut Image, tick: usize) {
    for cell in 0..SQUARE {
        let row: u32 = (cell / DIMENSIONS) as u32;
        let col: u32 = (cell % DIMENSIONS) as u32;

        if world[cell] {
            img.set_pixel(row, col, bmp::consts::WHITE);
            continue;
        }
        img.set_pixel(row, col, bmp::consts::BLACK);
    }

    let _ = img.save(format!("./images/world_{:04}.bmp", tick)).unwrap_or_else(|e| {
        panic!("Failed to save: {}", e)
    });
}

fn neighbours(world: &[bool; SQUARE], row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;

    for &rowi in [-1, 0, 1].iter() {
        if row == 0 && rowi == -1 { continue; } 
        if row == DIMENSIONS - 1 && rowi == 1 { continue; }
        for &coli in [-1, 0, 1].iter() {
            if col == 0 && coli == -1 { continue; }
            if col == DIMENSIONS - 1 && coli == 1 { continue; }
            if rowi == 0 && coli == 0 { continue; }

            let row_final = row as i64 + rowi;
            let col_final = col as i64 + coli;
            let pos = DIMENSIONS as i64 * row_final + col_final;
            if world[pos as usize] { count += 1 }
        }
    }

    count
}

fn next(world: [bool; SQUARE]) -> [bool; SQUARE] {
    let mut new_world = [false; SQUARE];

    for pos in 0..SQUARE {
        let row = pos / DIMENSIONS;
        let col = pos % DIMENSIONS;
        let neighbours = neighbours(&world, row, col);

        new_world[pos] = if world[pos] { 
            if neighbours == 2 || neighbours == 3 { true } 
            else { false } 
        } else { 
            if neighbours == 3 { true } else { false } 
        };
    }

    new_world
}


fn main() {
    let mut world = generate(DIMENSIONS * 20);
    let mut img = Image::new(DIMENSIONS as u32, DIMENSIONS as u32);
    save(&world, &mut img, 0);

    for tick in 1..500 {
        world = next(world);
        save(&world, &mut img, tick);
        println!("Finished tick: {tick}");
    }

    println!("Finished!");
}

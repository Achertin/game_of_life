use rand::prelude::*;
use bmp::Image;

const DIMENSIONS: usize = 256;
const SQUARE: usize = DIMENSIONS * DIMENSIONS; 

fn generate_world(starting_cells: u32) -> [bool; SQUARE] {
    let mut world = [false; SQUARE];
    let mut rng = rand::rng();
    
    for _ in 0..starting_cells {
        world[rng.random_range(0..SQUARE)] = true;
    }

    world
}

fn save_bitmap(world: &[bool; SQUARE], tick: u32) {
    let mut img = Image::new(DIMENSIONS as u32, DIMENSIONS as u32); 

    for cell in 0..SQUARE {
        let x: u32 = (cell / DIMENSIONS) as u32;
        let y: u32 = (cell % DIMENSIONS) as u32;

        if world[cell] {
            img.set_pixel(x, y, bmp::consts::WHITE);
            continue;
        }
        img.set_pixel(x, y, bmp::consts::BLACK);
    }

    let _ = img.save(format!("./images/world_{tick}.bmp")).unwrap_or_else(|e| {
        panic!("Failed to save: {}", e)
    });
}

fn main() {
    let world = generate_world(100);
    save_bitmap(&world, 10);
    println!("Finished!")
}

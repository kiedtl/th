mod dirs;
mod drunk;
mod features;
mod randrm;
mod rect;

use std::vec::Vec;
use rand::prelude::*;

type Dungeon = [[f64; 205]; 50];

fn main() {
    // 50 rows; 205 columns
    let mut map = [[0.0; 205]; 50];

    //drunk::walk(&mut map);
    //randrm::tunnel(&mut map);
    tunneler(&mut map);
    display(map);
}

fn display(map: Dungeon) {
    for y in 0..50 {
        for x in 0..205 {
            if map[y][x] > 3.0 {//|| map[y][x] <= 1.0 {
                print!(" ");
            } else {
                print!("#");
            }
            //print!("{}", map[y][x] as u8);
        }
        print!("\n");
    }
}

fn tunneler(map: &mut Dungeon) {
    // TODO: pass rng as argument
    let mut rng = rand::thread_rng();

    let mut features: Vec<features::Feature> = Vec::new();
    let mut feature_coords: Vec<rect::Rect> = Vec::new();

    let room_max_width  = 16;
    let room_min_width  = 05;

    let room_max_height = 09;
    let room_min_height = 04;

    //let max_rooms     = (50 * 205) /
    //    (room_max_width * room_max_height * 4);
    let max_rooms = 10;

    // plop the first room down randomly
    let w = rng.gen_range(room_min_width,  room_max_width);
    let h = rng.gen_range(room_min_height, room_max_height);
    let x = rng.gen_range(1, 205 - w - 2); // MAP_WIDTH
    let y = rng.gen_range(1, 50 - h - 2); // MAP_HEIGHT

    let initial = rect::Rect::new(x, y, x + w, y + h);
    create_room(map, &initial);
    features.push(features::Feature::Room); feature_coords.push(initial);

    let mut failed = 0;

    for _ in 1..max_rooms {
        // pick a random room/tunnel
        let roomidx = rng.gen_range(0, features.len());
        let room    = &feature_coords[roomidx];
        let (allowed_features, allowed_walls) = choose_next_feature(&features);

        // pick a random wall and tile
        let wall_direction =
            &allowed_walls[rng.gen_range(0, allowed_walls.len() - 1)];
        let wall = room.wall(&wall_direction);
        // the "wall" contains squares outside the
        // room, we don't want our tunnel starting outside the room
        //let tile = rng.choose(wall[1..(wall.len() - 1)]);
        let tile = wall[wall.len() / 2];

        let new_feature =
            &allowed_features[rng.gen_range(0, allowed_features.len() - 1)];
        let new_feature_coords: rect::Rect;
        match new_feature {
            features::Feature::HorizTunnel => {
                let w = rng.gen_range(room_min_width,  room_max_width);
                let h = 1; // TODO: variable heights for hallways
                //let x = rng.gen_range(1, 205 - w - 2); // MAP_WIDTH
                //let y = rng.gen_range(1, 50 - h - 2); // MAP_HEIGHT
                let (y, x) = tile;
                new_feature_coords = rect::Rect::new(x, y, x + w, y + h);
            },
            features::Feature::VertiTunnel => {
                let w = 1; // TODO: variable heights for hallways
                let h = rng.gen_range(room_min_height, room_max_height);
                //let x = rng.gen_range(1, 205 - w - 2); // MAP_WIDTH
                //let y = rng.gen_range(1, 50 - h - 2); // MAP_HEIGHT
                let (y, x) = tile;
                new_feature_coords = rect::Rect::new(x, y, x + w, y + h);
            },
            features::Feature::Room => {
                let w = rng.gen_range(room_min_width,  room_max_width);
                let h = rng.gen_range(room_min_height, room_max_height);
                //let x = rng.gen_range(1, 205 - w - 2); // MAP_WIDTH
                //let y = rng.gen_range(1, 50 - h - 2); // MAP_HEIGHT
                let (y, x) = tile;
                new_feature_coords = rect::Rect::new(x, y, x + w, y + h);
            },
        }

        // check if there's space for the feature
        let does_intersect =
            |r: &rect::Rect, group: &Vec<rect::Rect>| {
                for hovel in group {
                    if r.intersects(hovel, 0) {
                        return true;
                    }
                }

                false
            };

        if does_intersect(&new_feature_coords, &feature_coords) ||
            does_intersect(&new_feature_coords, &feature_coords) {
                failed += 1;
                continue; // try again
        }

        // so there was space after all. surprise surprise
        create_room(map, &new_feature_coords);
        feature_coords.push(new_feature_coords);
    }
}

fn create_room(map: &mut Dungeon, room: &rect::Rect) {
    // set all squares in rectange to 5.0
    for y in (room.y1 - 1)..room.y2 {
        for x in (room.x1 - 1)..room.x2 {
            map[y][x] = 5.0;
        }
    }
}

fn choose_next_feature(features: &Vec<features::Feature>) -> (&[features::Feature], &[dirs::Direction]) {
    let last = &features[features.len() - 1];
    match last {
        features::Feature::HorizTunnel => {
            return (&[features::Feature::Room],
                &[dirs::Direction::North, dirs::Direction::South])
        },
        features::Feature::VertiTunnel => {
            return (&[features::Feature::Room],
                &[dirs::Direction::East, dirs::Direction::West])
        },
        features::Feature::Room => {
            return (&[features::Feature::VertiTunnel, features::Feature::HorizTunnel],
                &[dirs::Direction::North, dirs::Direction::South,
                    dirs::Direction::East, dirs::Direction::West])
        },
    }
}

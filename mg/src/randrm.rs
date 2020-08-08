// this isn't a tunneling algorithm so much
// as it is just plopping dozens of random rooms
// around and then connecting them with
// overlapping tunnels later.

use crate::rect;
use std::vec::Vec;
use std::cmp::{min, max};
use rand::prelude::*;

pub fn tunnel(map: &mut [[f64; 205]; 50]) {
    // TODO: pass rng as argument
    let mut rng = rand::thread_rng();

    let room_max_width  = 16;
    let room_max_height = 09;
    let room_min_width  = 04;
    let room_min_height = 03;
    let max_rooms     = (50 * 205) /
        (room_max_width * room_max_height * 4);

    let mut rooms: Vec<rect::Rect> = Vec::new();
    let mut tunnels: Vec<rect::Rect> = Vec::new();
    let mut num_rooms = 0;

    let mut failed_rooms = 0;

    let mut does_intersect =
        |r: &rect::Rect, group: &Vec<rect::Rect>| {
            for hovel in group {
                if r.intersects(hovel) {
                    return true;
                }
            }

            false
        };

    for _ in 0..max_rooms {
        // random width and height
        let w = rng.gen_range(room_min_width,  room_max_width);
        let h = rng.gen_range(room_min_height, room_max_height);

        // random position within map boundaries
        let x = rng.gen_range(1, 205 - w - 2); // MAP_WIDTH
        let y = rng.gen_range(1, 50 - h - 2); // MAP_HEIGHT

        let new_room = rect::Rect::new(x, y, x + w, y + h);

        // check for overlap
        // some overlap is OK, though:
        //      a room cannot overlap a tunnel.
        //      a tunnel CAN overlap a room... sort of
        //      a tunnel CAN overlap a tunnel.
        //      a room cannot overlap a room.
        if !does_intersect(&new_room, &rooms)
                && !does_intersect(&new_room, &tunnels) {
            create_room(map, &new_room);

            rooms.push(new_room);
            num_rooms += 1;
        } else { failed_rooms += 1; }
    }

    // sort rooms by distance
    let mut closest_rooms = rooms.to_vec();
    closest_rooms.sort_unstable_by_key(|k| {
        k.distance(&rooms[0]);
    });

    // create some tunnels
    // all rooms after the first room connect
    // to another previously-created room
    for r in 1..num_rooms {
        let (new_x, new_y) = closest_rooms[r].center();

        //let other = rng.gen_range(0, num_rooms);
        let other = r - 1; // DEBUG
        let (prev_x, prev_y) = closest_rooms[other].center();

        // should the tunnel start
        // horizontally or vertically?
        let start_with_horiz_tunnel: bool = rng.gen();

        // tunnels
        let mut tun1: rect::Rect;
        let mut tun2: rect::Rect;

        // decide on the layour of the tunnels
        // in a way that still connects both rooms but
        // hopefully does not overlap other enclosures
        if start_with_horiz_tunnel {
            tun1 = create_horiz_tunnel(prev_x, new_x, prev_y);
            tun2 = create_verti_tunnel(prev_y, new_y, new_x);
        } else {
            tun1 = create_verti_tunnel(prev_y, new_y, prev_x);
            tun2 = create_horiz_tunnel(prev_x, new_x, new_y);
        }

        // draw the rooms
        create_room(map, &tun1); tunnels.push(tun1.clone());
        create_room(map, &tun2); tunnels.push(tun2.clone());
    }
}

fn create_room(map: &mut [[f64; 205]; 50], room: &rect::Rect) {
    // set all squares in rectange to 5.0
    for y in (room.y1 - 1)..room.y2 {
        for x in (room.x1 - 1)..room.x2 {
            map[y][x] = 5.0;
        }
    }
}

fn create_horiz_tunnel(x1: usize, x2: usize, y: usize) -> rect::Rect {
    rect::Rect::new(min(x1, x2), y, max(x1, x2), y)
}

fn create_verti_tunnel(y1: usize, y2: usize, x: usize) -> rect::Rect {
    rect::Rect::new(x, min(y1, y2), x, max(y1, y2))
}

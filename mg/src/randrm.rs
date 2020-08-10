// this isn't a tunneling algorithm so much
// as it is just plopping dozens of random rooms
// around and then connecting them with
// overlapping tunnels later.

use crate::rect::*;
use crate::dun_s1::*;
use std::vec::Vec;
use std::cmp::{min, max};
use rand::prelude::*;

pub struct RandomRooms<'a, R: Rng> {
pub    map: &'a mut DungeonS1,
pub    max_rooms: Option<usize>,
pub    room_max_width: usize,
pub    room_max_height: usize,
pub    room_min_width: usize,
pub    room_min_height: usize,
pub    rng: &'a mut R,
}

impl <'a, R: Rng> RandomRooms<'a, R> {
    pub fn new(map: &'a mut DungeonS1, rng: &'a mut R) -> RandomRooms<'a, R> {
        RandomRooms {
            map: map,
            max_rooms: None,
            room_max_width: 16,
            room_max_height: 8,
            room_min_width: 4,
            room_min_height: 2,
            rng: rng,
        }
    }

    pub fn max_rooms(&'a mut self, value: usize) -> &'a mut RandomRooms<'a, R> {
        self.max_rooms = Some(value);
        self
    }

    pub fn room_max_width(&'a mut self, value: usize) -> &'a mut RandomRooms<'a, R> {
        self.room_max_width = value;
        self
    }

    pub fn room_max_height(&'a mut self, value: usize) -> &'a mut RandomRooms<'a, R> {
        self.room_max_height = value;
        self
    }

    pub fn room_min_width(&'a mut self, value: usize) -> &'a mut RandomRooms<'a, R> {
        self.room_min_width = value;
        self
    }

    pub fn room_min_height(&'a mut self, value: usize) -> &'a mut RandomRooms<'a, R> {
        self.room_min_height = value;
        self
    }

    pub fn tunnel(&mut self) {
        let max_rooms: usize;
        match self.max_rooms {
            None => {
                max_rooms = (self.map.height * self.map.width) /
                    (self.room_max_width * self.room_max_height * 4);
            },
            Some(n) => max_rooms = n,
        }

        let mut rooms: Vec<Rect> = Vec::new();
        let mut tunnels: Vec<Rect> = Vec::new();
        let mut num_rooms = 0;

        let does_intersect =
            |r: &Rect, group: &Vec<Rect>| {
                for hovel in group {
                    if r.intersects(hovel, 4) {
                        return true;
                    }
                }
                false
            };

        for _ in 0..max_rooms {
            // random width and height
            let w = self.rng.gen_range(self.room_min_width,  self.room_max_width);
            let h = self.rng.gen_range(self.room_min_height, self.room_max_height);

            // random position within map boundaries
            let x = self.rng.gen_range(1, self.map.width  - w - 2);
            let y = self.rng.gen_range(1, self.map.height - h - 2);

            let new_room = Rect::new(x, y, x + w, y + h);

            // check for overlap
            if !does_intersect(&new_room, &rooms)
                    && !does_intersect(&new_room, &tunnels) {
                self.create_room(&new_room);

                rooms.push(new_room);
                num_rooms += 1;
            }
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

            let other = r - 1;
            let (prev_x, prev_y) = closest_rooms[other].center();

            // should the tunnel start
            // horizontally or vertically?
            let start_with_horiz_tunnel: bool = self.rng.gen();

            // tunnels
            let tun1: Rect;
            let tun2: Rect;

            fn create_horiz_tunnel(x1: usize, x2: usize, y: usize) -> Rect {
                Rect::new(min(x1, x2), y, max(x1, x2), y)
            }

            fn create_verti_tunnel(y1: usize, y2: usize, x: usize) -> Rect {
                Rect::new(x, min(y1, y2), x, max(y1, y2))
            }

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
            self.create_room(&tun1); tunnels.push(tun1.clone());
            self.create_room(&tun2); tunnels.push(tun2.clone());
        }
    }

    fn create_room(&mut self, room: &Rect) {
        // set all squares in rectange to 5.0
        for y in (room.y1 - 1)..room.y2 {
            for x in (room.x1 - 1)..room.x2 {
                self.map.set(x, y, TileType::Floor);
            }
        }
    }
}

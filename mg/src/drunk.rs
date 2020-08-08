use rand::prelude::*;

//struct Drunkard {
//}

pub fn drunkards(map: &mut [[char; 130]; 200]) {
    let mut rng = rand::thread_rng();

    let mapheight = 200.0;
    let mapwidth  = 130.0;

    let iterations = 500000;
    let weighted_towards_center = 0.05;
    let weighted_towards_previous_direction = 0.7;
    let percent_goal = 0.45;

    let mut filled = 0;
    let mut previous_direction = ""; // TODO: enum

    let filled_goal  = mapwidth * mapheight * percent_goal;
    let mut walker_pos_x: isize = rng.gen_range(2, (mapwidth as usize)  - 2) as isize;
    let mut walker_pos_y: isize = rng.gen_range(2, (mapheight as usize) - 2) as isize;

    let mut walk = || { // rustc, why do I need this mut again?
        // probability of going in a direction
        let mut north = 1.0;
        let mut south = 1.0;
        let mut east  = 1.0;
        let mut west  = 1.0;

        // weight the random walk against map edges
        if (walker_pos_x as f64) < mapwidth * 0.25 { // walker is at far left
            east += weighted_towards_center;
        } else if (walker_pos_x as f64) > mapwidth * 0.75 { // walker is at far right
            west += weighted_towards_center;
        }

        if (walker_pos_y as f64) < mapheight * 0.25 { // walker is at the top
            south += weighted_towards_center;
        } else if (walker_pos_y  as f64) > mapheight * 0.75 { // walker is at the bottom
            north += weighted_towards_center;
        }

        match previous_direction {
            "north" => north -= weighted_towards_previous_direction,
            "south" => south -= weighted_towards_previous_direction,
            "west"  => west  -= weighted_towards_previous_direction,
            "east"  => east  -= weighted_towards_previous_direction,
            _ => (),
        }

        // normalize probabilities so they form a range from 0..1
        let total = north + south + east + west;
        north /= total;
        south /= total;
        east /= total;
        west /= total;

        // choose the direction to walk into
        let mut direction = "";
        let dx: isize;
        let dy: isize;
        let choice: f64 = rng.gen();

        if 0.0 <= choice && choice < north {
            direction = "north";
            dx = 0;
            dy = -1;
        } else if north <= choice  && choice < (north + south) {
            direction = "south";
            dx = 0;
            dy = 1;
        } else if (north + south) <= choice  && choice < (north + south + east) {
            direction = "east";
            dx = 1;
            dy = 0;
        } else {
            direction = "west";
            dx = -1;
            dy = 0;
        }

        // the actual walking
        if (0 < walker_pos_x + dx && walker_pos_x + dx < (mapwidth as isize) - 1) &&
            (0 < walker_pos_y + dy && walker_pos_y + dy < (mapheight as isize) - 1) {
                walker_pos_x += dx;
                walker_pos_y += dy;

                if map[walker_pos_y as usize][walker_pos_x as usize] == '#' {
                    map[walker_pos_y as usize][walker_pos_x as usize] = ' ';
                    filled += 1;
                } else {
                    map[walker_pos_y as usize][walker_pos_x as usize] = '#';
                    //filled -= 1;
                }

                previous_direction = direction;
        }

        (filled as f64) >= filled_goal
    };

    for _ in 0..iterations {
        if walk() {
            break;
        }
    }
}


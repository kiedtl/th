use crate::state::*;
use lib::mob::*;
use lib::dun_s1::*;
use lib::math::*;
use crate::coord::*;
use doryen_fov::{
    FovAlgorithm,
    FovRecursiveShadowCasting,
    MapData
};

const PLAYER_VIEW_RADIUS: usize = 20;
const MAX_PLAYER_MEMORY: usize = 1024 * 1024 * 1024;

pub fn player_tick(st: &mut State) {
    let player = &mut st.dungeon.player;
    let player_y = player.coords.0;
    let player_x = player.coords.1;

    let map_width = st.dungeon.levels[player.level].width;
    let map_height = st.dungeon.levels[player.level].height;

    // so first we update the FOV
    let mut fov = FovRecursiveShadowCasting::new();
    let mut map = MapData::new(map_width, map_height);

    let starty = player_y.saturating_sub(map_height / 2);
    let endy   = clamp(player_y + (map_height / 2), 0, map_height);
    let startx = player_x.saturating_sub(map_width / 2);
    let endx   = clamp(player_x + (map_width / 2), 0, map_width);

    for y in starty..endy {
        for x in startx..endx {
            let tile = st.dungeon.levels[player.level].d[y][x].tiletype;

            let is_transparent = if tile == TileType::Wall {
                false
            } else {
                true
            };

            map.set_transparent(x, y, is_transparent);
        }
    }

    map.clear_fov();
    fov.compute_fov(&mut map, player_x, player_y, PLAYER_VIEW_RADIUS, true);

    player.in_fov = Vec::new();
    for y in startx..endy {
        for x in startx..endx {
            if map.is_in_fov(x, y) {
                // add to field of vision
                player.in_fov.push((y, x));

                // add to memory, so it will be shown even
                // if not in field of vision
                if !player.memory.contains(&(y, x)) {
                    if player.memory.len() < MAX_PLAYER_MEMORY {
                        player.memory.push((y, x));
                    }
                }
            }
        }
    }
}

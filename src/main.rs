mod wad;
use macroquad::prelude::*;

// Config for window
fn conf() -> Conf {
    Conf {
        window_title: "Doom Map Viewer".to_string(),
        fullscreen: false,

        window_width: 640,
        window_height: 480,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // Wad loading stuff
    let wad_file = wad::Wad::load("assets/freedoom1.wad");

    if wad_file.wad_id != "IWAD".to_string() {
        panic!("not an IWAD");
    }

    let e1m1 = wad_file.maps.get("E1M1\0\0\0\0").unwrap();
    // Get spawn location
    let mut loc = &e1m1.p1_spawn;
    let mut rot = e1m1.p1_rot as f64;

    loop {
        clear_background(BLACK);

        let sorted_ssecs = e1m1.traverse_bsp(&e1m1.nodes.len() - 1, loc);
        let cut_segs = e1m1.cut_nonvis(&sorted_ssecs, loc, rot);
        println!("{}", cut_segs.len());
        draw_segs(e1m1, cut_segs, loc, rot);

        next_frame().await;
    }
}

fn draw_segs(map: &wad::BspMap, segs: Vec<&wad::Seg>, loc: &wad::Vertex, rot: f64) {
    // Gives which columns are able to be drawn on
    let mut free: Vec<usize> = (0..640).collect();

    for seg in segs {
        if free.len() == 0 {
            return;
        }
        // Get info on vertices
        let start = &map.vertices[seg.start as usize];
        let end = &map.vertices[seg.end as usize];

        // Get linedef info
        let linedef = &map.linedefs[seg.linedef_num as usize];
        let has_front = linedef.front_sidedef != -1;
        let has_back = linedef.back_sidedef != -1;

        let direction = seg.direction;

        let mut start_angle = wad::norm(wad::pos_to_angle(loc, start));
        let start_x = angle_to_x(start_angle - rot);

        let end_angle = wad::norm(wad::pos_to_angle(loc, end));
        let end_x = angle_to_x(end_angle - rot);

        // println!(
        //     "{start_angle}, {end_angle}, {start_x}, {end_x}, {}, {}, {}",
        //     seg.linedef_num, seg.start, seg.end
        // );
        // draw_line(start_x, 0.0, start_x, 480.0, 1.0, WHITE);
        // draw_line(end_x, 0.0, end_x, 480.0, 1.0, WHITE);

        if seg.direction {
            let back_sidedef = &map.sidedefs[linedef.back_sidedef as usize];
            let back_sector = &map.sectors[back_sidedef.facing_sector as usize];

            let floor_height = back_sector.floor_height - 41;
            let ceiling_height = back_sector.ceiling_height - 41;
            let dist = ((start.x - loc.x).pow(2) as f64 + (start.y - loc.y).pow(2) as f64).sqrt();

            let scale = (90.0 - start_angle + rot).sin() * dist;

            draw_line(
                start_x,
                (floor_height as f64 / scale) as f32 * 320.0 - 320.0,
                start_x,
                (ceiling_height as f64 / scale) as f32 * 320.0 - 320.0,
                1.0,
                WHITE,
            )
        } else {
            let front_sidedef = &map.sidedefs[linedef.front_sidedef as usize];
            let front_sector = &map.sectors[front_sidedef.facing_sector as usize];

            let floor_height = front_sector.floor_height - 41;
            let ceiling_height = front_sector.ceiling_height - 41;
            let dist = (((start.x - loc.x) as f64).powi(2) as f64
                + ((start.y - loc.y) as f64).powi(2))
            .sqrt();

            let scale = (90.0 - start_angle + rot).sin() * dist;

            draw_line(
                start_x,
                (floor_height as f64 / scale) as f32 * 320.0 + 320.0,
                start_x,
                (ceiling_height as f64 / scale) as f32 * 320.0 + 320.0,
                1.0,
                WHITE,
            );
            draw_line(
                end_x,
                (floor_height as f64 / scale) as f32 * 320.0 + 320.0,
                end_x,
                (ceiling_height as f64 / scale) as f32 * 320.0 + 320.0,
                1.0,
                WHITE,
            );
        }
    }
}

fn angle_to_x(angle: f64) -> f32 {
    return (angle.to_radians().tan()) as f32 * 320.0 + 320.0;
}

mod wad;

fn main() {
    let wad_file = wad::Wad::load("assets/freedoom1.wad");
    
    if wad_file.wad_id != "IWAD".to_string() {
        panic!("not an IWAD");
    }

    let e1m1 = wad_file.maps.get("E1M1\0\0\0\0").unwrap();
    println!("{}", e1m1.sectors.len());
}
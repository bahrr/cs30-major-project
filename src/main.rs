use std::fs;
use std::str;

// A WAD is the primary way that Doom and it's source ports store data
struct Wad {
    // Header of the WAD file, used for identifying details
    identification: String, // Identifies the WAD as either an IWAD for the base game or a PWAD for a mod
    numlumps: u32, // Gets the size of the WAD in lumps
}

impl Wad {
    // Loads the file into a struct
    fn load(path: &str) -> Wad {
        // Opens the file
        let file = fs::read(path)
            .expect("File might not exist or some other problem opening it");

        let wad_id = str::from_utf8(&file[0..4])
            .expect("Header not valid");

        return Wad {
            identification: wad_id.to_string(),
            numlumps: file[4..8],
        }
    }
}

fn main() {
    let iwad = Wad::load("assets/freedoom.wad");

    // Just quickly reads out the IWAD's header
    println!("{}, {}", iwad.identification, iwad.numlumps);
}

use std::fs;
use std::str;
use std::path::Path;

// A WAD is the primary way that Doom and it's source ports store data
struct Wad {
    // Header of the WAD file, used for identifying details
    identification: bool, // Identifies the WAD as either an IWAD for the base game or a PWAD for a mod
    numlumps: u32, // Gets the size of the WAD in lumps
    infotableofs: u32, // Pointer to the location of directory
}

impl Wad {
    fn is_iwad(&self) {
        println!("{}, {}, {}",
            self.identification,
            self.numlumps,
            self.infotableofs
        );
    }
}

fn check_wad_type(file_path: &Path) -> String {
    let wad_file = fs::read(file_path).unwrap();
    return match String::from_utf8(wad_file[0..4].to_vec()) {
        Ok(result) => result,
        Err(error) => panic!("Problem with converting file: {}", error),
    };
}
fn main() {
    let path = Path::new("assets/freedoom1.wad");
    let wad = check_wad_type(path);
    println!("{}", wad);
}

use std::fs;
use std::str;
use byteorder::ByteOrder;
use byteorder::LittleEndian;

// A WAD is the primary way that Doom and it's source ports store data

struct Wad {
    // Header of the WAD file, used for identifying details
    identification: String, // Identifies the WAD as either an IWAD for the base game or a PWAD for a mod

    maps: Vec<BspMap>,
}
// Struct which stores Doom maps
struct  BspMap {
    things: Vec<Thing>,
}

// Holds onto raw lump data
struct Lump {
    name: String,

    // A vector to store the raw data of thy lump
    data: Vec<u8>,
}
// Things are 2d objects like monsters or items
struct Thing {
    x: i16,
    y: i16,
    angle: i16,
    thing_type: ThingType,

    // Keeps track of if the thing exists in a particular difficulty
    // or exists in multiplayer
    easy: bool,
    medium: bool,
    hard: bool,
    multiplayer: bool,

    // Is the monster waiting for an ambush later on
    ambush: bool
}

// Keeps track of what type of thing it is
enum ThingType {
}

impl Wad {
    // Loads the file into a struct
    fn load(path: &str) -> Wad {
        // Opens the file
        let file = fs::read(path)
            .expect("File might not exist or some other problem opening it");

        let wad_id = String::from_utf8(file[0..4].to_vec())
            .expect("Header not valid");

        let num_of_lumps = <LittleEndian as ByteOrder>::read_u32(&file[4..8]);

        // Points to where the directory which keeps track of lumps is
        let info_table = <LittleEndian as ByteOrder>::read_u32(&file[8..12]);

        let mut lumps: Vec<Lump> = Vec::new(); // Stores the raw lumps to go over in a list
        let mut maps:Vec<BspMap> = Vec::new(); // Stores the game maps

        // Appends the lump vector with lumps obtained from the WAD
        for i in 0..num_of_lumps {
            // Location of the start of the directory entry
            let dir_loc = (info_table + 16 * i).try_into().unwrap();

            // Where in the directory is the lump
            let lump_pos = <LittleEndian as ByteOrder>::read_u32(&file[dir_loc..dir_loc+4]) as usize;

            // How big is the lump
            let lump_size = <LittleEndian as ByteOrder>::read_u32(&file[dir_loc+4..dir_loc+8]) as usize;
            
            // The name of the lump
            let lump_name = String::from_utf8(file[dir_loc+8..dir_loc+16].to_vec())
                .unwrap();
            
            // The raw bytes of the lump as a vector
            let raw_bytes = file[lump_pos..lump_pos + lump_size].to_vec();

            lumps.push(Lump {name: lump_name, data: raw_bytes});
        }
        
        // Goes over map lumps to convert into something usable for a renderer
        let mut i = 0;
        let mut lump_name = lumps[0].name.as_str();

        while lump_name != "PLAYPAL\0" { // PLAYPAL with a null character happens to be the first non map lump in a WAD file
            println!("{lump_name}");

            let mut map_lumps: [Vec<u8>; 11];
            for j in i..i+11 {
                // TODO: make array of map lumps to make it easier for the function to work
            }
            i += 11;
            lump_name = lumps[i].name.as_str();
        }

        Wad {
            identification: wad_id,

            maps,
        }
    }
}

impl BspMap {
    fn new(data: &[Vec<u8>]) -> BspMap {

        BspMap { things: vec![] 
         }
    } 
}


fn main() {
    Wad::load("assets/freedoom1.wad");
}

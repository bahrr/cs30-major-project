use core::num;
use std::fs;
use std::str;
use std::collections::HashMap;
use byteorder::ByteOrder;
use byteorder::LittleEndian;

// A WAD is the primary way that Doom and it's source ports store data
struct Wad {
    // Header of the WAD file, used for identifying details
    identification: String, // Identifies the WAD as either an IWAD for the base game or a PWAD for a mod

    lumps: Vec<Lump>, // List of all the lumps
}

// A lump is the basic container for files in a WAD
struct Lump {
    name: String,
    data: Data,
}

// Enum to store the types of data
enum Data {
    MapHeader,
    Things(Vec<Thing>),
}

// Struct which stores Doom maps
struct  BspMap {

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

        let wad_id = str::from_utf8(&file[0..4])
            .expect("Header not valid");

        let numlumps = <LittleEndian as ByteOrder>::read_u32(&file[4..8]);

        // Points to where the directory which keeps track of lumps is
        let info_table = <LittleEndian as ByteOrder>::read_u32(&file[8..12]);

        let mut lumps: Vec<Lump> = Vec::new();

        // Appends the lump vector with lumps obtained from the WAD
        for i in 0..numlumps {
            // Location of the start of the directory entry
            let dir_loc = (info_table + 16 * i).try_into().unwrap();

            // Where in the directory is the lump
            let lump_pos = <LittleEndian as ByteOrder>::read_u32(&file[dir_loc..dir_loc+4]);

            // How big is the lump
            let lump_size = <LittleEndian as ByteOrder>::read_u32(&file[dir_loc+4..dir_loc+8]);
            
            // The name of the lump
            let lump_name = str::from_utf8(&file[dir_loc+8..dir_loc+16])
                .unwrap();
            
            let raw_bytes = file[lump_pos..lump_pos + lump_size];
            lumps.push(Lump::load(lump_name, raw_bytes));
            println!("{lump_name}, {lump_size}, {lump_pos}");
        }

        Wad {
            identification: wad_id.to_string(),
            lumps,
        }
    }
}

impl Lump {
   fn load(name: &str, data: &[u8]) -> Lump {

   }
}

fn main() {
    let iwad = Wad::load("assets/freedoom1.wad");

    // Just quickly reads out the IWAD's header
    println!("{}", iwad.identification);
}

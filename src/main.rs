use std::fs;
use std::str;
use byteorder::ByteOrder;
use byteorder::LittleEndian;

// Reads the bit at the index and returns it as a bool
fn bool_from_u16(int: u16, index: u32) -> bool {
    return int % u16::pow(2, index + 1) != 0;
}

// A WAD is the primary way that Doom and it's source ports store data
struct Wad {
    // Header of the WAD file, used for identifying details
    identification: String, // Identifies the WAD as either an IWAD for the base game or a PWAD for a mod

    maps: Vec<BspMap>,
}
// Struct which stores Doom maps
struct  BspMap {
    things: Vec<Thing>,
    linedefs: Vec<LineDef>,
}

// Holds onto raw lump data
struct Lump {
    name: String,

    // A vector to store the raw data of thy lump
    data: Vec<u8>,
}
// Things are 2d objects like monsters or items
struct Thing {
    x: u16,
    y: u16,
    angle: u16,
    thing_type: u16,

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
    Barrel,
}

// Line as well as flags which activate it
struct LineDef {

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
        let mut maps: Vec<BspMap> = Vec::new(); // Stores the game maps

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
            let mut map_lumps: Vec<Vec<u8>> = Vec::new();
            for j in i..i+11 {
                map_lumps.push(lumps[j].data.clone());
            }
            maps.push(BspMap::new(&map_lumps));
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
    fn new(data: &Vec<Vec<u8>>) -> BspMap {
        let things: Vec<Thing> = Thing::from_bytes(&data[1]);
        let linedefs: Vec<LineDef> = LineDef::from_bytes(&data[2]);
        BspMap { things,
            linedefs
         }
    } 
}

impl Thing {
    fn from_bytes(data: &Vec<u8>) -> Vec<Thing> {
        let mut things: Vec<Thing> = Vec::new();
        // Adds things to the vector
        for i in 0..(data.len() / 10) {
            // The offset of the thing in bytes
            let thing_loc: usize = i * 10;

            // Gets the x and y of the thing
            let x = <LittleEndian as ByteOrder>::read_u16(&data[thing_loc..thing_loc+2]);
            let y = <LittleEndian as ByteOrder>::read_u16(&data[thing_loc+2..thing_loc+4]);

            // Convieniently Doom stores angles as degrees
            let angle = <LittleEndian as ByteOrder>::read_u16(&data[thing_loc+4..thing_loc+6]);

            // Gets the type of the thing
            let thing_type = <LittleEndian as ByteOrder>::read_u16(&data[thing_loc+6..thing_loc+8]);
            
            // Gets the bytes used for the flags
            let int_flags = <LittleEndian as ByteOrder>::read_u16(&data[thing_loc+8..thing_loc+10]);

            // Rust doesn't really support bits so I have to use a function I wrote to convert the bytes to booleans
            let easy = bool_from_u16(int_flags, 0);
            let medium = bool_from_u16(int_flags, 1);
            let hard = bool_from_u16(int_flags, 2);
            let ambush = bool_from_u16(int_flags, 3);
            let multiplayer = bool_from_u16(int_flags, 4);

            // Finally pushes the data into a Thing object
            things.push(Thing {
                x,
                y,
                angle,
                easy,
                hard,
                thing_type,
                medium,
                multiplayer,
                ambush,
            })
        }

        return things;
    }
}

impl LineDef {
    // Gets a vector of LineDefs
    fn from_bytes(data: &Vec<u8>) -> Vec<LineDef> {
        let mut linedefs: Vec<LineDef> = Vec::new();

        for i in 0..(data.len() / 14) {
            // Offset of the linedef in bytes
            let linedef_loc: usize = i * 14;

            // Gets indexes of the vertices
            let start = <LittleEndian as ByteOrder>::read_u16(&data[linedef_loc..linedef_loc+2]);
            let end = <LittleEndian as ByteOrder>::read_u16(&data[linedef_loc+2..linedef_loc+4]);

            // Gets the flags to be converted as an int
            let int_flags = <LittleEndian as ByteOrder>::read_u16(&data[linedef_loc+4..linedef_loc+6]);


            println!("{start}, {end}");
        }

        return  linedefs;
    }
}

fn main() {
    Wad::load("assets/freedoom1.wad");
}

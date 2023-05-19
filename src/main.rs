use std::fs;
use std::str;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use std::collections::HashMap;

// Reads the bit at the index and returns it as a bool
fn bool_from_i16(int: i16, index: u32) -> bool {
    return int % i16::pow(2, index + 1) != 0;
}

// A WAD is the primary way that Doom and it's source ports store data
struct Wad {
    // Header of the WAD file, used for identifying details
    identification: String, // Identifies the WAD as either an IWAD for the base game or a PWAD for a mod

    maps: HashMap<String, BspMap>,
}

// Struct which stores Doom maps
struct  BspMap {
    things: Vec<Thing>,
    linedefs: Vec<LineDef>,
    sidedefs: Vec<SideDef>,
    vertices: Vec<Vertex>,
    segs: Vec<Seg>,
    subsectors: Vec<SubSector>,
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
    thing_type: i16,

    // Keeps track of if the thing exists in a particular difficulty
    // or exists in multiplayer
    easy: bool,
    medium: bool,
    hard: bool,
    multiplayer: bool,

    // Is the monster waiting for an ambush later on
    ambush: bool
}

// Line as well as flags which activate it
struct LineDef {
    // Indexes of the vertices on both ends
    start: i16,
    end: i16,

    // Flags
    block_players_and_monsters: bool,
    block_monsters: bool,

    two_sided: bool, // If the linedef is 2 sided and seperates 2 sectors
    
    // "Pegs" the texture to a point on the linedef
    upper_unpegged: bool,
    lower_unpegged: bool,

    secret: bool,
    block_sound: bool,

    never_automap: bool,
    always_automap: bool,

    special_type: i16, // What type of linedef is it
    sector_tag: i16, // What sector is it a part of
    
    // Indexes of the sidedefs
    front_sidedef: i16,
    back_sidedef: i16,
}

// Contains the indexes to the textures used by a linedef
struct SideDef {
    // Offsets of the texture
    x_offset: i16,
    y_offset: i16,

    // Names of the textures used
    upper_texture: String,
    lower_texture: String,
    middle_texture: String,

    // What sector the sidedef faces
    facing_sector: i16,
}

// Might as well make it easy to access the coordinates as x and y
struct Vertex {
    x: i16,
    y: i16,
}

//  A Seg is a segment of a linedef which is used to build a subsector
struct Seg {
    // Start and end vertices
    start: i16,
    end: i16,

    // Angle in degrees
    angle: f64,

    // What linedef is it a segment of
    linedef_num: i16,
    
    // If true it's pointing in the opposite direction of the linedef
    direction: bool,

    // Distance along linedef to start seg
    offset: i16,
}

// A SubSector is a convex part of a sector
struct SubSector {
    // The indexes to the segs used to build the subsector
    segs: Vec<i16>,
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
        let mut maps: HashMap<String, BspMap> = HashMap::new(); // Stores the game maps

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
        let mut lump_name = lumps[0].name.clone();

        while &lump_name.as_str() != &"PLAYPAL\0" { // PLAYPAL with a null character happens to be the first non map lump in a WAD file
            let mut map_lumps: Vec<Vec<u8>> = Vec::new();
            for j in i..i+11 {
                map_lumps.push(lumps[j].data.clone());
            }
            maps.insert(lump_name, BspMap::new(&map_lumps));
            // maps.push(BspMap::new(&map_lumps));
            i += 11;
            lump_name = lumps[i].name.clone();
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
        let sidedefs: Vec<SideDef> = SideDef::from_bytes(&data[3]);
        let vertices: Vec<Vertex> = Vertex::from_bytes(&data[4]);
        let segs: Vec<Seg> = Seg::from_bytes(&data[5]);
        let subsectors: Vec<SubSector> = SubSector::from_bytes(&data[6]);

        BspMap {
            things,
            linedefs,
            sidedefs,
            vertices,
            segs,
            subsectors,
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
            let x = <LittleEndian as ByteOrder>::read_i16(&data[thing_loc..thing_loc+2]);
            let y = <LittleEndian as ByteOrder>::read_i16(&data[thing_loc+2..thing_loc+4]);

            // Convieniently Doom stores angles as degrees
            let angle = <LittleEndian as ByteOrder>::read_i16(&data[thing_loc+4..thing_loc+6]);

            // Gets the type of the thing
            let thing_type = <LittleEndian as ByteOrder>::read_i16(&data[thing_loc+6..thing_loc+8]);
            
            // Gets the bytes used for the flags
            let int_flags = <LittleEndian as ByteOrder>::read_i16(&data[thing_loc+8..thing_loc+10]);

            // Rust doesn't really support bits so I have to use a function I wrote to convert the bytes to booleans
            let easy = bool_from_i16(int_flags, 0);
            let medium = bool_from_i16(int_flags, 1);
            let hard = bool_from_i16(int_flags, 2);
            let ambush = bool_from_i16(int_flags, 3);
            let multiplayer = bool_from_i16(int_flags, 4);

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
            let start = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc..linedef_loc+2]);
            let end = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc+2..linedef_loc+4]);

            // Gets the flags to be converted as an int and converts it into booleans
            let int_flags = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc+4..linedef_loc+6]);

            let block_players_and_monsters = bool_from_i16(int_flags, 0);
            let block_monsters = bool_from_i16(int_flags, 1);
            let two_sided = bool_from_i16(int_flags, 2);
            let upper_unpegged = bool_from_i16(int_flags, 3);
            let lower_unpegged = bool_from_i16(int_flags, 4);
            let secret = bool_from_i16(int_flags, 5);
            let block_sound = bool_from_i16(int_flags, 6);
            let never_automap = bool_from_i16(int_flags, 7);
            let always_automap = bool_from_i16(int_flags, 8);

            //  What type of linedef is it
            let special_type = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc+6..linedef_loc+8]);

            // Index of the sector
            let sector_tag = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc+8..linedef_loc+10]);

            // Get the indexes of the sidedefs
            let front_sidedef = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc+10..linedef_loc+12]);
            let back_sidedef = <LittleEndian as ByteOrder>::read_i16(&data[linedef_loc+12..linedef_loc+14]);

            linedefs.push(LineDef {
                start,
                end,
                block_players_and_monsters,
                block_monsters,
                two_sided,
                upper_unpegged,
                lower_unpegged,
                secret,
                block_sound,
                never_automap,
                always_automap,
                special_type,
                sector_tag,
                front_sidedef,
                back_sidedef,
            });
        }

        return  linedefs;
    }
}

impl SideDef {
    fn from_bytes(data: &Vec<u8>) -> Vec<SideDef> {
        let mut sidedefs: Vec<SideDef> = Vec::new();

        for i in 0..(data.len() / 30) {
            // Location of the sidedef in the data
            let sidedef_loc: usize = i * 30;

            // Offsets of the texture
            let x_offset = <LittleEndian as ByteOrder>::read_i16(&data[sidedef_loc..sidedef_loc+2]);
            let y_offset = <LittleEndian as ByteOrder>::read_i16(&data[sidedef_loc+2..sidedef_loc+4]);

            // Gets the names of the textures used
            let upper_texture = String::from_utf8(data[sidedef_loc+4..sidedef_loc+12].to_vec())
                .expect("invalid upper texture");
            let lower_texture = String::from_utf8(data[sidedef_loc+12..sidedef_loc+20].to_vec())
                .expect("invalid upper texture");
            let middle_texture = String::from_utf8(data[sidedef_loc+20..sidedef_loc+28].to_vec())
                .expect("invalid upper texture");

            // What sector the sidedef faces
            let facing_sector = <LittleEndian as ByteOrder>::read_i16(&data[sidedef_loc+28..sidedef_loc+30]);

            sidedefs.push(SideDef {
                x_offset,
                y_offset,
                upper_texture,
                lower_texture,
                middle_texture,
                facing_sector,
            });
        }

        return sidedefs;
    }
}

impl Vertex {
   fn from_bytes(data: &Vec<u8>) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();

        for i in 0..(data.len() / 4) {
            let vert_loc: usize = i * 4;

            let x = <LittleEndian as ByteOrder>::read_i16(&data[vert_loc..vert_loc+2]);
            let y = <LittleEndian as ByteOrder>::read_i16(&data[vert_loc+2..vert_loc+4]);

            vertices.push(Vertex {
                x,
                y,
            });
        }
        
        return vertices;
   } 
}

impl Seg {
    fn from_bytes(data: &Vec<u8>) -> Vec<Seg> {
        let mut segs: Vec<Seg> = Vec::new();

        for i in 0..(data.len() / 12) {
            let seg_loc: usize = i * 12;

            let start = <LittleEndian as ByteOrder>::read_i16(&data[seg_loc..seg_loc+2]);
            let end = <LittleEndian as ByteOrder>::read_i16(&data[seg_loc+2..seg_loc+4]);

            // The 16 bit binary angle which goes from -32768 to 32767
            let bin_angle = <LittleEndian as ByteOrder>::read_i16(&data[seg_loc+4..seg_loc+6]);

            // The degree angle I will use
            let angle = bin_angle as f64 * 45.0 / 8192.0;


            let linedef_num = <LittleEndian as ByteOrder>::read_i16(&data[seg_loc+6..seg_loc+8]);

            // Converts direction to a boolean
            let int_direction = <LittleEndian as ByteOrder>::read_i16(&data[seg_loc+8..seg_loc+10]);
            let direction = int_direction == 1;
            
            let offset = <LittleEndian as ByteOrder>::read_i16(&data[seg_loc+10..seg_loc+12]);

            segs.push(Seg {
                start,
                end,
                angle,
                linedef_num,
                direction,
                offset,
            })
        }

        return segs;
    }
}

impl SubSector {
   fn from_bytes(data: &Vec<u8>) -> Vec<SubSector> {
        let mut subsectors: Vec<SubSector> = Vec::new();

        

        return subsectors;
   }
}

fn main() {
    Wad::load("assets/freedoom1.wad");
}

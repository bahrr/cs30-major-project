# Major Project

## Project Description

Renders Doom Maps in Rust

## Needs To Have List

- [x] Should be in Rust
- [x] Read WAD files
  - [x] Get WAD header working
  - [x] Convert directory to something more useful
  - [x] Convert every lump type into a struct
- [started] Render maps using some 3d graphics library
  - [ ] Convert sidedefs, sectors, and things into triangles so that modern graphics APIs can use them
    - [ ] If the graphics library doesn't have triangulation, I'll probably end up using the ear clipping method
  - [x] Use binary space partitioning to cut down on the number of polygons rendered at a time
- [x] Executable for Windows and Linux

## Nice To Have List

- [ ] Basic movement physics (i.e. not hitting walls or falling down properly)
- [ ] Read demo files (won't be in sync due to the lack of enemies)
- [ ] Play music from the game using either a MIDI library or writing one myself
- [ ] Version in WebAssembly, with a Github live demo if possible

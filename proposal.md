# Major Project
## Project Description
Renders Doom Maps in Rust
## Needs To Have List
- [x] Should be in Rust
- [ ] Read WAD files
    - [x] Get WAD header working
    - [ ] Convert directory to array of lumps
- [ ] Render maps using some 3d graphics library
    - [ ] Convert sidedefs, sectors, and things into triangles so that modern graphics APIs can use them
        - [ ] If the graphics library doesn't have triangulation, I'll probably end up using the ear clipping method
    - [ ] Use binary space partitioning to cut down on the number of polygons rendered at a time
- [ ] Executable for Windows
## Nice To Have List
- [ ] Basic movement physics (i.e. not hitting walls or falling down properly)
- [ ] Read demo files (won't be in sync due to the lack of enemies)
- [ ] Play music from the game using either a MIDI library or writing one myself
- [ ] Version in WebAssembly, with a Github live demo if possible
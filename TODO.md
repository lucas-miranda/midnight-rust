# Next Tasks

- [X] Fix warnings
- [X] Draw a grid
- [-] Draw text
- [ ] Refactor rendering code (related to shader pipeline)
- [ ] ...
- [ ] Add error handling to everything related to rendering and ECS

####################

# Engine Architecture

- [ ] Think if rendering and math should share a struct (to avoid repetition)
- [ ] How threading will work?
    - [ ] Split work into threads (which can spawn sub-threads to speed up work): main, rendering, physics, ECS

# Rendering

- [X] Replace gfx-hal to wgpu-hal
- [X] Handle windows resize needs to recreate surface (causing a panic by outdated)
- [X] Add properly errors when something can fail
- [X] Commit everything
- [X] Add more draw/render methods while trying to improve user usability
- [X] Commit everything, don't forget to commit
- [ ] Add a properly fps counter and handler (when trying to lock to max fps values)
- [ ] Add batch rendering
- [ ] Add support to switch api backend at runtime

# Graphics

- [ ] Add Color

# Math

- [-] 2D Geometries
    - [X] Add Point<T> (as shorthand to Vector2<T>)
    - [X] Add Triangle<T>
    - [ ] Add Size<T>
    - [ ] Add Rectangle<T>
    - [ ] Add Circle<T>
    - [ ] Add Line<T>
    - [ ] Add Polygon<T>
    - [ ] Add Range<T>
    - [ ] Add Curve<T>
- [ ] 3D Geometries
    - [X] Add Vector3<T> / Vec3<T>
    - [ ] Add Plane<T>
    - [ ] Add Cube<T>
    - [ ] Add Sphere<T>
- [ ] 4D Geometries
    - [X] Add Vector4<T> / Vec4<T>
    - [ ] Add Quaternion<T>

# ECS

- [X] Figure out how entities will be able to render something (through it's components)
- [ ] Change `entity_id` to `eid`
- [ ] Extend system-component queries
    - [ ] Support more combinations of components (A, B, C, ..)

# Error Handling

- [ ] Improve error handling at module
    - [ ] shaders

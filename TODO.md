# To do

## General
- [ ] Make a build script
    - [ ] Should be easy to run on pi after building

## Hoo Base
- [ ] Add easier ways to build light states and animations
    - [ ] Macro for builder?
- [ ] Make animations asynchronous
- [ ] Enhance Color struct
    - [ ] Check out `palette` crate
    - [ ] Add HSL
    - [ ] Add more options for argument types in constructors (u8 for rgb)
    - [ ] RGB conversions might be broken
    - [ ] Unify colors throughout modules
- [ ] Add ability to save states and animations. Maybe SQLite?

## Hoo Server
- [ ] Make fake api
- [ ] Finish config file support
- [ ] Add command line parameters
- [ ] Run animation based on current settings
- [ ] Look at the warp crate

## Hoo Frontend
- [ ] Animation editor with previews
- [ ] General Controls should represent Hoo state
- [ ] Make everything not ugly

## Hue Api
- [ ] LightCollection shouldn’t act like a tuple. Type alias or test out Deref
- [ ] Try out GraphQL

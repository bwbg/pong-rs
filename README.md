# Pong

Implementation of the ubiquitous Pong arcade game as part of the 100 days of
code challenge [`#100DaysOfCode`][HDOC].

## Development Log

### Day 1

This is the first day of the [`#100DaysOfCode`][HDOC] challenge. Today I setup the project's repository and made an initial commit with some basic additions to the configuration file and code structure.

The first choice I had to make was what library to use to display the game and handle input. I decided against the [famous bevy](BEVY) and favoured [SDL2](SDL2) because I already met SDL with PyGame.

[HDOC]: https://100daysofcode.com "#100DaysOfCode"
[REPO]: https://github.com/bwbg/pong-rs.git "Project's Repository"
[BEVY]: https://bevyengine.org/ "Bevy Game Engine"
[SDL2]: https://libsdl.org/ "Simple DirectMedia Layer"

#### Addendum

I was itching to present at least a blank window to myself so I typed the example from the SDL2 documentation. I added the `config` module to avoid magic constants in my code. Addidtional 20 minutes worth of coding today.

Notes:

- On Windows add the SDL2 development libraries (`*.lib`) to `%USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib` as well as the `SDL2.dll` to your project's root directory.
- Use `cargo doc --open` to view your own crate's documentation as well as the documentation from any dependencies.

# Pong Rust implementation

This repository contains a simple Pong game implementation in Rust.

![alt text](/images/intro-picture2.png)

# How to run

Download/pull the git repository and execute `cargo run`. Make sure you have rust installed, see https://www.rust-lang.org/tools/install.

## Known problems
- On WSL the ball speed parameter needs to be lowered significantly compared to Windows, probably has something to do with how WSL runs windowed applications.
- Have not tested it (yet) on bare metal linux.

## Future features
- Menu with possibility to change ball speed, player speed and windows size.
- Self serve
- Stopping/resetting game when a player reaches a certain score

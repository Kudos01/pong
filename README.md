# Pong Rust implementation

This repository contains a simple Pong game implementation in Rust.

![alt text](/images/intro-picture2.png)

# How to run

Download/pull the git repository and execute `cargo run`. Make sure you have rust installed, see https://www.rust-lang.org/tools/install.

## Known problems
- `BALL_SPEED` and `PLAYER_SPEED` parameters need to be configured differently in linux versus windows, at least in my testing. On Windows parameters need to be significantly higher to achieve the same movement speed as linux.

## Future features
- Menu with possibility to change ball speed, player speed and windows size.
- Self serve
- Stopping/resetting game when a player reaches a certain score

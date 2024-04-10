use macroquad::prelude::*;
type Point = (f32, f32);

const RECTANGLE_WIDTH: f32 = 20.;

const RECTANGLE_HEIGHT: f32 = 100.;

const OFFSET: f32 = 20.;

const CUBE_SIDE: f32 = 10.;

#[derive(Debug, Clone, Copy)]
struct Player {
    pos: Point,
    score: i16,
}

struct Ball {
    pos: Point
}

fn draw_player(p: Player){
    draw_rectangle(p.pos.0, p.pos.1, RECTANGLE_WIDTH, RECTANGLE_HEIGHT, BLACK);
}

#[macroquad::main("Pong")]
async fn main() {

    // Center the players at first

    let mut p1: Player = Player { pos: (OFFSET, screen_height()/2. - RECTANGLE_HEIGHT + OFFSET), score: (0) };
    let mut p2: Player = Player { pos: (screen_width() - OFFSET*2., screen_height()/2. - RECTANGLE_HEIGHT + OFFSET), score: (0)};

    let mut ball: Ball = Ball { pos: (screen_width()/2., screen_height()/2.) };

    loop {
        clear_background(GRAY);
        
        draw_player(p1);
        draw_player(p2);

        draw_rectangle(ball.pos.0, ball.pos.1, CUBE_SIDE, CUBE_SIDE, GOLD);
        
        // TODO Player movement

        // TODO Ball movement

        // TODO Collisions

        next_frame().await;
    }
}

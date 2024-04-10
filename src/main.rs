use macroquad::prelude::*;

const RECTANGLE_WIDTH: f32 = 20.;

const RECTANGLE_HEIGHT: f32 = 100.;

const OFFSET: f32 = 20.;

const CUBE_SIDE: f32 = 10.;

const BALL_SPEED: f32 = 5.;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug, Clone, Copy)]
struct Player {
    pos: Point,
    score: i16,
}

struct Ball {
    pos: Point,
    dir: Point
}

fn draw_player(p: Player) {
    draw_rectangle(p.pos.x, p.pos.y, RECTANGLE_WIDTH, RECTANGLE_HEIGHT, BLACK);
}

fn check_move_player(p: &mut Player, keycode_up: KeyCode, keycode_down: KeyCode) {
    if is_key_down(keycode_up) && p.pos.y > 0.{
        p.pos.y -= 3.;
    } else if is_key_down(keycode_down) && p.pos.y < screen_height() - RECTANGLE_HEIGHT {
        p.pos.y += 3.;
    }
}

// Point should be unit vector
fn move_ball(b: &mut Ball) {

    b.pos.x += b.dir.x * BALL_SPEED;
    b.pos.y += b.dir.y * BALL_SPEED;

    // Wall collision
    if b.pos.y > screen_height()-CUBE_SIDE || b.pos.y < 0.0{
        b.dir.y = -b.dir.y;
    }

    if b.pos.x > screen_width()-CUBE_SIDE || b.pos.x < 0.0 {
        b.dir.x = -b.dir.x;
    }
}

#[macroquad::main("Pong")]
async fn main() {

    // Center the players at first

    let mut p1: Player = Player { pos: Point {x: OFFSET, y: screen_height()/2. - RECTANGLE_HEIGHT + OFFSET}, score: (0) };
    let mut p2: Player = Player { pos: Point {x: screen_width() - OFFSET*2., y: screen_height()/2. - RECTANGLE_HEIGHT + OFFSET}, score: (0)};

    let mut ball: Ball = Ball { pos: Point { x: screen_width()/2., y: screen_height()/2. }, dir: Point {x: 1., y: 0.}};

    loop {
        clear_background(GRAY);
        
        draw_player(p1);
        draw_player(p2);

        draw_rectangle(ball.pos.x, ball.pos.y, CUBE_SIDE, CUBE_SIDE, GOLD);
        
        // Player movement

        // p1 movement
        check_move_player(&mut p1, KeyCode::W, KeyCode::S);

        // p2 movement
        check_move_player(&mut p2, KeyCode::Up, KeyCode::Down);

        // TODO Ball movement
        move_ball(&mut ball);
        // TODO Collisions

        next_frame().await;
    }
}

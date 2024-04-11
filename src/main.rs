use macroquad::prelude::*;
use ::rand::prelude::*;

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

fn conf() -> Conf {
    Conf {
    window_title:"Pong".to_owned(),
    window_resizable: false,
    ..Default::default()
  }
}


// Perhaps it would be better for the players and ball to store their own rectangle but here we are
fn ball_collision_with_player(player : &Player, ball : &mut Ball) {

    let ball_rect = Rect::new(ball.pos.x, ball.pos.y, CUBE_SIDE, CUBE_SIDE);

    let player_rect = Rect::new(player.pos.x, player.pos.y, RECTANGLE_WIDTH, RECTANGLE_HEIGHT);

    if ball_rect.intersect(player_rect).is_some() {
        ball.dir.x = -ball.dir.x;
    }
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

// dir of ball should be unit vector
fn move_ball(b: &mut Ball) {

    b.pos.x += b.dir.x * BALL_SPEED;
    b.pos.y += b.dir.y * BALL_SPEED;

    // Wall collision
    if b.pos.y > screen_height()-CUBE_SIDE || b.pos.y < 0.0{
        b.dir.y = -b.dir.y;
    }
}

fn check_scored_points(b: &mut Ball, p1 : &mut Player, p2 : &mut Player) {
        // If ball surpasses x axis on either side, players score points
        if b.pos.x > screen_width()-CUBE_SIDE {
            // b.dir.x = -b.dir.x;
            p1.score += 1;
            // reset ball pos
            b.pos.x = screen_width()/2.;
            b.pos.y = screen_height()/2.;

            let point = get_new_ball_dir();

            b.dir.x = point.x;
            b.dir.y = point.y;

        } else if b.pos.x < 0.0 {
            p2.score += 1;
            b.pos.x = screen_width()/2.;
            b.pos.y = screen_height()/2.;

            let point = get_new_ball_dir();

            b.dir.x = point.x;
            b.dir.y = point.y;
        }
}

fn draw_scores(p1 : &Player, p2 : &Player) {
    let text_params =  TextParams {
        font_size:70,
        ..Default::default()
    };

    draw_text_ex(&format!("{}",p1.score).as_str(),100., 100.,text_params);
    draw_text_ex(&format!("{}",p2.score).as_str(),screen_width()-100., 100.,text_params);
}


// function to spawn ball in middle with random direction

fn get_new_ball_dir() -> Point {
    let mut rng = thread_rng();

    let dir_x = rng.gen_range(0. ..1.);
    let dir_y = rng.gen_range(0. ..1.);

    let modulus = ((dir_x*dir_x + dir_y*dir_y) as f64).sqrt();

    let unit_x: f32 = (dir_x / modulus) as f32;
    let unit_y: f32 = (dir_y / modulus) as f32;

    Point { x: unit_x, y: unit_y }
}

#[macroquad::main(conf)]
async fn main() {

    // Center the players at first

    let mut p1: Player = Player { pos: Point {x: OFFSET, y: screen_height()/2. - RECTANGLE_HEIGHT + OFFSET}, score: (0) };
    let mut p2: Player = Player { pos: Point {x: screen_width() - OFFSET*2., y: screen_height()/2. - RECTANGLE_HEIGHT + OFFSET}, score: (0)};

    let point = get_new_ball_dir();

    let mut ball: Ball = Ball { pos: Point { x: screen_width()/2., y: screen_height()/2. }, dir: Point {x: point.x, y: point.y}};

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

        // Ball movement
        move_ball(&mut ball);

        check_scored_points(&mut ball, &mut p1, &mut p2);

        draw_scores(&p1, &p2);

        // TODO Collisions
        ball_collision_with_player(&p1, &mut ball);
        ball_collision_with_player(&p2, &mut ball);

        next_frame().await;
    }
}

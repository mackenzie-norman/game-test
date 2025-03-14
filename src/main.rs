use std::iter::Enumerate;

use console_engine::crossterm::style;
use console_engine::pixel;
use console_engine::pixel::Pixel;
use console_engine::rect_style::BorderStyle;
use console_engine::screen;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use rand::Rng;

/// custom function for generating a random u32 bound into [0;max[
fn random(max: u32) -> u32 {
    rand::random::<u32>() % max
}

/// Direction the snake can face
enum Direction {
    North,
    East,
    South,
    West,
}

/// Snake structure :  
/// The game logic fits in it
struct Snake {
    playing: bool,
    bound_w: u32,
    bound_h: u32,
    direction: Direction,
    old_dx: i8,
    old_dy: i8,
    pos_x: u32,
    pos_y: u32,
    apple_x: u32,
    apple_y: u32,
    body: Vec<(u32, u32)>,
}

impl Snake {
    /// Game initialization
    pub fn init(game_width: u32, game_height: u32) -> Snake {
        Snake {
            playing: false,
            bound_w: game_width,
            bound_h: game_height,
            direction: Direction::East,
            old_dx: 1, // start condition should be 1 due to starting direction being East
            old_dy: 0,
            pos_x: 4,
            pos_y: 4,
            apple_x: 0,
            apple_y: 0,
            body: vec![(3, 4), (2, 4)],
        }
    }

    /// Generates an apple in the board
    fn gen_apple(&mut self) {
        let mut count_fallback = 0;
        loop {
            // randomly get coordinates
            let x = random(self.bound_w);
            let y = random(self.bound_h);

            // check if the coordinates aren't colliding with the snake's body
            // sets the position if no collision
            if !self.body.contains(&(x, y)) {
                self.apple_x = x;
                self.apple_y = y;
                return;
            }
            count_fallback += 1;
            // if 50 tries did not succeed
            if count_fallback > 50 {
                // bruteforce the first available position
                for y in 0..self.bound_h {
                    for x in 0..self.bound_w {
                        if !self.body.contains(&(x, y)) {
                            self.apple_x = x;
                            self.apple_y = y;
                            return;
                        }
                    }
                }
                // if bruteforce failed, game has been won
                self.playing = false;
                return;
            }
        }
    }

    pub fn input(&mut self, engine: &ConsoleEngine) {
        if self.playing {
            // Change snake's direction based on a keypad layout
            if engine.is_key_pressed(KeyCode::Char('8')) || engine.is_key_pressed(KeyCode::Up) {
                self.direction = Direction::North;
            }
            if engine.is_key_pressed(KeyCode::Char('6')) || engine.is_key_pressed(KeyCode::Right) {
                self.direction = Direction::East;
            }
            if engine.is_key_pressed(KeyCode::Char('2')) || engine.is_key_pressed(KeyCode::Down) {
                self.direction = Direction::South;
            }
            if engine.is_key_pressed(KeyCode::Char('4')) || engine.is_key_pressed(KeyCode::Left) {
                self.direction = Direction::West;
            }
        } else {
            // check when the player starts the game with space
            if engine.is_key_pressed(KeyCode::Char(' ')) {
                // Initialize game values to a starting state
                self.playing = true;
                self.direction = Direction::East;
                self.old_dx = 1;
                self.old_dy = 0;
                self.pos_x = 4;
                self.pos_y = 4;
                self.body = vec![(3, 4), (2, 4)];
                self.gen_apple();
            }
        }
    }

    pub fn update_position(&mut self) {
        if self.playing {
            // calculates the delta_x and delta_y
            // based on facing direction
            let mut dx = 0;
            let mut dy = 0;
            match self.direction {
                Direction::North => dy = -1,
                Direction::East => dx = 1,
                Direction::South => dy = 1,
                Direction::West => dx = -1,
            }

            // checks to see if old inputed direction overlaps with actual inputed direction
            // such as East then West.. This would cause the game to think that the snake collided
            // with itself causing a gameover >>
            // if dx's or dy's are opposites then continue moving in old direction
            if self.old_dx + dx == 0 || self.old_dy + dy == 0 {
                dx = self.old_dx;
                dy = self.old_dy;
            } else {
                self.old_dx = dx;
                self.old_dy = dy;
            }

            // if the snake collides with top and left boundaries, game over
            // this check need to be made first to bypass an underflowing
            if self.pos_x == 0 && dx == -1 || self.pos_y == 0 && dy == -1 {
                self.playing = false;
                return;
            }

            // calculate new position, can't underflow because of the check above
            let new_pos = (
                (self.pos_x as i32 + dx as i32) as u32,
                (self.pos_y as i32 + dy as i32) as u32,
            );

            // if collide with bottom and right boundaries, game over
            if new_pos.0 >= self.bound_w || new_pos.1 >= self.bound_h {
                self.playing = false;
                return;
            }

            // if collide with own tail, game over
            if self.body.contains(&new_pos) {
                self.playing = false;
                return;
            }

            // if collide with apple, add a new segment in snake's body
            // and generate a new apple
            if new_pos == (self.apple_x, self.apple_y) {
                self.body.insert(0, (self.pos_x, self.pos_y));
                self.gen_apple();
            }

            // if still alive, move the body
            if self.playing {
                self.body.insert(0, (self.pos_x, self.pos_y));
                self.pos_x = new_pos.0;
                self.pos_y = new_pos.1;
                self.body.pop();
            }
        }
    }

    pub fn draw(&self, engine: &mut ConsoleEngine) {
        if self.playing {
            // draw apple
            engine.set_pxl(
                self.apple_x as i32,
                self.apple_y as i32,
                pixel::pxl_fg('O', Color::Red),
            );
            // draw snake's body
            for segment in self.body.iter() {
                engine.set_pxl(
                    segment.0 as i32,
                    segment.1 as i32,
                    pixel::pxl_fg('█', Color::Green),
                );
            }
            // don't forget snake's head !
            engine.set_pxl(
                self.pos_x as i32,
                self.pos_y as i32,
                pixel::pxl_fg('█', Color::DarkGreen),
            )
        } else {
            // blink a message, inviting the player to press space
            // and display controls on the other side
            if engine.frame_count % 8 >= 4 {
                engine.print_fbg(2, 1, "Press", Color::Yellow, Color::Black);
                engine.print_fbg(2, 2, "Space", Color::Yellow, Color::Black);
                engine.print_fbg(3, 3, "To", Color::Yellow, Color::Black);
                engine.print_fbg(2, 4, "Play", Color::Yellow, Color::Black);
            } else {
                engine.print(4, 1, "8");
                engine.print(4, 2, "^");
                engine.print(1, 3, "4 < > 6");
                engine.print(4, 4, "v");
                engine.print(4, 5, "2");
            }
            // score is always displayed
            engine.print(1, 8, format!("Score:{}", self.body.len() - 2).as_str());
        }
    }
}
/* 
fn main() {
    // initializes a screen filling the terminal of at least 10x10 of size with a target of 4 frame per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(10, 10, 16).unwrap();

    // initialize game here, providing term size as boundaries
    let mut snake = Snake::init(engine.get_width(), engine.get_height());

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
                             // engine.check_resize(); here we do not want to resize the terminal because it could break the boundaries of the game

        // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        engine.clear_screen(); // reset the screen

        // run the game
        snake.input(&engine);
        snake.update_position();
        // draw the game in engine's screen
        snake.draw(&mut engine);

        engine.draw(); // draw the screen
    }
}
use console_engine::rect_style::BorderStyle;
use console_engine::screen;

fn main() {
    let mut scr = screen::Screen::new(9, 10);

    scr.rect_border(0, 0, 3, 2, BorderStyle::new_simple());

    // print the screen to the terminal
    scr.draw();
}

//use console_engine::pixel;
//use console_engine::KeyCode;
use console_engine::MouseButton;

fn main() {
    // initializes a screen filling the terminal with a target of 30 frames per second
    let mut engine = console_engine::ConsoleEngine::init_fill(30).unwrap();

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }

        // prints a 'P' where the mouse's left button has been pressed
        let mouse_pos = engine.get_mouse_press(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('P'));
        }

        // prints a 'H' where the mouse is currently held
        let mouse_pos = engine.get_mouse_held(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('H'));
        }

        // prints a 'R' where the mouse has been released
        let mouse_pos = engine.get_mouse_released(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            engine.set_pxl(mouse_pos.0 as i32, mouse_pos.1 as i32, pixel::pxl('R'));
        }

        engine.draw(); // draw the screen
    }
}
*/
fn bogey(engine: &mut ConsoleEngine, frame:i32, start_val: i32, bottom:i32){
    engine.fill_circle(start_val + frame  , bottom, 4, pixel::pxl_fg('@', Color::DarkGrey));
    engine.fill_rect(start_val + frame, bottom, start_val + frame - 17, bottom + 2,pixel::pxl_fg('@', Color::DarkGrey) );
    engine.fill_circle(start_val + frame -17 , bottom, 4, pixel::pxl_fg('@', Color::DarkGrey) );

}
//use console_engine::{pixel, KeyCode};
fn station_enter_anim(engine: &mut ConsoleEngine, frame:i32 ){
    //Locomotive Body
    let end_val = -80;
    let start_val = 60;
    let height =  20;
    let bottom = height + 20;
    //WHEELS
    bogey(engine, frame, start_val - 17, bottom);
    bogey(engine, frame, end_val + 17, bottom);
    //Train
    engine.line(end_val + frame , height, bottom + frame, height, pixel::pxl_fg('#', Color::DarkBlue));
    engine.line(end_val + frame , height + 1, bottom + frame + 1, height +1, pixel::pxl_fg('#', Color::DarkBlue));
    engine.line(end_val + frame , height + 2, bottom + frame + 2, height +2, pixel::pxl_fg('#', Color::DarkBlue));

    engine.line(bottom + frame , height, start_val + frame, height + 10, pixel::pxl_fg('#', Color::DarkRed));
    engine.line(bottom + frame , height +1, start_val + frame - 1, height + 9, pixel::pxl_fg('#', Color::DarkRed));

    engine.line(start_val + frame , height + 10, start_val + frame, bottom, pixel::pxl('#'));
    //bottom
    for fill_line in 2..11{
        engine.line(end_val + frame , bottom-fill_line, start_val  -36 + frame, bottom-fill_line, pixel::pxl_fg('#', Color::Blue));
        engine.line(start_val+ frame , bottom-fill_line, start_val  -36 + frame, bottom-fill_line, pixel::pxl_fg('#', Color::Blue));

    }

    //engine.line(end_val + frame , bottom-4, start_val  -36 + frame, bottom-4, pixel::pxl_fg('#', Color::Blue));
    //engine.line(start_val+ frame , bottom-4, start_val  -36 + frame, bottom-4, pixel::pxl_fg('#', Color::Blue));

    //engine.line(end_val + frame , bottom-3, start_val  -36 + frame, bottom-3, pixel::pxl_fg('#', Color::Blue));
    //engine.line(start_val+ frame , bottom-3, start_val  -36 + frame, bottom-3, pixel::pxl_fg('#', Color::Blue));

    //engine.line(end_val + frame , bottom -1 -1, start_val  -36 + frame, bottom -1 -1, pixel::pxl_fg('#', Color::Grey));
    //engine.line(start_val+ frame , bottom -1 -1, start_val  -36 + frame, bottom -1 -1, pixel::pxl_fg('#', Color::Grey));

    engine.line(end_val + frame , bottom -1, start_val + frame, bottom -1, pixel::pxl_fg('#', Color::Grey));
    engine.line(end_val + frame , bottom, start_val + frame, bottom, pixel::pxl_fg('#', Color::Grey));
    
    //Back
    engine.line(end_val + frame , height, end_val + frame, bottom, pixel::pxl('#'));
    // Door
    engine.rect_border(start_val - 36 + frame, height+ 6, start_val - 26 + frame, bottom  -2, BorderStyle::new_simple());
    engine.set_pxl(start_val - 27 + frame,bottom - 7,pixel::pxl('*'));
    //TRAIN NUMBERS
    //engine.line(start_val - 38 + frame , height + 6, start_val -28 + frame, height + 6, pixel::pxl('#'));
    //engine.line(start_val - 38 + frame , height + 6, start_val -38 + frame, bottom, pixel::pxl('#'));
    //engine.line(start_val - 38 + frame , height + 6, start_val -38 + frame, bottom, pixel::pxl('#'));
    
}
fn tree(engine: &mut ConsoleEngine, frame:i32 , x1: i32,y1:i32,scale: i32, ground: i32){
    let height = 4 * scale;
    let width = 2* scale;
    engine.fill_rect(x1 + (width/2) , y1, x1 +width + (width/2) , ground , pixel::pxl_fg('#', Color::AnsiValue(58)));
    if scale %3 == 1 || scale > 6{

        engine.fill_triangle(x1, y1, x1 + width, y1 - height, x1 + width + width, y1, pixel::pxl_fg('*', Color::AnsiValue(29)));
    }else if  scale % 3 ==2{
        engine.fill_triangle(x1, y1, x1 + width, y1 - height, x1 + width + width, y1, pixel::pxl_fg('*', Color::AnsiValue(28)));
        
    }else{

        engine.fill_triangle(x1, y1, x1 + width, y1 - height, x1 + width + width, y1, pixel::pxl_fg('*', Color::Green));
    }
}
fn rock(engine: &mut ConsoleEngine, frame:i32 , x1: i32,y1:i32,scale: i32, ground: i32){
    let height = 4 * scale;
    let width = 2* scale;
    engine.fill_rect(x1 + (width/2) , y1, x1 +width + (width/2) , ground , pixel::pxl_fg('#', Color::Grey));
}
fn moving_background_anim(engine: &mut ConsoleEngine, frame:i32, tree_count: i32 , mut space: i32, rand_arr: &Vec<i32>){
    let gnd = 60;
    let heaven_line = gnd -12;
    let draw_sky = false;
    //TODO ADDD ASSERT
    for i in  heaven_line..gnd{
        engine.line(0 , i, engine.get_width() as i32, i, pixel::pxl_fg('#', Color::AnsiValue(101)));

    }
    if draw_sky{
        for i in  0..heaven_line{
            engine.line(0 , i, engine.get_width() as i32, i, pixel::pxl_fg(' ', Color::AnsiValue(39)));

        }

    }
    // draw our trees
    for tple in ((0-tree_count)..0).enumerate(){
        let i: usize= tple.0;
        let x1 = tple.1;
        let scale =  rand_arr[i as usize];
        space -=  3*  rand_arr[i as usize];
        //tree(engine, frame, space + frame, gnd - 14 * scale/rand_arr[i as usize], scale, gnd - rand_arr[i as usize]);
        if rand_arr[i as usize] % 3 == 0{
            //rock(engine, frame,space, gnd , 1, gnd + rand_arr[i as usize] %2, );
            power_line(engine, frame, space , space + 12);
        }
    }



}
fn curve_gen(x1: i32, x2: i32, y1: i32, droop: i32) -> Vec<(i32, i32)> {
    let mut curve_vec: Vec<(i32, i32)> = Vec::new();

    // Midpoint is where the curve reaches its lowest point
    let mid = (x1 + x2) / 2;
    let y2 = y1 - droop;

    // Calculate 'a' for the quadratic formula based on the desired droop
    let a = (y1 - y2) as f64 / ((x1 - mid).pow(2) as f64);

    for x in x1..=x2 {
        // Quadratic equation: y = a * (x - mid)^2 + y2
        let y = (a * (x - mid).pow(2) as f64 + y2 as f64).round() as i32;
        curve_vec.push((x, y));
    }

    curve_vec
}

fn main() {
    let curve = curve_gen(0, 10, 20, 5);
    for (x, y) in curve {
        println!("({}, {})", x, y);
    }
}
fn power_line(engine: &mut ConsoleEngine, frame:i32, x1: i32, x2:i32){
    let gnd = 60;
    let heaven_line = gnd -12;
    let width = 4;
    let height = 8;
    engine.fill_rect(x1 + frame   , heaven_line + 1, x1 +width + frame , heaven_line - height , pixel::pxl_fg('#', Color::AnsiValue(58)));
    
    engine.fill_rect(x2 + frame   , heaven_line + 1, x2 +width + frame , heaven_line - height , pixel::pxl_fg('#', Color::AnsiValue(58)));
    for tple in curve_gen(x1, x2, (heaven_line- height), 2){
        engine.set_pxl(tple.0 + frame,tple.1,pixel::pxl('*'));
    }
    


}

fn train_window_static(engine: &mut ConsoleEngine, ){
    //fill bottom
    let windows = 3;
    let screen_width =(engine.get_width()) as i32;
    let screen_height =(engine.get_height()) as i32;
    //let mut spacing = screen_width /10;
    let mut spacing = 4;
    //spacing 
    let  mut window_start_x  =((screen_width ) /12)  as i32;
    let mut  window_start_y =((screen_height )/8 ) as i32;
    if windows > 6{
        window_start_x  =((screen_width ) /12)  as i32;
        window_start_y =((screen_height ) / 3 )  as i32;

    }
    let window_width =  (screen_width - (spacing * (windows-1)) - (window_start_x * 2)) / windows;
    let window_height =  screen_height - (window_start_y * 2);
    engine.fill_rect(0, 0, screen_width, window_start_y -1 , pixel::pxl_fg('X', Color::DarkGrey));
    engine.fill_rect(0, screen_height, screen_width, screen_height -window_start_y +1 , pixel::pxl_fg('X', Color::DarkGrey));
    engine.fill_rect(0, 0, window_start_x, screen_height , pixel::pxl_fg('X', Color::DarkGrey));
    for i in 1..=windows{
        let window_end_x = window_start_x + ( window_width);
        engine.rect_border(window_start_x, window_start_y, window_end_x , screen_height - window_start_y, BorderStyle::new_double());
        engine.fill_rect(window_end_x + 1 , 0, window_end_x + spacing - 1, screen_height , pixel::pxl_fg('X', Color::DarkGrey));
        window_start_x = window_end_x + spacing;
        //engine.fill_rect(screen_width - window_start_x - 1, 0, screen_width, screen_height , pixel::pxl_fg('X', Color::DarkGrey));

    }
    engine.fill_rect(window_start_x - 1, 0, screen_width, screen_height , pixel::pxl_fg('X', Color::DarkGrey));

}
fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(20).unwrap();
    let mut frame = 0;
    
    let mut rng = rand::rng();
    let tree_count = 200;//rng.random_range(0..12);
    let mut space = 8;
    let rand_arr: Vec<i32> = (0..tree_count).map(|x| rng.random_range(1..=6)).collect();
    loop {
        engine.wait_frame();
        engine.clear_screen();

        // draw a rectangle with an emoji inside
        //engine.rect(0, 0, 5, 4, pixel::pxl('#'));
        moving_background_anim(&mut engine, frame, tree_count, space, &rand_arr);
        train_window_static(&mut engine);
        //station_enter_anim(&mut engine, frame);
        //engine.set_pxl(2, 2, pixel::pxl('👍'));

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
        frame += 1;
    }
}
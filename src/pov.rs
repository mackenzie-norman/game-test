use std::f32::consts::FRAC_1_PI;

use console_engine::pixel::{self};
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use crate::dialouge;

#[macro_export]
macro_rules! debug_engine {
    ($engine:expr, $fmt:literal, $($arg:tt)*) => {
        let debug_str = format!($fmt, $($arg)*);
        $engine.fill_rect(1,1,debug_str.len() as i32 + 2 , 4, pixel::pxl_bg(' ', Color::Blue));
        $engine.rect_border(1,1,debug_str.len() as i32 + 2 , 4, BorderStyle::new_simple());
        $engine.print_fbg(2,2,&debug_str, Color::Black, Color::Blue);


    };
}
fn fill_parrallegram(engine: &mut ConsoleEngine, frame:i32){

}
fn calculate_x(x1: i32, y1: i32, x2: i32, y2: i32, y_need_x: i32) -> i32 {
    if x1 == x2 {
        panic!("Slope is undefined (vertical line).");
    }

    let slope = (y2 - y1) as f64 / (x2 - x1) as f64;
    let x = (y_need_x as f64 /slope) as f64;
    
    x.round() as i32 + x1// Round to nearest integer if needed
}
fn calculate_y(x1: i32, y1: i32, x2: i32, y2: i32, x_need_y: i32) -> i32 {
    if x1 == x2 {
        panic!("Slope is undefined (vertical line).");
    }

    let slope = (y2 - y1) as f64 / (x2 - x1) as f64;
    let y = y1 as f64 + slope * (x_need_y - x1) as f64;
    
    y.round() as i32 // Round to nearest integer if needed
}
fn forward_chair(engine: &mut ConsoleEngine, frame:i32, x:i32,y:i32, scale:i32){
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let vanishing_x: i32 = screen_width/2;
    //let scale = 1;
    let chair_height = screen_height/12 * scale;
    let chair_width = screen_width/16 * scale;
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue(52));
    let seat_border_char = pixel::pxl_fg('%', Color::AnsiValue(255));
    engine.fill_circle(x+ chair_width/2, y - chair_height , (chair_width/2).try_into().unwrap(), seat_char);
    engine.circle(x+ chair_width/2, y- chair_height , (chair_width/2).try_into().unwrap(), seat_border_char);
    engine.fill_rect(x, y , x + chair_width, y- chair_height, seat_char);

    let end_y = calculate_y(x, y, vanishing_x, 0, x - chair_width/2 );
    engine.line(x,y,x  -chair_width/2, end_y, seat_char);
    let end_y = calculate_y(x + chair_width, y, vanishing_x, 0, x + chair_width/2 );
    engine.line(x + chair_width,y,x  +chair_width/2, end_y, seat_char);
    //engine.line(x,y + chair_height,x - chair_width/2, end_y + chair_height, seat_char);
    engine.rect(x, y , x + chair_width, y- chair_height, seat_border_char);
}
fn forward_view(engine: &mut ConsoleEngine, frame:i32){
    let window_char = pixel::pxl_fbg('=', Color::AnsiValue(51) , Color::AnsiValue(57));
    let window_char = pixel::pxl_fg('=', Color::AnsiValue(57));
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue(58));
    let floor_char = pixel::pxl_fg('#', Color::AnsiValue(236));
    let walkway_char = pixel::pxl_fg('#', Color::AnsiValue(238));

    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let vanishing_x: i32 = screen_width/2;
    let offset = 6;
    let angle = screen_width/3;
    let floor_height = screen_height - screen_height/6;
    engine.fill_rect(0,0, screen_width, screen_height , floor_char);
    engine.fill_rect(vanishing_x - offset, screen_height,vanishing_x + offset , 0, walkway_char);
    for offset in  1..=6{
        engine.line(angle, screen_height , vanishing_x - offset , 0, walkway_char);
        engine.line(screen_width - angle, screen_height, vanishing_x +offset, 0, walkway_char);

    }
    for angle in angle..=screen_width-angle{
    engine.line(angle, screen_height , vanishing_x  , 0, walkway_char);

    }
    engine.fill_rect(0, 0, screen_width, floor_height, seat_char);
    engine.fill_rect(vanishing_x - 2*offset , 0 , vanishing_x + offset *2, floor_height, pixel::pxl_fg('#', Color::Black));
    //forward_chair(engine, frame, (vanishing_x - 6* offset), 0 + 24,1);
    //color wall
    for i in 0..screen_height/2 + screen_height/4{
        let start_x;
        if i> screen_height/4{
            start_x = calculate_x(angle , screen_height, vanishing_x - offset , 0, floor_height-i);
        }else{
            start_x = calculate_x(angle , screen_height, vanishing_x - offset , 0, floor_height );
        }
        engine.line(0, screen_height/2 + screen_height/4 - i,  start_x, floor_height-i, seat_char);
        engine.line(calculate_x(angle , screen_height, vanishing_x - offset , 0, floor_height ), floor_height , calculate_x(angle , screen_height, vanishing_x - offset , 0, floor_height ), 0, pixel::pxl_fg('#', Color::Black) );
        //engine.fill_rect( vanishing_x - 2*offset ,0, calculate_x(angle , screen_height, vanishing_x - offset , 0, screen_height/4 ), floor_height, seat_char);
    }
    for i in (floor_height..screen_height).step_by(12){
        let start_x = calculate_x(angle , screen_height, vanishing_x , 0, i);
        forward_chair(engine, frame, start_x,  i,2);

    }
}

pub fn station_pov_simple(engine: &mut ConsoleEngine, frame:i32){
    
    let walkway_char = pixel::pxl_fg('#', Color::AnsiValue(238));
    let post_char = pixel::pxl_fg('#', Color::AnsiValue(238));
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let vanishing_x: i32 = screen_width/2;
    let station_y: i32 = screen_height/ 4;
     
    let door_open_time = 120;
    while frame < door_open_time{
        engine.line(0, screen_height, screen_width, station_y, walkway_char);
    }
    // from the bottom third? 
    //draw bricks 



}

pub fn open_eyes(engine: &mut ConsoleEngine, frame:i32) -> bool{
    let eye_speed = 60; // really this is how many frames this takes
    let frame = frame /2;
    let closed_char = pixel::pxl_bg(' ', Color::AnsiValue(232));
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let start = screen_height/2;
    let curve = |x: i32| -> i32 {
    (x * x)// Example: downward-opening parabola returning an integer
    };
    engine.fill_rect(0, 0, screen_width, start - curve(frame),   closed_char);
    engine.fill_rect(0, screen_height, screen_width, start + curve(frame),   closed_char);
    return (curve(frame)- start) <= 0
}
pub fn close_eyes(engine: &mut ConsoleEngine, frame:i32) -> bool{
    let eye_speed = 60; // really this is how many frames this takes
    let frame = frame /2;
    let closed_char = pixel::pxl_bg(' ', Color::AnsiValue(232));
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    //let start = screen_height/2;
    let curve = |x: i32| -> i32 {
    -(x +x)  // Example: downward-opening parabola returning an integer
    };
    engine.fill_rect(0, 0, screen_width, 0- curve(frame),   closed_char);
    engine.fill_rect(0, screen_height, screen_width, screen_height + curve(frame),   closed_char);
    return curve(frame) >= screen_height/2
}
pub fn waking_up(engine: &mut ConsoleEngine, mut frame:i32) -> bool
{
let mut cur_fn: fn(&mut ConsoleEngine, i32) -> bool = close_eyes;
let mut swap = 10;
match frame {
    x if x < 21 => {
    cur_fn = open_eyes;

    } ,
    x if 20 < x && x <= 31 => {
    frame -=20;
    cur_fn = close_eyes;
    } ,
    x if 30 < x && x <= 36 => {
    frame -=30;
    cur_fn = open_eyes;
    } ,
    x if 35 < x && x <= 46 => {
    frame -=45;
    cur_fn = close_eyes;
    } ,
    x if x > 70 && x < 90=> {
    frame -=70;
    cur_fn = open_eyes;
    } ,

    _ => {
        return false;

    }
}
cur_fn(engine,frame);
//debug_engine!(engine, "{}", &"testing our macro");
true
}

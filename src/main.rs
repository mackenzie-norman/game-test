use std::clone;
use std::iter::Enumerate;

use console_engine::crossterm::style;
use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::screen;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use rand::Rng;
mod dialouge;
use dialouge::{Dialouge, pt_in_box};
mod character;
use character::Character;

use serde_json::Result;

#[allow(dead_code, unused)]

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
    let gnd = 52;
    let heaven_line = gnd -12;
    let draw_sky = false;
    let mut og_space = space;
    //TODO ADDD ASSERT
    for i in  heaven_line..gnd + 10{
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
        tree(engine, frame, space + frame, gnd - 14 * scale/rand_arr[i as usize], scale, gnd - rand_arr[i as usize] - 4) ;
        if scale % 2 == 0{
            //bush(engine, frame, space + frame, gnd - (scale * 2) , scale.try_into().unwrap_or(1) -1);
        }
    }
    let mut pl_space = og_space;
    for i in 0..tree_count{
            pl_space -= 40; 
            power_line(engine, frame, pl_space , pl_space + 40 , gnd+14);
    }
    road(engine, frame, 0, gnd +4,  5,tree_count*(og_space ));


fn road(engine: &mut ConsoleEngine, frame: i32, x1:i32, y1: i32, width:i32, length:i32 ){
    let dash_amt = 4;
    let mid: i32 = width/2;
    for i in 0..width{
        if i != mid{

            engine.line(x1 - length + frame, y1 -i , x1  + frame, (y1 - i) , pixel::pxl_fg('$', Color::DarkGrey))
        }else{

            for x in (x1 - length + frame).. (x1 + frame){
                if x% dash_amt == 0{
                    engine.set_pxl(x, y1- i,pixel::pxl_fg('$', Color::DarkGrey) );
                }else{
                    engine.set_pxl(x, y1 - i,pixel::pxl_fg('$', Color::White) );
                }
            }
        }
    }
}
}
fn curve_gen(x1: i32, x2: i32, y1: i32, droop: i32) -> Vec<(i32, i32)> {
    let mut curve_vec: Vec<(i32, i32)> = Vec::new();

    // Midpoint is where the curve reaches its lowest point
    let mid = (x1 + x2) / 2;
    let y2 = y1  + droop;

    // Calculate 'a' for the quadratic formula based on the desired droop
    let a = (y1 - y2) as f64 / ((x1 - mid).pow(2) as f64);

    for x in x1..=x2 {
        // Quadratic equation: y = a * (x - mid)^2 + y2
        let y = (a * (x - mid).pow(2) as f64 + y2 as f64).round() as i32;
        curve_vec.push((x, y));
    }

    curve_vec
}


fn power_line(engine: &mut ConsoleEngine, frame:i32, x1: i32, x2:i32, gnd:  i32){

    //let gnd = 70;
    let heaven_line = gnd -12;
    let width = 4;
    let height = 20;
    
    engine.fill_rect(x2 + frame   , heaven_line + 1, x2 +width + frame , heaven_line - height , pixel::pxl_fg('#', Color::AnsiValue(16)));
    engine.fill_rect(x1 + frame   , heaven_line + 1, x1 +width + frame , heaven_line - height , pixel::pxl_fg('#', Color::AnsiValue(16)));
    for tple in curve_gen(x1 + (width/2), x2  + (width/2), (heaven_line- height), (4)){
        engine.set_pxl(tple.0 + frame,tple.1,pixel::pxl('*'));
    }
    


}
fn bush(engine: &mut ConsoleEngine, frame:i32, x1: i32, y1: i32, scale:u32){
    if scale %3 == 1 || scale > 6{

        engine.fill_circle(x1 + frame, y1, scale, pixel::pxl_fg('@', Color::AnsiValue(29)));
    }else if  scale % 3 ==2{
        engine.fill_circle(x1 + frame, y1, scale, pixel::pxl_fg('@', Color::AnsiValue(29)));
        
    }else{
        engine.fill_circle(x1 + frame, y1, scale, pixel::pxl_fg('@', Color::AnsiValue(28)));
    }

}


fn train_window_static(engine: &mut ConsoleEngine, windows:i32, draw_seats: bool ){
    //fill bottom
    //let windows = 3;
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue((52)));
    //let draw_seats = true;
    let wall_char  = pixel::pxl_fg('X', Color::DarkGrey);
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
    engine.fill_rect(0, 0, screen_width, window_start_y -1 , wall_char);
    engine.fill_rect(0, screen_height, screen_width, screen_height -window_start_y +1 , wall_char);
    engine.fill_rect(0, 0, window_start_x, screen_height , wall_char);
    for i in 1..=windows{
        let window_end_x = window_start_x + ( window_width);
        engine.rect_border(window_start_x, window_start_y, window_end_x , screen_height - window_start_y, BorderStyle::new_double());
        engine.fill_rect(window_end_x + 1 , 0, window_end_x + spacing - 1, screen_height , wall_char);
        if draw_seats{

        //add a seat
        
            let seat_width = 20;
            let seat_x1 = window_start_x + (window_width/4);
            let seat_x2 = seat_x1 + seat_width;
            let seat_y1 = window_start_y + (window_start_y/2);
            let seat_y2 = screen_height -1;
            engine.fill_rect(seat_x1, seat_y1, seat_x2 , seat_y2, seat_char);
            engine.fill_triangle(seat_x2, seat_y1,seat_x2 , seat_y2, seat_x2 + (seat_width), seat_y2, seat_char);
            engine.fill_triangle(seat_x1 - (seat_width ), seat_y1,seat_x1 , seat_y1, seat_x1 ,seat_y2, seat_char);
        }
        window_start_x = window_end_x + spacing;
        //engine.fill_rect(screen_width - window_start_x - 1, 0, screen_width, screen_height , wall_char);

    }
    engine.fill_rect(window_start_x - 1, 0, screen_width, screen_height , wall_char);

}

struct TrainCar<'a> {
    back: &'a TrainCar<'a>,
    front: &'a TrainCar<'a>,
    people: [ i32; 20]

}
fn pretty_line(engine: &mut ConsoleEngine, x1:i32,y1:i32,x2:i32,y2:i32){
    ()
}
fn top_down_telephone_poles(engine: &mut ConsoleEngine, frame:i32 , x1:i32, y1: i32, x2: i32, size:i32){
    let pole_char = pixel::pxl_fg('@', Color::AnsiValue(16));
    let wire_char = pixel::pxl_fg('*', Color::AnsiValue(250));
    for i in x2..=x1{
        if i % 40  == 0{

            for px in curve_gen(i + frame,  i+ frame + 40, y1, 0){
                engine.set_pxl(px.0, px.1, wire_char);
            }
            engine.fill_circle(i + frame, y1, size as u32, pole_char);
        }
    }



}
fn top_down_tracks(engine: &mut ConsoleEngine, frame:i32 , x1:i32, y1: i32, x2: i32, y2:i32){
    let rail_char = pixel::pxl_fg('#', Color::AnsiValue(242));
    let rail_char = pixel::pxl_fg('#', Color::AnsiValue(242));
    let dirt_char = pixel::pxl_fg('@', Color::AnsiValue(58));
    let track_width = 2;
    let tie_width = 2;
    engine.fill_rect(x1 + frame, y1-4, x2 + frame, y2 + 4, dirt_char);
    engine.fill_rect(x1 + frame, y1 +1 , x2 + frame, y1 + track_width, rail_char);
    engine.fill_rect(x1 + frame, y2 -1 , x2 + frame, y2 - track_width, rail_char);
    for i in x2..=x1{
        if i % (tie_width * 4) == 0{
            engine.fill_rect(x1 + i + frame, y1 - track_width , x1 + i + frame + tie_width , y2 + track_width, rail_char);
        }

    }
}
fn top_down_view(engine: &mut ConsoleEngine, frame:i32,) -> Vec<((i32, i32), (i32, i32))>{// _car:TrainCar){
    let window_char = pixel::pxl_fbg('=', Color::AnsiValue(51) , Color::AnsiValue(57));
    let window_char = pixel::pxl_fg('=', Color::AnsiValue(57));
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue((52)));
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let car_x1: i32 = screen_width/6;
    let car_x2: i32 = screen_width - car_x1;
    
    let car_y1 = 0 + screen_height/3;// + screen_height/36;
    let car_y2 = screen_height - car_y1;
    // Lets add some scenery
    engine.fill_rect(40 + frame, 0,-400 + frame, screen_height , pixel::pxl_fg('#', Color::AnsiValue(101)));
    top_down_tracks(engine, frame, 40, car_y2 + 4, -400, car_y2 + 12);
    top_down_tracks(engine, frame, 40, car_y1 + 8, -400, car_y1 + 16);
    
    engine.fill_rect(car_x1, car_y1, car_x2, car_y2, pixel::pxl_fg('+', Color::DarkGrey));
    engine.rect_border(car_x1, car_y1, car_x2, car_y2, BorderStyle::new_simple());
    let mut seat_boxs: Vec<((i32, i32), (i32, i32))> = Vec::new();
    //draw windows
    let window_width = 9;
    let seat_height = 2;
    let seat_width = 4;
    for i in car_x1..=car_x2{
        if i%20 == 0{
            for add_v in 0..=window_width{
            engine.set_pxl(i + add_v, car_y1, window_char);
            engine.set_pxl(i + add_v, car_y2, window_char);
            }
            engine.fill_rect(i+2, car_y1 + 1  , i + seat_width +2, car_y1  + seat_height, seat_char);
            engine.fill_rect(i+2, car_y1 + 2 + seat_height , i + seat_width +2, car_y1 + 1 +  2* seat_height, seat_char);
            engine.fill_rect(i+2, car_y2 - 1  , i + seat_width +2, car_y2  - seat_height, seat_char);
            engine.fill_rect(i+2, car_y2 - 2 - seat_height , i + seat_width +2, car_y2 - 1 -  2* seat_height, seat_char);
            seat_boxs.push(((i, car_y1 + 1 ), ( i + seat_width +2 ,car_y1  + seat_height)));
            seat_boxs.push(((i, car_y1 + 2 + seat_height),(  i + seat_width +3, car_y1 + 1 +  2* seat_height)));
            seat_boxs.push(((i, car_y2 - 1  ), (i + seat_width +3, car_y2  - seat_height)));
            seat_boxs.push(((i, car_y2 - 2 - seat_height ),( i + seat_width +3, car_y2 - 1 -  2* seat_height, )));
            //engine.set_pxl(i, car_y1, window_char);
            //engine.set_pxl(i, car_y2, window_char);
            //engine.set_pxl(i+ 1, car_y1, window_char);
            //engine.set_pxl(i+1, car_y2, window_char);

        }
    }
    //lets do a walkway next
    engine.fill_rect(car_x1+2, car_y1 + (seat_height *2) + 3, car_x2 -2, car_y2 -((seat_height *2) + 3) , pixel::pxl_fg('#', Color::AnsiValue(236)));
    engine.rect(car_x1+2, car_y1 + (seat_height *2) + 3, car_x2 -2, car_y2 -((seat_height *2) + 3) , pixel::pxl_fg('=', Color::AnsiValue(236)));
    engine.line( car_x1, car_y1 + (seat_height *2) + 4, car_x1 , car_y2 -((seat_height *2) + 4) , pixel::pxl_fg('#', Color::AnsiValue(236)));
    top_down_telephone_poles(engine, frame, 40, car_y1 - 6, -400, 2);
    // front car
    let train_spacing = 4;
    let new_car_x2 =  car_x1 - train_spacing;
    //walkway
    engine.fill_rect( new_car_x2, car_y1 + (seat_height *2) + 5, car_x1 +1 , car_y2 -((seat_height *2) + 5) , pixel::pxl_fg('#', Color::AnsiValue(236)));
    let car_x1 = -1;
    engine.rect_border(car_x1, car_y1, new_car_x2, car_y2, BorderStyle::new_simple());
    //gradient for roof
    for i in 1..=(car_y2 - car_y1)/2 {
        engine.line(car_x1, car_y1 + i, new_car_x2 -1, car_y1 + i, pixel::pxl_fg('=',Color::AnsiValue((235 + i).try_into().unwrap())));
        engine.line(car_x1, car_y2 -i, new_car_x2 -1, car_y2 - i, pixel::pxl_fg('=',Color::AnsiValue((235 + i).try_into().unwrap())));

    }

    let car_x1 = car_x2 + train_spacing;
    let new_car_x2 =  screen_width + 1;
    //walkway
    engine.fill_rect( car_x1 - train_spacing -1 , car_y1 + (seat_height *2) + 5,  car_x1, car_y2 -((seat_height *2) + 5) , pixel::pxl_fg('#', Color::AnsiValue(236)));
    engine.rect_border(car_x1, car_y1, new_car_x2, car_y2, BorderStyle::new_simple());
    //gradient for roof
    for i in 1..=(car_y2 - car_y1)/2 {
        engine.line(car_x1 + 1, car_y1 + i, new_car_x2 -1, car_y1 + i, pixel::pxl_fg('=',Color::AnsiValue((235 + i).try_into().unwrap())));
        engine.line(car_x1 +1, car_y2 -i, new_car_x2 -1, car_y2 - i, pixel::pxl_fg('=',Color::AnsiValue((235 + i).try_into().unwrap())));

    }
    seat_boxs

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
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue((52)));
    let seat_border_char = pixel::pxl_fg('%', Color::AnsiValue((255)));
    engine.fill_circle(x+ chair_width/2, y - chair_height , (chair_width/2).try_into().unwrap(), seat_char);
    //engine.circle(x+ chair_width/2, y- chair_height , (chair_width/2).try_into().unwrap(), seat_border_char);
    engine.fill_rect(x, y , x + chair_width, y- chair_height, seat_char);

    let end_y = calculate_y(x, y, vanishing_x, 0, x - chair_width/2 );
    engine.line(x,y,x  -chair_width/2, end_y, seat_char);
    let end_y = calculate_y(x + chair_width, y, vanishing_x, 0, x + chair_width/2 );
    engine.line(x + chair_width,y,x  +chair_width/2, end_y, seat_char);
    //engine.line(x,y + chair_height,x - chair_width/2, end_y + chair_height, seat_char);
    //engine.rect(x, y , x + chair_width, y- chair_height, seat_border_char);
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
    let floor_height = screen_height/2;
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
        let mut start_x;
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


fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(20).unwrap();
    let mut frame = 0;
    
    let mut rng = rand::rng();
    let tree_count = 200;//rng.random_range(0..12);
    let mut space = 8;
    let rand_arr: Vec<i32> = (0..tree_count).map(|x| rng.random_range(1..=5)).collect();
    let mut in_seat = false;
    let mut second_diag = Dialouge::new(vec!["??", "I already bought the tickets!!"], "What! Ahh Hell Nah!!".to_string());
    //let mut insanity = binding.clone();
    //binding.child_diags.push(& mut insanity);

    let mut first_diag = Dialouge::new( vec!["Fuck off , Dick", "Dick Gobbla? I hardly know er"],
     "Hey! Dick Gobbla here, how the hell are ya!".to_string());
    //let mut cur_diag = & mut first_diag;
    
    let mut dick_g = Character::new("Dick Gobbla".to_string(), &mut first_diag, 1);
    dick_g.add_dialouge(&mut second_diag);
    let mut oth_d = Dialouge::new(vec![], "To the days beyond this one which are still perfect.\n\nCome On.".to_string());
    let mut dcb = Character::new("David Berman".to_string(), & mut oth_d, 2);

    let mut in_diag = false;
    let mut seats: Vec<((i32, i32), (i32, i32))> = top_down_view(&mut engine, frame);




    loop {
        engine.wait_frame();
        engine.clear_screen();

        // draw a rectangle with an emoji inside
        //engine.rect(0, 0, 5, 4, pixel::pxl('#'));
        
        if in_seat{
            
            moving_background_anim(&mut engine, frame, tree_count, space, &rand_arr);
            train_window_static(&mut engine, 2, false);
            //in_diag = dick_g.talk_to(&mut engine, frame); 
            dick_g.draw_face(&mut engine, frame, 12, 10);
            in_diag = dcb.talk_to(&mut engine, frame);
            //cur_diag = cur_diag.write_prompt(&mut engine,frame, "Dick Gobbla");
            //in_diag = cur_diag.is_active;
            
        }
        else
        {

            //seats = top_down_view(&mut engine, frame);
            forward_view(&mut engine, frame);

        }
        let mouse_pos = engine.get_mouse_press(MouseButton::Left);
        if let Some(mouse_pos) = mouse_pos {
            //lets see if we clicked a seat!
            if !in_diag{
                let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
                for sb in &seats{
                    if pt_in_box(new_mouse_pos, *sb){
                        println!("WORKING");
                        in_seat = true;
                        // !in_seat;

                        break;
                    }
                }
                in_seat = !in_seat;
                frame = 200;

            }
        }

        //station_enter_anim(&mut engine, frame);
        //engine.set_pxl(2, 2, pixel::pxl('👍'));
        /* 
        */
        
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
        frame += 1;
        frame = frame % 600;
    }
    let mut second_diag = Dialouge::new(vec!["??", "I already bought the tickets!!"], "What! Ahh Hell Nah!!".to_string());
    let j = serde_json::to_string(&second_diag);
    
    println!("test print: {}", j.unwrap());
    println!("GHELP");
}
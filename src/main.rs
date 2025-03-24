
use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use figlet_rs::FIGfont;
use rand::{random, Rng, rngs::StdRng, SeedableRng};
mod dialouge;
use dialouge::{Dialouge, pt_in_box};
mod character;
use character::Character;
mod pov;
use pov::{close_eyes, open_eyes, waking_up};
mod smart_drawing;
use smart_drawing::{line, fill_triangle};

#[allow(dead_code, unused)]

fn bogey(engine: &mut ConsoleEngine, frame:i32, start_val: i32, bottom:i32){
    engine.fill_circle(start_val + frame  , bottom, 4, pixel::pxl_fg('@', Color::DarkGrey));
    engine.fill_rect(start_val + frame, bottom, start_val + frame - 17, bottom + 2,pixel::pxl_fg('@', Color::DarkGrey) );
    engine.fill_circle(start_val + frame -17 , bottom, 4, pixel::pxl_fg('@', Color::DarkGrey) );

}
fn basic_building(engine: &mut ConsoleEngine, frame:i32, ground:i32, x:i32, scale:i32, grey: Pixel, window: Pixel) -> (i32,i32){
    let mut height :i32= (engine.get_height()/ 4).try_into().unwrap() ;
    let mut width:i32 = (engine.get_width()/20).try_into().unwrap();
    height *= scale;
    width *= scale;
    engine.fill_rect(x + frame, ground, x +width + frame, ground - height, grey);
    let window_height = height/8;
    let window_width = width/4;
    //debug_engine!(engine, "{}", window_height);
    for i in (2..height).into_iter().step_by((window_height + 1) as usize){
        //engine.fill_rect(x + frame, ground - i  , x +width + frame, ground - i   - window_height + (window_height/2), window_yellow);
        engine.fill_rect(x + frame + window_width/4, ground - i , x +window_width , ground - i   - window_height + (window_height/2), window);
        engine.fill_rect(x + frame + window_width  + window_width/2, ground - i , x +window_width * 2, ground - i   - window_height + (window_height/2), window);
        engine.fill_rect(x + frame + window_width * 2 + window_width/2, ground - i , x +window_width * 3, ground - i   - window_height + (window_height/2), window);
        engine.fill_rect(x + frame + window_width * 3 + window_width/2, ground - i , x +window_width * 4, ground - i   - window_height + (window_height/2), window);

    }
    return (x + width, ground - height); 
     
}
fn pointy_building(engine: &mut ConsoleEngine, frame:i32, ground:i32, x:i32, scale:i32, grey: Pixel, window: Pixel) -> (i32,i32){
    let (width, height) = basic_building(engine, frame, ground, x, scale, grey, window); 
    let building_center = x + ((width -x )/2);
    let total_height = height - ((ground-height) /3);
    engine.line(building_center, height , building_center, total_height, grey);
    engine.fill_triangle(x, height, width, height, building_center, height - ((ground-height) /4), grey);
    return (width ,total_height)
}
fn building(engine: &mut ConsoleEngine, frame:i32, ground:i32, x:i32, scale:i32, building_type: i32){
    match building_type {
        _x if _x == 1 => {
            let grey = pixel::pxl_fg('#', Color::DarkGrey);
            let window_yellow = pixel::pxl_fg('#', Color::DarkYellow);
            let (width, height) = pointy_building(engine, frame, ground, x, scale, grey, window_yellow); 
            
        }
       _ => {
        let grey = pixel::pxl_fg('#', Color::DarkGrey);
        let window_yellow = pixel::pxl_fg('#', Color::DarkYellow);
        basic_building(engine, frame, ground, x, scale, grey, window_yellow); 
       } 
    }
}
fn title(engine: &mut ConsoleEngine, frame:i32){
    let orig_message = "Why is it so Empty?";
    let hold_message: String = orig_message.chars().take(frame as usize).collect();
    let my_message = &hold_message;
    let  width:i32 = (engine.get_width()).try_into().unwrap();
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(my_message).unwrap();
    //assert!(figure.is_some());
    let print_str = &format!("{}",figure);
    //engine.print((width/2) - (print_str.len().try_into().unwrap_or(0)),0,&print_str );
    let text_width: i32  = my_message.len().try_into().unwrap();
    let start_x = (width/2) - text_width *3;
    let start_y = 4;
    let padding = 0;
    engine.fill_rect( start_x - padding, start_y - padding , start_x + text_width  + padding, start_y + figure.height as i32 + padding, pixel::pxl_bg(' ', Color::Black));
    engine.print(start_x,start_y,&print_str );
    if frame > orig_message.len() as i32 && frame %4 != 0{

        engine.print(width/2 - 12, start_y + 24 + figure.height as i32, "Press Any Button to start");
    }
}
fn night_sky(engine: &mut ConsoleEngine, frame:i32, skybox: (i32,i32,i32,i32), density: f64){
    let mut rng = StdRng::seed_from_u64(2);
    let stars = vec!['x', '.' , '+', 'o'];
    let star_colors = vec![Color::AnsiValue(241), Color::AnsiValue(248), Color::AnsiValue(245)];
    for x in skybox.0..skybox.2{
        for y in skybox.1..skybox.3{
            //
            //engine.set_pxl(x + frame, y, pixel::pxl_fg(stars[0], Color::DarkYellow) );
            if rng.random_bool(density){
                engine.set_pxl(x + frame, y, pixel::pxl_fg(stars[rng.random_range(0..stars.len() )], star_colors[rng.random_range(0..star_colors.len() )]) );
            }
        }
    }

}
fn draw_platform(engine: &mut ConsoleEngine, frame:i32, height:i32){
    let platform_color = Color::AnsiValue(236);
    let platform_char = pixel::pxl_fbg(' ', platform_color, platform_color);
    let split_char = pixel::pxl_fbg('|', Color::Black, platform_color);
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32;
    engine.fill_rect(0, height, screen_width,screen_height, platform_char);
    let spacing = screen_width/12;
    for i in (0..screen_width).into_iter().step_by(spacing as usize){

        engine.line(i, height, i, screen_height, split_char);
    }
}
fn light_pole(engine: &mut ConsoleEngine, frame:i32, x:i32, y:i32){
    let platform_color = Color::AnsiValue(236);
    let pole_char = pixel::pxl_fg('#', platform_color);
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32;
    let height = screen_height/4;
    let light_color = Color::DarkYellow;
    let light_char = pixel::pxl_fg('*', light_color);
    engine.fill_rect(x, y, x + 2 , y - height, pole_char);
    for c in curve_gen(x- 12, x , y-height , -3){
        
        engine.set_pxl(c.0, c.1,pole_char);
        
    }
    for c in curve_gen(x- 11, x + 1 , y-height , -3){
        engine.set_pxl(c.0, c.1,pole_char);
        
        
    }
    for c in curve_gen(x- 10, x + 2 , y-height , -3){
        engine.set_pxl(c.0, c.1,pole_char);
    }
    line(engine,x -24,y , x -12, y -height, light_char);
    line(engine,x ,y , x -10, y -height, light_char);
    fill_triangle(engine,x -24, y, x, y, x-11, y- height, light_char);
    engine.fill_circle(x - 11, y-height, 1, pole_char);
    
    for c in curve_gen(x- 24, x  , y , 1){
        let my_pxl =engine.get_pxl(c.0, c.1);
        //debug_engine!(engine, "{:?}", my_pxl.bg);
        if my_pxl.is_ok(){
            engine.set_pxl(c.0, c.1,pixel::pxl_fbg('*', light_color, my_pxl.unwrap().bg));

        }else{

            engine.set_pxl(c.0, c.1,light_char);
        }
    }
    
}
fn station_sign(engine: &mut ConsoleEngine, frame:i32, x:i32, y:i32){
    let platform_color = Color::AnsiValue(236);
    let pole_char = pixel::pxl_fg('#', platform_color);
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32;
    let height = screen_height/9;
    let width = screen_width/18;
    engine.fill_rect(x, y , x + 2, y - height, pole_char);
    engine.fill_rect(x +width -2, y , x + width, y - height, pole_char);

    engine.fill_rect(x - 1, y - height/2, x + width + 1, y - height, pole_char);
    engine.print(x + 4, y - height + 1, "Tukwilla");
    engine.print(x + 1, y - height + 3, "Amtrak Station");
}
//use console_engine::{pixel, KeyCode};
fn station_enter_anim(engine: &mut ConsoleEngine, mut frame:i32 ){
    let frame_max = 200;
    let wait_time = 30 * 1;
    let max_height = 90;
    let mut height: i32 =  (engine.get_height()/2).try_into().unwrap();
    let mut chars = 0;
    let amtrak_dark_blue = Color::AnsiValue(19);
    let amtrak_light_blue = Color::AnsiValue(27);
    let amtrak_grey = Color::AnsiValue(66);

    let train_blue_pixel = pixel::pxl_fbg('#', amtrak_dark_blue, amtrak_dark_blue);
    let train_grey_pixel = pixel::pxl_fbg('#', amtrak_grey, amtrak_grey);
    if frame > frame_max{
        let diff = frame - frame_max;
        frame = frame_max;
        
        if diff > wait_time{
            
            height += (diff - wait_time)/2;
        }
        if height   > max_height{
            chars = height - max_height;
            height = max_height + 1; 
        }
        

    }
    //Locomotive Body
    let end_val = -80;
    let start_val = 60;
    //let height: i32 =  (engine.get_height()/2).try_into().unwrap();
    let bottom = height + 20;
    night_sky(engine, 0, (0,0, 400, height - 20), 0.001);
    if height   > max_height{
        height = max_height; 
        title(engine, chars);
    }
    building(engine, 0, height , 120, 1, 1);
    building(engine, 0, height , 70, 1, 0);
    //building(engine, 0, height , 70, 2, 1);
    building(engine, 0, height+4  , 90, 2, 1);
    building(engine, 0, height + 6 , 140, 2, 0);
    //WHEELS
    bogey(engine, frame, start_val - 17, bottom);
    bogey(engine, frame, end_val + 17, bottom);
    //Train
    engine.line(end_val + frame , height, bottom + frame, height, train_blue_pixel);
    engine.line(end_val + frame , height + 1, bottom + frame + 1, height +1, train_blue_pixel);
    engine.line(end_val + frame , height + 2, bottom + frame + 2, height +2, train_blue_pixel);

    engine.line(bottom + frame , height, start_val + frame, height + 10, pixel::pxl_fg('#', Color::DarkRed));
    engine.line(bottom + frame , height +1, start_val + frame - 1, height + 9, pixel::pxl_fg('#', Color::DarkRed));

    engine.line(start_val + frame , height + 10, start_val + frame, bottom, pixel::pxl('#'));
    //bottom
    for fill_line in 2..10{
        engine.line(end_val + frame , bottom-fill_line, start_val  -36 + frame, bottom-fill_line, train_blue_pixel);
        engine.line(start_val+ frame , bottom-fill_line, start_val  -36 + frame, bottom-fill_line, train_blue_pixel);

    }
    for fill_line in 10..19{
        engine.line(end_val  + frame , bottom-fill_line, start_val  -5 + frame, bottom-fill_line, train_grey_pixel);
        //engine.line(start_val+ frame , bottom-fill_line, start_val  -25 + frame, bottom-fill_line, train_grey_pixel);

    }
    engine.fill_triangle(start_val - 5+ frame, bottom - 10 , start_val -5  +frame, bottom - 18,  start_val + frame, bottom -10, train_grey_pixel);



    engine.line(end_val + frame , bottom -1, start_val + frame, bottom -1, pixel::pxl_fg('#', Color::Grey));
    engine.line(end_val + frame , bottom, start_val + frame, bottom, pixel::pxl_fg('#', Color::Grey));
    
    //Back
    engine.line(end_val + frame , height, end_val + frame, bottom, pixel::pxl('#'));
    // Door
    engine.rect_border(start_val - 36 + frame, height+ 6, start_val - 26 + frame, bottom  -2, BorderStyle::new_simple());
    engine.set_pxl(start_val - 27 + frame,bottom - 7,pixel::pxl('*'));

    //TRAIN NUMBERS
    let standard_font = FIGfont::from_file("../slant.flf").unwrap();
    let basic_font = FIGfont::from_file("../basic.flf").unwrap();
    let figure = standard_font.convert("30C").unwrap();

    engine.print_fbg(end_val + 3 + frame , bottom - figure.height as i32 - 2, &format!("{}", figure).trim_end(), Color::White, amtrak_dark_blue);
    let figure = basic_font.convert("AMTRAK").unwrap();
    engine.print_fbg(end_val + 20 + frame, height + 3,&format!("{}", figure).trim_end(), Color::White, amtrak_grey );
    draw_platform(engine, frame, bottom - 1);
    //light_pole(engine, frame, 260, bottom + 6);
    light_pole(engine, frame, 220, bottom + 6);
    //light_pole(engine, frame, 180, bottom + 6);
    light_pole(engine, frame, 70, bottom + 6);
    station_sign(engine, frame, 45, bottom + 8);
    
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
    let gnd = (engine.get_height() - engine.get_height()/6) as i32;
    let heaven_line = gnd -12;
    let draw_sky = false;
    let og_space = space;
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
    //road(engine, frame, 0, gnd +4,  5,tree_count*(og_space ));


fn road(engine: &mut ConsoleEngine, frame: i32, x1:i32, y1: i32, width:i32, length:i32 ){
    let dash_amt = 4;
    let mid: i32 = width/2;
    for i in 0..width{
        if i != mid{

            engine.line(x1 - length + frame, y1 -i , x1  + frame, y1 - i , pixel::pxl_fg('$', Color::DarkGrey))
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
    for tple in curve_gen(x1 + (width/2), x2  + (width/2), heaven_line- height, 4){
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
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue(52));
    //let draw_seats = true;
    let wall_char  = pixel::pxl_fg('X', Color::DarkGrey);
    let screen_width =(engine.get_width()) as i32;
    let screen_height =(engine.get_height()) as i32;
    //let mut spacing = screen_width /10;
    let spacing = 4;
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
    let seat_char = pixel::pxl_fg('%', Color::AnsiValue(52));
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
            seat_boxs.push(((i, car_y1  ), ( i + seat_width +2 ,car_y1  + 1 + seat_height)));
            seat_boxs.push(((i, car_y1 + 1 + seat_height),(  i + seat_width +3, car_y1 + 2 +  2* seat_height)));
            seat_boxs.push(((i,  car_y2  - 1- seat_height  ), (i + seat_width +3, car_y2)));
            seat_boxs.push(((i, car_y2 - 2 -  2* seat_height),( i + seat_width +3, car_y2  - seat_height  )));


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


fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(20).unwrap();
    let mut frame = 0;
    
    let mut rng = rand::rng();
    let tree_count = 200;//rng.random_range(0..12);
    let space = 8;
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
    let mut char_seat_map : Vec<Character> = vec![dick_g,dcb];
    //eprintln!("test print: {:?}", seats);

    
    let mut waking = true;
    let mut cur_seat:i32 = -1;
    loop {
        engine.wait_frame();
        engine.clear_screen();

        // draw a rectangle with an emoji inside
        //engine.rect(0, 0, 5, 4, pixel::pxl('#'));
        //if were in a seat render it. 
        /*
        if cur_seat != -1{
            //in_diag = true; 
            engine.clear_screen();
            moving_background_anim(&mut engine, frame, tree_count, space, &rand_arr);
            train_window_static(&mut engine, 2, false);
            if cur_seat as usize <= char_seat_map.len() {

                //dick_g.draw_face(&mut engine, frame, 12, 10);
                in_diag = char_seat_map[cur_seat as usize ].talk_to(&mut engine, frame);
                if !in_diag{
                    cur_seat = -1;
                }
            }else{
                let mouse_pos = engine.get_mouse_press(MouseButton::Left);
                if let Some(mouse_pos) = mouse_pos {
                    cur_seat = -1;
                }
            }
            //in_diag = dick_g.talk_to(&mut engine, frame); 
            //cur_diag = cur_diag.write_prompt(&mut engine,frame, "Dick Gobbla");
            //in_diag = cur_diag.is_active;
            
        }
        else
        {

            seats = top_down_view(&mut engine, frame);
            //forward_view(&mut engine, frame);
            let mouse_pos = engine.get_mouse_press(MouseButton::Left);
            if let Some(mouse_pos) = mouse_pos {
            
                let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
                for sb in seats.iter().enumerate(){
                    if pt_in_box(new_mouse_pos, *sb.1){
                        eprintln!("WORKING {}",sb.0);
                        cur_seat = sb.0 as i32;
                        frame = 200;
                        break;
                    }
                }

            }

        }

         */
        
        station_enter_anim(&mut engine, frame);

        //moving_background_anim(&mut engine, frame+200, tree_count, space, &rand_arr);
        //train_window_static(&mut engine, 2, false);
        //if waking{
            //waking = waking_up(&mut engine, frame);
        //}
        //debug_engine!(engine, "{}", waking);
        //engine.set_pxl(2, 2, pixel::pxl('👍'));
        /* 
        */
        //station_pov_simple(&mut engine, frame);
        
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
        frame += 1;
        //frame = frame % 600;
    }
    
    let second_diag = Dialouge::new(vec!["??", "I already bought the tickets!!"], "What! Ahh Hell Nah!!".to_string());
    let j = serde_json::to_string(&second_diag);
    
    eprintln!("test print: {:?}", j.unwrap());
    
}
use console_engine::pixel::{self};
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

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

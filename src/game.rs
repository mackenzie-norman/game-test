use std::fmt::format;
use std::io;
use std::sync::mpsc::RecvTimeoutError;
use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{screen, Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use figlet_rs::FIGfont;
use crate::dialouge::{Dialouge, pt_in_box};
use crate::character;
use character::Character;
use crate::debug_engine;

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
fn barcode(engine: &mut ConsoleEngine, boxx: (i32,i32,i32,i32)){
    let array = [true, false,true,false,false,true];
    for i in boxx.1..= boxx.3{
        if array[(i% array.len() as i32) as usize ]{
            if i % 4 == 0 {
                engine.line(boxx.0, i, boxx.2, i, pixel::pxl('-'));
            }else{

                engine.line(boxx.0, i, boxx.2, i, pixel::pxl('='));
            }
        }
    }
}
fn confirm(engine: &mut ConsoleEngine, name:String){
    loop{
    engine.wait_frame();
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;

    let bg_char = pixel::pxl_bg(' ', Color::Black);
    let box_x1: i32 = screen_width/6;
    let box_x2: i32 = screen_width - box_x1;
     
    let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
    let box_y2 = screen_height - screen_height/6 + screen_height/24;
        //TODO chunk chars to pages 
    engine.fill_rect(box_x1, box_y1, box_x2, box_y2, bg_char);
    engine.rect_border(box_x1, box_y1, box_x2, box_y2, BorderStyle::new_heavy());
    let print_str: String = format!("Are you sure {} is your name? (press enter to confirm)", name);
    engine.print(box_x1 + 1, box_y1 + 1,&print_str );
    if engine.is_key_pressed(KeyCode::Enter ){
        break;
    }
    engine.draw();

    }
}
fn get_text(engine: &mut ConsoleEngine, ) -> Option<char>{
    let lower_case = ('a'..='z').into_iter().collect::<Vec<char>>();
    let upper_case = ('A'..='Z').into_iter().collect::<Vec<char>>();
    for ch in lower_case{
        if engine.is_key_pressed(KeyCode::Char(ch)){
            return Some(ch);
        }
    }
    for ch in upper_case{
        if engine.is_key_pressed(KeyCode::Char(ch)){
            return Some(ch);
        }
    }
    if engine.is_key_pressed(KeyCode::Char(' ')){
        return Some(' ');
    }
    return None; 
}
pub fn ticket_screen(engine: &mut ConsoleEngine, frame:i32){
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let padding_x = screen_width/24;
    let padding_y = screen_height /4;
    let mut first_name = "".to_string();
    let mut frame = frame;
    let mut in_diag = true;
    let mut oth_d = Dialouge::new(vec!["Here it is.", "Ok."], "I need to see your ticket.\n If you have one.".to_string());
    let mut dcb = Character::new("Henry Chianski".to_string(), & mut oth_d, 2);
    loop{
    engine.wait_frame();
    engine.clear_screen();
    
    draw_platform(engine, frame, 0 );
    if in_diag{
        in_diag = dcb.talk_to(engine, frame);
    }else{

    engine.fill_rect(padding_x, padding_y , screen_width - padding_x, screen_height - padding_y, pixel::pxl_bg(' ', Color::Black));
    engine.rect_border(padding_x, padding_y , screen_width - padding_x, screen_height - padding_y, BorderStyle::new_simple());

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("AMTRAK").unwrap();
    //assert!(figure.is_some());
    let print_str = &format!("{}",figure);
    engine.print(padding_x +1, padding_y +1,&print_str);
    engine.line(screen_width - padding_x - 60, padding_y + 1, screen_width - padding_x - 60, screen_height - padding_y -1, pixel::pxl('#'));
    barcode(engine, (screen_width - padding_x - 80, padding_y + 6, screen_width - padding_x - 64, screen_height- padding_y - 6));
    engine.print_fbg(padding_x + 4, padding_y + 10, "Departing Station", Color::DarkGrey, Color::Black);
    engine.print_fbg(screen_width - padding_x - 58, padding_y + 10, &format!("Passenger Name: {} ", ""), Color::DarkGrey, Color::Black);
    

    engine.print_fbg(screen_width - padding_x - 58 + 15, padding_y + 10, &first_name, Color::White, Color::Black);
    if frame% 4 == 0{
        engine.print_fbg(screen_width - padding_x - 58 + 15 + first_name.len() as i32, padding_y + 10, "_", Color::DarkGrey, Color::DarkGrey);

    }

    
    engine.print(screen_width - padding_x - 40, padding_y + 10, "",);
    engine.print(padding_x + 4, padding_y + 11, "Tukwilla");

    if engine.is_key_pressed(KeyCode::Enter ){
        confirm(engine, first_name);
        break;
    }
    if engine.is_key_pressed(KeyCode::Backspace) && first_name.len() > 0{
        first_name = first_name.chars().take(first_name.len() -1).collect();
    }
    let charr  = get_text(engine);
    if charr.is_some(){
        let mut new_str = format!("{}{}", first_name, charr.unwrap());
        first_name = new_str.clone().to_owned();

    }
    }
    engine.draw();
    frame +=1;
    }

}
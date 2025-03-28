use console_engine::pixel;
use console_engine::pixel::{ Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;
use figlet_rs::FIGfont;
//chat cbd
fn insert_newline_every_n_chars(s: &str, n: usize) -> String {
    insert_newline_with_hyphen(s, n)
}
fn insert_newline_with_hyphen(s: &str, n: usize) -> String {
    let mut result = String::new();
    let mut current_line = String::new();

    for c in s.chars() {
        current_line.push(c);

        if current_line.len() == n {
            if c != ' ' {
                current_line.push('-');
            }
            result.push_str(&current_line);
            result.push('\n');
            current_line.clear();
        }
    }

    // Add any remaining characters
    if !current_line.is_empty() {
        result.push_str(&current_line);
    }

    result
}
pub struct Article{
    pub width:i32,
    pub height:i32,
    title_font:FIGfont,
    title:String,
    text:String
}
impl Article{
    pub fn new(width:i32, height:i32, title:String, text:String) -> Self{
        return Article{width: width,
            height: height,
            title_font:FIGfont::standard().unwrap(),
            title:title,
            text:text   
        };
    }
    pub fn to_engine(&self, engine: &mut ConsoleEngine, x :i32, y:i32, fill_color:Color){
        let print_text = insert_newline_every_n_chars(&self.text, self.width as usize) ;
        engine.print_fbg(x, y, &print_text, Color::Black, fill_color);
    }
    pub fn get_height(&self) -> i32{
        insert_newline_every_n_chars(&self.text,self.width as usize).chars().filter(|&c| c == '\n').count() as i32

    }

}
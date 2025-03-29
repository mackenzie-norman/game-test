use console_engine::pixel;
use console_engine::pixel::{ Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;
use figlet_rs::FIGfont;
//chat cbd

fn insert_newline_with_hyphen(s: &str, n: usize, max_lines: usize) -> String {
    let mut result = String::new();
    let mut current_line = String::new();
    let max_break = 5;
    let mut line_count =0;
    for c in s.chars() {
        current_line.push(c);

        if current_line.len() == n || (current_line.len() > n - max_break && c.is_whitespace() ){
            if c != ' ' {
                current_line.push('-');
            }
            if line_count < max_lines{

            result.push_str(&current_line);
            result.push('\n');
            }
            line_count += 1;
            current_line.clear();
        }
    }

    // Add any remaining characters
    if !current_line.is_empty()  && line_count < max_lines{
        result.push_str(&current_line);
    }

    result
}
pub struct Article{
    pub width:i32,
    pub height:i32,
    title_font:FIGfont,
    title:String,
    text:String,
    original_text: String

}

///GOOD LETTERS 
/// - 𝙰
/// - 𝓐
impl Article{
    pub fn new(width:i32, height:i32, title:String, text:String) -> Self{
        let mut art = Article{width: width,
            height: height,
            title_font:FIGfont::standard().unwrap(),
            title:title,
            text:text.clone(),
            original_text: text.clone()
        };
        art.format_for_print();
        return art;
    }
    pub fn to_engine(&self, engine: &mut ConsoleEngine, x :i32, y:i32, fill_color:Color){
        
        engine.print_fbg(x, y, &string_to_unicode_offset(&self.text, '𝙰' as u32), Color::Black, fill_color);
    }
    pub fn get_height(&self) -> i32{
        self.height
    }
    ///This is destructive
    pub fn format_for_print(&mut self){
        self.text = insert_newline_with_hyphen(&self.original_text, self.width as usize, self.height as usize);
    }

}

pub fn base_newspaper_anim( engine: &mut ConsoleEngine, frame:i32, ){
    let black_background = Color::AnsiValue(247);
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;
    let paper_x1: i32 = screen_width/4;
    let paper_x2: i32 = screen_width - paper_x1;
    let paper_y1: i32 =  2;
    let paper_y2: i32 =  screen_height - paper_y1;
    engine.rect_border(paper_x1, paper_y1, paper_x2, paper_y2, BorderStyle::new_simple());
    engine.fill_rect(paper_x1, paper_y1, paper_x2, paper_y2, pixel::pxl_bg(' ' , black_background));
    //title and text
    let title = "Amtrak Times";
    let standard = FIGfont::standard().unwrap();
    let print_str = format!("{}", standard.convert(title).unwrap());
    engine.print_fbg(paper_x1 + 4, paper_y1 + 2, &print_str ,Color::Black, black_background);
    engine.line(paper_x1 +1 ,  paper_y1+8, paper_x2 -1, paper_y1+8, pixel::pxl_fbg('=' ,  Color::Black, black_background));

    let art = Article::new((paper_x2 - paper_x1  )/2  , paper_y2-paper_y1 + 30, "Miss Lonelyhearts".to_string(), "Dear Miss Lonelyhearts of Miss Lonelyhearts--

I am twenty-six years old and in the newspaper game. Life for me is a desert empty of comfort. I cannot find pleasure in food, drink, or women--nor do the arts give me joy any longer. The Leopard of Discontent walks the streets of my city; the Lion of Discouragement crouches outside the walls of my citadel. All is desolation and a vexation of the spirit. I feel like hell. How can. I believe, how can I have faith in this day and age? Is it true that the greatest scientists believe again in you?

I read your column and like it very much. There you once wrote: 'When the salt has lost its savour, who shall savour it again?' Is the answer: 'None but the Saviour?'

Thanking you very much for a quick reply, I remain yours truly,

A Regular Subscriber".to_owned());
    art.to_engine(engine, (paper_x2 - (paper_x2 - paper_x1 -2 )) +1 , paper_y1 + 30  ,black_background);
    //engine.line(paper_x1 + 1 ,  paper_y2 - art.get_height() -1 , paper_x2 -1, paper_y2 - art.get_height() - 1 , pixel::pxl_fbg(' ' ,  Color::Black, black_background));
    //engine.set_pxl(paper_x1 +2, paper_y1 + 2, Pixel { bg: black_background, fg: Color::Black, chr: '𝑎' });
    //debug_engine!(engine, "{}",art.get_height());
}

pub fn char_to_unicode_offset(c: char, base: u32 ) -> Option<char>{
    if !c.is_ascii_alphabetic(){
        return Some(c);
    }
    //println!("{}", c as u32);
    //println!("{}", base );
    let mut offset = 65;
    if c.is_ascii_lowercase(){
        offset = 97;
    }
    return char::from_u32(base + ( c as u32 - offset));

}
pub fn string_to_unicode_offset(test: &str, base: u32) -> String{
    return test.chars().map(|c:char|{ char_to_unicode_offset(c, base).unwrap()}).collect();
}
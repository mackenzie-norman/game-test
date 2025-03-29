

use console_engine::pixel;
use console_engine::rect_style::BorderStyle;
use console_engine::{Color,MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use serde::Serialize;
pub fn display_prompt(engine: &mut ConsoleEngine, current_char:i32, prompt:String, name: String){
    let max_chars = 1024;
    let bg_char = pixel::pxl_bg(' ', Color::Black);
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;

    let box_x1: i32 = screen_width/6;
    let box_x2: i32 = screen_width - box_x1;
     
    let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
    let box_y2 = screen_height - screen_height/6 + screen_height/24;
        //TODO chunk chars to pages 
    engine.fill_rect(box_x1, box_y1, box_x2, box_y2, bg_char);
    engine.rect_border(box_x1, box_y1, box_x2, box_y2, BorderStyle::new_heavy());
    let print_str: String = prompt.chars().take(current_char.try_into().unwrap()).collect();
    engine.print(box_x1 + 1, box_y1 + 1,&print_str );
    if current_char % 3 != 0 && current_char > prompt.len().try_into().unwrap() {
        engine.print(box_x2 -1, box_y2 -1, "V");
    }

}
pub fn display_choices(engine: &mut ConsoleEngine, choices: Vec<&str>) -> i32{
    
    1
}
pub fn pt_in_box(pt:(i32,i32), boxx: ((i32,i32),(i32,i32))) -> bool{
    let box_x1: i32 = boxx.0.0;
    let box_x2: i32 = boxx.1.0;
    
    let box_y1 = boxx.0.1;
    let box_y2 = boxx.1.1;

    pt.0 < box_x2 && pt.0 > box_x1 && pt.1 > box_y1 && pt.1 < box_y2

}
fn is_skipping(engine: &mut ConsoleEngine) -> bool{
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;

    let box_x1: i32 = screen_width/6;
    let box_x2: i32 = screen_width - box_x1;
     
    let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
    let box_y2 = screen_height - screen_height/6 + screen_height/24;
    if engine.is_key_pressed(KeyCode::Enter){
        return false;
                //return self;
    }
    let mouse_pos = engine.get_mouse_press(MouseButton::Left);
    if let Some(mouse_pos) = mouse_pos {
        let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
        if new_mouse_pos.0 < box_x2 && new_mouse_pos.0 > box_x1 && new_mouse_pos.1 > box_y1 && new_mouse_pos.1 < box_y2 {
            return false;
        } 
                
    }
    true
}
pub fn tutorial_skipping(engine: &mut ConsoleEngine) -> bool{
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;

    let box_x1: i32 = screen_width/6;
    let box_x2: i32 = screen_width - box_x1;
     
    let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
    let box_y2 = screen_height - screen_height/6 + screen_height/24;
    if engine.is_key_pressed(KeyCode::Enter){
        return false;
                //return self;
    }
    let mouse_pos = engine.get_mouse_press(MouseButton::Left);
    if let Some(mouse_pos) = mouse_pos {
        let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
        if new_mouse_pos.0 < box_x2 && new_mouse_pos.0 > box_x1 && new_mouse_pos.1 > box_y1 && new_mouse_pos.1 < box_y2 {
            return false;
        } 
                
    }
    engine.print_fbg(box_x1 +1, box_y2 - 2, "Tip: You will know if someone is done talking by the flashing arrow. Click in the box or press enter to continue", Color::Grey, Color::Black);
    true
}
fn leave(engine: &mut ConsoleEngine) -> bool{
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;

    let box_x1: i32 = screen_width/6;
    let box_x2: i32 = screen_width - box_x1;
     
    let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
    let box_y2 = screen_height - screen_height/6 + screen_height/24;
    let mouse_pos = engine.get_mouse_press(MouseButton::Left);
    if let Some(mouse_pos) = mouse_pos {
    let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
    if new_mouse_pos.0 < box_x2 && new_mouse_pos.0 > box_x1 && new_mouse_pos.1 > box_y1 && new_mouse_pos.1 < box_y2 {
    }else{
        return false;
    }
    }
    return true;
}


//#[derive(Serialize)]
pub struct Dialouge<'a> {
    choices: Vec<&'a str>,
    prompt : String,
    pub is_prompting:bool,
    pub is_active: bool,
    pub current_char: u32,
    pub choice: i32,
    pub leaving_fn : fn(engine: &mut ConsoleEngine) -> bool,
    pub skipping_fn: fn(engine: &mut ConsoleEngine) -> bool
}

impl <'a> Dialouge<'a>{
    pub fn new( choices: Vec<&'a str>,  prompt : String, ) -> Dialouge<'a>{
        Dialouge{
            choices: choices,
            prompt: prompt,
            is_prompting: true, 
            is_active: true, 
            choice: -1, 
            current_char:0,
            leaving_fn : leave,
            skipping_fn: is_skipping
            


        }
        
    }
    pub fn write_prompt(& mut self, engine: &mut ConsoleEngine, frame:i32, speaker_name : &str) -> usize{
        self.is_active = true;
        let max_chars = 1024;
        let bg_char = pixel::pxl_bg(' ', Color::Black);
        let screen_width: i32 =(engine.get_width()) as i32;
        let screen_height: i32 =(engine.get_height()) as i32;

        let box_x1: i32 = screen_width/6;
        let box_x2: i32 = screen_width - box_x1;
     
        let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
        let box_y2 = screen_height - screen_height/6 + screen_height/24;
        let mut set_speaker_name =speaker_name;
    // Lets add some scenery
        //TODO chunk chars to pages 
        engine.fill_rect(box_x1, box_y1, box_x2, box_y2, bg_char);
        engine.rect_border(box_x1, box_y1, box_x2, box_y2, BorderStyle::new_heavy());
        if self.is_prompting {
            //let mut print_str = self.prompt.clone();
            let print_str: String = self.prompt.chars().take(self.current_char.try_into().unwrap()).collect();
            engine.print(box_x1 + 1, box_y1 + 1,&print_str );
            if frame % 3 != 0 && self.current_char > self.prompt.len() as u32{
                engine.print(box_x2 -1, box_y2 -1, "V");
            }
            self.current_char += 2;
            self.is_prompting = (self.skipping_fn)(engine);
            
        }
        else if self.choice == -1{
            set_speaker_name = "You";
            self.current_char = 0;
            let mut opt_boxs: Vec<((i32, i32), (i32, i32))> = Vec::new();
            for i in self.choices.iter().enumerate(){

                engine.rect_border(box_x1 + 1, box_y1  +1+  3 * (i.0 as i32), box_x1 +2 + i.1.len() as i32, box_y1  +  3 * (i.0 as i32) + 3, BorderStyle::new_simple());
                engine.print(box_x1 + 2,box_y1  + 2+ 3 * (i.0 as i32) , i.1);
                opt_boxs.push(((box_x1 + 2  , box_y1  +  3 * (i.0 as i32)),( box_x1 + 10 + i.1.len() as i32, box_y1  +  3 * (i.0 as i32) + 3)));

            }
            let mouse_pos = engine.get_mouse_press(MouseButton::Left);
            if let Some(mouse_pos) = mouse_pos {
                let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
                if new_mouse_pos.0 < box_x2 && new_mouse_pos.0 > box_x1 && new_mouse_pos.1 > box_y1 && new_mouse_pos.1 < box_y2{
                    //self.is_prompting = false;
                    
                    for i in opt_boxs.iter().enumerate(){
                        if pt_in_box(new_mouse_pos, *i.1){
                            
                            //engine.fill_rect(box_x1, box_y1, box_x2, box_y2, bg_char);
                            //engine.print(box_x1 + 1, box_y1 + 1,self.choices[i.0]); 
                            self.choice = i.0.try_into().unwrap_or(-1);
                        }
                    };
                    
                } 
                else{  
                    self.is_prompting = true;
                }
            }
        }else if self.choice > -1{
            set_speaker_name = "You";
            let print_str: String = self.choices[self.choice as usize].chars().take(self.current_char.try_into().unwrap()).collect();
            engine.print(box_x1 + 1, box_y1 + 1,&print_str );
            self.current_char += 2;
            if frame % 3 != 0 && self.current_char > self.prompt.len() as u32{
                engine.print(box_x2 -1, box_y2 -1, "V");
            }
            let mouse_pos = engine.get_mouse_press(MouseButton::Left);
            if let Some(mouse_pos) = mouse_pos {
                let new_mouse_pos = (mouse_pos.0.try_into().unwrap_or(0), mouse_pos.1.try_into().unwrap_or(0));
                if new_mouse_pos.0 < box_x2 && new_mouse_pos.0 > box_x1 && new_mouse_pos.1 > box_y1 && new_mouse_pos.1 < box_y2{
                    //self.is_prompting = false;
                    return (self.choice + 1) as usize;

                } 
                else{
                    self.reset();
                    return 0;
                }
            }

        }
        self.is_active = (self.leaving_fn)(engine);
        engine.print_fbg(box_x1 + 1, box_y1, set_speaker_name, Color::Green, Color::Black);
        return 0;
    }
    pub fn reset(& mut self){
        self.is_prompting = true;
        self.is_active = false;
        self.choice = -1;
        self.current_char = 0;

    }
    
}
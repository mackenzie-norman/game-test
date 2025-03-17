use crate::dialouge;
use dialouge::Dialouge;

use console_engine::ConsoleEngine;

pub struct Character<'a>{
    name:String,
    dialouge_tree: Vec<&'a mut Dialouge<'a>> ,
    cur_dialouge : usize,    
    seat: i32

}
impl<'a> Character<'a>{
    pub fn new(name: String, dialouge_tree: &'a mut Dialouge<'a>, seat: i32 ) -> Character {
        let mut diag_tree = Vec::new();
        diag_tree.insert(0, dialouge_tree);
        Character{
            name:name,
            dialouge_tree:diag_tree,
            cur_dialouge : 0,
            seat:seat,
        }

    }
    pub fn talk_to(&mut self, engine: & mut ConsoleEngine, frame:i32) -> bool {
        //let new_ref: &mut Dialouge<'a>  = 
        let diag_val = self.dialouge_tree[self.cur_dialouge].write_prompt(engine, frame, self.name.as_str());
        if diag_val != 0{
            self.dialouge_tree[self.cur_dialouge].reset();
            self.cur_dialouge += diag_val;
        }
        if self.dialouge_tree.len() > self.cur_dialouge{
            return  self.dialouge_tree[self.cur_dialouge].is_active;

        }else{
            self.cur_dialouge = 0;
            return false;
        }
        
        //self.dialouge_tree = new_ref; 
        //self.dialouge_tree = new_ref;
    }
    pub fn add_dialouge(& mut self,  dialouge:&'a mut Dialouge<'a>){
        self.dialouge_tree.push(dialouge);
    }
    pub fn draw_face(& self,  engine: & mut ConsoleEngine, frame:i32, x: i32,y: i32){
        engine.print(x+ 3, y, ".------\\ /------.");
        engine.print(x+ 3,y+1, "|       -       |");
        engine.print(x+ 3,y+2, "|               |");
        engine.print(x+ 3,y+3, "|               |");
        engine.print(x+ 3,y+4, "|               |");
        engine.print(x,y+5, "_______________________");
        engine.print(x,y+6, "===========.===========");
        engine.print(x+2,y+7, "/ ~~~~~     ~~~~~ \\");
        engine.print(x+1,y+8, "/|     |     |\\");
        engine.print(x,y+9, "W   ---  / \\  ---   W");
        engine.print(x+1,y+10, "\\.      |o o|      ./");
        engine.print(x+2,y+11, "|                 |");
        engine.print(x+3,y+12, "\\    #########    /");
        engine.print(x+4,y+13, "\\  ## ----- ##  /");
        engine.print(x+5,y+14, "\\##         ##/");
        engine.print(x+6,y+15, "\\_____v_____/");
    }
}
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;



struct Ennemy {
    x : f32,
    y : f32,
    width : f32,
    height : f32,

}

impl Ennemy {
    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: &u32){
        draw_rect(buf, self.x as u32, self.y as u32, width_screen, self.width as u32, self.height as u32, 0x1b8024);
    }
}
struct Vaisseau {
    x : f32,
    y : f32,
    dx : f32,
    width : u32,
    height : u32,
}

impl Vaisseau {

    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: &u32) {
        draw_rect(buf, self.x as u32, self.y as u32, width_screen, self.width, self.height, 0x000000);
    }

    pub fn update(&mut self, width_screen: u32) {
        self.x += self.dx;

        if self.x < 0.0 {
            self.x = width_screen as f32;
        } else if self.x > width_screen as f32 {
            self.x = 0.0;
        }
    }
}    

fn draw_pixel(buf: &mut Vec<u32>, x: u32, y: u32, width: &u32, colour : u32) {
    // Calculer l'index uniquement si (x, y) est dans les limites du buffer
    if x < *width && y * width < buf.len() as u32 {
        let index = (y * width + x) as usize;
        buf[index] = colour; // Couleur noire
    }
}

fn draw_rect(buf : &mut Vec<u32>, x : u32, y : u32, width_screen: &u32, width : u32, height : u32, colour : u32){
    for pos_y in y..(y+height){
        for pos_x in x..(x+width){
            draw_pixel(buf, pos_x, pos_y, width_screen, colour);
        }
    }
}

struct Missile {
    x : f32,
    y : f32,
    width : u32,
    height : u32,
}

impl Missile {

    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: &u32) {
        draw_rect(buf, self.x as u32, self.y as u32, width_screen, self.width, self.height, 0x5DE2E7)
    }
}

struct Jeu {
    width : u32,
    height : u32,
    buffer : Vec<u32>,
    window : Window,
    fps : usize,
    space_pressed :bool,
    frame_count : u32,
    player : Vaisseau,
    liste_tirs : Vec<Missile>,
    liste_ennemy : Vec<Ennemy>
}

impl Jeu {

    pub fn new(width: u32, height: u32, fps: usize, player: Vaisseau, liste_tirs: Vec<Missile>, liste_ennemy : Vec<Ennemy>) -> Self {
        let buffer = vec![0; (width * height) as usize];
        let mut  window = Window::new(
            "Waza",
            width as usize,
            height as usize,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.set_target_fps(fps);
        Self {
            space_pressed : false,
            frame_count :  0,
            width,
            height,
            buffer,
            window,
            fps,
            player,
            liste_tirs,
            liste_ennemy,
        }
    }

    pub fn new_missile(&mut self) {

        self.liste_tirs.push(Missile{x : self.player.x  + 20 as f32, y : self.player.y - 20 as f32, width : 10, height : 20});
    }

    pub fn new_ennemy(&mut self){
        let mut rng = rand::thread_rng();
        self.liste_ennemy.push(Ennemy{x : rng.gen_range(1..=self.width) as f32, y : -20.0 , width : 16.0, height : 16.0});
    }

    pub fn run(&mut self){
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();
            // Mettre à jour le contenu de la fenêtre avec le framebuffer
            self.buffer = vec![0xFF0000; self.width as usize * self.height as usize];
            self.player.draw(&mut self.buffer, &self.width);
            //self.player.update(&self.window, &self.width);

            for tir in &self.liste_tirs {
                tir.draw(&mut self.buffer, &self.width);
            }

            for ennemy in &self.liste_ennemy{
                ennemy.draw(&mut self.buffer, &self.width);
            }

            self.window.update_with_buffer(&self.buffer, self.width as usize, self.height as usize).unwrap();
            
        }
    }
    pub fn update(&mut self){
        self.player.dx = 0.0;

        if self.frame_count % 60 == 0{
            println!("{}",self.frame_count);
            self.new_ennemy();
        }
        
        for key in self.window.get_keys() {
            match key {
                Key::Left => self.player.dx = -5.0,
                Key::Right => self.player.dx = 5.0,
                _ => {}  
            }
        }
        self.player.update(self.width);
        //if (self.player.x - self.player.dx >0.0)
        //if (self.player.x + self.player.width as f32 + self.player.dx < self.width as f32)
        if self.window.is_key_pressed(Key::Space, KeyRepeat::No) && !self.space_pressed{
            self.new_missile();
            self.space_pressed = true;
        }
        else {
            self.space_pressed = false;
        }
        for tir in &mut self.liste_tirs {
            tir.y -= 5.0;
        };
        for ennemy in &mut self.liste_ennemy{
            ennemy.y += 2.0;
        }
        self.liste_tirs.retain(|tir| tir.y > 0.0);
        self.liste_ennemy.retain(|ennemy| ennemy.y < self.height as f32 + 10 as f32);
        self.frame_count += 1

    }
}


fn main() {

    let player = Vaisseau {
        x: 50.0,
        y : 300.0,
        dx : 0.0,
        width : 50,
        height : 50,
    };
    let mut jeu_dans_main_vrai = Jeu::new(640, 360, 60, player, Vec::new(), Vec::new());
    jeu_dans_main_vrai.run(); 
}
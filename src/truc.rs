use minifb::{Key, Window, WindowOptions};

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

    //pub fn update(&mut self, window : &Window, width_screen: &u32) {}
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
}

impl Missile {

    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: &u32) {
        draw_rect(buf, self.x as u32, self.y as u32, width_screen, 10, 20, 0x5DE2E7)
    }
}

struct Jeu {
    width : u32,
    height : u32,
    buffer : Vec<u32>,
    window : Window,
    fps : usize,
    player : Vaisseau,
    liste_tirs : Vec<Missile>
}

impl Jeu {

    pub fn new(width: u32, height: u32, fps: usize, player: Vaisseau, liste_tirs: Vec<Missile>) -> Self {
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
            width,
            height,
            buffer,
            window,
            fps,
            player,
            liste_tirs,
        }
    }

    pub fn new_missile(&mut self) {

        self.liste_tirs.push(Missile{x : self.player.x  + 20 as f32, y : self.player.y - 20 as f32});
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
            self.window.update_with_buffer(&self.buffer, self.width as usize, self.height as usize).unwrap();
            
        }
    }
    pub fn update(&mut self){
        for key in self.window.get_keys() {
            match key {
                Key::Left if (self.player.x - self.player.dx >0.0)  => self.player.x -= self.player.dx,
                Key::Right if (self.player.x + self.player.width as f32 + self.player.dx < self.width as f32) => self.player.x += self.player.dx,
                Key::Space => self.new_missile(),
                _ => {}
            }
        };
        for tir in &mut self.liste_tirs {
            tir.y -= 5.0;
        };
        self.liste_tirs.retain(|tir| tir.y > 0.0)
    }
}


fn main() {

    let player = Vaisseau {
        x: 50.0,
        y : 300.0,
        dx : 5.0,
        width : 50,
        height : 50,
    };
    let mut jeu_dans_main_vrai = Jeu::new(640, 360, 165, player, Vec::new());
    jeu_dans_main_vrai.run(); 
}
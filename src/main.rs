use minifb::{Key, Window, WindowOptions, KeyRepeat};
use rand::Rng;

fn draw_pixel(buf: &mut Vec<u32>, x: u32, y: u32, width: u32, colour : u32) {
    // Calculer l'index uniquement si (x, y) est dans les limites du buffer
    if x < width && y * width < buf.len() as u32 {
        let index = (y * width + x) as usize;
        buf[index] = colour; // Couleur noire
    }
}

fn draw_rect(buf : &mut Vec<u32>, x : u32, y : u32, width_screen: u32, width : u32, height : u32, colour : u32){
    for pos_y in y..(y+height){
        for pos_x in x..(x+width){
            draw_pixel(buf, pos_x, pos_y, width_screen, colour);
        }
    }
}


struct ElementGraphique {
    x : f32,
    y : f32,
    dx : f32,
    dy : f32,
    width : u32,
    height : u32,
    col : u32,
}

impl ElementGraphique {

    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: u32, width : u32, height : u32) {
        draw_rect(buf, self.x as u32, self.y as u32, width_screen, width, height, self.col)
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy; 
    }

    pub fn collision(&self, other : ElementGraphique) -> bool  {
        return !(
               (other.x as u32 >= self.x as u32 + self.width)   
            || (other.x as u32 + other.width <= self.x as u32 )  
            || (other.y as u32 >= self.y as u32 + self.height ) 
            || (other.y as u32 + other.height <= self.y as u32)
        );
    }
}

struct Vaisseau {
    parent: ElementGraphique,
    liste_tirs : Vec<Missile>,
}

impl Vaisseau {
    pub fn new( x : f32, y : f32, dx : f32, dy : f32, width : u32, height : u32, col : u32,) -> Self {
        Vaisseau {
            parent : ElementGraphique {
                x : x,
                y : y,
                dx : dx,   
                dy : dy,
                width : width,
                height : height,
                col : col,
            },
            liste_tirs : Vec::new(),
        } 
    }

    pub fn update(&mut self, width_screen: u32) {
        self.parent.x += self.parent.dx;

        if self.parent.x < 0.0 {
            self.parent.x = width_screen as f32;
        } else if self.parent.x > width_screen as f32 {
            self.parent.x = 0.0;
        }
    }

    pub fn tirer(&mut self){
        self.liste_tirs.push(Missile::new((self.parent.x + (self.parent.x+self.parent.width as f32))/2.0, self.parent.y - 5.0, 0.0, -5.0, 10, 15, 0xfff000));
    }

}

struct Ennemi {
    parent: ElementGraphique,
}

impl Ennemi {
    pub fn new( x : f32, y : f32, dx : f32, dy : f32, width : u32, height : u32, col : u32,) -> Self {
        Ennemi {
            parent : ElementGraphique {
                x : x,
                y : y,
                dx : dx,   
                dy : dy,
                width : width,
                height : height,
                col : col,
            }
        } 
    }
}

struct Missile {
    parent: ElementGraphique,
}

impl Missile {
     
    pub fn new( x : f32, y : f32, dx : f32, dy : f32, width : u32, height : u32, col : u32,) -> Self {
        Missile {
            parent : ElementGraphique {
                x,
                y,
                dx,   
                dy,
                width,
                height,
                col,
            }
        } 
    }
}

struct Jeu {
    width_screen : u32,
    height_screen : u32,
    buffer : Vec<u32>,
    window : Window,
    fps : usize,
    space_pressed :bool,
    frame_count : u32,
    player : Vaisseau,
    liste_ennemy : Vec<Ennemi>
}

impl Jeu {
    pub fn new(width: u32, height: u32, fps: usize, player: Vaisseau) -> Self {
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
            width_screen: width,
            height_screen: height,
            buffer,
            window,
            fps,
            player,
            liste_ennemy : Vec::new(),
        }
    }

    pub fn spawn_ennemy(&mut self){
        let mut rng = rand::thread_rng();
        self.liste_ennemy.push(Ennemi::new(rng.gen_range(50..=750) as f32, -20.0, 0.0, 2.0, 40, 40 ,0xffA5B6));
    }

    pub fn input(&mut self){
        self.player.parent.dx = 0.0;
        for key in self.window.get_keys() {
            match key {
                Key::Left => self.player.parent.dx = -5.0,
                Key::Right => self.player.parent.dx = 5.0,
                _ => {}  
            }
        }
        
        if self.window.is_key_pressed(Key::Space, KeyRepeat::No) && !self.space_pressed{
            self.player.tirer();
            self.space_pressed = true;
        }
        else {
            self.space_pressed = false;
        }
        
        self.player.update(self.width_screen);
    }

    pub fn update(&mut self){
        if self.frame_count%120 == 0{
            self.spawn_ennemy();
        }
        self.input();

        for tir in &mut self.player.liste_tirs {
            tir.parent.y -= 5.0;
        };
        for ennemy in &mut self.liste_ennemy{
            ennemy.parent.y += 2.0;
        }

        // ################## Gère les collisions #####################
        let mut tirs_a_supprimer: Vec<usize> = Vec::new();
        let mut ennemis_a_supprimer: Vec<usize> = Vec::new();
        
        for (i, tir) in self.player.liste_tirs.iter().enumerate() {
            for (j, ennemi) in self.liste_ennemy.iter().enumerate() {
                if tir.parent.collision(ennemi.parent) {
                    tirs_a_supprimer.push(i);
                    ennemis_a_supprimer.push(j);
                }
            }
        }
        
        self.player.liste_tirs.retain(|_, i| !tirs_a_supprimer.contains(&i));
        self.liste_ennemy.retain(|_, j| !ennemis_a_supprimer.contains(&j));
        

        self.player.liste_tirs.retain(|tir| tir.parent.y > 0.0);
        self.liste_ennemy.retain(|ennemy| ennemy.parent.y < self.height_screen as f32 + 10 as f32);
        self.frame_count += 1

    }

    pub fn draw(&mut self){
        self.player.parent.draw(&mut self.buffer, self.width_screen,self.player.parent.width,self.player.parent.height);
            //self.player.update(&self.window, &self.width);

            for tir in &self.player.liste_tirs {
                tir.parent.draw(&mut self.buffer, self.width_screen, tir.parent.width,tir.parent.height);
            }

            for ennemy in &self.liste_ennemy{
                ennemy.parent.draw(&mut self.buffer, self.width_screen, ennemy.parent.width,ennemy.parent.height);
            }
    }

    pub fn run(&mut self){
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();
            self.buffer = vec![0xFF0000; self.width_screen as usize * self.height_screen as usize];
            self.draw();
            self.window.update_with_buffer(&self.buffer, self.width_screen as usize, self.height_screen as usize).unwrap();
            
        }
    }

}

fn main() {

    //let player = Vaisseau {
    //    x: 50.0,
    //    y : 300.0,
    //    dx : 5.0,
    //    width : 50,
    //    height : 50,
    //};
    //let mut jeu_dans_main_vrai = Jeu::new(640, 360, 165, player, Vec::new());
    //jeu_dans_main_vrai.run();
    let mut player = Vaisseau::new(10.0, 550.0, 0.0, 0.0, 50, 50, 0x000000);
    let mut machin = Jeu::new(800,600,60,player);

    machin.run();
    
}
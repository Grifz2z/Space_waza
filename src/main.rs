use minifb::{Key, Window, WindowOptions, KeyRepeat};


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

    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: u32) {
        draw_rect(buf, self.x as u32, self.y as u32, width_screen, 10, 20, self.col)
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
    element: ElementGraphique,
    liste_tirs : Vec<Missile>,
}

impl Vaisseau {
    /* 
    pub fn new( x : f32, y : f32, dx : f32, dy : f32, width : u32, height : u32, col : u32,) -> Self {
        Vaisseau {
            element : ElementGraphique {
                x : x,
                y : y,
                dx : dx,   
                dy : dy,
                width : width,
                height : height,
                col : col,
            }
        } 
    }*/
    pub fn update(&mut self, width_screen: u32) {
        self.element.x += self.element.dx;

        if self.element.x < 0.0 {
            self.element.x = width_screen as f32;
        } else if self.element.x > width_screen as f32 {
            self.element.x = 0.0;
        }
    }

    pub fn tirer(&mut self){
        self.liste_tirs.push(Missile{x : self.player.x  + 20 as f32, y : self.player.y - 20 as f32, width : 10, height : 20});
    }

}

struct Ennemi {
    element: ElementGraphique,
}

impl Ennemi {
    /* 
    pub fn new( x : f32, y : f32, dx : f32, dy : f32, width : u32, height : u32, col : u32,) -> Self {
        Ennemi {
            element : ElementGraphique {
                x : x,
                y : y,
                dx : dx,   
                dy : dy,
                width : width,
                height : height,
                col : col,
            }
        } 
    }*/
    pub fn update(&mut self) {
        self.element.update();
    }
    pub fn collision(&self, other : ElementGraphique) -> bool{
        self.element.collision(other)
    }
}

struct Missile {
    element: ElementGraphique,
}

impl Missile {
    /* 
    pub fn new( x : f32, y : f32, dx : f32, dy : f32, width : u32, height : u32, col : u32,) -> Self {
        Missile {
            element : ElementGraphique {
                x : x,
                y : y,
                dx : dx,   
                dy : dy,
                width : width,
                height : height,
                col : col,
            }
        } 
    }*/
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
    pub fn new(width: u32, height: u32, fps: usize, player: Vaisseau, liste_ennemy : Vec<Ennemi>) -> Self {
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
            liste_ennemy,
        }
    }

    pub fn input(&mut self){
        self.player.update(self.width_screen);
        if self.window.is_key_pressed(Key::Space, KeyRepeat::No) && !self.space_pressed{
            //self.new_missile();
            self.space_pressed = true;
        }
        else {
            self.space_pressed = false;
        }
        if self.window.is_key_pressed(Key::Right, KeyRepeat::Yes){
            self.player.element.dx = 5.0;
        }
        else if self.window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            self.player.element.dx = -5.0;
        }
        else {
            self.player.element.dx = 5.0;
        }
    }

    pub fn update(&mut self){
        self.input();

        for tir in &mut self.player.liste_tirs {
            tir.element.y -= 5.0;
        };
        for ennemy in &mut self.liste_ennemy{
            ennemy.element.y += 2.0;
        }
        self.player.liste_tirs.retain(|tir| tir.element.y > 0.0);
        self.liste_ennemy.retain(|ennemy| ennemy.element.y < self.height_screen as f32 + 10 as f32);
        self.frame_count += 1

    }

    pub fn draw(&mut self){
        self.player.element.draw(&mut self.buffer, self.width_screen);
            //self.player.update(&self.window, &self.width);

            for tir in &self.player.liste_tirs {
                tir.element.draw(&mut self.buffer, self.width_screen);
            }

            for ennemy in &self.liste_ennemy{
                ennemy.element.draw(&mut self.buffer, self.width_screen);
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
}
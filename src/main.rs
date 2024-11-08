use minifb::{Key, Window, WindowOptions};
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
    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: &u32) {
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
    pub fn update(&mut self) {
        self.element.update();
    }
    pub fn collision(&self, other : ElementGraphique) -> bool{
        self.element.collision(other)
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
impl Ennemi {
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
    pub fn update(&mut self) {
        self.element.update();
    }
    pub fn collision(&self, other : ElementGraphique) -> bool{
        self.element.collision(other)
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
    println!("Waza !")
}
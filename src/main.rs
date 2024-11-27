use minifb::{Key, Window, WindowOptions, KeyRepeat};
use rand::{seq::index, Rng};
use std::{fs::File, path};
use std::io::BufReader;
use png::Decoder;

///Fonction permettant de dessiner les pyxel des coordonnées (x; y) à (x; y+width)
fn get_img_buff (path : &str) -> Vec<(u8, u8, u8, u8)> {

    // ouvrir et extraire le png
    let image = File::open(path).expect("waza erreur 1");
    let decodeur = Decoder::new(BufReader::new(image));
    let mut lu = decodeur.read_info().expect("waza erreur 2");
    let mut buf = vec![0; lu.output_buffer_size()];
    let info = lu.next_frame(&mut buf).expect("waza erreur 3");
    let bytes = &buf[..info.buffer_size()];

    let mut buffer_a_return: Vec<(u8, u8, u8, u8)> = Vec::new();

    for i in 0..bytes.len(){
        if i % 4 == 3 {
            let pixel: (u8, u8, u8, u8) = (
                bytes[i-3],  //R
                bytes[i-2],  //G
                bytes[i-1],  //B
                bytes[i],    //A 
                );

            buffer_a_return.push(pixel);
        }
    }

    return buffer_a_return;

}

fn f(x: u32, y: u32, w: u32) -> u32{ // permet de retrouver l'indice d'un pixel avec ses coordonnées
    return w*y +x;
}

fn g(i: u32, w: u32) -> (u32, u32) { // permet de retrouver un pixel avec son indice dans le buffer
    let coo_x = i%w;
    let coo_y= i/w; //div entière
    return (coo_x, coo_y);
}

fn reduce_buf(img_buf : &mut Vec<(u8, u8, u8, u8)>, u: u32, w_total: u32, w: u32, v: u32) -> Vec<(u8, u8, u8, u8)>{
    let indice_img_choisie: u32 = f(u,v, w_total);
    let mut nouveau_buffer_de_l_image = Vec::new();
    for y in 0..w{
        for x in 0..w{
            let coordonne : u32 = indice_img_choisie + f(x,y,w);
            nouveau_buffer_de_l_image.push(img_buf[coordonne as usize]);
        }
    }
    return nouveau_buffer_de_l_image;
}

///Fonction permettant de dessiner les pyxel des coordonnées (x; y) à (x; y+width)
fn draw_pixel(buf: &mut Vec<u32>, x: u32, y: u32, width: u32, colour : u32) {
    
    if x < width && y * width < buf.len() as u32 { // Calculer l'index uniquement si (x, y) est dans les limites du buffer
        let index = (y * width + x) as usize;
        buf[index] = colour; // Couleur noire
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn dessiner_img (buf: &mut Vec<u32>, w_screen : u32, x: u32, y: u32, w_img_buffer: u32, w: u32, u: u32, v: u32, path_: &str) {
    let mut img_buff: Vec<(u8, u8, u8, u8)> = get_img_buff(path_);
    let buffer_img_mais_le_vrai = reduce_buf(&mut img_buff, u, w_img_buffer, w, v);
    for i in 0..w{
        for j in 0..w{
            draw_pixel(buf, x+j, y+i, w_screen, from_u8_rgb(buffer_img_mais_le_vrai[f(j, i, w) as usize].0,buffer_img_mais_le_vrai[f(j, i, w) as usize].1, buffer_img_mais_le_vrai[f(j, i, w) as usize].2));
        }
    }
}

///Fonction permettant de dessiner un rectangle en partant des coordonnées (x; y) et d'une largeur ```width``` et hauteur ```height```
/// # Exemple
/// ```
/// let buffer = vec![0; (10 * 10) as usize];
/// draw_rect(buf, 2, 2, 10, 4, 4, self.col)
/// ```
///Cela dessinera un rectangle de 2 de longueur et de largeur aux coordonées (2; 2)
fn draw_rect(buf : &mut Vec<u32>, x : u32, y : u32, width_screen: u32, width : u32, height : u32, colour : u32) {
    for pos_y in y..(y+height){
        for pos_x in x..(x+width){
            draw_pixel(buf, pos_x, pos_y, width_screen, colour);
        }
    }
}

/// On crée un implémentation d'un élément graphique
/// L'élément graphique (un rectangle), doit avoir : deux coordonées (x; y), des vecteurs de déplacement dx et dy,
/// une longueur et une larguer ainsi qu'une couleur.
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
    /// Dessine le rectangle qui forme l'élément graphique
    pub fn draw(&self, buf : &mut Vec<u32>, width_screen: u32, w_img_buffer: u32,u:u32,v:u32 , path_ : &str) {
        dessiner_img(buf, width_screen, self.x as u32, self.y as u32, w_img_buffer, self.width, u, v, path_);
    }

    /// Met à jour l'élément graphique
    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy; 
    }

    /// Fonction qui gère les colisions entre deux éléments graphiques
    pub fn collision(&self, other : &ElementGraphique) -> bool  {
        return !(
               (other.x as u32 >= self.x as u32 + self.width)   
            || (other.x as u32 + other.width <= self.x as u32 )  
            || (other.y as u32 >= self.y as u32 + self.height ) 
            || (other.y as u32 + other.height <= self.y as u32)
        );
    }
}


/// On crée un implémentation d'un Vaisseau
/// Le Vaisseau (un rectangle) tient, en partie son état et ses comportements de l'ElementGraphique
struct Vaisseau {
    parent: ElementGraphique,
    liste_tirs : Vec<Missile>,
    vies : i32,
    vivant : bool,
}

impl Vaisseau {
    /// Crée un Vaissau à partir d'un parent (ElementGraphique)
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
            vies : 8,
            vivant : true,            
        } 
    }

    /// Met à jour le vaisseau selon ses propres conditions dans un espace thorique
    pub fn update(&mut self, width_screen: u32) {
        self.parent.x += self.parent.dx;

        if self.parent.x < 0.0 {
            self.parent.x = width_screen as f32;
        } else if self.parent.x > width_screen as f32 {
            self.parent.x = 0.0;
        }
    }

    /// Ajoute un objet de type missile avec certains attributs dans la liste des tirs du Vaisseau
    pub fn tirer(&mut self){
        self.liste_tirs.push(Missile::new(
            (self.parent.x + (self.parent.x+self.parent.width as f32))/2.0,
            self.parent.y - 5.0,
            0.0, 
            -5.0, 
            64, 
            64, 
            0xfff000
        ));
    }

}

/// On crée un implémentation d'un Ennemi
/// L'Ennemi (un rectangle) tient son état et ses comportements de l'ElementGraphique
struct Ennemi {
    parent: ElementGraphique,
}

impl Ennemi {
    /// Crée un Ennemi à partir d'un parent (ElementGraphique)
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


/// On crée un implémentation d'un Missile
/// Le Missile (un rectangle) tient son état et ses comportements de l'ElementGraphique
struct Missile {
    parent: ElementGraphique,
}

impl Missile {
    
    /// Crée un Missile à partir d'un parent (ElementGraphique)
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

/// La structure principale du jeu, c'est celle-ci qui gère toutes les fonctionnalités d'un jeu
/// 
/// # Exemple
/// 
/// ```
/// let player = Vaisseau::new(10.0, 550.0, 0.0, 0.0, 50, 50, 0x000000);
/// let mut app = Jeu::new(800,600,60,player);
/// app.run();
/// ```
struct Jeu {
    width_screen : u32,
    height_screen : u32,
    buffer : Vec<u32>,
    window : Window,
    fps : usize,
    space_pressed :bool,
    frame_count : u32,
    player : Vaisseau,
    liste_ennemis : Vec<Ennemi>
}

impl Jeu {
    /// Cree un jeu en prennant la taille de la fenêtre, les fps et un joueur
    pub fn new(width: u32, height: u32, fps: usize, player: Vaisseau) -> Self {

        // buffer qui gère l'ensemble des pixels de la fenêtre
        let buffer = vec![0; (width * height) as usize];

        // Création de la fenêtre de jeu
        let mut  window = Window::new(
            "Waza",
            width as usize,
            height as usize,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // On met les fps
        window.set_target_fps(fps);

        // Attributs
        Self {
            space_pressed : false,
            frame_count :  0,
            width_screen: width,
            height_screen: height,
            buffer,
            window,
            fps,
            player,
            liste_ennemis : Vec::new(),
        }
    }

    /// Fait apparaître un ennemi au dessus de la fenêtre à des absysses aléatoires
    pub fn spawn_ennemy(&mut self){
        let mut rng = rand::thread_rng();
        self.liste_ennemis.push(Ennemi::new(
            rng.gen_range(50..=750) as f32, 
            -20.0, 
            0.0, 
            2.0, 
            128, 
            128,
            0xffA5B6));
    }

    ///Fonction qui gère les entrées utilisateur du clavier
    pub fn input(&mut self){
        // Valeur de base du vecteur vitesse dx de player
        self.player.parent.dx = 0.0;

        // Mettre à jour le ecteur vitesse dx de player selon les touches appuiyées
        if self.window.is_key_down(Key::Left) {
            self.player.parent.dx = -5.0;
        } else if self.window.is_key_down(Key::Right){
            self.player.parent.dx = 5.0;
        }

        // Conditions pour tirer
        if self.window.is_key_pressed(Key::Space, KeyRepeat::No) && !self.space_pressed{
            self.player.tirer();
            self.space_pressed = true;
        }
        else {
            self.space_pressed = false;
        }
        
        self.player.update(self.width_screen);
    }

    /// Fonction qui gère la mise à jour l'ensemble des éléments du Jeu
    pub fn update(&mut self){
        self.frame_count += 1;

        if self.frame_count%50 == 0{
            self.spawn_ennemy();
        }
        self.input();

        for tir in &mut self.player.liste_tirs {
            tir.parent.y -= 5.0;
        };
        for ennemy in &mut self.liste_ennemis{
            ennemy.parent.y += 2.0;
        }
        self.check_collisions();
    }

    /// Truc bidule qui check les collision + ça casse le cerveau waza
    pub fn check_collisions(&mut self) {

        self.liste_ennemis.retain(|enn| {
            if !enn.parent.collision(&self.player.parent){
                return true;
            }
            self.player.vies -= 1;
            false
        });

        self.liste_ennemis.retain(|enn| {
            let mut non_enlever_ennemi = true;
            if enn.parent.y == self.height_screen as f32{
                non_enlever_ennemi = false;
                self.player.vies -= 1;
            }
            self.player.liste_tirs.retain(|tir| {
                if tir.parent.collision(&enn.parent) {
                    non_enlever_ennemi = false;
                    return false;
                }
                if tir.parent.y == 0.0 {
                    false
                }
                else {
                    true
                }
            });
            non_enlever_ennemi
        });

    }

    /// Dessine l'ensemble des choses présentes dans le jeu
    pub fn draw(&mut self){
        self.player.parent.draw(&mut self.buffer, self.width_screen, 126, 0, 0, "assets/spaceship.png");

        for tir in &self.player.liste_tirs {
            tir.parent.draw(&mut self.buffer, self.width_screen, 64, 0, 0, "assets/capybara_xs.png");
        }

        for ennemy in &self.liste_ennemis{
            ennemy.parent.draw(&mut self.buffer, self.width_screen, 200, 0, 0, "assets/capybara.png");
        }

        for i in 0..self.player.vies{
            draw_rect(&mut self.buffer, 10+i as u32*60, self.height_screen-50, self.width_screen, 30, 30,0xff0000);
        }
    }

    /// Fonction qui execute les fonctions du Jeu par une boucle
    pub fn run(&mut self){
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();
            self.buffer = vec![0x1355c1; self.width_screen as usize * self.height_screen as usize];
            self.draw();
            self.window.update_with_buffer(&self.buffer, self.width_screen as usize, self.height_screen as usize).unwrap();
            
        }
    }

}

fn main() {

    let player = Vaisseau::new(10.0, 800.0, 0.0, 0.0, 128, 128, 0xd6270f);
    let mut app = Jeu::new(1600,1000,165,player);

    app.run();
    
}
use std::{fs::File, path};
use std::io::BufReader;
use png::Decoder;
use minifb::{Window, WindowOptions,Key};

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

fn dessiner_img (buf: &mut Vec<u32>, w_screen : u32, x: u32, y: u32, w_total: u32, w: u32, u: u32, v: u32, path_: &str) {
    let mut img_buff: Vec<(u8, u8, u8, u8)> = get_img_buff(path_);
    let buffer_img_mais_le_vrai = reduce_buf(&mut img_buff, u, w_total, w, v);
    for i in 0..w{
        for j in 0..w{
            draw_pixel(buf, x+j, y+i, w_screen, from_u8_rgb(buffer_img_mais_le_vrai[f(j, i, w) as usize].0,buffer_img_mais_le_vrai[f(j, i, w) as usize].1, buffer_img_mais_le_vrai[f(j, i, w) as usize].2));
        }
    }
}

fn main() {

    let mut buffer = vec![0; (128 * 128) as usize];

    let mut  window = Window::new(
        "Waza",
        128,
        128,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    dessiner_img(&mut buffer, 128, 50, 50, 16, 16, 0, 0, "assets/low_res.png");
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&mut buffer, 128 as usize, 128 as usize).unwrap();
        
    }
}
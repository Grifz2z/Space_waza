use std::fs::File;
use std::io::BufReader;
use png::Decoder;

fn show_img(png_path : &str, buffer :&mut Vec<u32>, x : u32, y : u32){
    let file = File::open(png_path).expect("Waza erreur 1");
    let reader = BufReader::new(file);

    let decoder = Decoder::new(reader);
    let mut reader = decoder.read_info().expect("Waza erreur 2");

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("Waza erreur 3");

    for i in 0..len(buf){
        for j in 0..len(buf[0]){
            buffer[x+i][y+j] = buf[i][j]
        }
    }
}
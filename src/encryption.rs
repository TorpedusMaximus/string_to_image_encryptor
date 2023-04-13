pub mod encryptor {
    use std::{vec, path::Path, fs};

    use image::{GrayImage, Luma};

    pub fn encrypt() {
        let mut line = String::new();
        println!("Enter string to encrypt :");

        std::io::stdin().read_line(&mut line).unwrap();
        let path = line.trim();

        if Path::new(path).is_file(){
            println!("File {:?} found",line);
            let  contents= fs::read_to_string(path).expect("Could not read file");
            line = String::new();
            line.push_str(&contents);
        }

        let line_length = line.len() as u32;
        let mut counter = 1u32;

        loop {
            counter += 1;

            if counter * counter >= line_length {
                break;
            }
        }

        let n = counter;
        let exact_h = f64::from(line_length) / f64::from(n);
        let h = exact_h.ceil() as u32;
        let image_size = (n * h) as usize;

        let mut img = GrayImage::new(n, h);
        let mut pixels_values = vec![Luma([0u8]); image_size];

        for (index, character) in line.chars().enumerate() {
            let character_in_ascii = character as u8;
            pixels_values[index] = Luma([character_in_ascii]);
        }

        let mut pixel_id = 0;

        for x in 0..h {
            for y in 0..n {
                let pixel = pixels_values[pixel_id];
                img.put_pixel(y, x, pixel);
                pixel_id += 1;
            }
        }

        let mut path = String::new();
        println!("Path to image :");
        std::io::stdin().read_line(&mut path).unwrap();
        path.remove(path.len() - 1);
        if path.is_empty() {
            path.push_str("image.png");
        }

        let _result = img.save(path);
    }
}

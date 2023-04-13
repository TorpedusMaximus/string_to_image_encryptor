pub mod encryptor {
    use std::{fs, path::Path, vec};

    use image::{GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
    fn rgb(line: String) -> RgbImage {
        let mut line_length = line.len() as u32;
        let rgb_pixel_number = (line_length + (line_length % 3)) / 3;

        let mut counter = 1u32;

        loop {
            counter += 1;

            if counter * counter >= rgb_pixel_number {
                break;
            }
        }
        let n = counter;
        let exact_h = f64::from(rgb_pixel_number) / f64::from(n);
        let h = exact_h.ceil() as u32;
        let image_size = (n * h) as usize;

        let mut img = RgbImage::new(n, h);
        let mut pixels_values = vec![Rgb([0u8, 0u8, 0u8]); image_size];

        for index in 0..rgb_pixel_number as usize {
            let char1 = line.chars().nth(3 * index).unwrap() as u8;
            let char2 = line.chars().nth(3 * index + 1).unwrap() as u8;
            let char3 = line.chars().nth(3 * index + 2).unwrap() as u8;

            let rgb_pixel = Rgb([char1, char2, char3]);
            pixels_values[index] = rgb_pixel;
        }

        let mut pixel_id = 0;

        for x in 0..h {
            for y in 0..n {
                let pixel = pixels_values[pixel_id];
                img.put_pixel(y, x, pixel);
                pixel_id += 1;
            }
        }

        return img;
    }
    fn grayscale(line: String) -> GrayImage {
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

        return img;
    }

    pub fn encrypt() {
        let mut line = String::new();
        println!("Enter string to encrypt :");

        std::io::stdin().read_line(&mut line).unwrap();
        let mut path = line.trim();
        if path.is_empty(){
            path = "text.txt";
        }


        if Path::new(path).is_file(){
            println!("File {:?} found", path);
            let contents = fs::read_to_string(path).expect("Could not read file");
            line = String::new();
            line.push_str(&contents);
        }

        let mut if_compressed = String::new();
        println!("Compress to RGB (y/n) :");
        std::io::stdin().read_line(&mut if_compressed).unwrap();
        let path = if_compressed.trim();

        let mut compressed=true;

        if if_compressed.contains('y'){
            compressed=true;
        }else{
            compressed=false;
        }

        if compressed {
            let img = rgb(line);
            let mut path = String::new();
            println!("Path to image :");
            std::io::stdin().read_line(&mut path).unwrap();
            path.remove(path.len() - 1);
            if path.is_empty() {
                path.push_str("image.png");
            }
    
            let _result = img.save(path);
        } else {
            let img = grayscale(line);
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
}

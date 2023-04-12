pub mod decryptor {
    use image::open;

    pub fn decrypt() {
        let mut path = String::new();
        println!("Path to image :");
        std::io::stdin().read_line(&mut path).unwrap();
        path.remove(path.len()-1);
        if path.is_empty() {
            path.push_str("image.png");
        }


        let mut image_values = open(path).unwrap().into_luma8().to_vec();
        image_values.reverse();

        for i in 0..image_values.len() {
            if image_values[i] != 0 {
                image_values.splice(0..(i), Vec::new()); // possibly i-1 ??????
                break;
            }
        }
        image_values.reverse();

        let mut line = String::new();

        for pixel in image_values {
            let character = pixel as char;
            line.push(character);
        }

        println!("{:?}", line)
    }
}

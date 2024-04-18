pub mod typer {

    use core::time;
    use enigo::*;
    use rand::{self, Rng};
    use std::{
        fs::{self, File},
        thread::sleep,
    };

    fn get_file_input(file_path: &String) -> Vec<char> {
        if File::open(&file_path).is_err() {
            let _ = File::create(file_path).unwrap();
        }

        let text = fs::read_to_string(file_path).unwrap();
        let chars: Vec<char> = text.chars().collect();

        chars
    }

    fn simulate_keypress(key: char) {
        let mut eng = Enigo::new();

        match key {
            '?' => eng.key_click(enigo::Key::Layout('?')),
            '"' => eng.key_click(enigo::Key::Layout('"')),
            '.' => eng.key_click(enigo::Key::Layout('.')),
            ',' => eng.key_click(enigo::Key::Layout(',')),
            ' ' => eng.key_click(enigo::Key::Space),
            'á' => eng.key_click(enigo::Key::Layout('á')),
            'é' => eng.key_click(enigo::Key::Layout('é')),
            'č' => eng.key_click(enigo::Key::Layout('č')),
            'ď' => eng.key_click(enigo::Key::Layout('ď')),
            'ě' => eng.key_click(enigo::Key::Layout('ě')),
            'í' => eng.key_click(enigo::Key::Layout('í')),
            'ó' => eng.key_click(enigo::Key::Layout('ó')),
            'ř' => eng.key_click(enigo::Key::Layout('ř')),
            'š' => eng.key_click(enigo::Key::Layout('š')),
            'ť' => eng.key_click(enigo::Key::Layout('ť')),
            'ú' => eng.key_click(enigo::Key::Layout('ú')),
            'ů' => eng.key_click(enigo::Key::Layout('ů')),
            'ý' => eng.key_click(enigo::Key::Layout('ý')),
            'ž' => eng.key_click(enigo::Key::Layout('ž')),
            _ => eng.key_click(Key::Layout(key)),
        }
    }

    fn typer(text: Vec<char>) {
        let mut rng = rand::thread_rng();
        sleep(time::Duration::from_millis(1000));

        for key in text {
            sleep(time::Duration::from_millis(rng.gen_range(100..=1000)));
            print!("{}", key);
            simulate_keypress(key)
        }
    }


    pub fn start_typing(){
        let text = get_file_input(&"test.txt".to_string());

        typer(text);
    }
}

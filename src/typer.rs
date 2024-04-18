pub mod typer {

    use core::time;
    use enigo::*;
    use rand::{self, Rng};
    use std::{
        fs::{self, File},
        thread::sleep,
    };

    trait Convert {
        fn from_chars<'a>(text: Vec<Chars>) -> Text;
        fn from_char(chrs: Vec<char>) -> Text;
    }

    pub enum Col {
        Typed,
        NotTyped,
    }

    #[derive(impl_new::New)]
    pub struct Text {
        text: Vec<Chars>,
    }
    
    #[derive(impl_new::New)]
    struct Chars {
        key: char,
        col: Col,
    }

    impl Convert for Chars {
        fn from_chars<'a>(text: Vec<Chars>) -> Text {
            let txt: Text = Text::new(text);

            txt
        }

        fn from_char(chrs: Vec<char>) -> Text {
            let txt: Vec<Chars> = chrs
                .iter()
                .map(|ch| Chars {
                    key: *ch,
                    col: Col::NotTyped,
                })
                .collect();

            let text: Text = Text::new(txt);

            text
        }
    }

    pub fn get_file_input(file_path: &String) -> Text {
        if File::open(&file_path).is_err() {
            let _ = File::create(file_path).unwrap();
        }

        let text = fs::read_to_string(file_path).unwrap();
        let chars: Vec<char> = text.chars().collect();

        let chrs: Text = Chars::from_char(chars);

        chrs
    }

    fn simulate_keypress(key: char) {
        let mut eng = Enigo::new();

        match key {
            '?' => eng.key_click(enigo::Key::Layout('?')),
            '"' => eng.key_click(enigo::Key::Layout('"')),
            '.' => eng.key_click(enigo::Key::Layout('.')),
            ',' => eng.key_click(enigo::Key::Layout(',')),
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
            ' ' => eng.key_click(enigo::Key::Space),
            _ => eng.key_click(Key::Layout(key)),
        }
    }

    fn typer(text: Text) {
        let mut rng = rand::thread_rng();
        sleep(time::Duration::from_millis(1000));

        for key in text.text {
            sleep(time::Duration::from_millis(rng.gen_range(100..=1000)));
            simulate_keypress(key.key)
        }
    }

    pub fn start_typing() {
        let text = get_file_input(&"test.txt".to_string());

        typer(text);
    }
}

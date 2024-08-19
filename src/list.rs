use std::{io::Cursor, ops::Index};

use bimap::BiHashMap;
use rand::Rng;

pub struct List {
    // The pokedex id's & the corresponding filenames.
    ids: BiHashMap<usize, String>,

    /// Key is the file name, Value is the formatted name.
    names: Vec<String>,
}

impl<'a> Index<usize> for List {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        self.ids.get_by_left(&index).unwrap()
    }
}

impl<'a> Index<&String> for List {
    type Output = usize;

    fn index(&self, index: &String) -> &Self::Output {
        self.ids.get_by_right(index).unwrap()
    }
}

impl List {
    pub fn read() -> Self {
        let file: &'static str = include_str!("../data/names.csv");

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(file));

        const CAPACITY: usize = 1000;

        let mut ids = BiHashMap::with_capacity(CAPACITY);
        let mut names = Vec::with_capacity(CAPACITY);

        for (i, entry) in reader.deserialize().enumerate() {
            let record: (String, String) = entry.unwrap();

            ids.insert(i, record.1);
            names.push(record.0);
        }

        Self { ids, names }
    }

    pub fn format_name(&self, filename: &String) -> String {
        self.names[self[filename]].clone()
    }

    pub fn random(&self) -> String {
        let mut rand = rand::thread_rng();

        let idx = rand.gen_range(0..self.ids.len());
        self[idx].clone()
    }
}

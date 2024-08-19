use std::{io::Cursor, ops::Index};

use bimap::BiHashMap;
use rand::Rng;

pub struct List {
    // The Pokedex IDs and their corresponding filenames.
    ids: BiHashMap<usize, String>,

    /// All the proper, formatted names in order of Pokedex ID.
    names: Vec<String>,
}

impl<'a> Index<usize> for List {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        self.ids.get_by_left(&index).unwrap()
    }
}

impl<'a> Index<&str> for List {
    type Output = usize;

    fn index(&self, index: &str) -> &Self::Output {
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

    pub fn format_name(&self, filename: &str) -> String {
        // Unfortunately, there needs to be a clone here because all the data is owned here in this struct.
        // It's also impossible to pass references because then we would have to make it static,
        // And consider that not all filenames may originate from the file.
        self.names[self[filename]].clone()
    }

    pub fn random(&self) -> String {
        let mut rand = rand::thread_rng();

        let idx = rand.gen_range(0..self.ids.len());

        // See `format_name` for information about why the clone is here.
        self[idx].clone()
    }
}
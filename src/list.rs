#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use std::io::Cursor;

use bimap::BiHashMap;
use inflector::Inflector;
use rand::Rng;

/// A parsed representation of `names.csv`.
///
/// Used to derive filenames from Pokedex ID's, and to
/// format image filenames back into proper pokemon names.
pub struct List {
    /// The Pokedex IDs and their corresponding filenames.
    ids: BiHashMap<usize, String>,
    /// All the proper, formatted names in order of Pokedex ID.
    names: Vec<String>,
    /// The language-specific names mapping
    lang_names: Vec<String>,
}

impl List {
    pub fn read(lang: &str) -> Self {
        const FILE_EN: &str = include_str!("../data/names.csv");
        const FILE_DE: &str = include_str!("../data/names_de.csv");
        
        let file = match lang {
            "de" => FILE_DE,
            _ => FILE_EN,
        };

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(file));

        // Also read the English names for sprite lookups
        let mut en_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(FILE_EN));

        const CAPACITY: usize = 1000;

        let mut ids = BiHashMap::with_capacity(CAPACITY);
        let mut names = Vec::with_capacity(CAPACITY);
        let mut lang_names = Vec::with_capacity(CAPACITY);

        // Read English names and IDs first
        for (i, entry) in en_reader.deserialize().enumerate() {
            let record: (String, String) = entry.unwrap();
            ids.insert(i, record.1);
            names.push(record.0);
        }

        // Read language-specific names
        for entry in reader.deserialize() {
            let record: (String, String) = entry.unwrap();
            lang_names.push(record.0);
        }

        Self { ids, names, lang_names }
    }

    /// Gets a pokemon filename by a Dex ID.
    pub fn get_by_id(&self, id: usize) -> Option<&String> {
        self.ids.get_by_left(&id)
    }

    /// Gets the English name for a given name in another language
    pub fn get_english_name(&self, foreign_name: &str) -> Option<String> {
        // Find the index of the foreign name in lang_names
        if let Some(idx) = self.lang_names.iter().position(|name| name.to_lowercase() == foreign_name.to_lowercase()) {
            // Return the English name at the same index
            self.names.get(idx).cloned()
        } else {
            None
        }
    }

    /// Gets a random pokemon & returns it's filename.
    pub fn random(&self) -> String {
        let mut rand = rand::thread_rng();

        let idx = rand.gen_range(0..self.ids.len());
        self.ids.get_by_left(&idx).unwrap().clone()
    }

    pub fn format_name(&self, filename: &str) -> String {
        let raw_fmt = |x: &str| x.replace('-', " ").replace('\'', "").to_title_case();

        let Some(id) = self.ids.get_by_right(filename) else {
            return raw_fmt(filename);
        };
        let Some(name) = self.lang_names.get(*id) else {
            return raw_fmt(filename);
        };

        name.clone()
    }
}

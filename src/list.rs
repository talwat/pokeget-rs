use std::{io::Cursor, ops::Index};

use bimap::BiHashMap;
use rand::Rng;

/// A parsed representation of `names.csv`.
///
/// Used to derive filenames from Pokedex ID's, and to
/// format image filenames back into proper pokemon names.
pub struct List {
    // The Pokedex IDs and their corresponding filenames.
    ids: BiHashMap<usize, String>,

    /// All the proper, formatted names in order of Pokedex ID.
    names: Vec<String>,
}

impl Index<usize> for List {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        self.ids.get_by_left(&index).unwrap()
    }
}

impl Index<&str> for List {
    type Output = usize;

    fn index(&self, index: &str) -> &Self::Output {
        self.ids.get_by_right(index).unwrap()
    }
}

impl List {
    /// Reads a new [`List`] from `data/names.csv`.
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

    /// Takes a filename and looks up the proper display name.
    ///
    /// # Examples
    ///
    /// ```
    /// use pokeget::list::List;
    /// let list = List::read();
    /// assert_eq!(list.format_name("mr-mime"), "Mr. Mime")
    /// ```
    pub fn format_name(&self, filename: &str) -> String {
        // Unfortunately, there needs to be a clone here because all the data is owned here in this struct.
        // It's also impossible to pass references because then we would have to make it static,
        // And consider that not all filenames may originate from the file.
        self.names[self[filename]].clone()
    }

    /// Gets a random pokemon & returns it's filename.
    pub fn random(&self) -> String {
        let mut rand = rand::thread_rng();

        let idx = rand.gen_range(0..self.ids.len());

        // See `format_name` for information about why the clone is here.
        self[idx].clone()
    }
}

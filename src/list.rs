#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use std::io::Cursor;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::pokemon::Region;
use bimap::BiHashMap;

/// A parsed representation of `names.csv`.
///
/// Used to derive filenames from Pokedex ID's, and to
/// format image filenames back into proper pokemon names.
pub struct List {
    /// The Pokedex IDs and their corresponding filenames.
    ids: BiHashMap<usize, String>,

    /// All the proper, formatted names in order of Pokedex ID.
    names: Vec<String>,

    /// Set of banned pokemon filenames that should not appear in random selections
    banlist: HashSet<String>,
}

impl List {
    /// Reads the banlist from ~/.config/pokeget/banlist
    fn read_banlist() -> HashSet<String> {
        let mut banlist = HashSet::new();

        // Get the config directory path
        let config_path = if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/pokeget/banlist")
        } else if let Ok(home) = std::env::var("USERPROFILE") {
            // Windows support
            PathBuf::from(home).join(".config/pokeget/banlist")
        } else {
            return banlist;
        };

        // Read the banlist file if it exists
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    // Normalize the name to match filename format
                    let normalized = trimmed
                        .replace([' ', '_'], "-")
                        .replace(['.', '\'', ':'], "")
                        .to_lowercase();
                    banlist.insert(normalized);
                }
            }
        }

        banlist
    }

    /// Reads a new [`List`] from `data/names.csv`.
    pub fn read() -> Self {
        const FILE: &'static str = include_str!("../data/names.csv");

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(FILE));

        const CAPACITY: usize = 1000;

        let mut ids = BiHashMap::with_capacity(CAPACITY);
        let mut names = Vec::with_capacity(CAPACITY);

        for (i, entry) in reader.deserialize().enumerate() {
            let record: (String, String) = entry.unwrap();

            ids.insert(i, record.1);
            names.push(record.0);
        }

        let banlist = Self::read_banlist();

        Self { ids, names, banlist }
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
        let Some(id) = self.ids.get_by_right(filename) else {
            return filename.to_owned();
        };

        let Some(name) = self.names.get(*id) else {
            return filename.to_owned();
        };

        name.clone()
    }

    /// Gets a pokemon filename by a Dex ID.
    pub fn get_by_id(&self, id: usize) -> Option<&String> {
        self.ids.get_by_left(&id)
    }

    /// Saves a pokemon filename to ~/.config/pokeget/last-name
    pub fn save_last_name(filename: &str) -> std::io::Result<()> {
        let config_path = if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/pokeget")
        } else if let Ok(home) = std::env::var("USERPROFILE") {
            // Windows support
            PathBuf::from(home).join(".config/pokeget")
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine home directory",
            ));
        };

        // Create the config directory if it doesn't exist
        std::fs::create_dir_all(&config_path)?;

        let last_name_path = config_path.join("last-name");
        std::fs::write(last_name_path, filename)?;

        Ok(())
    }

    /// Gets a random pokemon & returns it's filename.
    pub fn random(&self) -> String {
        loop {
            let idx = rand::random_range(0..self.ids.len());
            let pokemon = self.ids.get_by_left(&idx).unwrap();

            if !self.banlist.contains(pokemon.as_str()) {
                return pokemon.clone();
            }
        }
    }

    /// Gets a random pokemon by region
    pub fn get_by_region(&self, region: Region) -> String {
        let region_range = match region {
            Region::Kanto => 0..=151,
            Region::Johto => 152..=251,
            Region::Hoenn => 252..=386,
            Region::Sinnoh => 387..=493,
            Region::Unova => 494..=649,
            Region::Kalos => 650..=721,
            Region::Alola => 722..=809,
            Region::Galar => 810..=905,
        };

        loop {
            let idx = rand::random_range(region_range.clone());
            let pokemon = self.ids.get_by_left(&idx).unwrap();

            if !self.banlist.contains(pokemon.as_str()) {
                return pokemon.clone();
            }
        }
    }
}

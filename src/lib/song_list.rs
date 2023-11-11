use crate::song::Song;
use crate::print_arrow;
use csv::StringRecord;
use rand::{thread_rng, seq::SliceRandom};
use tabled::Table;
use std::{error::Error, ffi::OsString, fs::File};

/// Represents a list of songs
#[derive(Clone)]
pub struct SongList {
    songs: Vec<Song>
}

impl SongList {
    /// Creates a new song list
    pub fn new() -> Self {
        SongList {
            songs: Vec::new()
        }
    }

    /// Getter for field 'songs'
    pub fn songs(&self) -> &Vec<Song> {
        &self.songs
    }

    /// Reads song data from a CSV file
    pub fn read_song_data(&mut self, file_path: OsString) -> Result<(), Box<dyn Error>> {
        let file = File::open(&file_path)?;
        print_arrow!("Reading song data from: {}", 
                     file_path.to_string_lossy());

        // Read, extract and add the data from the CSV file
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.records() {
            let record = result?;
            let song_data = self.extract_song_data(&record);
            self.add(song_data);
        }
        Ok(())
    }

    /// Extracts the data of a song from a CSV record
    pub fn extract_song_data(&self, record: &StringRecord) -> Song {
        print_arrow!("Extracting song data of: {}", &record[0]);

        // Record order: Name,Artist,Album,Year,Length 
        let song_name = &record[0];
        let song_artist = &record[1];
        let song_album = &record[2];
        let song_year = match record[3].parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                print_arrow!("Failed to parse integer");
                0
            },
        };
        let song_length = match record[4].parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                print_arrow!("Failed to parse float");
                0.0
            }
        };

        Song::new(song_name.to_string(),
            song_artist.to_string(),
            song_album.to_string(),
            song_year,
            song_length)
    }

    /// Shuffles a song list
    pub fn shuffle(&mut self) {
        print_arrow!("Shuffling song list...");

        // Create RNG and shuffle the list
        let mut rng = thread_rng();
        self.songs.shuffle(&mut rng);

        print_arrow!("Song list shuffled!");
    }

    /// Adds a song to a song list
    pub fn add(&mut self, song: Song) {
        print_arrow!("Adding song '{}' to song list", song.name());
        self.songs.push(song);
    }

    /// Removes a song from a song list
    pub fn remove(&mut self, song_name: &str) -> Result<(), String> {
        if let Some(index) = self.songs.
                             iter()
                             .position(|s| s.name() == song_name) {
            print_arrow!("Removing song '{}' from song list", song_name);
            self.songs.remove(index);
            Ok(())
        } else {
            Err(format!("Song '{}' was not found from song list", song_name))
        }
    }


    /// Finds a song from a song list
    // Maybe of a future use?
    pub fn find(&self, song_name: &str) -> Option<&Song> {
        print_arrow!("Trying to find song '{}' from song list", song_name);
        self.songs.iter().find(|s| s.name() == song_name)
    }

    /// Lists all songs in a song list
    // Maybe of a future use?
    pub fn list(&self) {
        print_arrow!("Listing all songs in the song list");
        let mut iter = self.songs.iter().peekable();

        while let Some(song) = iter.next() {
            print_arrow!("{} - {} - {}", 
                         song.name(),
                         song.artist(), 
                         song.album());
        }
    }

    /// Lists all songs in a song list as a table
    pub fn list_as_table(&self) {
        let table = Table::new(&self.songs).to_string();
        println!("{}", table.as_str());
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;
    use std::ffi::OsString;
    use csv::StringRecord;
    use super::SongList;
    use crate::song::Song;

    #[test]
    fn test_find_found_song() {
        let mut song_list = SongList::new();
        let song = Song::new("The Killchain".to_string(), 
                             "Bolt Thrower".to_string(), 
                             "Those Once Loyal".to_string(), 
                             2005, 
                             4.41);
        song_list.add(song);
 
        if let Some(found_song) = song_list.find("The Killchain") {
            assert_eq!(found_song.name(), 
                       "The Killchain", 
                       "Song name does not match");
            assert_eq!(found_song.artist(), 
                       "Bolt Thrower", 
                       "Song artist does not match");
            assert_eq!(found_song.album(), 
                       "Those Once Loyal", 
                       "Song album does not match");
            assert_eq!(found_song.year(), 
                       2005, 
                       "Song year does not match");
            assert!((found_song.length() - 4.41).abs() < f64::EPSILON, 
                    "Song length does not match");
        } else {
            panic!("The song 'The Killchain' was not found");
        }
    }

    #[test]
    fn test_find_not_found_song() {
        let song_list = SongList::new();
        assert_eq!(song_list.find("The Killchain"), None, "Song was found");
    }

    #[test]
    fn test_extract_song_data() {
        let record = StringRecord::from(vec![
            "The Killchain", 
            "Bolt Thrower", 
            "Those Once Loyal", 
            "2005", 
            "4.41"
        ]);

        let song_list = SongList::new();
        let song = song_list.extract_song_data(&record);

        assert_eq!(song.name(), 
                   "The Killchain", 
                   "Song name does not match");
        assert_eq!(song.artist(), 
                   "Bolt Thrower", 
                   "Song artist does not match");
        assert_eq!(song.album(), 
                   "Those Once Loyal", 
                   "Song album does not match");
        assert_eq!(song.year(), 
                   2005,
                   "Song year does not match");
        assert!((song.length() - 4.41).abs() < f64::EPSILON, 
                "Song length does not match");
    }

    #[test]
    fn test_read_song_data_valid_path() {
        // Create a temporary file containing song metadata for this test
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, 
                "Name,Artist,Album,Year,Length")
                .unwrap();
        writeln!(temp_file, 
                "The Killchain,Bolt Thrower,Those Once Loyal,2005,4.41")
                .unwrap();

        let mut song_list = SongList::new();
        let test_data_path = temp_file.path().to_str().unwrap();

        assert!(song_list.read_song_data(test_data_path.into())
                .is_ok(), 
                "A path to a song list was invalid");
        assert_eq!(song_list.songs().len(), 
                   1, 
                   "Song list is empty");
        assert_eq!(song_list.songs()[0].name(), 
                   "The Killchain", 
                   "Song name does not match");
        assert_eq!(song_list.songs()[0].artist(), 
                   "Bolt Thrower", 
                   "Song artist does not match");
        assert_eq!(song_list.songs()[0].album(), 
                   "Those Once Loyal", 
                   "Song album does not match");
        assert_eq!(song_list.songs()[0].year(), 
                   2005, 
                   "Song year does not match");
        assert!((song_list.songs()[0].length() - 4.41).abs() < f64::EPSILON, 
                "Song length does not match");
    }

    #[test]
    fn test_read_song_data_invalid_path() {
        let mut song_list = SongList::new();
        assert!(song_list.read_song_data(OsString::from(""))
                .is_err(), 
                "A path to a song list was valid");
    }

    #[test]
    fn test_add() {
        let mut song_list = SongList::new();
        let song = Song::new("The Killchain".to_string(), 
                             "Bolt Thrower".to_string(), 
                             "Those Once Loyal".to_string(), 
                             2005, 
                             4.41);

        song_list.add(song);
        assert_eq!(song_list.songs().len(), 1, "Song list is empty");
    }

    #[test]
    fn test_remove_valid_song() {
        let mut song_list = SongList::new();
        let song = Song::new("The Killchain".to_string(), 
                             "Bolt Thrower".to_string(),
                             "Those Once Loyal".to_string(), 
                             2005, 
                             4.41);
        song_list.add(song);
        let song = Song::new("The Killchain".to_string(), 
                             "Bolt Thrower".to_string(), 
                             "Those Once Loyal".to_string(), 
                             2005, 
                             4.41);

        assert!(song_list.remove(song.name())
                .is_ok(), 
                "Removing a valid song should be succesfull");
        assert!(song_list.find(song.name())
                .is_none(), 
                "After remova, the song should not be found");
    }

    #[test]
    fn test_remove_invalid_song() {
        let mut song_list = SongList::new();
        assert!(song_list.remove("The Killchain")
                .is_err(), 
                "Invalid song should return an error");
    }

    #[test]
    fn test_shuffle() {
        let mut song_list = SongList::new();
        
        let song = Song::new("The Killchain".to_string(), 
                             "Bolt Thrower".to_string(), 
                             "Those Once Loyal".to_string(), 
                             2005, 
                             4.41);
        song_list.add(song);
        let song2 = Song::new("You Only Live Once".to_string(), 
                              "Suicide Silence".to_string(), 
                              "The Black Crown".to_string(), 
                              2011, 
                              3.13);
        song_list.add(song2);
        let song3 = Song::new("Pull the Plug".to_string(), 
                              "Death".to_string(),
                              "Leprosy".to_string(), 
                              1988, 
                              4.27);
        song_list.add(song3);
        let song4 = Song::new("Tyende Sang".to_string(), 
                              "Afsky".to_string(), 
                              "Ofte Jeg Drømmer Mig Død".to_string(), 
                              2020, 
                              8.39);
        song_list.add(song4);

        let mut shuffled_song_list = song_list.clone();

        shuffled_song_list.shuffle();
        assert_ne!(song_list.songs(), 
                   shuffled_song_list.songs(), 
                   "Song list should be shuffled and different from the original");
    }
}

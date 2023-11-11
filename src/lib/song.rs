use tabled::Tabled;

/// Represents the metadata of a song
#[derive(Tabled, Debug, Clone, PartialEq)]
pub struct Song {
    name: String,
    artist: String,
    album: String,
    year: u32,
    length: f64
}


impl Song {
    /// Creates a new song
    pub fn new(name: String, artist: String, album: String, year: u32, length: f64) -> Self {
        Song {
            name,artist,album,year,length
        }
    }

    /// Getter for field 'name'
    pub fn name(&self) -> &str {
        &self.name
    }
        
    /// Getter for field 'artist'
    pub fn artist(&self) -> &str {
        &self.artist
    }

    /// Getter for field 'album'
    pub fn album(&self) -> &str {
        &self.album
    }

    /// Getter for field 'year'
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Getter for field 'length'
    pub fn length(&self) -> f64 {
        self.length
    }
}

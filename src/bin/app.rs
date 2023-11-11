use std::{process, env, ffi::OsString};
use songlist_shuffler::print_arrow;
use songlist_shuffler::song_list::SongList;

/// Parse file path from command-line arguments
fn parse_file_path_from_args(args: Vec<String>) -> Option<OsString> {
    match args.get(0) {
        Some(arg) => Some(OsString::from(arg)),
        None => None,
    }
}

fn main() {
    // Parse command-line arguments to get the path to a song list csv file
    let args = env::args().skip(1).collect::<Vec<_>>();
    let file_path: OsString = match parse_file_path_from_args(args) {
        Some(fp) => fp,
        None => {
            print_arrow!("Expected a path to a song list, but got none");
            process::exit(1);
        },
    };

    // Create a new empty song list and read the song data from the given
    // song list csv file to it
    let mut song_list = SongList::new();
    if let Err(err) = song_list.read_song_data(file_path) {
        print_arrow!("{}", err);
        process::exit(1);
    }

    println!("\n");

    // Shuffle the song list to get a random song list order
    song_list.shuffle();

    // List all the songs in the list
    song_list.list_as_table();
}

#[cfg(test)]
mod tests {
    use super::parse_file_path_from_args;
    use std::ffi::OsString;

    #[test]
    fn test_parse_file_path_from_args_valid_path() {
        let args: Vec<String> = vec!["song_list.csv".to_string()];
        assert_eq!(parse_file_path_from_args(args), 
                   Some(OsString::from("song_list.csv")),
                   "Expected a valid path, but got invalid");
    }

    #[test]
    fn test_parse_file_path_from_args_invalid_path() {
        let args: Vec<String> = Vec::new();
        assert_eq!(parse_file_path_from_args(args), 
                   None, 
                   "Expected an invalid path, but got valid");
    }
}

use crate::song::Song;

/// Prints a line of text, but with an arrow before it
#[macro_export]
macro_rules! print_arrow {
    ($($arg:tt)*) => {
        print!("\x1b[32m==>\x1b[0m ");
        println!("\x1b[1m{}\x1b[0m", format_args!($($arg)*));
    };
}

pub fn are_vectors_equal(vector1: &Vec<Song>, vector2: &Vec<Song>) -> bool {
    vector1 == vector2
}

#[cfg(test)]
mod tests {
    use crate::song::Song;
    use super::are_vectors_equal;

    #[test]
    fn test_are_vectors_equal() {
        let song = Song::new(
                "The Killchain".to_string(), 
                "Bolt Thrower".to_string(), 
                "Those Once Loyal".to_string(), 
                2005, 
                4.41);
        let song2 = Song::new(
                "You Only Live Once".to_string(), 
                "Suicide Silence".to_string(), 
                "The Black Crown".to_string(),
                2011, 
                3.13);
        let song3 = Song::new(
                "Pull the Plug".to_string(), 
                "Death".to_string(), 
                "Leprosy".to_string(), 
                1988, 
                4.27);
        let song4 = Song::new(
                "Tyende Sang".to_string(), 
                "Afsky".to_string(), 
                "Ofte Jeg Drømmer Mig Død".to_string(), 
                2020,
                8.39);

        let vec1 = vec![
                song.clone(), 
                song2.clone(), 
                song3.clone(), 
                song4.clone()];
        let vec2 = vec![
                song, 
                song2, 
                song3,
                song4];

        assert_eq!(are_vectors_equal(&vec1, &vec2), 
                true, 
                "Both vectors should be equal to each other");
    }

    #[test]
    fn test_are_vectors_not_equal() {
        let song = Song::new(
                "The Killchain".to_string(), 
                "Bolt Thrower".to_string(), 
                "Those Once Loyal".to_string(), 
                2005, 
                4.41);
        let song2 = Song::new(
                "You Only Live Once".to_string(), 
                "Suicide Silence".to_string(), 
                "The Black Crown".to_string(), 
                2011,
                3.13);
        let song3 = Song::new(
                "Pull the Plug".to_string(), 
                "Death".to_string(), 
                "Leprosy".to_string(), 
                1988, 
                4.27);
        let song4 = Song::new(
                "Tyende Sang".to_string(), 
                "Afsky".to_string(), 
                "Ofte Jeg Drømmer Mig Død".to_string(), 
                2020, 
                8.39);

        let vec1 = vec![
                song.clone(), 
                song2.clone(), 
                song3.clone(), 
                song4.clone()];
        let vec2 = vec![
                song2, 
                song,
                song4, 
                song3];

        assert_eq!(are_vectors_equal(&vec1, &vec2), 
                false, 
                "Both vectors should not be equal to each other");
    }
}

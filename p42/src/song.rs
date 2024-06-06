use std::fmt;

struct SongIter {
    current_line: u32,
    next_line: u32,
}

pub trait Iterator {
    type Item;
    //fn iterate_song(&self, song: &String) -> &String;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for SongIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_line = self.current_line;
        self.current_line = self.next_line;

        self.next_line = curr_line + self.next_line;

        Some(curr_line)
    }

    // pub fn get_song_iterator(&mut self) -> SongIter{

    // }
}

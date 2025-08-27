use crate::edit::line::Line;
use crate::edit::view::Location;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    file_name: Option<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(file_name)?;
        let lines = contents.lines().map(|line| Line::from(line)).collect();
        Ok(Self { 
            lines,
            file_name: Some(file_name.to_string()),
        })
    }

    pub fn insert_char(&mut self, ch: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&ch.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(ch, at.grapheme_index);
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.height() > at.line_index.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                self.lines[at.line_index].append(&next_line);
            } else {
                self.lines[at.line_index].delete(at.grapheme_index);
            }
        }
    }

    pub fn insert_newline(&mut self, at: Location) {
        if at.line_index == self.height() {
            self.lines.push(Line::default());
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new_line = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new_line);
        }
    }
    
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(file_name) = self.file_name.as_deref() {
            let contents = self.lines.iter().map(|line| line.to_string()).collect::<Vec<_>>().join("\n");
            std::fs::write(file_name, contents)?;
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }
}

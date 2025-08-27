use std::fmt::{Display, Formatter};
use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Copy, Clone)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(&self, other: usize) -> usize {
        match self {
            GraphemeWidth::Half => other.saturating_add(1),
            GraphemeWidth::Full => other.saturating_add(2),
        }
    }
}
struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

#[derive(Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self
            .fragments
            .iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect::<String>();
        write!(f, "{result}")
    }
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self { fragments }
    }

    pub fn insert_char(&mut self, character: char, at: usize) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == at {
                result.push(character);
            }
            result.push_str(&fragment.grapheme);
        }
        if at >= self.fragments.len() {
            result.push(character);
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.fragments.len() {
            return;
        }
        self.fragments.remove(at);
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut curr_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(curr_pos);
            if curr_pos >= fragment_end {
                break;
            }
            if fragment_end > range.start {
                if fragment_end > range.end || curr_pos < range.start {
                    result.push('~');
                } else if let Some(ch) = fragment.replacement {
                    result.push(ch);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            curr_pos = fragment_end;
        }

        result
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn append(&mut self, other: &Self) {
        let mut concat = self.to_string();
        concat.push_str(other.to_string().as_str());
        self.fragments = Self::str_to_fragments(concat.as_str());
    }

    pub fn split(&mut self, at: usize) -> Self {
        if at > self.fragments.len() {
            Self::default()
        } else {
            let remainder = self.fragments.split_off(at);
            Self {
                fragments: remainder,
            }
        }
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect()
    }

    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }
}

use crossterm::terminal::size;

pub fn panel(word_1: &str, word_2: &str) -> String {
    format!(
        "╭─{}─╮   ╭─{}─╮\n\
         │ {} │   │ {} │\n\
         ╰─{}─╯   ╰─{}─╯",
        "─".repeat(word_1.len()),
        "─".repeat(word_2.len()),
        word_1,
        word_2,
        "─".repeat(word_1.len()),
        "─".repeat(word_2.len())
    )
}

pub fn wrap_text(paragraph: &str, indent_length: usize) -> String {
    let words = paragraph.split_whitespace().collect::<Vec<&str>>();
    let indent = " ".repeat(indent_length);
    let (cols, _) = size().unwrap();
    let max_width = (cols as usize).saturating_sub(7);
    let mut space_left = (cols as usize).saturating_sub(7);

    words
        .iter()
        .enumerate()
        .fold(String::new(), |mut wrapped, (i, word)| {
            if i == 0 {
                wrapped.push_str(&format!("{}{}", indent, word));
                space_left -= word.len();
            } else if word.len() + 1 > space_left {
                wrapped.push('\n');
                wrapped.push_str(&format!("{}{}", indent, word));
                space_left = max_width.saturating_sub(word.len());
            } else {
                if !wrapped.ends_with('\n') {
                    wrapped.push(' ');
                    space_left -= 1;
                }
                wrapped.push_str(word);
                space_left -= word.len();
            }
            wrapped
        })
}

use std::fs;
use std::path::Path;

use syntect::util::LinesWithEndings;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{start_highlighted_html_snippet, append_highlighted_html_for_styled_line};
use syntect::html::IncludeBackground;
use syntect::parsing::{SyntaxSet, SyntaxReference};

static SYNTAXES: &[u8] =
    include_bytes!("../../resources/syntaxes/syntaxes.bin");
static THEMES: &[u8] =
    include_bytes!("../../resources/themes/ayu_dark.tmTheme");

pub fn highlighted_html_for_string_newline_class(
    s: &str,
    ss: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &Theme,
) -> String {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let (mut output, bg) = start_highlighted_html_snippet(theme);

    for line in LinesWithEndings::from(s) {
        output.push_str("<span class='newline'>");
        let regions = highlighter.highlight(line, ss);
        append_highlighted_html_for_styled_line(
            &regions[..],
            IncludeBackground::IfDifferent(bg),
            &mut output,
        );
        output.push_str("</span>");
    }
    output.push_str("</pre>\n");

    output
}

pub fn get_pretty_body(path: &Path, ext: &str) -> std::io::Result<String> {
    let ss: SyntaxSet = syntect::dumps::from_binary(SYNTAXES);

    let mut theme_cursor = std::io::Cursor::new(THEMES);
    let theme = ThemeSet::load_from_reader(&mut theme_cursor).unwrap();

    let content = fs::read_to_string(path)?;
    let syntax = ss
        .find_syntax_by_token(ext)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    Ok(highlighted_html_for_string_newline_class(&content, &ss, syntax, &theme))
}

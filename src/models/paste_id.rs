use sanitise_file_name::{sanitize_with_options, Options as sanitizeOptions};
use std::borrow::Cow;
use std::fmt;

use rocket::request::FromParam;

use rand::{self, distributions::Alphanumeric, Rng};

pub struct PasteId<'a>(Cow<'a, str>);

fn valid_id(id: &str) -> bool {
    id.chars().all(char::is_alphanumeric)
}

impl PasteId<'_> {
    pub fn new(size: usize, custome_name: &str) -> PasteId<'static> {
        let mut id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect();
        let custome_name_options = sanitizeOptions {
            extension_cleverness: false,
            url_safe: true,
            collapse_replacements: true,
            ..Default::default()
        };

        if !custome_name.is_empty() {
            let sanitized_custom_name: String =
                sanitize_with_options(custome_name, &custome_name_options)
                    .replace(".", "_");

            id.insert(0, '-');
            id.insert_str(0, sanitized_custom_name.as_str());
        }

        PasteId(Cow::Owned(id))
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match valid_id(param) {
            true => Ok(PasteId(Cow::Borrowed(param))),
            false => Err(param),
        }
    }
}

impl fmt::Display for PasteId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

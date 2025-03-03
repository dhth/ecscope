use regex::Regex;

const TAG_REGEX_STR: &str = r"^[a-z0-9_-]{1,20}$";

pub struct Profile(String);

impl TryFrom<&str> for Profile {
    type Error = String;

    #[allow(clippy::expect_used)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(TAG_REGEX_STR).expect("profile regex is invalid");

        if !re.is_match(value) {
            return Err(format!("valid regex: {}", TAG_REGEX_STR));
        }

        Ok(Self(value.to_string()))
    }
}

impl Profile {
    pub fn name(&self) -> &str {
        &self.0
    }
}

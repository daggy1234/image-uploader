use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::borrow::Cow;
use std::fmt;
use std::iter;

pub struct PostId<'a>(Cow<'a, str>);

impl<'a> PostId<'a> {
    pub fn generate() -> PostId<'static> {
        let mut rng = thread_rng();
        let rand_string: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(10)
            .collect();
        PostId(Cow::Owned(rand_string))
    }
}

impl<'a> fmt::Display for PostId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

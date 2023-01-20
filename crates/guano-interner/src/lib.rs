use std::sync::Arc;

use guano_common::once_cell::sync::OnceCell;
use guano_common::sync::Map;
use guano_common::{internment::ArcIntern, num::rug};

pub type InternedInteger = ArcIntern<rug::Integer>;
pub type InternedString = ArcIntern<String>;
#[derive(Debug, Default)]
pub struct Interner {
    integers: Map<InternedString, InternedInteger>,
    floats: Map<InternedString, Arc<rug::Float>>,
    strings: Map<InternedString, InternedString>,
    chars: Map<InternedString, char>,
}

impl Interner {
    #[inline]
    fn instance() -> &'static Interner {
        static INSTANCE: OnceCell<Interner> = OnceCell::new();
        INSTANCE.get_or_init(Interner::default)
    }

    #[inline]
    fn get_integer(&self, key: InternedString) -> Option<InternedInteger> {
        self.integers.get(&key).map(|r| r.value().clone())
    }

    #[inline]
    fn set_integer(&self, key: InternedString, value: InternedInteger) -> InternedInteger {
        match self.get_integer(key.clone()) {
            Some(i) => i,
            None => {
                self.integers.insert(key, value.clone());

                value
            }
        }
    }

    #[inline]
    fn get_float(&self, key: InternedString) -> Option<Arc<rug::Float>> {
        self.floats.get(&key).map(|r| r.value().clone())
    }

    #[inline]
    fn set_float(&self, key: InternedString, value: rug::Float) -> Arc<rug::Float> {
        match self.get_float(key.clone()) {
            Some(f) => f,
            None => {
                let value = Arc::new(value);
                self.floats.insert(key, value.clone());
                value
            }
        }
    }

    #[inline]
    fn get_string(&self, key: InternedString) -> Option<InternedString> {
        self.strings.get(&key).map(|r| r.value().clone())
    }

    #[inline]
    fn set_string(&self, key: InternedString, value: InternedString) -> InternedString {
        match self.get_string(key.clone()) {
            Some(s) => s,
            None => {
                self.strings.insert(key, value.clone());
                value
            }
        }
    }

    #[inline]
    fn get_char(&self, key: InternedString) -> Option<char> {
        self.chars.get(&key).map(|r| *r.value())
    }

    #[inline]
    fn set_char(&self, key: InternedString, value: char) -> char {
        match self.get_char(key.clone()) {
            Some(c) => c,
            None => {
                self.chars.insert(key, value);

                value
            }
        }
    }
}

#[inline]
pub fn integer(key: &str) -> Option<InternedInteger> {
    let key = InternedString::from_ref(key);
    Interner::instance().get_integer(key)
}

#[inline]
pub fn set_integer(key: &str, value: rug::Integer) -> InternedInteger {
    let key = InternedString::from_ref(key);
    let value = InternedInteger::new(value);
    Interner::instance().set_integer(key, value)
}

#[inline]
pub fn float(key: &str) -> Option<Arc<rug::Float>> {
    let key = InternedString::from_ref(key);
    Interner::instance().get_float(key)
}

#[inline]
pub fn set_float(key: &str, value: rug::Float) -> Arc<rug::Float> {
    let key = InternedString::from_ref(key);
    Interner::instance().set_float(key, value)
}

#[inline]
pub fn string(key: &str) -> Option<InternedString> {
    let key = InternedString::from_ref(key);
    Interner::instance().get_string(key)
}

#[inline]
pub fn set_string(key: &str, value: &str) -> InternedString {
    let key = InternedString::from_ref(key);
    let value = InternedString::from_ref(value);
    Interner::instance().set_string(key, value)
}

#[inline]
pub fn char(key: &str) -> Option<char> {
    let key = InternedString::from_ref(key);
    Interner::instance().get_char(key)
}

#[inline]
pub fn set_char(key: &str, value: char) -> char {
    let key = InternedString::from_ref(key);
    Interner::instance().set_char(key, value)
}

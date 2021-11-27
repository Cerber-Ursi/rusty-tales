use std::ops::Index;

pub struct Label {
    ru: &'static str,
    en: &'static str,
}

impl<'b> Index<&'b str> for Label {
    type Output = &'static str;
    fn index<'a>(&'a self, code: &'b str) -> &'a &'static str {
        match code {
            "ru" => &self.ru,
            "en" => &self.en,
            _ => panic!("Unsupported index"),
        }
    }
}

pub static TO_CODE: Label = Label {
    ru: "К исходному коду",
    en: "To source code",
};
pub static TO_CHAPTER: Label = Label {
    ru: "К главе",
    en: "To chapter",
};

//! Translations for text

#[derive(Serialize, Deserialize)]
pub(super) struct Translations {
    /// English
    en: Option<String>,
    /// Spanish (Español)
    es: Option<String>,
    // FIXME: The rest of the languages.
    /// Constructed language for automatic translation.
    rv: Option<String>,
}

impl Translations {
    pub(super) fn english(&self) -> Option<&str> {
        self.en.as_ref().or(self.rv.as_ref()).map(|a| a.as_str())
    }
}

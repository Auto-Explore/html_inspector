mod lang_constraints;
mod lang_detect_warnings;
mod meta_charset;
mod meta_http_equiv_charset;
mod unicode_normalization_nfc_warning;
mod xml_lang_consistency;

use html_inspector::Rule;

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(unicode_normalization_nfc_warning::UnicodeNormalizationNfcWarning::default()),
        Box::new(xml_lang_consistency::XmlLangConsistency),
        Box::new(meta_charset::MetaCharsetUtf8::default()),
        Box::new(lang_constraints::LangConstraints::default()),
        Box::new(lang_detect_warnings::LangDetectWarnings::default()),
        Box::new(meta_http_equiv_charset::MetaHttpEquivCharset),
    ]
}

#[cfg(test)]
mod tests;

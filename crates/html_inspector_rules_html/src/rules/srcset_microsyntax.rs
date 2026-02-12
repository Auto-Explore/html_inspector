use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrcsetMode {
    None,
    Width,
    Density,
}

pub fn is_invalid_srcset(srcset: &str, sizes_present: bool) -> bool {
    validate_srcset(srcset, sizes_present).is_err()
}

pub fn srcset_mode(srcset: &str) -> Option<SrcsetMode> {
    validate_srcset(srcset, false).ok()
}

fn validate_srcset(srcset: &str, sizes_present: bool) -> Result<SrcsetMode, ()> {
    let s = srcset.trim();
    if s.is_empty() {
        return Err(());
    }
    if s.contains("/*") || s.contains("*/") {
        return Err(());
    }

    let mut saw_width = false;
    let mut saw_density = false;
    let mut saw_omitted = false;

    let mut widths: HashSet<u32> = HashSet::new();
    let mut densities: HashSet<u64> = HashSet::new();

    for candidate in s.split(',') {
        let candidate = candidate.trim();
        if candidate.is_empty() {
            return Err(());
        }
        let mut tokens = candidate.split_ascii_whitespace();
        let url = tokens.next().ok_or(())?;
        if url.is_empty() {
            return Err(());
        }
        if url_has_disallowed_chars(url) {
            return Err(());
        }
        if url_is_broken_scheme_only(url) {
            return Err(());
        }

        let Some(desc) = tokens.next() else {
            saw_omitted = true;
            continue;
        };
        if tokens.next().is_some() {
            return Err(());
        }

        if let Some(num) = desc.strip_suffix('w') {
            if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
                return Err(());
            }
            let w = num.parse::<u32>().map_err(|_| ())?;
            if w == 0 {
                return Err(());
            }
            if !widths.insert(w) {
                return Err(());
            }
            saw_width = true;
        } else if let Some(num) = desc.strip_suffix('x') {
            if num.starts_with('+') || num.starts_with('-') || num.is_empty() {
                return Err(());
            }
            let v = num.parse::<f64>().map_err(|_| ())?;
            if !(v.is_finite() && v > 0.0) {
                return Err(());
            }
            let bits = v.to_bits();
            if !densities.insert(bits) {
                return Err(());
            }
            saw_density = true;
        } else {
            return Err(());
        }
    }

    // If any descriptor is used, all candidates must use one.
    if (saw_width || saw_density) && saw_omitted {
        return Err(());
    }

    // Can't mix descriptor kinds.
    if saw_width && saw_density {
        return Err(());
    }

    let mode = if saw_width {
        SrcsetMode::Width
    } else if saw_density {
        SrcsetMode::Density
    } else {
        SrcsetMode::None
    };

    if sizes_present && mode != SrcsetMode::Width {
        return Err(());
    }

    Ok(mode)
}

fn url_is_broken_scheme_only(url: &str) -> bool {
    url.strip_suffix(':')
        .is_some_and(|prefix| !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_alphabetic()))
}

fn url_has_disallowed_chars(url: &str) -> bool {
    url.chars()
        .any(|c| matches!(c, '{' | '}' | '[' | ']' | '|' | '<' | '>' | '(' | ')'))
}

#[cfg(test)]
mod tests {
    use super::{is_invalid_srcset, srcset_mode, SrcsetMode};

    #[test]
    fn srcset_rejects_multiple_descriptors() {
        assert!(is_invalid_srcset("a 1w 2w", false));
        assert!(is_invalid_srcset("a 1x 2x", false));
        assert!(is_invalid_srcset("a 1w 2x", false));
    }

    #[test]
    fn srcset_accepts_single_descriptor_or_omitted() {
        assert_eq!(srcset_mode("a 1w"), Some(SrcsetMode::Width));
        assert_eq!(srcset_mode("a 2x"), Some(SrcsetMode::Density));
        assert_eq!(srcset_mode("a"), Some(SrcsetMode::None));
    }

    #[test]
    fn srcset_rejects_scheme_only_urls() {
        assert!(is_invalid_srcset("http: 1w", false));
        assert!(is_invalid_srcset("data: 1x", false));
    }

    #[test]
    fn srcset_rejects_nonpositive_or_nonfinite_density_descriptors() {
        assert!(is_invalid_srcset("a 0x", false));
        assert!(is_invalid_srcset("a NaNx", false));
        assert!(is_invalid_srcset("a infx", false));
    }
}

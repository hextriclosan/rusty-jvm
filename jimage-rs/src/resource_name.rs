use crate::error::Result;
use crate::JImage;
use std::borrow::Cow;
use std::iter::FusedIterator;

/// Represents the components of a resource name in a JImage file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceName<'a> {
    pub module: Cow<'a, str>,
    pub parent: Cow<'a, str>,
    pub base: Cow<'a, str>,
    pub extension: Cow<'a, str>,
}

impl<'a> ResourceName<'a> {
    /// Gets the full resource name as a tuple of (module, full_path).
    /// The full path is constructed by combining the parent, base, and extension.
    /// For example, if the module is `"java.base"`, parent is `"java/lang"`,
    /// base is `"String"`, and extension is `"class"`, the full path will be
    /// `"java/lang/String.class"`.
    pub fn get_full_name(&self) -> (String, String) {
        let mut full_name = String::new();

        full_name.push_str(self.parent.as_ref());
        if !self.parent.is_empty() {
            full_name.push('/');
        }

        full_name.push_str(self.base.as_ref());
        if !self.extension.is_empty() {
            full_name.push('.');
        }
        full_name.push_str(self.extension.as_ref());

        (self.module.to_string(), full_name)
    }
}

/// An iterator over the resource names in a JImage file.
pub struct ResourceNamesIter<'a> {
    jimage: &'a JImage,
    front_index: usize,
    back_index: usize,
}

impl<'a> ResourceNamesIter<'a> {
    pub fn new(jimage: &'a JImage) -> Self {
        Self {
            jimage,
            front_index: 0,
            back_index: jimage.items_count(),
        }
    }
}

impl<'a> Iterator for ResourceNamesIter<'a> {
    type Item = Result<ResourceName<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.front_index < self.back_index {
            let idx = self.front_index;
            self.front_index += 1;
            match self.jimage.resource_at_index(idx) {
                Ok(Some(r)) => return Some(Ok(r)),
                Ok(None) => continue,
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.back_index.saturating_sub(self.front_index)))
    }
}

impl<'a> DoubleEndedIterator for ResourceNamesIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.front_index < self.back_index {
            self.back_index -= 1;
            match self.jimage.resource_at_index(self.back_index) {
                Ok(Some(r)) => return Some(Ok(r)),
                Ok(None) => continue,
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

impl FusedIterator for ResourceNamesIter<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    fn make(parent: &'static str, base: &'static str, ext: &'static str) -> ResourceName<'static> {
        ResourceName {
            module: Cow::Borrowed("java.base"),
            parent: Cow::Borrowed(parent),
            base: Cow::Borrowed(base),
            extension: Cow::Borrowed(ext),
        }
    }

    #[test]
    fn full_name_with_parent_and_extension() {
        let r = make("java/lang", "String", "class");
        assert_eq!(
            r.get_full_name(),
            (
                "java.base".to_string(),
                "java/lang/String.class".to_string()
            )
        );
    }

    #[test]
    fn full_name_without_parent() {
        let r = make("", "README", "md");
        assert_eq!(
            r.get_full_name(),
            ("java.base".to_string(), "README.md".to_string())
        );
    }

    #[test]
    fn full_name_without_extension() {
        let r = make("java/lang", "noext", "");
        assert_eq!(
            r.get_full_name(),
            ("java.base".to_string(), "java/lang/noext".to_string())
        );
    }

    #[test]
    fn full_name_without_parent_and_without_extension() {
        let r = make("", "bare", "");
        assert_eq!(
            r.get_full_name(),
            ("java.base".to_string(), "bare".to_string())
        );
    }

    #[test]
    fn iterator_size_hint_and_double_ended_using_real_jimage() {
        // Use a real jimage fixture to drive the iterator.
        let path = "tests/test_data/lib/non-compressed_little-endian.jimage";
        if !std::path::Path::new(path).exists() {
            // Fixtures excluded from published crate; skip.
            return;
        }
        let image = JImage::open(path).expect("open jimage");
        let mut it = image.resource_names_iter();
        let (lo, hi) = it.size_hint();
        assert_eq!(lo, 0);
        assert!(hi.unwrap() >= 1);

        // Forward then backward should still yield items, and the iterator
        // must be fused once exhausted.
        let first = it.next();
        assert!(first.is_some());
        let last = it.next_back();
        assert!(last.is_some());

        // Drain the rest.
        let _: Vec<_> = it.by_ref().collect();
        // Fused: subsequent calls must keep returning None.
        assert!(it.next().is_none());
        assert!(it.next().is_none());
        assert!(it.next_back().is_none());
    }
}

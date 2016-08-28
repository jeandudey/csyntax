//! This module contains one function to replace trigraphs on a source file with
//! the "real" character.
//!
//! The following are valid trigraphs:
//! ```
//! ??=      #
//! ??(      [
//! ??/      \
//! ??)      ]
//! ??'      ^
//! ??<      {
//! ??!      |
//! ??>      }
//! ??-      ~
//! ```

/// Replace all trigraphs on `source_text`
///
/// # Example
/// ```
/// use csyntax::c89::parse::trigraphs::replace_trigraphs;
///
/// let result = replace_trigraphs(r"void main() ??<??>");
///
/// assert_eq!(result, "void main() {}".to_owned());
/// ```
///
/// # Warning
/// This is slow as hell, is you're a C developer consider buying a new keyboard
pub fn replace_trigraphs<S: Into<String>>(source_text: S) -> String {
    let s = source_text.into();
    s.replace("??=", "#")
        .replace("??(", "[")
        .replace("??/", "\\")
        .replace("??)", "]")
        .replace("??'", "^")
        .replace("??<", "{")
        .replace("??!", "|")
        .replace("??>", "}")
        .replace("??-", "~")
}

#[cfg(test)]
mod test {
    use super::replace_trigraphs;

    #[test]
    fn test_replace_trigraphs() {
        let result = replace_trigraphs(r"??= ??( ??/ ??) ??' ??< ??! ??> ??-");

        assert_eq!(result, r"# [ \ ] ^ { | } ~".to_owned());
    }
}

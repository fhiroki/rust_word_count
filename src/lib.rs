//! wordcount はシンプルな文字、単語、行の出現頻度の計算機能を提供します。
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを参照してください。
#![warn(missing_docs)]

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// Unicodeの１文字
    Char,
    /// 正規表現の \w+ にマッチする単語
    Word,
    /// \n で区切られた一行
    Line,
}

/// オプションのデフォルトは [`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// input から一行ずつ文字列を読み込み。頻度を数える。
///
/// 頻度を数える対象はオプションによって制御される。
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char)
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word)
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line)
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use fhiroki_bicycle_book_wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
///
/// # Panics
/// 入力がUTF-8フォーマットされていない場合
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }

    freqs
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value:expr),*}) => {
            $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn word_count_works() {
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 2);
        exp.insert("cc".to_string(), 1);
        assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
    }

    #[test]
    fn word_count_works2() {
        use std::io::Cursor;

        let mut exp = HashMap::new();
        exp.insert("a".to_string(), 2);
        exp.insert("c".to_string(), 2);
        exp.insert("d".to_string(), 3);

        assert_eq!(count(Cursor::new("aaccddd"), CountOption::Char), exp);
    }

    #[test]
    fn word_count_works3() {
        let freqs = count(Cursor::new("aa bb cc"), CountOption::Word);
        assert_map!(freqs, {"aa" => 1, "bb" => 1, "cc" => 1});
    }

    #[test]
    #[should_panic]
    fn word_count_do_not_contain_unknown_words() {
        use std::io::Cursor;

        count(Cursor::new([b'a', 0xf0, 0x90, 0x80]), CountOption::Word);
    }
}

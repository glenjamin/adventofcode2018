use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let ids: Vec<_> = load_ids(io::stdin().lock()).collect();

    println!("Checksum: {:?}", checksum(ids.iter().map(|s| &s[..])));
    println!("ID: {:?}", common_id(&ids));
}

fn load_ids(stream: impl BufRead) -> impl Iterator<Item = String> {
    stream.lines().flatten().filter(|s| startswith_alpha(s))
}

fn startswith_alpha(string: &str) -> bool {
    string.chars().next().unwrap_or(' ').is_ascii_alphabetic()
}

fn checksum<'a>(ids: impl Iterator<Item = &'a str>) -> i32 {
    let mut twice_total = 0;
    let mut thrice_total = 0;
    for id in ids {
        let (twice, thrice) = has_repeats(id);
        if twice {
            twice_total += 1;
        }
        if thrice {
            thrice_total += 1;
        }
    }

    twice_total * thrice_total
}

fn has_repeats(id: &str) -> (bool, bool) {
    let counts = count_letters(id);
    let mut twice = false;
    let mut thrice = false;
    for count in counts.values() {
        if *count == 2 {
            twice = true;
        }
        if *count == 3 {
            thrice = true;
        }
        if twice && thrice {
            break;
        }
    }

    (twice, thrice)
}

fn count_letters(id: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for ch in id.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    map
}

fn common_id(ids: &[String]) -> Option<String> {
    for id in ids.iter() {
        for other in ids.iter() {
            if let Some(common) = similar_ids(id, other) {
                return Some(common);
            }
        }
    }

    None
}

fn similar_ids(id: &str, other: &str) -> Option<String> {
    let mut differing = id
        .char_indices()
        .zip(other.chars())
        .filter(|((_, a), b)| a != b)
        .map(|((i, _), _)| i);

    if let Some(i) = differing.next() {
        if differing.next().is_none() {
            return Some((&id[..i]).to_owned() + &id[(i + 1)..]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_load_ids() {
        assert_eq!(
            load_ids(BufReader::new(
                "abcdef
asdsadsad
asddsfghesrgsg
sdfsdfsdf
"
                .as_bytes()
            ))
            .collect::<Vec<_>>(),
            vec!["abcdef", "asdsadsad", "asddsfghesrgsg", "sdfsdfsdf"]
        )
    }

    #[test]
    fn test_checksum() {
        let input = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        assert_eq!(checksum(input.into_iter()), 12);
    }

    #[test]
    fn test_common_id() {
        let input: Vec<_> = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(common_id(&input), Some("fgij".to_owned()));
    }

    #[test]
    fn test_similar_ids() {
        assert_eq!(similar_ids("abc", "def"), None);
        assert_eq!(similar_ids("abc", "abc"), None);
        assert_eq!(similar_ids("abc", "ade"), None);
        assert_eq!(similar_ids("abc", "abb"), Some("ab".to_owned()));
        assert_eq!(similar_ids("abcdef", "abcjef"), Some("abcef".to_owned()));
    }

    #[test]
    fn test_count_letters() {
        assert_eq!(
            count_letters("bababc"),
            [('a', 2), ('b', 3), ('c', 1)].iter().cloned().collect()
        )
    }
}

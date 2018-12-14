use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let ids = load_ids(stdin_lock);

    println!("Checksum: {:?}", ids.collect::<Vec<String>>());
}

fn load_ids(stream: impl BufRead) -> impl Iterator<Item = String> {
    stream
        .lines()
        .flatten()
        .map(|s| s.trim_end().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_load_ids() {
        assert_eq!(
            load_ids(string_reader(
                "abcdef
asdsadsad
asddsfghesrgsg
sdfsdfsdf
    ",
            ))
            .collect::<Vec<_>>(),
            vec!["abcdef", "asdsadsad", "asddsfghesrgsg", "sdfsdfsdf"]
        )
    }

    fn string_reader(string: &'static str) -> impl BufRead {
        BufReader::new(string.as_bytes())
    }
}

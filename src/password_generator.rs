use itertools::Itertools;

pub fn generate(alpha: bool, numeric: bool, special_symbols: bool) -> impl Iterator<Item = String> {
    let mut charset = String::new();

    if alpha {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }

    if numeric {
        charset.push_str("0123456789");
    }

    if special_symbols {
        charset.push_str(" !\"#$%&'()*+,-./:;<=>?@^_`{|}~");
    }

    (1..=20)
        .flat_map(move |len| {
            charset
                .clone()
                .chars()
                .combinations_with_replacement(len)
                .map(move |combos| (combos, len))
                .collect::<Vec<_>>()
        })
        .flat_map(|(combos, len)| combos.into_iter().permutations(len))
        .dedup()
        .map(|chars| chars.into_iter().collect())
}

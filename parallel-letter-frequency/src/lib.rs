use itertools::Itertools;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = if input.len() > worker_count {
        input.len() / worker_count + 1
    } else {
        1
    };

    let threads = input
        .chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .flat_map(|s| {
                    s.chars()
                })
                .collect()
        })
        .map(|chunk: Vec<char>| {
            thread::spawn(move || {
                chunk.into_iter().filter(|c| c.is_alphabetic())
                .filter_map(|c| c.to_lowercase().next())
                    .sorted()
                    .group_by(|&c| c)
                    .into_iter()
                    .map(|(c, g)| (c, g.count()))
                    .collect::<HashMap<char, usize>>()
            })
        })
        .collect::<Vec<_>>();
    threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .fold(HashMap::new(), |mut map, freqs| {
            freqs
                .into_iter()
                .for_each(|(k, v)| *map.entry(k).or_insert(0) += v);
            map
        })
}

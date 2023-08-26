use std::collections::HashMap;
use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::cmp;

fn calc_frequency(lines: &[&str], store: &mut HashMap<char, AtomicUsize>) {
    for &line in lines {
        let chars = line.chars().filter(|ch| ch.is_alphabetic());
        for ch in chars {
            let &lowercased = ch.to_lowercase().collect::<Vec<_>>().first().unwrap();
            let count = store.entry(lowercased).or_insert(AtomicUsize::new(0));
            count.fetch_add(1, Ordering::SeqCst);
        }
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks = input.chunks(worker_count);
    let mut store = HashMap::<char, AtomicUsize>::new();
    
    input.chunks(worker_count)
        .for_each(|chunk| {
            thread::scope(|scope| {
                scope.spawn(|| {
                    calc_frequency(chunk, &mut store);
                });
            });
        });

    store
        .iter()
        .map(|(&k, v)| (k, v.load(Ordering::SeqCst)))
        .collect::<HashMap<_, _>>()
}

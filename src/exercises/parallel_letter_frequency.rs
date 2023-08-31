use std::cmp;
use std::collections::hash_map::HashMap;
use std::hash::Hash;
use std::iter::{repeat, repeat_with};
use std::ops::Add;
use std::slice::Chunks;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

// pub mod channels {
//     use std::cmp;
//     use std::collections::HashMap;
//     use std::iter::{self, repeat_with};
//     use std::slice::Chunks;
//     use std::sync::atomic::{AtomicUsize, Ordering};
//     use std::sync::{mpsc, Arc, Mutex};
//     use std::thread::{self, JoinHandle};
    fn process_line<T, S>(processor: T, line: S)
    where
        T: FnMut(char) -> (),
        S: AsRef<str>,
    {
        line.as_ref()
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .collect::<String>()
            .to_lowercase()
            .chars()
            .for_each(processor)
    }
    fn calc_frequency(lines: &[&str]) -> HashMap<char, usize> {
        let mut store = HashMap::<char, usize>::new();

        for line in lines {
            process_line(
                |ch| {
                    *store.entry(ch).or_insert(0) += 1;
                },
                line,
            );
        }

        store
    }

    fn split<'a>(input: &'a [&'a str], len: usize) -> Chunks<'a, &str> {
        let chunk_size = input.len() / len + input.len() % len;
        input.chunks(chunk_size)
    }

    pub fn frequency<'a>(input: &'a [&str], worker_count: usize) -> HashMap<char, usize> {
        let worker_count = cmp::min(worker_count, input.len());
        let mut store = HashMap::<char, usize>::new();

        let (senders, receivers) = repeat_with(|| mpsc::channel::<&str>())
            .take(worker_count)
            .fold((vec![], vec![]), |mut acc, (sender, receiver)| {
                acc.0.push(sender);
                acc.1.push(receiver);
                acc
            });

        thread::scope(|scope| {
            scope.spawn(|| {
                input.into_iter().enumerate().for_each(|(index, &line)| {
                    let sender = senders.get(index % worker_count).unwrap();
                    sender.send(line).unwrap();
                });
                drop(senders);
            });
        });

        receivers.into_iter().for_each(|receiver| {
            thread::scope(|scope| {
                scope.spawn(|| {
                    for line in receiver {
                        process_line(
                            |ch| {
                                *store.entry(ch).or_insert(0) += 1;
                            },
                            line,
                        );
                    }
                });
            });
        });

        store
    }
// }

pub mod mutex {
    use std::cmp;
    use std::collections::HashMap;
    use std::iter::{self, repeat_with};
    use std::slice::Chunks;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread::{self, JoinHandle};

    fn calc_frequency(lines: String, store: Arc<Mutex<HashMap<char, usize>>>) {
        lines
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .collect::<String>()
            .to_lowercase()
            .chars()
            .for_each(|ch| {
                let store_guard = &mut store.lock().unwrap();
                *store_guard.entry(ch).or_insert(0) += 1;
            });
    }

    // fn demo() {
    //     use std::cmp;

    //     let x = 5;
    //     let y = 6;

    //     let z = max(&x, &y);

    //     print!("{:#?}", x);
    //     print!("{:#?}", z);

    //     fn max<T: cmp::PartialOrd + Copy>(a: &T, b: &T) -> T {
    //         if a > b {
    //             *a
    //         } else {
    //             *b
    //         }
    //     }
    // }

    pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
        let store = Mutex::new(HashMap::<char, usize>::new());
        let store_rc = Arc::new(store);
        let chunk_size = input.len() / worker_count + input.len() % worker_count;

        let handlers = input
            .chunks(chunk_size)
            .map(|chunk| chunk.join(""))
            .map(|chunk| {
                let th_store = store_rc.clone();
                thread::spawn(move || {
                    calc_frequency(chunk, th_store);
                })
            })
            .for_each(|handle| {
                handle.join();
            });

        let hash_map = store_rc.lock().unwrap();
        hash_map.clone()
    }
}
pub mod scope {
    use std::cmp;
    use std::collections::HashMap;
    use std::iter;
    use std::slice::Chunks;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn calc_frequency(lines: &[&str]) -> HashMap<char, usize> {
        let mut store = HashMap::<char, usize>::new();

        for line in lines {
            line.chars()
                .filter(|ch| ch.is_alphabetic())
                .collect::<String>()
                .to_lowercase()
                .chars()
                .for_each(|ch| {
                    *store.entry(ch).or_insert(0) += 1;
                });
        }

        store
    }

    fn split<'a>(input: &'a [&'a str], len: usize) -> Chunks<'a, &str> {
        let chunk_size = input.len() / len + input.len() % len;
        input.chunks(chunk_size)
    }

    pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
        let chunks = split(input, worker_count);
        let mut store = HashMap::<char, usize>::new();

        chunks.for_each(|chunk| {
            thread::scope(|scope| {
                scope.spawn(|| {
                    calc_frequency(chunk).iter().for_each(|(&k, &v)| {
                        *store.entry(k).or_insert(0) += v;
                    });
                });
            });
        });

        store
    }
}

mod scope_atomic {
    use std::cmp;
    use std::collections::HashMap;
    use std::iter;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn calc_frequency(lines: &[&str], store: &mut HashMap<char, AtomicUsize>) {
        for &line in lines {
            let chars = line
                .chars()
                .filter(|ch| ch.is_alphabetic())
                .collect::<String>()
                .to_lowercase();
            for ch in chars.chars() {
                let count = store.entry(ch).or_insert(AtomicUsize::new(0));
                count.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    pub fn frequency_scope(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
        let chunk_size = input.len() / worker_count + input.len() % worker_count;
        let chunks = input.chunks(chunk_size);
        let mut store = HashMap::<char, AtomicUsize>::new();

        input.chunks(worker_count).for_each(|chunk| {
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
}

// Left to adjust debug environment for unit tests
// #[test]
// fn test_no_texts() {
//     assert_eq!(frequency(&[], 4), HashMap::new());
// }

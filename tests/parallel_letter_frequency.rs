use exercism_2022::exercises::parallel_letter_frequency as frequency;
use std::{borrow::BorrowMut, collections::HashMap, mem::{size_of, align_of}, rc::Rc};

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];
// Dutch national anthem
const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];
// American national anthem
const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];
#[test]
fn test_no_texts() {
    assert_eq!(frequency::frequency(&[], 4), HashMap::new());
}
#[test]
#[ignore]
fn test_one_letter() {
    let mut hm = HashMap::new();
    hm.insert('a', 1);
    assert_eq!(frequency::frequency(&["a"], 4), hm);
}
#[test]
#[ignore]
fn test_case_insensitivity() {
    let mut hm = HashMap::new();
    hm.insert('a', 2);
    assert_eq!(frequency::frequency(&["aA"], 4), hm);
}
#[test]
#[ignore]
fn test_many_empty_lines() {
    let v = vec![""; 1000];
    assert_eq!(frequency::frequency(&v[..], 4), HashMap::new());
}
#[test]
#[ignore]
fn test_many_times_same_text() {
    let times = 1000;
    let v = vec!["abc"; times];
    let mut hm = HashMap::new();
    hm.insert('a', times);
    hm.insert('b', times);
    hm.insert('c', times);
    assert_eq!(frequency::frequency(&v[..], 4), hm);
}
#[test]
#[ignore]
fn test_punctuation_doesnt_count() {
    assert!(!frequency::frequency(&WILHELMUS, 4).contains_key(&','));
}
#[test]
#[ignore]
fn test_numbers_dont_count() {
    assert!(!frequency::frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
}
#[test]
#[ignore]
fn test_all_three_anthems_1_worker() {
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency::frequency(&v[..], 1);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}
#[test]
#[ignore]
fn test_all_three_anthems_3_workers() {
    let mut v = Vec::new();
    for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
        for line in anthem.iter() {
            v.push(*line);
        }
    }
    let freqs = frequency::frequency(&v[..], 3);
    assert_eq!(freqs.get(&'a'), Some(&49));
    assert_eq!(freqs.get(&'t'), Some(&56));
    assert_eq!(freqs.get(&'ü'), Some(&2));
}
#[test]
#[ignore]
fn test_non_integer_multiple_of_threads() {
    let len = 999;
    let v = vec!["abc"; len];
    let mut hm = HashMap::new();
    hm.insert('a', len);
    hm.insert('b', len);
    hm.insert('c', len);
    assert_eq!(frequency::frequency(&v[..], 4), hm);
}

#[test]
fn foo() {
    // #[derive(Debug)]
    // struct Foo<'a>(&'a str, bool, i64);

    // let x = Foo("aaa", true, 3);

    // // |  x  |         y         |         z        
    // // | Foo | <- 0x700007e3f548 | <- 0x700007e3f568
    // let y = &x as *const _;
    // let z1 = &y as *const _;
    // let z2 = &z1 as *const _;
    // let diff = z2 as usize - z1 as usize;

    // println!("{x:?} {y:p} {z1:p} {z2:p} {diff}");
    // println!("{:?}", size_of::<Foo>());
    // // println!("{:?}", size_of::<&Foo>());
    // println!("{:?}", align_of::<Foo>());
    // println!("{:?}", size_of::<i8>());
    // println!("{:?}", size_of::<[i16; 2]>());
    // println!("{:?}", size_of::<Vec<i64>>());
    // // println!("{:?}", size_of::<&mut i8>());
    // // println!("{:?}", align_of::<&i8>());
    // // println!("{:?}", size_of::<&i32>());
    // // println!("{:?}", size_of::<&mut i32>());
    // // println!("{:?}", align_of::<&i32>());
    // // println!("{:?}", size_of::<i8>());
    // // println!("{:?}", size_of::<i32>());
    // // println!("{:?}", align_of::<&str>());
    // // println!("{:?}", align_of::<&i32>());

    // Slice and vec
    // let a = [123, 234, 456];
    // let b = &a[0..2];

    // println!("{:?}", a);
    // println!("{:?} {:p}", b, b);

    // let v = vec![123, 234, 456];
    // let v1 = v.as_ptr();
    // let w = &v[0..2];

    // println!("{:?} {:?}", v, v);
    // println!("{:?} {:p}", v, v1);

    // let s1 = "aaa";
    // let s2 = s1.clone();
    // println!("{:p} {:p} {:p}", s1, s2, s3);
    
    let S1 = String::from("bbb");
    let s1_ptr = S1.as_ptr();
    println!("{:?} {:p}", S1, s1_ptr); 


    let Rc1 = Rc::new(String::from("ccc"));
    println!("{:?} {:p}", "Rc1.as_ptr()", Rc1.as_ptr());

    // println!("{:?}", size_of::<&[i8]>());
    // println!("{:?}", size_of::<&str>());
    // println!("{:?}", size_of::<String>());
}

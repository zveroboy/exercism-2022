pub struct Allergies {
    score: u32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Allergen {
    Eggs = 0b00000001,
    Peanuts = 0b00000010,
    Shellfish = 0b00000100,
    Strawberries = 0b00001000,
    Tomatoes = 0b00010000,
    Chocolate = 0b00100000,
    Pollen = 0b01000000,
    Cats = 0b10000000,
}

impl Allergen {
    const VALUES: [Self; 8] = [
        Self::Eggs,
        Self::Peanuts,
        Self::Shellfish,
        Self::Strawberries,
        Self::Tomatoes,
        Self::Chocolate,
        Self::Pollen,
        Self::Cats,
    ];
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        &self.score & *allergen as u32 != 0
    }

    fn allergies(&self) -> Vec<Allergen> {
        let mut vec = Vec::new();
        for al in Allergen::VALUES.iter() {
            if self.is_allergic_to(al) {
                vec.push(*al);
            }
        }
        vec
    }
}

fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
    for element in expected {
        if !actual.contains(element) {
            panic!("Allergen missing\n  {element:?} should be in {actual:?}");
        }
    }

    if actual.len() != expected.len() {
        panic!(
            "Allergy vectors are of different lengths\n  expected {expected:?}\n  got {actual:?}"
        );
    }
}

#[test]
fn is_not_allergic_to_anything() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Peanuts));
    assert!(!allergies.is_allergic_to(&Allergen::Cats));
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
#[ignore]
fn is_allergic_to_eggs() {
    assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
}

#[test]
#[ignore]
fn is_allergic_to_eggs_and_shellfish_but_not_strawberries() {
    let allergies = Allergies::new(5);
    assert!(allergies.is_allergic_to(&Allergen::Eggs));
    assert!(allergies.is_allergic_to(&Allergen::Shellfish));
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
#[ignore]
fn no_allergies_at_all() {
    let expected = &[];
    let allergies = Allergies::new(0).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_just_eggs() {
    let expected = &[Allergen::Eggs];
    let allergies = Allergies::new(1).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_just_peanuts() {
    let expected = &[Allergen::Peanuts];
    let allergies = Allergies::new(2).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_just_strawberries() {
    let expected = &[Allergen::Strawberries];
    let allergies = Allergies::new(8).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_eggs_and_peanuts() {
    let expected = &[Allergen::Eggs, Allergen::Peanuts];
    let allergies = Allergies::new(3).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_eggs_and_shellfish() {
    let expected = &[Allergen::Eggs, Allergen::Shellfish];
    let allergies = Allergies::new(5).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_many_things() {
    let expected = &[
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(248).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_everything() {
    let expected = &[
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(255).allergies();

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]
fn scores_over_255_do_not_trigger_false_positives() {
    let expected = &[
        Allergen::Eggs,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(509).allergies();

    compare_allergy_vectors(expected, &allergies);
}

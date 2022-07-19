use itertools::Itertools;
pub struct Allergies {
    scora: u32,
}

#[derive(Clone, Eq, Hash, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // unimplemented!(
        //     "Given the '{}' score, construct a new Allergies struct.",
        //     score
        // );
        Self { scora: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // unimplemented!(
        //     "Determine if the patient is allergic to the '{:?}' allergen.",
        //     allergen
        // );

        let x = match allergen {
            &Allergen::Eggs => 1,
            &Allergen::Peanuts => 2,
            &Allergen::Shellfish => 4,
            &Allergen::Strawberries => 8,
            &Allergen::Tomatoes => 16,
            &Allergen::Chocolate => 32,
            &Allergen::Pollen => 64,
            &Allergen::Cats => 128,
        };
        x <= self.scora
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        // unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
        let mut x = self.scora;
        let mut v: Vec<Allergen> = vec![]; 
        while x !=0 {
            if x >= 128 {
                v.push(Allergen::Cats);
                x -= 128;
            } else if x >= 64 {
                v.push(Allergen::Pollen);
                x -= 64
            } else if x >= 32 {
                v.push(Allergen::Chocolate);
                x -= 32;
            } else if x >= 16 {
                v.push(Allergen::Tomatoes);
                x -= 16;
            } else if x >= 8 {
                v.push(Allergen::Strawberries);
                x -= 8;
            } else if x >= 4 {
                v.push(Allergen::Shellfish);
                x -= 4;
            } else if x >= 2 {
                v.push(Allergen::Peanuts);
                x -= 2;
            } else if x == 1 {
                v.push(Allergen::Eggs);
                x -= 1;
            }
        }
        let v: Vec<_> = v.into_iter().unique().collect();
        v
        }
    }


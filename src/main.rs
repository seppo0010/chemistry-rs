#![allow(non_upper_case_globals, dead_code)]

#[derive(Debug, PartialEq)]
enum Identifier {
    OneLetter(char),
    TwoLetters(char, char),
}

#[derive(Debug, PartialEq)]
struct Atom {
    identifier: Identifier,
    atomic_number: u16,
}

#[derive(Debug, PartialEq)]
enum Group {
    Group1,
    Group2,
    Group3,
    Group4,
    Group5,
    Group6,
    Group7,
    Group8,
    Group9,
    Group10,
    Group11,
    Group12,
    Group13,
    Group14,
    Group15,
    Group16,
    Group17,
    Group18,
    Ungrouped,
}

#[derive(Debug, PartialEq)]
struct Period {
    mod_groups: &'static [Group],
}

const Period1: Period = Period {
    mod_groups: &[Group::Group1, Group::Group18],
};

const Period2: Period = Period {
    mod_groups: &[
        Group::Group1,
        Group::Group2,
        Group::Group13,
        Group::Group14,
        Group::Group15,
        Group::Group16,
        Group::Group17,
        Group::Group18,
        Group::Group18,
    ],
};

const Periods: &'static [Period] = &[Period1, Period2];

impl Atom {
    fn group(&self) -> &'static Group {
        let mut min = 1;
        for period in Periods {
            let max = min + period.mod_groups.len() as u16;
            if self.atomic_number >= min && self.atomic_number < max {
                return &period.mod_groups[(self.atomic_number - min) as usize];
            }
            min = max;
        }
        unimplemented!();
    }

    fn period(&self) -> &'static Period {
        let mut min = 1;
        for period in Periods {
            let max = min + period.mod_groups.len() as u16;
            if self.atomic_number >= min && self.atomic_number < max {
                return period;
            }
            min = max;
        }
        unimplemented!();
    }
}

const H: Atom = Atom {
    identifier: Identifier::OneLetter('H'),
    atomic_number: 1,
};
const He: Atom = Atom {
    identifier: Identifier::TwoLetters('H', 'e'),
    atomic_number: 2,
};
const O: Atom = Atom {
    identifier: Identifier::OneLetter('O'),
    atomic_number: 8,
};

fn main() {
    println!("{:?}", vec![H, He, O]);
    println!("{:?}", vec![H.group(), He.group(), O.group()]);
    println!("{:?}", vec![H.period(), He.period(), O.period()]);
}

#[test]
fn test_oxigen_period_group() {
    assert_eq!(O.group(), &Group::Group16);
    assert_eq!(O.period(), &Period2);
}

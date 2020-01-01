#[derive(Debug)]
enum Identifier {
    OneLetter(char),
    TwoLetters(char, char),
}

#[derive(Debug)]
struct Atom {
    identifier: Identifier,
    atomic_number: u16,
}

const H: Atom = Atom {
    identifier: Identifier::OneLetter('H'),
    atomic_number: 1,
};
#[allow(non_upper_case_globals)]
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
}

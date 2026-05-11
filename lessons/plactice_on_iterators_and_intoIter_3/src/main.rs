struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8; // this needs to be fixed
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.r, self.g, self.b].into_iter()
    }
}

fn main() {
    let p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    let p = p.into_iter();

    for component in p {
        println!("{}", component);
    }
}

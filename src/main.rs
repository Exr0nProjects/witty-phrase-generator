use rand::prelude::{ thread_rng, IteratorRandom };

fn main() {
    let path_intensifiers = String::from_utf8_lossy(include_bytes!("intensifiers.txt"));
    let path_adjectives   = String::from_utf8_lossy(include_bytes!("adjectives.txt")  );
    let path_nouns        = String::from_utf8_lossy(include_bytes!("nouns.txt")       );

    let mut rng = thread_rng();
    let     sep = '-';
    println!("{}{}{}{1}{}",
             path_intensifiers.lines().choose(&mut rng).unwrap(),
             sep,
             path_adjectives  .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap()
        );
}

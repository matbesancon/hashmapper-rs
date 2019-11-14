extern crate hashmapper;

use hashmapper::*;

#[test]
fn std_example() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }
}

#[test]
fn entry_api() {
    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    }

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats
        .entry("defence")
        .or_insert_with(random_stat_buff);

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();
}

#[test]
fn custom_keys() {
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        /// Creates a new Viking.
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }

    // Use a HashMap to store the vikings' health points.
    let mut vikings = HashMap::new();

    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

#[test]
fn key_value_iterators() {
    let mut map = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(5, 3);

    for val in map.values() {
        assert!(&1 <= val && val <= &3);
    }
    for k in map.keys() {
        assert!(&1 <= k && k <= &5);
    }
}

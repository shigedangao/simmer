use simmer;

fn main() {
    let stem = simmer::stem("excellent").unwrap();
    assert_eq!(stem, "excel");

    let sentence = simmer::stem_sentence("Alex was an excellent dancer.").unwrap().join(" ");
    assert_eq!(sentence, "alex wa an excel dancer");
}

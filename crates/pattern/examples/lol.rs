use providence_pattern::Pattern;

const PATTERN: Pattern<48> = Pattern::new("48 8B 05 ?? ?? ?? ?? 8B 38 E8 ?? ?? ?? ?? 89 C7");

fn main() {
    println!("Pattern {PATTERN:?}");
    println!("Regex   {:?}", PATTERN.regex());

    let bytes = b"hello\x48\x8B\x05xxxx\x8B\x38\xE8xxxx\x89\xC7world";
    let result = PATTERN.regex().find(bytes);

    println!("{result:?}");
}

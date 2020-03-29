use minimad::*;
fn main() {
    let z = parse_text(r#"|x *d*|y|
|-|-|
|zz|zz|
***Hello***"#
    );
    // minimad::CompositeStyle::Header;

    println!("{:?}", z.lines);
}

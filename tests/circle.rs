use lib_svg::SVGWriter;

const TARGET: &str = include_str!("./circle.svg");

#[test]
fn circle() {
    let mut output = String::new();

    let mut writer = SVGWriter::new(&mut output, 500, 500);
    writer.circle().cx(250.0).cy(250.0).r(240.0).fill("red");
    drop(writer);

    assert_eq!(output, TARGET);
}

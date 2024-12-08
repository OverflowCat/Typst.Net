// use typst::text::Font;
use typst_as_lib::TypstTemplate;

static TEMPLATE_FILE: &str = r#"#set page(width: auto, height: auto, margin: 0.2em);#set text(font: ("Libertinus Serif", "Noto Serif CJK SC", "Times New Roman"))
#box(width: 4em, height: 1em, fill: gradient.linear(red, purple, space: color.hsv))"#;
// static FONT: &[u8] = include_bytes!("./fonts/texgyrecursor-regular.otf");
// static OUTPUT: &str = "./examples/output.pdf";
// static IMAGE: &[u8] = include_bytes!("./templates/images/typst.png");

pub fn to_svg_string(input: &str) -> String {
    // let font = Font::new(Bytes::from(FONT), 0).expect("Could not parse font!");
    let input = format!("{}\n{}", TEMPLATE_FILE, input);
    // println!("template is {:?}", input);
    let template = TypstTemplate::new(vec![], input);
    let doc = template
        .compile()
        .output
        .expect("typst::compile() returned an error!");
    // println!("Total pages: {}", doc.pages.len());
    // println!("Doc info: {:#?}", doc.info);
    if let Some(page) = doc.pages.get(0) {
        typst_svg::svg(page)
    } else {
        String::from("")
    }
}

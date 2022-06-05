mod backend;
mod view;

// const OUT_FILE_NAME: &'static str = "./plotters-doc-data/data.png";
// const IN_FILE_NAME: &'static str = "./plotters-doc-data/data.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_file = std::env::args().nth(1).expect("Need input .json file");
    let out_file = std::env::args().nth(2).expect("Need output .png file");

    let data = backend::Rep::read_data(in_file.as_str())?;
    view::plot(out_file.as_str(), data, "Sales")
}

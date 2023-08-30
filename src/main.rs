mod converter;

use converter::file::Format;

fn main() {
    let src_path = "static/people.json".to_owned();
    let src_format = Format::JSON;
    let dst_path = "static/people.csv".to_owned();
    let dst_format = Format::CSV;

    let file = converter::file::File::new(src_path, src_format, dst_path, dst_format);

    file.read_file();
}

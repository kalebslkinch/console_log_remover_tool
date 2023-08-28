mod remove;
mod file;
mod io;

fn main() {
    println!("Enter the path to the file: ");

    let path = io::read();

    let filename = path.trim().to_string();

    let data = file::read(&filename);

    let new_data = remove::console_log(&data);

    file::write(&new_data, format!("{}", filename));

    println!("Done!");
}

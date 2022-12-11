use clap::Parser;
use disk_space::loader;

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}
fn main() {
    const TOTAL_SPACE: usize = 70000000;
    const MINIMUM_REQUIRED_SPACE: usize = 30000000;
    let args = Args::parse();
    let mut dirs = loader::load_directory_tree(&args.input);
    println!("Total dirs: {}", dirs.len());
    let total_used_space = dirs[0].borrow().get_directory_size();
    let free_space = TOTAL_SPACE - total_used_space;
    let missing_space = MINIMUM_REQUIRED_SPACE - free_space;
    println!("Total used space: {}", total_used_space);
    println!("Total free space: {}", free_space);
    println!("Missing space: {}", missing_space);

    dirs.sort_by(|x, y| {
        x.borrow()
            .get_directory_size()
            .cmp(&y.borrow().get_directory_size())
    });

    for dir in dirs.iter() {
        let borrowed_dir = dir.borrow();
        if borrowed_dir.get_directory_size() >= missing_space {
            println!(
                "Directory {} with size {} frees enough space",
                borrowed_dir.name(),
                borrowed_dir.get_directory_size()
            );
            break;
        }
    }

    let sum_filtered_dirs = dirs
        .iter()
        .filter(|dir| dir.borrow().get_directory_size() <= 100000)
        .fold(0, |acc, dir| acc + dir.borrow().get_directory_size());
    println!("Filtered dirs size: {}", sum_filtered_dirs)
}

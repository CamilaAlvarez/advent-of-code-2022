use calorie_count::elves::loader;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}
fn main() {
    let args = Args::parse();
    let mut elves = loader::load_elves_from_file(&args.filename).expect("Couldn't load elves");
    elves.sort_by(|x, y| y.calories().cmp(&x.calories()));
    if elves.len() > 0 {
        println!("Max calories: {}", elves[0].calories());
    }
    if elves.len() >= 3 {
        let num_elves = 3;
        let mut sum_calories = 0;
        for i in 0..num_elves {
            sum_calories += elves[i].calories();
        }
        println!("Sum calories top 3 elves: {}", sum_calories);
    }
}

use clap::Parser;
use fruit_salad_cli::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Stefan Kornemann",
    about = "Create a fruit salad with random fruits."
)]
struct Opts {
    #[clap(short, long, default_value = "3")]
    num_fruit: usize,
    // #[clap(short, long, default_value = "3")]
    fruits: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits from the command line
    let num_fruit = opts.num_fruit;

    // Get the fruits from the command line
    let mut fruit_salad = opts.fruits;

    if fruit_salad.len() == 0 {
        // Create the fruit salad
        fruit_salad = create_fruit_salad(num_fruit);
    }

    fruit_salad.sort();

    // Print the fruit salad
    println!(
        "Created Fruit Salad with {} fruits: {:?}",
        num_fruit, fruit_salad
    );
}

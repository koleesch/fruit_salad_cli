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
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits from the command line
    let num_fruit = opts.num_fruit;

    // Create the fruit salad
    let fruit_salad = create_fruit_salad(num_fruit);

    // Print the fruit salad
    println!(
        "Created Fruit Salad with {} fruits: {:?}",
        num_fruit, fruit_salad
    );
}

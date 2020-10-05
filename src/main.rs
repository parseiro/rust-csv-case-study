#![warn(anonymous_parameters)] //#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_results)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::cast_possible_truncation, clippy::cast_possible_wrap,
clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::integer_arithmetic)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map, clippy::filter_map_next)]
#![warn(clippy::if_not_else, clippy::nonminimal_bool, clippy::single_match_else)]
#![warn(clippy::int_plus_one)]
#![warn(clippy::similar_names)]
#![warn(clippy::mutex_integer)]
//#![warn(clippy::print_stdout,clippy::use_debug)]
#![warn(clippy::unwrap_used, clippy::map_unwrap_or)]
//#![warn(clippy::unwrap_in_result)]

use std::path::Path;

// use csv::Error;
use serde::Deserialize;
use std::error::Error;
use std::{io, fs, fmt};
use serde::export::Formatter;

/*static USAGE: &'static str = "
Usage: city-pop [options] <data-path> <city>
       city-pop --help

Options:
    -h, --help     Show this usage message.
";

struct Args {
    arg_data_path: String,
    arg_city: String,
}*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Linha {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

struct PopulationCount {
    country: String,
    accent_city: String,
    population: u64,
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Csv(csv::Error),
    NotFound,
}

impl Error for CliError {
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Csv(ref err) => err.fmt(f),
            CliError::NotFound => write!(f, "No matching cities with a population were found."),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> Self {
        CliError::Csv(err)
    }
}

fn main() {
    /*    let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.decode())
            .unwrap_or_else(|err| err.exit());*/
    let a = search_city(&Some("worldcitiespop.csv"), "Londrina");

    // println!("Ja obtive a resposta");

    match a {
        Ok(vector) => for city in vector {
            println!("Cidade: {}, País: {}, População: {}", city.accent_city, city.country, city.population)
        },
        Err(err) => println!("Error: {}", err),
    }
}

// TODO: entender por que usar Error+Send+Sync
// O texto original diz: There is one big gotcha in this code: we used Box<Error + Send + Sync> instead of
// Box<Error>. We did this so we could convert a plain string to an error type. We
// need these extra bounds so that we can use the corresponding From impls (...)
fn search_city<P: AsRef<Path>>(file_path: &Option<P>, city: &str) -> Result<Vec<PopulationCount>, CliError> {
    println!("Procurando \"{}\"...", city);

    let city_lowercase = city.to_lowercase();

    let input: Box<dyn io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(fs::File::open(file_path)?),
    };

    let mut rdr = csv::Reader::from_reader(input);
    // let mut rdr = csv::Reader::from_path(file_path)?;


    let rows = rdr.deserialize::<Linha>();

    let mut found: Vec<PopulationCount> = vec![];

    for row in rows {
        row.map(|cidade| {
            if cidade.city == city_lowercase {
                found.push(PopulationCount {
                    country: cidade.country,
                    accent_city: cidade.accent_city,
                    population: cidade.population.unwrap(),
                });
            }
        })?;

        /*        row.map_or_else(|err| println!("Error: {}", err),
                                |linha| {
                                    if linha.city == "tshibizena" {
                                        let population: String = linha.population.map_or_else(
                                            || "(sem informações)".to_string(), |pop| pop.to_string());

                                        println!("City: {} {}", linha.city, population)
                                    }
                                },
                );*/
    }

    if found.is_empty() {
        Err(CliError::NotFound)
    } else
    {
        Ok(found)
    }
}
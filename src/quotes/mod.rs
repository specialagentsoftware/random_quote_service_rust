use self::quotes_model::QuoteModel;
use rand::Rng;
use std::fs::File;
use std::path::Path;
mod quotes_model;

///quotes struct provides the quote model,
/// a way to generate a new object and a
/// way to generate a formatted string based on that quote object
pub struct Quotes {
    pub quotes: Vec<QuoteModel>,
}

impl Quotes {
    /// this implements block adds a constructor that opens the csv file, parses the contents,
    /// generates a quote object for each line, creates a vector of the objects and adds a function
    /// that provides a formatted string related to a randomly selected quote

    pub fn new() -> Quotes {
        // the new function is called when a quotes object is created. It handles opening and parsing the file
        // and generating quote objects using the quote model and adding those quotes to a collection of vectors
        let mut quotevec: Vec<QuoteModel> = Vec::new();
        let path: &Path = Path::new("src/data/quotes.csv");
        let display: std::path::Display = path.display();

        let file: File = match File::open(&path) {
            Err(why) => panic!("Could not open {}:{}", display, why),
            Ok(file) => file,
        };

        let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

        for result in rdr.records() {
            let record: Result<csv::StringRecord, csv::Error> = result;
            let res: csv::StringRecord = record.unwrap();
            let qm: QuoteModel =
                QuoteModel::new(res[0].to_string(), res[1].to_string(), res[2].to_string());
            quotevec.push(qm);
        }

        let quote: Quotes = Quotes { quotes: quotevec };
        quote
    }

    pub fn get_random_quote(&mut self) -> String {
        //get_random_quote generates a random integer and uses that to randomly selecting
        // a quote from the quote vector and then calling a method on the quote model that provides
        // a formatted string
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let quote: &mut QuoteModel = &mut self.quotes[rng.gen_range(0..48390)];
        let quote_text: String = quote.quote_mut();
        quote_text
    }
}

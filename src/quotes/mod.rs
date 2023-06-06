use rand::Rng;
use std::fs::File;
use std::path::Path;
use self::quotes_model::QuoteModel;
mod quotes_model;
pub struct Quotes {
    pub quotes: Vec<QuoteModel>
}

impl Quotes {

    pub fn new() -> Quotes {
        let mut quotevec: Vec<QuoteModel> = Vec::new();
        let path = Path::new("src/data/quotes.csv");
        let display = path.display();
        
        let file = match File::open(&path){
            Err(why) => panic!("Could not open {}:{}",display,why),
            Ok(file) => file,
        };

        let mut rdr = csv::Reader::from_reader(file);
        
        for result in rdr.records() {
            let record: Result<csv::StringRecord, csv::Error> = result;
            let res: csv::StringRecord = record.unwrap();
            let qm : QuoteModel = QuoteModel::new(res[0].to_string(), res[1].to_string(), res[2].to_string());
            quotevec.push(qm);
        }
      
        let quote: Quotes = Quotes {
            quotes : quotevec
        };
        quote
    }

    pub fn get_random_quote(&mut self) -> String {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let quote: &mut QuoteModel = &mut self.quotes[rng.gen_range(0..48390)];
        let quote_text: String = quote.quote_mut();
        quote_text
    }
}
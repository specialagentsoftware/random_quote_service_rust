#[derive(Debug)]
/// the quote model allows for the structuring of quotes
pub struct QuoteModel {
    author: String,
    category: String,
    quote: String,
}

/// this implementation block allows for the construction of a quote and a function that returns
/// a formatted string related to the current quote object
impl QuoteModel {
    pub fn new(author: String, category: String, quote: String) -> QuoteModel {
        let qm: QuoteModel = QuoteModel {
            author: author,
            category: category,
            quote: quote,
        };
        return qm;
    }

    pub fn quote_mut(&mut self) -> String {
        format!("<div style='margin: 25px 50px 75px 100px'><ul style='list-style-type: none'><li>Author: {} </li><li>Category {} </li> <li>Quote {}</li><ul></div>",self.author,self.category,self.quote)
    }
}

use rand::Rng;

pub struct Quotes {
    pub quotes: Vec<String>
}

impl Quotes {

    pub fn new() -> Quotes {
        let mut quotevec: Vec<String> = Vec::new();
        quotevec.push(String::from("dis is it"));
        quotevec.push(String::from("whoomp there it is"));
        quotevec.push(String::from("lets get this jiggle going"));
        quotevec.push(String::from("jammit to hell"));
        quotevec.push(String::from("force recon"));
        let quote: Quotes = Quotes {
            quotes : quotevec
        };
        quote
    }

    pub fn get_random_quote(&mut self) -> String {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let result: &String = &self.quotes[rng.gen_range(0..4)];
        String::from(result)
    }
}
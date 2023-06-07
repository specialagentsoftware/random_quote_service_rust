# random_quote_service_rust
## Three different language challenge

I decided to write a simple web endpoint that displays random quotes (Author, Category, Quote) when someone hits the endpoint in three different languages. (Python, Go and Rust) just to see how the implementation was different and to figure out what I wanted to start using for other projects. This is the rust version of the quotes service. 

### Overview
Basic webserver is using actix_web due to the simplicty of this project.
The quote module uses the csv module from the rust standard library.
Also included is a cargo.toml and the cargo.lock file. 

The project is broken down into modules for ease of understanding and maintenance. 

The application entry point is src/main.rs. It starts the actix webserver and instantiates the quote module.
The quote mod handles connecting to the csv file and parsing that into a list of Quote objects that are genrated by the quotes_model. It then selects a random quote and utilizes the quotes_model to retrieve a formatted html unordered list of the author, category and quote. This is then passed back to the webserver for display to the requester. 

There are comments in each module about the specifics of what is happening when it makes sense.

### Instalation
I might do some installation notes.

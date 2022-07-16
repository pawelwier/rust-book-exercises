mod collections;
use collections::practice::{ get_mid, get_mode, to_pig_latin };

mod cli;
use cli::users::handle_organization;

mod file_reader;
use file_reader::file_reader::get_file_contents;

mod traits;
use traits::traits::{Article, Tweet, Book, Summary, summarize, largest};

mod minigrep;
use minigrep::minigrep::{get_lines_with_query, get_bin_name};

fn run_collection_exercises() {
    let mut arr1 = Vec::from([3, 3, 1, 2, 4, 5, 6]);
    get_mid(&mut arr1);

    let mut arr2 = Vec::from([3, 1, 3, 3, 1, 3, 5, 1, 2, 3]);
    get_mode(&mut arr2);

    to_pig_latin(&mut "apple".to_string());
    to_pig_latin(&mut "onet".to_string());
    to_pig_latin(&mut "first".to_string());
    to_pig_latin(&mut "kret".to_string());

    let users = Vec::new();
    let departments = Vec::new();
    handle_organization(users, departments);
}

fn run_file_reader_exercises() {
    let text = get_file_contents("text".to_string());
    println!("{}", text);
}

fn run_trait_exercises() {
    let article = Article {author: String::from("kret"), page: 32, content: String::from("sdg dg dg"), topic: String::from("cars")};
    let tweet = Tweet {author: String::from("fant"), content: String::from("kjij kumhku zbczc")};
    let book = Book {author: String::from("jan"), title: String::from("cook book"), pages: 389};

    println!("{}", article.get_summary());
    println!("{}", tweet.get_summary());
    println!("{}", book.get_summary());

    println!("{}", summarize(&article));

    println!("{:?}", largest(&vec![4, 2, 12, 6, 3, 9]));
}

fn main() {
    // run_collection_exercises();
    // run_file_reader_exercises();
    // run_trait_exercises();

    get_bin_name();
    // get_lines_with_query();
}
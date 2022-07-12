// mod collections;
// use collections::practice::{ get_mid, get_mode, to_pig_latin };

// mod cli;
// use cli::users::handle_organization;

mod file_reader;
use file_reader::file_reader::get_file_contents;

mod text_forms;
use text_forms::text_forms::{Article, Tweet, Book, Summary};

fn main() {
    /* 
    println!("*** practice ***");
    println!("");

    let mut arr1 = Vec::from([3, 3, 1, 2, 4, 5, 6]);
    get_mid(&mut arr1);

    let mut arr2 = Vec::from([3, 1, 3, 3, 1, 3, 5, 1, 2, 3]);
    get_mode(&mut arr2);

    to_pig_latin(&mut "apple".to_string());
    to_pig_latin(&mut "onet".to_string());
    to_pig_latin(&mut "first".to_string());
    to_pig_latin(&mut "kret".to_string());

    println!("");
    println!("*** practice ***");

    let users = Vec::new();
    let departments = Vec::new();
    handle_organization(users, departments);
    */  

    // let text = get_file_contents("text".to_string());
    // println!("{}", text);

    let article = Article {author: String::from("kret"), page: 32, content: String::from("sdg dg dg"), topic: String::from("cars")};
    let tweet = Tweet {author: String::from("fant"), content: String::from("kjij kumhku zbczc")};
    let book = Book {author: String::from("jan"), title: String::from("cook book"), pages: 389};

    println!("{}", article.get_summary());
    println!("{}", tweet.get_summary());
    println!("{}", book.get_summary());
}
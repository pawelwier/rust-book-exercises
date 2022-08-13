mod collections;
use collections::practice::{ get_mid, get_mode, to_pig_latin };

mod cli;
use cli::users::handle_organization;

mod file_reader;
use file_reader::file_reader::get_file_contents;

mod traits;
use traits::traits::{ Article, Tweet, Book, Summary, summarize, largest };

mod minigrep;
use minigrep::minigrep::{ get_lines_with_query, get_bin_name };

mod closures;
use closures::shirts::{ get_shirt_color_with_pref, get_shirt_color_without_pref };

mod smart_pointers;
use smart_pointers::smart_pointers::{ make_list, say_hello, MyBox };

enum List {
    Cons(i32, Box<List>),
    Nil
}

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

fn run_io_project() {
    get_bin_name();
    get_lines_with_query();
}

fn run_closure_exercises() {
    println!("{:?}", get_shirt_color_with_pref());
    println!("{:?}", get_shirt_color_without_pref());
}

fn run_smart_pointers() {
    // deref coercion
    let my_box = MyBox::new(String::from("krab"));
    say_hello(&my_box);
}

fn main() {
    // run_collection_exercises();
    // run_file_reader_exercises();
    // run_trait_exercises();
    // run_io_project();
    // run_closure_exercises();

    run_smart_pointers();
}
pub mod practice {
    use std::{ collections::HashMap };

    pub fn get_mid(arr: &mut Vec<i32>) -> () {
        arr.sort();
        let mid = *arr.get(arr.len() / 2).unwrap();
        println!("{}", mid);
    }

    pub fn get_mode(arr: &mut Vec<i32>)-> () {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in arr {
            let count = map.entry(*num).or_insert(0);
            *count += 1
        }
        let (x, y) = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        println!("{} is most frequent. It appears {} times.", &x, &y);
        ()
    }

    fn starts_with_vowel(word: &str) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        return vowels.contains(&String::from(word).chars().nth(0).unwrap());
    }

    pub fn to_pig_latin(word: &mut String) -> () {     
        if starts_with_vowel(word) {
            word.push_str("hay");
        } else {
            let first: &str = &word[..1];
            *word = word[1..].to_string() + first + "ay";
        }
        println!("{:?}", word);
    }
}
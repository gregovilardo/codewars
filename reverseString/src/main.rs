fn main() {
    fn reverse_words(str: &str) -> String {
        // keep the index of all white spaces
        let mut white_space_indexs: Vec<usize> = vec![];
        let mut i = 0;
        for c in str.to_string().chars().collect::<Vec<char>>() {
            if c == ' ' {
                white_space_indexs.push(i);
                println!("{}", i);
            }
            i = i + 1;
        }

        // reverse each word
        fn rv_word(str: &str) -> String {
            let word_chars: Vec<char> = str.to_string().chars().collect();
            let mut rev_word: Vec<char> = vec![];
            for c in word_chars.into_iter().rev() {
                rev_word.push(c);
            }
            return String::from_iter(rev_word);
        }

        let mut words: Vec<String> = str
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        for word in words.iter_mut() {
            *word = rv_word(&word);
        }

        let mut reverse_phrase: Vec<char> = words.join("").chars().collect();

        for space_index in white_space_indexs {
            reverse_phrase.insert(space_index, ' ');
        }

        return reverse_phrase.into_iter().collect();
    }
    println!("{}", reverse_words3("Gregorio  Vilardo"));
}


//GODS

fn reverse_words3(str: &str) -> String {
    for word in str.to_string().split(" ") {
        println!("esta es la palabra {} cachai", String::from_iter(word.chars().rev()));
    }
    

    str.to_string()
      .split(" ")
      .map(|sub| sub.chars().rev().collect())
      .collect::<Vec<String>>()
      .join(" ")
}


fn reverse_words2(str: &str) -> String {
    /*
    
    The following one-liner in the function only works because of
    the following trick:
    
    ```Rust
    let x = "    a  b c".to_string();
    let d: Vec<_> = x.split(' ').collect();
    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
    ```
    
    Note that the string has 4 spaces before a, 2 spaces before b,
    and 1 space before c. However, when split, the resulting vector
    has 4 spaces, then a, then 1 space, then b, then 0 spaces, then c.
    
    Contiguous separators can lead to possibly surprising behavior 
    when whitespace is used as the separator. This trick allows this
    to work on sentences with an arbitrary number of spaces in between
    words.
    
    */
    
    
    str.split(" ")
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn reverse_words4(str: &str) -> String {
    let mut res = String::with_capacity(str.len());
    for w in str.split(' ') {
        res.extend(w.chars().rev());
        if res.len() < str.len() {
            res.push(' ')
        }
    }
    res
}

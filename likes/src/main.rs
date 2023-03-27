/* []                                -->  "no one likes this"
["Peter"]                         -->  "Peter likes this"
["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this" */

fn main() {
    fn likes(names: &[&str]) -> String {
        match names.len() {
            0 => return "no one likes this".to_string(),
            1 => return [names[0], "likes this"].join(" "),
            2 => return [names[0], "and", names[1], "like this"].join(" "),
            3 => return [names[0], ", ", names[1], " and ", names[2], " like this"].join(""),
            _ => return [names[0], ", ", names[1], " and ", &(names.len()-2).to_string(),  " others like this"].join(""),
        }
    }
    println!("{}", likes2(&["Mark", "Jhon", "Walter", "Grego"]));
}

//GOD MOD
//
fn likes2(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}

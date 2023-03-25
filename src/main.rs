mod levenshtein;
fn main() 
{
    let str1 = "kitten";
    let str2 = "sitting";
    println!("{}", levenshtein::distance(str1, str2, &[1, 6, 5]));
    println!("{}", levenshtein::distance(str1, str2, &[1, 1, 1]));
}


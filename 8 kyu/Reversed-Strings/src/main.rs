fn solution(phrase: &str) -> String {
    let mut res = String::new();
    for i in phrase.chars().rev(){
        res.push(i);
    }
    res
}
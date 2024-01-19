fn main() {
    let url = "https://books.toscrape.com/";
    let response = reqwest::blocking::get(url).expect("could not load url");
    let body = response.text().unwrap();
    print!("{}", body);
}



mod conf;
mod units;
mod endpoints;
mod endpoint;
mod test;
mod responses;

fn main() {
    println!("Hello, world!");
    let request = reqwest::blocking::get("http://httpbin.org/get");
    println!("{:#?}", request.as_ref());
    println!("{:#?}", request.unwrap().text().unwrap());
}
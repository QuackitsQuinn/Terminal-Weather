

mod conf;
mod units;
mod endpoints;
mod endpoint;
mod test;
mod ip_response;

fn main() {
    println!("Hello, world!");
    let request = reqwest::blocking::get("http://httpbin.org/get");
    println!("{:#?}", request.as_ref());
    println!("{:#?}", request.unwrap().text().unwrap());
}
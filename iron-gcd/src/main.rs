extern crate iron;
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::response;
use iron::status;
use iron::status::Status::Ok;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", get_from, "root");
    router.get("/gcd", post_gcd, "gcd");
    println!("Serving on http://localhost:3000...");
    Iron::new(get_from).http("localhost:3000").unwrap();
}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
           <input type="text" name="n">
           <input type="text" name="m">
           <button type="submit">Computer GCD</buton>
        </from>
    "#,
    );
    Ok(response)
}

extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match response.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    let unparsed_numbers = match form_data.ge("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => response.set_mut(status::Ba),
        }
    }
}

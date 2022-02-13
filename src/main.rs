extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;
use router::Router;
use iron::prelude::*;
use iron::status;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
<title>GCD Calculator</title>
    <div style="display: flex;justify-content: center;box-sizing: border-box;min-height: 100vh;align-items: center;background: #212B4F;">
        <form style="margin: auto 0; display: flex; align-items: center;justify-content: center;height: 100%; flex-direction: column;"
              action="/gcd" method="post">
            <div >
                <input style="
                    background: #FFFFFF;
                    border: 7px solid #3BBE6F;
                    box-sizing: border-box;
                    border-radius: 14px;
                    width: 366px;
                    height: 51px;
                    margin-right: 10px;" placeholder="Number 1:" type="text" name="n"/>
                <input style="
                    background: #FFFFFF;
                    border: 7px solid #3BBE6F;
                    box-sizing: border-box;
                    border-radius: 14px;
                    width: 366px;
                    height: 51px;" placeholder="Number 2:" type="text" name="n"/>
            </div>
            <button style="    background: #4A3096;
                margin-top: 33px;
                border-radius: 14px;
                color: #fff;
                padding: 13px 100px;
                font-family: Roboto;
                font-style: normal;
                font-weight: 500;
                font-size: 21px;
                line-height: 25px;
                box-shadow: none;
                border: none;" type="submit">Compute GCD</button>
        </form>
"#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not a number: {:?}\n",
                            unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
                numbers, d));
    Ok(response)
}




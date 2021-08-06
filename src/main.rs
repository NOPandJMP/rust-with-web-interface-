extern crate iron;
extern crate router;
extern crate urlencoded;

#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

use sqlite::State;
use sqlite::Value;

fn main() {
    let mut router = Router::new();
    router.get("/", get_autch, "root");
    router.get("/signin", get_reg, "signin");
    router.post("/autch", post_autch, "autch");
    router.post("/reg", post_reg, "reg");

    println!("Server started on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_autch(_requst: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#" 
    <title>Autch test</title>
    <form action="/autch" method="post">
    <input type="text" name="n"/>Login<Br>
    <input type="text" style="-webkit-text-security: disc;" name="n"/>Password<Br>
    <button type="submit">Autch</button>
    </form>
    "#,
    );
    Ok(response)
}

fn get_reg(_requst: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#" 
    <title>sign in test</title>
    <form action="/reg" method="post">
    <input type="text"  name="n"/>Your login<Br>
    <input type="text" style="-webkit-text-security: disc;" name="n"/>Your password<Br>
    <input type="text" style="-webkit-text-security: disc;" name="n"/>Repeat password<Br>
    <button type="submit">Sign in </button>
    </form>
    "#,
    );
    Ok(response)
}

fn check_exist(login: &str, pwd: &str) -> (u32, u32) {
    let connection = sqlite::open("src/test.db").unwrap();

    let sql_request = format!(
        "SELECT * FROM user WHERE login = '{}' and pwd = '{}' ;",
        login, pwd
    );

    let mut log = 0;
    let mut pass = 0;
    connection
        .iterate(sql_request, |pairs| {
            for &(column, value) in pairs.iter() {
                if (column.trim_end() == "login" && value.unwrap() == login) {
                    log += 1;
                }
                if (column.trim_end() == "pwd" && value.unwrap() == pwd) {
                    pass += 1;
                }
            }
            true
        })
        .unwrap();
    return (log, pass);
}

fn insert_new_info(login: &str, pwd: &str) {
    let connection = sqlite::open("src/test.db").unwrap();

    let sql_request = format!("INSERT INTO user VALUES ('{}', '{}');", login, pwd);
    connection.execute(sql_request).unwrap();
}

fn post_autch(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    let unparsed_string = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parametr\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut string = Vec::new();
    for unparsed in unparsed_string {
        match String::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parametr not a string: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                string.push(n);
            }
        }
    }

    let (log, pass) = check_exist(&string[0], &string[1]);

    if (log == 1 && pass == 1) {
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset= Utf8));
        response.set_mut(format!("Log in succes"));
        Ok(response)
    } else {
        response.set_mut(status::BadRequest);
        response.set_mut(format!("wrong login or password  "));
        return Ok(response);
    }
}

fn post_reg(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    let unparsed_string = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parametr\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut string = Vec::new();
    for unparsed in unparsed_string {
        match String::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parametr not a string: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                string.push(n);
            }
        }
    }

    if (string[1].trim_end() != string[2].trim_end()) {
        response.set_mut(status::BadRequest);
        response.set_mut(format!("Repeated password entered incorrectly"));
        return Ok(response);
    }

    let (log, pass) = check_exist(&string[0], &string[1]);

    if (log == 1) {
        response.set_mut(status::BadRequest);
        response.set_mut(format!(
            "this login already exists, please choose another one "
        ));
        return Ok(response);
    } else {
        insert_new_info(&string[0], &string[1]);
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset= Utf8));
        response.set_mut(format!("Successful registration "));
        Ok(response)
    }
}

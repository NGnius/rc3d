#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::{Content, Response};
use rocket::http::{ContentType, RawStr};
use std::io::{BufWriter, Cursor};

mod converter;
mod sandbox_account;

#[get("/")]
fn index() -> Content<&'static str> {
    // static landing page because laziness
    Content(ContentType::HTML,
"<!DOCTYPE html>
<html>
    <head>
        <title>RC3D</title>
        <meta charset=\"utf-8\"/>
        <script type=\"application/javascript\">function do_the_thing(event) {
            //console.log(event);
            let url = document.getElementById(\"url_input\").value;
            // parse url to get robot id
            const regex = /\\?id=(\\d+)/;
            let m = url.match(regex)
            if (m == null || m.length != 2) { // bad url (no match)
                document.getElementById(\"url_input\").value = \"Invalid URL!\";
                return;
            }
            let id_num = Number(m[1]);
            //console.log(url);
            //console.log(id_num);
            let download_url = window.location.href+\"/api/3d/\"+id_num;
            //console.log(download_url);
            window.location.href = download_url;
        }</script>
    </head>
    <body>
        <div>
            <div>Welcome to RC3D, for all your Robocraft -> 3D model conversion needs*</div>
            <div>
                <label for=\"\">CRF URL:</label>
                <input type=\"text\" id=\"url_input\">
                <button type=\"button\" onclick=\"do_the_thing(event)\">Download</button>
            </div>
            <div>*if those needs only include a janky website and <a href=\"https://en.wikipedia.org/wiki/Wavefront_.obj_file\">Wavefront OBJ</a> model download</div>
            <div><a href=\"https://github.com/NGnius/rc3d\">Source code</a></div>
        </div>
    </body>
</html>")
}

#[get("/api/3d/<id>")]
fn convert<'r>(id: usize) -> Result<Content<Response<'r>>, ()> {
    let buf = Vec::<u8>::new();
    let mut writer = BufWriter::new(buf);
    let result = converter::robot_by_id_to_3d(id)?;
    if let Err(e) = result.data.write_to_buf(&mut writer) {
        println!("Error converting to OBJ: {}", e);
        return Err(());
    }
    let body = Cursor::new(String::from_utf8(writer.into_inner().unwrap()).unwrap());
    if let Ok(resp) = Response::build()
        .raw_header("Content-Disposition", "attachment; filename=\"robot.obj\"")
        .sized_body(body)
        .ok::<()>() {
        return Ok(Content(ContentType::Plain, resp));
    } else {
        println!("Error building response")
    }
    Err(())
}

#[get("/account")]
fn account() -> Content<&'static str> {
    // static page, also because laziness
    Content(ContentType::HTML,
"<!DOCTYPE html>
<html>
    <head>
        <title>Account Info</title>
        <meta charset=\"utf-8\"/>
        <script type=\"application/javascript\">function clickety(event) {
            //console.log(event);
            let name = document.getElementById(\"name_input\").value;
            let pwd = document.getElementById(\"password_input\").value;
            let data = window.btoa(name+\":::\"+pwd);
            let new_url = window.location.origin+\"/api/sandbox/account_info/\"+data;
            //console.log(new_url);
            window.location.href = new_url;
        }</script>
    </head>
    <body>
        <div>
            <div>View info about your user account.
            </div>
            <div>
                <label for=\"name_input\">Username:</label>
                <input type=\"text\" id=\"name_input\">
                <label for=\"password_input\">Password:</label>
                <input type=\"password\" id=\"password_input\">
                <button type=\"button\" onclick=\"clickety(event)\">Get Info</button>
            </div>
            <div>
                <p>
                Generally, it's a bad idea to input credentials into some random website.
                This is using a secure connection and the source code is open source, but still do this at your own risk.
                </p>
            </div>
            <div><a href=\"https://github.com/NGnius/rc3d\">Source code</a></div>
        </div>
    </body>
</html>")
}

#[get("/api/sandbox/account_info/<data>")]
fn account_info<'r>(data: &RawStr) -> Result<Content<Response<'r>>, String> {
    let result = sandbox_account::parse_then_request_username(&data.as_str())?;
    let body = Cursor::new(result);
    if let Ok(resp) = Response::build()
        .sized_body(body)
        .ok::<()>() {
        return Ok(Content(ContentType::JSON, resp));
    } else {
        println!("Error building response")
    }
    Err("Unknown Error".to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![index, convert, account, account_info]).launch();
}


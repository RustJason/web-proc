extern crate iron;
extern crate params;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use iron::status;
use iron::prelude::*;
use iron::mime::Mime;
use params::Params;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct ProcPidStat {
    pid: i32,
    cmd: String
}

#[derive(RustcEncodable)]
struct JsonResp {
    errno: i32,
    errmsg: String,
    data: ProcPidStat
}


fn main() {
    // Read the file contents into a string, returns `io::Result<usize>`

    Iron::new(move |req: &mut Request| {
        let mut path_str = String::from("/proc/");
        println!("{:?}", req.get_ref::<Params>());
        println!("URL path: {:?}", req.url.path());

        if req.url.path()[0].parse::<i32>().is_err() {
            println!("parse int error {}", req.url.path()[0]);
            return Ok(Response::with((status::Ok, "Error")));
        }
        path_str.push_str(req.url.path()[0]);
        path_str.push_str("/");
        path_str.push_str(req.url.path()[1]);

        let path = Path::new(path_str.as_str());
//        let display = path.display();

        let file = File::open(&path);
        if file.is_err() {
            return Ok(Response::with((status::Ok, "file.open error!")));
        }

        let mut s = String::new();
        let x = file.unwrap().read_to_string(&mut s);
        if x.is_err(){
            return Ok(Response::with((status::Ok, "read_to_string error")));
        }

        let v: Vec<&str> = s.split(' ').collect();
        //println!("{:?}", v);
        let content_type = "application/json".parse::<Mime>().unwrap();
        //Ok(Response::with((content_type, status::Ok, v.join("</br>"))))
        let stat = ProcPidStat {pid: v[0].parse::<i32>().ok().unwrap(), cmd: v[1].to_string()};
        let res = JsonResp { errno:0, errmsg:String::from(""), data: stat};
        let out = json::encode(&res).unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    }).http("0.0.0.0:3000").unwrap();
}

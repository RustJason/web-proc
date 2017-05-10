extern crate iron;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use iron::status;
use iron::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {

        println!("argument needed");
        return;
    }
    let mut path_str = String::from("/proc/");
    path_str.push_str(args[1].as_str());
    path_str.push_str("/stat");

    // Read the file contents into a string, returns `io::Result<usize>`

    Iron::new(move |_: &mut Request| {
        let path = Path::new(path_str.as_str());
        let display = path.display();
        let mut s = String::new();
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        if file.read_to_string(&mut s).is_err() {
            println!("can't read file");
            return Ok(Response::with((status::Ok, "read file error!")));
        } 

        let v: Vec<&str> = s.split(' ').collect();
        //println!("{:?}", v);

        Ok(Response::with((status::Ok, v.join("</br>"))))
    }).http("0.0.0.0:3000").unwrap();
}

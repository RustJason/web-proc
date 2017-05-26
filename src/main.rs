extern crate iron;
extern crate params;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use iron::status;
use iron::prelude::*;
use iron::mime::Mime;
use params::Params;

#[derive(Serialize, Deserialize)]
struct ProcPidStat {
    pid: i32,
    cmd: String
}

#[derive(Serialize, Deserialize)]
struct ProcInterrupts {
    name: String,
    values: Vec<i32>
}

#[derive(Serialize, Deserialize)]
struct JsonResp<T> {
    errno: i32,
    errmsg: String,
    data: T
}


fn read_file(path_str: String) -> Result<String, &'static str>{
    
    let path = Path::new(path_str.as_str());

    let file = File::open(&path);
    if file.is_err() {
        return Err("file open error")
    }

    let mut s = String::new();
    let x = file.unwrap().read_to_string(&mut s);
    if x.is_err(){
        return Err("read file error")
    }
    Ok(s)
}

fn parse_interrupts(content: String) -> Result<Vec<ProcInterrupts>, &'static str> {
    let mut lines = content.lines();
    let cpu_count = lines.nth(0).unwrap().split("CPU").count() - 1;
    
    let mut ret = Vec::new();
    for line in  lines {
        if line.starts_with("ERR") || line.starts_with("MIS") {
            continue
        }
        let mut val = Vec::new();
        for cpu_idx in 0..cpu_count {
            let n: String = line.chars().skip(4 + cpu_idx * 11).take(11).collect();
            val.push(n.trim().parse::<i32>().ok().unwrap());
        }
        let r = ProcInterrupts {name: String::from(line.split_at(3).0), values: val};
        ret.push(r);
    }
    Ok(ret)
}


fn main() {
    // Read the file contents into a string, returns `io::Result<usize>`

    Iron::new(move |req: &mut Request| {
        let mut path_str = String::from("/proc/");
        println!("{:?}", req.get_ref::<Params>());
        println!("URL path: {:?}", req.url.path());

        if req.url.path()[0] == "interrupts" {
            
            path_str.push_str(req.url.path()[0]);
            let r = read_file(path_str);
            if r.is_err() {
                return Ok(Response::with((status::Ok, "file.open error!")));
            }
            let r = parse_interrupts(r.unwrap());
            if r.is_err() {
                return Ok(Response::with((status::Ok, "parse_interrupts error!")));
            }
            let out = serde_json::to_string(&r.unwrap());
            let content_type = "application/json".parse::<Mime>().unwrap();
            if out.is_err() {
                return Ok(Response::with((content_type, status::Ok, "tojson error")));
            }
            return Ok(Response::with((status::Ok, out.unwrap())));
        }
        if req.url.path()[0].parse::<i32>().is_err() {
            println!("parse int error {}", req.url.path()[0]);
            return Ok(Response::with((status::Ok, "Error")));
        }

        path_str.push_str(req.url.path()[0]);
        path_str.push_str("/");
        path_str.push_str(req.url.path()[1]);

        let r = read_file(path_str);
        if r.is_err(){
            return Ok(Response::with((status::Ok, "read_to_string error")));
        }
        let s = r.unwrap();

        let v: Vec<&str> = s.split(' ').collect();
        //println!("{:?}", v);
        let content_type = "application/json".parse::<Mime>().unwrap();
        //Ok(Response::with((content_type, status::Ok, v.join("</br>"))))
        let stat = ProcPidStat {pid: v[0].parse::<i32>().ok().unwrap(), cmd: v[1].to_string()};
        let res = JsonResp::<ProcPidStat> { errno:0, errmsg:String::from(""), data: stat};
        let out = serde_json::to_string(&res);
        if out.is_err() {
            return Ok(Response::with((content_type, status::Ok, "tojson error")));
        }
        Ok(Response::with((content_type, status::Ok, out.unwrap())))
    }).http("0.0.0.0:3000").unwrap();
}

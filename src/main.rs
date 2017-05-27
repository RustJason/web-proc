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
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ProcPidStat {
    pid: i32,
    cmd: String
}

#[derive(Serialize)]
struct ProcInterrupts<'a> {
    cpu_count: usize,
    item: HashMap<&'a str, Vec<i32>>,
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

fn parse_interrupts<'a>(content: &'a String) -> Result<ProcInterrupts<'a>, &'static str> {
    let mut lines = content.lines();
    let cpu_count = lines.nth(0).unwrap().split("CPU").count() - 1;
    
    let mut item = HashMap::new();
    for line in  lines {
        if line.starts_with("ERR") || line.starts_with("MIS") {
            continue
        }
        let mut val = Vec::new();
        for cpu_idx in 0..cpu_count {
            let n: String = line.chars().skip(4 + cpu_idx * 11).take(11).collect();
            val.push(n.trim().parse::<i32>().ok().unwrap());
        }
        //let r = (String::from(line.split_at(3).0), val);
        item.insert(line.split_at(3).0, val);
    }
    let ret = ProcInterrupts {cpu_count: cpu_count, item: item};
    Ok(ret)
}

fn process_interrupts() -> Result<iron::response::Response, iron::IronError> {
    let mut path_str: String = String::from("/proc/interrupts");
    let r = read_file(path_str);
    if r.is_err() {
        return Ok(Response::with((status::Ok, "file.open error!")));
    }
    let content = &r.unwrap();
    let r = parse_interrupts(content);
    if r.is_err() {
        return Ok(Response::with((status::Ok, "parse_interrupts error!")));
    }
    let res = JsonResp::<ProcInterrupts> { errno:0, errmsg:String::from(""), data: r.unwrap()};
    let content_type = "application/json".parse::<Mime>().unwrap();
    let out = serde_json::to_string(&res);
    if out.is_err() {
        return Ok(Response::with((content_type, status::Ok, "tojson error")));
    }
    return Ok(Response::with((content_type, status::Ok, out.unwrap())));

}

fn process_pid_stat(pid: usize) -> Result<iron::response::Response, iron::IronError> {

    let mut path_str: String = String::new();
    path_str.push_str(&pid.to_string());
    path_str.push_str("/");
    path_str.push_str("stat");

    let r = read_file(path_str);
    if r.is_err(){
        return Ok(Response::with((status::Ok, "read_to_string error")));
    }
    let s = r.unwrap();

    let v: Vec<&str> = s.split(' ').collect();
    let content_type = "application/json".parse::<Mime>().unwrap();
    //Ok(Response::with((content_type, status::Ok, v.join("</br>"))))
    let stat = ProcPidStat {pid: v[0].parse::<i32>().ok().unwrap(), cmd: v[1].to_string()};
    let res = JsonResp::<ProcPidStat> { errno:0, errmsg:String::from(""), data: stat};
    let out = serde_json::to_string(&res);
    if out.is_err() {
        return Ok(Response::with((content_type, status::Ok, "tojson error")));
    }
    Ok(Response::with((content_type, status::Ok, out.unwrap())))
}

fn main() {
    Iron::new(move |req: &mut Request| {
        let mut path_str = String::from("/proc/");
        println!("{:?}", req.get_ref::<Params>());
        println!("URL path: {:?}", req.url.path());

        if req.url.path()[0] == "interrupts" {
           return process_interrupts() 
        }
        if req.url.path()[0].parse::<i32>().is_err() {
            println!("parse int error {}", req.url.path()[0]);
            return Ok(Response::with((status::Ok, "Error")));
        }
        let pid = req.url.path()[0].parse::<usize>().unwrap();
        return process_pid_stat(pid);

    }).http("0.0.0.0:3000").unwrap();
}

extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::error::Error;
use std::io::prelude::*;
use std::collections::{ HashMap, BTreeMap };
use std::process::{ Command, Stdio };

use nickel::status::StatusCode;
use nickel::{ Nickel, HttpRouter, JsonBody, Mountable, StaticFilesHandler, MediaType, MiddlewareResult, Request, Response };
use rustc_serialize::json::{ Json, ToJson };

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Essay {
    text: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct IpfsObject {
    hash: String,
}

impl ToJson for IpfsObject {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("hash".to_string(), self.hash.to_json());
        Json::Object(map)
    }
}

fn add_to_ipfs(essay: Essay) -> IpfsObject {
    let mut process = match Command::new("ipfs")
                                .args(&["add", "-q"])
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("could'nt spawn ipfs: {}", why.description()),
        Ok(process) => process,
    };

    match process.stdin.as_mut().unwrap().write_all(essay.text.as_bytes()) {
        Err(why) => panic!("couldn't write to ipfs stdin: {}", why.description()),
        Ok(_) => println!("sent essay to ipfs"),

    };

    let result = process
                    .wait_with_output()
                    .expect("failed to wait on ipfs cat");

    let result = String::from_utf8(result.stdout).unwrap();
    let hash = result.trim();
    IpfsObject { hash: String::from(hash) }
}

// it is recommended to proxy to go-ipfs instead in production
fn get_from_ipfs(hash: &str) -> String {
    let process = match Command::new("ipfs")
                                .args(&["cat", hash])
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("could'nt spawn ipfs: {}", why.description()),
        Ok(process) => process,
    };

    let result = process
                    .wait_with_output()
                    .expect("failed to wait on ipfs cat");

    String::from_utf8(result.stdout).unwrap()
}

fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger middleware: {:?}", req.origin.uri);
    res.next_middleware()
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(logger_fn);

    server.get("/", middleware! { |_, res|
        let mut data = HashMap::new();
        data.insert("name", "user");
        return res.render("assets/index.html", &data);
    });

    server.get("/e/:hash", middleware! { |req, res|
        let hash = req.param("hash").unwrap();

        let mut data = HashMap::new();
        data.insert("hash", hash);
        return res.render("assets/view.html", &data);
    });

    server.get("/ipfs/:hash", middleware! { |req, mut res|
        res.set(MediaType::Txt);
        let hash = req.param("hash").unwrap();

        println!("get ipfs object: {}", hash);
        let result = get_from_ipfs(hash);

        println!("ipfs responded: {:?}", result);
        result
    });

    server.post("/publish", middleware! { |req, res|
        let essay = try_with!(res, {
            req.json_as::<Essay>().map_err(|e| (StatusCode::BadRequest, e))
        });
        println!("{:?}", essay);

        let obj = add_to_ipfs(essay);
        println!("{:?}", obj);

        obj.to_json()
    });

    server.mount("/assets/", StaticFilesHandler::new("assets/"));

    server.listen("127.0.0.1:6969").unwrap();
}

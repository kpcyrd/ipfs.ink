extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::error::Error;
use std::io::prelude::*;
use std::collections::{ HashMap, BTreeMap };
use std::process::{ Command, Stdio };

use nickel::status::StatusCode;
use nickel::{ Nickel, JsonBody, Mountable, StaticFilesHandler, MediaType, MiddlewareResult, Request, Response };
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
    let mut process = match Command::new("contrib/ipfs-add")
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
                    .expect("failed to wait on ipfs add");

    let result = String::from_utf8(result.stdout).unwrap();
    let hash = result.trim();
    IpfsObject { hash: String::from(hash) }
}

// it is recommended to proxy to go-ipfs instead in production
fn get_from_ipfs(hash: &str) -> String {
    let process = match Command::new("contrib/ipfs-cat")
                                .arg(hash)
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

    server.utilize(router! {
        get "/" => |_, res| {
            let mut data = HashMap::new();
            data.insert("name", "user");
            return res.render("templates/index.html", &data);
        }

        get "/e/:hash" => |req, res| {
            let hash = req.param("hash").unwrap();

            let mut data = HashMap::new();
            data.insert("hash", hash);
            return res.render("templates/view.html", &data);
        }

        get "/ipfs/:hash(/:file\\.:ext)?" => |req, mut res| {
            res.set(MediaType::Txt);
            let hash = req.param("hash").unwrap();

            let path = match req.param("file") {
                Some(file) => {
                    let ext = req.param("ext").unwrap();
                    format!("/ipfs/{}/{}.{}", hash, file, ext)
                },
                None => format!("/ipfs/{}", hash),
            };

            println!("get ipfs object: {}", path);
            let result = get_from_ipfs(path.as_str());

            println!("ipfs responded: {:?}", result);
            result
        }

        post "/publish" => |req, res| {
            let content_type = match req.origin.headers.get_raw("content-type") {
                Some(header) => {
                    String::from_utf8(header[0].clone()).unwrap()
                },
                None => String::new()
            };
            assert_eq!(content_type, "application/json");

            let essay = try_with!(res, {
                req.json_as::<Essay>().map_err(|e| (StatusCode::BadRequest, e))
            });
            println!("{:?}", essay);

            let obj = add_to_ipfs(essay);
            println!("{:?}", obj);

            obj.to_json()
        }
    });

    server.mount("/assets/", StaticFilesHandler::new("public/assets/"));

    server.keep_alive_timeout(None);
    server.listen("0.0.0.0:6767").unwrap();
}

extern crate rustc_serialize;
extern crate hyper;
extern crate multipart;
#[macro_use] extern crate nickel;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use nickel::status::StatusCode;
use nickel::{ Nickel, JsonBody, Mountable, StaticFilesHandler, MediaType, MiddlewareResult, Request, Response };

mod ipfs;
mod structs;

use structs::Essay;

fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger middleware: {:?}", req.origin.uri);
    res.next_middleware()
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(logger_fn);

    #[allow(resolve_trait_on_defaulted_unit, unreachable_code)]
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
            let result = ipfs::cat(&path).unwrap();

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

            let obj = ipfs::add(essay).unwrap();
            println!("{:?}", obj);

            serde_json::to_string(&obj).unwrap()
        }
    });

    server.mount("/assets/", StaticFilesHandler::new("public/assets/"));

    server.keep_alive_timeout(None);
    server.listen("0.0.0.0:6767").unwrap();
}

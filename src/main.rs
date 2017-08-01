extern crate rustc_serialize;
extern crate hyper;
extern crate multipart;
#[macro_use] extern crate nickel;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::env;
use std::collections::HashMap;

use nickel::status::StatusCode;
use nickel::{ Nickel, JsonBody, Mountable, StaticFilesHandler, MediaType, MiddlewareResult, Request, Response };

mod ipfs;
mod structs;
mod webpack;

use structs::Essay;

fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger middleware: {:?}", req.origin.uri);
    res.next_middleware()
}

fn set_prefetch<'mw>(res: &mut Response<'mw>, prefetch: &Vec<u8>) {
    res.headers_mut().set_raw("Link", vec![prefetch.clone()]);
}

fn main() {
    let mut server = Nickel::new();

    let (bundle, style) = match env::var("WEBPACK_DEV") {
        Ok(_) => {
            println!("Using developer asset paths");
            (String::from("bundle.js"), String::from("style.css"))
        },
        Err(_) => {
            let assets = webpack::load("webpack-assets.json")
                            .expect("failed to load webpack-assets.json, set WEBPACK_DEV=1 for development");

            let bundle = webpack::find(&assets, "js", "bundle.js").expect("couldn't find bundle.js").clone();
            let style = webpack::find(&assets, "css", "style.css").expect("couldn't find style.css").clone();

            (bundle, style)
        },
    };

    let prefetch = format!("</assets/{}>; rel=prefetch, </assets/{}>; rel=prefetch", style, bundle).into_bytes();

    // workaround for capturing router! macro
    let bundle_ = bundle.clone();
    let style_ = style.clone();
    let prefetch_ = prefetch.clone();

    server.utilize(logger_fn);

    #[allow(resolve_trait_on_defaulted_unit, unreachable_code)]
    server.utilize(router! {
        get "/" => |_, mut res| {
            set_prefetch(&mut res, &prefetch);

            let mut data = HashMap::new();
            data.insert("name", "user");
            data.insert("bundle_js", &bundle);
            data.insert("style_css", &style);
            return res.render("templates/index.html", &data);
        }

        get "/e/:hash" => |req, mut res| {
            set_prefetch(&mut res, &prefetch_);

            let hash = req.param("hash").unwrap();

            let mut data = HashMap::new();
            data.insert("hash", hash);
            data.insert("bundle_js", &bundle_);
            data.insert("style_css", &style_);
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

use futures::{future,Future};
use hyper::{Body,Error,Method,Request,Response,Server,StatusCode};
use hyper::service::service_fn;
use lazy_static::lazy_static;
use regex::Regex;
use slab::Slab;
use std::fmt;
use std::sync::{Arc,Mutex};

type UserId = u64;
type UserDb = Arc<Mutex<Slab<UserData>>>;

struct UserData;

lazy_static!{
  static ref INDEX_PATH: Regex = Regex::new("^/(index\\.html?)?$").unwrap();
  static ref USER_PATH: Regex = Regex::new("^/user/((?P<user_id>\\d+?)/?)?$").unwrap();
  static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
}

impl fmt::Display for UserData {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
    f.write_str("{}")
  }
}

fn main() {
  //create address to bind to
  let addr = ([127,0,0,1],8080).into();
  //create a server instance
  let builder = Server::bind(&addr);
  // create a userdb (shared state) instance
  let user_db = Arc::new(Mutex::new(Slab::new()));
  //set a requests handler
  let server = builder.serve(move || {
    let user_db = user_db.clone();
    service_fn(move |req| microservice_handler(req, &user_db))
  }); 
  //drop any errors
  let server = server.map_err(drop);
  //add service instance to runtime
  hyper::rt::run(server);
}

fn microservice_handler(req:Request<Body>,user_db:&UserDb) -> impl Future<Item=Response<Body>,Error=Error> {
  let response = {
    let method = req.method();
    let path = req.uri().path();
    let mut users = user_db.lock().unwrap();
    
    if INDEX_PATH.is_match(path) {
      if method == &Method::GET {
        Response::new(INDEX.into())
      } else {
        response_with_code(StatusCode::METHOD_NOT_ALLOWED)
      }
    } else if USERS_PATH.is_match(path) {
      if method == &Method::GET {
        let list = users.iter()
          .map(|(id,_)|id.to_string())
          .collect::<Vec<String>>()
          .join(",");
        Response::new(list.into())
      } else {
        response_with_code(StatusCode::METHOD_NOT_ALLOWED)
      }
    } else if let Some(cap) = USER_PATH.captures(path) {
      let user_id = cap.name("user_id").and_then(|m| {
        m.as_str()
         .parse::<UserId>()
         .ok()
         .map(|x| x as usize) //this may not work on some systems
      });
      match(method,user_id) {
        //POST, Create
        (&Method::POST,None) => {
          let id = users.insert(UserData);
          Response::new(id.to_string().into()) //HTTP 200 (default Response value)
        },
        (&Method::POST,Some(_)) => {
          response_with_code(StatusCode::BAD_REQUEST)  //HTTP 400
        },
        //GET, Read
        (&Method::GET,Some(id)) => {
          if let Some(data) = users.get(id){
            Response::new(data.to_string().into())
          } else {
            response_with_code(StatusCode::NOT_FOUND)  //HTTP 404
          }
        },
        //PUT, Update
        (&Method::PUT,Some(id)) => {
          if let Some(user) = users.get_mut(id) {
            //deref and replace
            *user = UserData;
            response_with_code(StatusCode::OK)  //HTTP 200
          } else {
            response_with_code(StatusCode::NOT_FOUND)  //HTTP 404
          }
        }, 
        //DELETE
        (&Method::DELETE,Some(id)) => {
          if users.contains(id) {
            users.remove(id);
            response_with_code(StatusCode::OK)
          } else {
            response_with_code(StatusCode::NOT_FOUND)
          }
        },
        _ => response_with_code(StatusCode::METHOD_NOT_ALLOWED) //HTTP 405  
      }
    } else {
      response_with_code(StatusCode::NOT_FOUND)
    }
  };
  future::ok(response)
}

fn response_with_code(status_code: StatusCode) -> Response<Body> {
  Response::builder().status(status_code)
                     .body(Body::empty())
                     .unwrap()
}

//an index page
const INDEX:&'static str = r#"
<!doctype html>
<HTML>
  <HEAD>
    <TITLE>Some Rust Microservice</TITLE>
  </HEAD>
  <BODY>
    <H3>Some Rust Microservice</H3>
  </BODY>
</HTML>
"#;

use hyper::{Body,Response,Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() {
  //create address to bind to
  let addr = ([127,0,0,1],8080).into();
  //create a server instance
  let builder = Server::bind(&addr);
  //set a requests handler
  let server = builder.serve(|| {
    service_fn_ok(|_| {
      let random_byte = rand::random::<u8>();
      Response::new(Body::from(random_byte.to_string()))
    })
  }); 
  //drop any errors
  let server = server.map_err(drop);
  //add service instance to runtime
  hyper::rt::run(server);
}

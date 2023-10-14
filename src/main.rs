use hyper::{Body,Response,Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use log::{debug,info,trace};

fn main() {
  //initialize logging
  logger::init();
  info!("Rando Microservice - v0.1.0");
  trace!("Starting...");
  //create address to bind to
  let addr = ([127,0,0,1],8080).into();
  debug!("Trying to bind server to address: {}", addr);
  //create a server instance
  let builder = Server::bind(&addr);
  trace!("Creating service handler...");
  //set a requests handler
  let server = builder.serve(|| {
    service_fn_ok(|_| {
      trace!("Incoming request is: {:?}",req);
      let random_byte = rand::random::<u8>();
      debug!("Generated value is: {}", random_byte);
      Response::new(Body::from(random_byte.to_string()))
    })
  }); 
  info!("Used address: {}",server.local_addr());
  //drop any errors
  let server = server.map_err(drop);
  //add service instance to runtime
  debug!("Run!");
  hyper::rt::run(server);
}

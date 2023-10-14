use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App};
use dotenv::dotenv;
use hyper::{Body,Response,Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use log::{debug,info,trace};
use std::env;

fn main() {
  dotenv().ok();
  //initialize logging
  env_logger::init();
  //parse environmental vars
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(Arg::with_name("address")
       .short("a")
       .long("address")
       .value_name("VAR_ADDRESS")
       .help("Sets an address")
       .takes_value(true))
    .arg(Arg::with_name("config")
       .short("c")
       .long("config")
       .value_name("VAR_CONFIG_FILE")
       .help("Sets a custom config file")
       .takes_value(true))
    .get_matches(); 
  info!("Rando Microservice - {:?}", matches.value_of("version"));
  trace!("Starting...");
  //determine address to bind to
  let addr = matches.value_of("address")
    .map(|s|s.to_owned())
    .or(env::var("VAR_ADDRESS").ok())
    .unwrap_or_else(|| "127.0.0.1:8080".into())
    .parse()
    .expect("can't parse VAR_ADDRESS variable");  
  debug!("Trying to bind server to address: {}", addr);
  //create a server instance
  let builder = Server::bind(&addr);
  trace!("Creating service handler...");
  //set a requests handler
  let server = builder.serve(|| {
    service_fn_ok(|req| {
      trace!("Incoming request is: {:?}", req);
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

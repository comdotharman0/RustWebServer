use std::sync::Arc;
mod webserver;
use std::error::Error;
use webserver::WebServer;
fn main()->Result<(),Box<dyn Error>> {
    let mut  app = WebServer::new(4);
app.route("/","src/templates/index.html")?;
app.route("/formresponse",
"src/templates/formresponse.html")?;
Arc::new(app).run("127.0.0.1:8080")?;
    println!("Hello, world!");
Ok(())
}

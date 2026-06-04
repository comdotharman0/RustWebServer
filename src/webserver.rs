use std::{net::{TcpStream,TcpListener},
io::{Error,Write,BufReader,prelude::*},
fs,process::Command};
use std::{sync::{Arc,Mutex,mpsc},thread};
use std::collections::HashMap;
mod requestparser;
//use requestparser;


type Job = Box<dyn FnOnce() +Send + 'static>;


#[derive(Debug)]
pub struct WebServer
{
pub pages: HashMap<String,String>,
//pub pages:  Vec<PageRoute>
pub sender: mpsc::Sender<Job>,
pub workers: Vec<Worker>
}

#[derive(Debug)]
pub struct Worker{
id: usize,
thread: thread::JoinHandle<()>
}


#[derive(Debug)]
pub struct PageRoute
{
pub page_path:String,
pub page_content: String
}

impl Worker{
fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>>)
->Worker{
let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
}
}


impl WebServer
{
pub fn new(size:usize)->Self{
assert!(size>0);
let (sender,receiver) = mpsc::channel();
let receiver = Arc::new(Mutex::new(receiver));
let mut workers=Vec::with_capacity(size);
(0..size).for_each(|i|{
workers.push(Worker::new(i,Arc::clone(&receiver)));
});
WebServer{
pages: HashMap::new(),
sender,
workers
}
}


pub fn route(&mut self,page_path:&str,
file_path:&str)->Result<(),Error>{
self.pages.insert(
page_path.to_string(),
fs::read_to_string(file_path)?
);
Ok(())
}

pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }


pub fn run_command(&self,command:&str)->
Result<String,Error>{
let command: Vec<_>= command
.split_whitespace()
.collect();
let output = Command::new(&command[0])
.args(&command[1..]).output()?;
Ok(String::from_utf8(output.stdout)
.expect("invalid characters"))

}



pub fn handle_connections(&self,mut stream:TcpStream)->
Result<(),Error>{ 

/*let content = fs::read_to_string(
"src/templates/index.html")?;
*/
let  buf_reader = BufReader::new(&stream);
 /*let http_request = buf_reader
   .lines().next().unwrap()?;
    println!("Request: {:#?}",http_request);  
*/

let reqpar = requestparser::Request::parse(
buf_reader);
//println!("Request: {:#?}",reqpar);	
stream.write_all(
format!(
"HTTP/1.1 200 OK \r\n\r\n{}",self.pages.get(
&reqpar?.url).map(|s| s)
        .unwrap_or(&"404 Not Found".to_string()))
.as_bytes())?;

//println!("{:#?}",&reqpar);
Ok(())
}




pub fn run(self:Arc<Self>,addr:&str)->Result<(),Error>{
let listener = TcpListener::bind(addr)?;
println!("Running on address {addr}!");
for stream in listener.incoming(){
let stream=stream.unwrap();
let self2 = Arc::clone(&self);
self.execute(move ||{
self2.handle_connections(stream).unwrap_or_else(|e|{
println!("Error is {:#?}",e);
});
});
}
Ok(())

}






}



impl Drop for WebServer {
    fn drop(&mut self) {
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}


use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Cada worker es responsable de ejecutar tareas.
// Un solo worker mantiene un hilo y escucha las tareas enviadas a través de un canal
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // Lógica de la ejecución de la tarea
            loop {
                // Recibir y ejecutar la tarea
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// Job es un alias de tipo para un puntero a una función que toma cero argumentos,
// devuelve (), se puede llamar una vez, es seguro para enviar a través de threads
// y tiene una duración estática.
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); // Crear un canal para comunicación

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Clonar el sender para cada worker
            let sender = sender.clone();
            // Clonar el receptor envuelto en Arc y Mutex
            let receiver = Arc::clone(&receiver);
            // Crear y almacenar los workers
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\nHello from Rust server!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // Adjust the number of threads in the pool as needed

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

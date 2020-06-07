

// Pseudo and not ACTUAL implementation of executor and block_on_all

struct Executor;

impl Executor {
    fn run_all(&mut self, Vec<Future>) -> Vec<Result<Future::Item, Future::Error> { // see line 23
        // polling all Futures
        let mut done = 0;
        let mut results = Vec::with_capacity(futures.len())
        while done!=futures.len() {
            for f in futures.iter_mut().enumerate(){
                match f.poll(){ // POLL == DO MORE WORK
                    Ok(Async::Ready(t)) => {
                        results[i] = Ok(t)
                        done++;
                    }ll
                    Err(e) => {
                        results[i] = Err(e)
                        done++;
                    }
                    Ok(Async::NotReady(t)) => {
                        continue;
                    }
                }
            }
        }
    }




// POLL RUST DOC. DEFN OLD

type Poll<T,E> = Result<Async<T>,E>;

// Async
// https://docs.rs/futures/0.1.29/futures/enum.Async.html
//
pub enum Async<T> {
    Ready(T), 
    NotReady,
}



// Pseudo and not ACTUAL implementation of executor and block_on_all

struct Executor(Arc<Mute<Vec<bool>>>);
struct MyNotifier(Mutex<Vec<bool>>);

impl Notify for MyNotifier {

    fn notify(&self, id:usize) {
        self.0.lock()[id] = true;
    }
        
}


impl Executor {
    fn run_all(&mut self, Vec<Future>) -> Vec<Result<Future::Item, Future::Error> { // see line 23
        // polling all Futures
        let mut done = 0;
        
        let mut results = Vec::with_capacity(futures.len())
        let mut tasks = Vec::with_capacity(futures.len())
        let nf = Arc::new(MyNotifier(Mutex::new(vec![true; futures.len ]);
        let notifier = NotifyHandle::from(nf);

            
            
        while done!=futures.len() {
            for (i,f) in futures.iter_mut().enumerate(){
                
                // Don't pull futures that can't make progress
                //if !tasks[i].notified() {
                //    continue;
                //}
            
                if was_notified = self.0.lock() {
                if !was_notified[i] {
                    continue;
                }
                was_notified[i] = false;

                match exeutor::with_notify(&notifier, i, || f.poll()) {
                //match f.poll(){ // POLL == DO MORE WORK. Calls futures::task::current internally
                    // task::curren() - Task
                    // let t: Task;
                    // t.notify()
                    Ok(Async::Ready(t)) => {
                        results[i] = Ok(t)
                        done++;
                    }
                    Err(e) => {
                        results[i] = Err(e)
                        done++;
                    }
                    Ok(Async::NotReady(t)) => {
                        // meanwhile ..
                        // Another Thread T notices a network packet arrived. Which wakes up thread
                        // using Task::notify 
                        // f *must* have arranged for task[i] (its task) to be notified later
                        continue;
                    }
                }
            }

            
        
        }
        results

    }

}


// -------------------------------------------------------------------------------

// POLL RUST DOC. DEFN OLD

type Poll<T,E> = Result<Async<T>,E>;

// Async
// https://docs.rs/futures/0.1.29/futures/enum.Async.html
//
pub enum Async<T> {
    Ready(T), 
    NotReady,
}

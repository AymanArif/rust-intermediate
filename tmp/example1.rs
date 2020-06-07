
// Pseudo code
fn main() {
    
    // Streaming world
    let x = TcpStream::connect("127.0.0.1");
    let y = TcpStream::connect("127.0.0.1");
    x.write("foobarx");
    y.write("foobary");
    assert_eq!(x.read, "barfoo");
    assert_eq!(y.read, "barfoo");
    // Con of above: y is waiting for x to finish. 
    // Better approach would be to send x and y simultaneously; 
    // and wait for response based on first-come-first basis.





    // Future World
    // x
    let fut_x = TcpStream::connect("127.0.0.1")
                .and_then(|connection| connection.write("foobarx"))
                .and_then(|connection| connection,read())
                .and_then(|connection, bytes| byte ==  "barfoo"))
    println("{:?}", fut_x); // doesn't return boolean instantly.
    
    // y
    let fut_y = TcpStream::connect("127.0.0.1")
                .and_then(|connection| connection.write("foobarx"))
                .and_then(|connection| connection,read())
                .and_then(|connection, bytes| byte ==  "barfoo"))

    println("{:?}", fut_y); // doesn't return boolean instantly.

    // Executor will return the future value
    let a : Executor;
        
    let x = a.run(fut_x)
    let y = a.run(fut_y)

    // Spawn: Tells Executor to run future in the background    
    a.spawn(fut_x.and_then(|eq| assert!(eq))
    b.spawn(fut_y.and_then(|eq| assert!(eq))
            
}


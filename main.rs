
use std::io::timer;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

static N : uint = 5;

fn philosopher(seat : uint, vec : &[Mutex<uint>]) {
    let leftFork = if seat != 0 { seat } else { (seat + 1) % N };
    let rightFork = if seat != 0 { (seat + 1) % N } else { seat };
    println!("Philosopher {} {} {}", seat, leftFork, rightFork);
    let ref left = vec[leftFork];
    let ref right = vec[rightFork];
    loop {
        let f1 = left.lock();
        println!("Philosopher {} picks up fork {}", seat, *f1);
        timer::sleep(Duration::milliseconds(250));
        let f2 = right.lock();
        println!("Philosopher {} picks up fork {}", seat, *f2);
        timer::sleep(Duration::milliseconds(250));
        println!("Philosopher {} is eating", seat);
        timer::sleep(Duration::milliseconds(250));
        println!("Philosopher {} puts down forks", seat);
    }
}

fn main() {
    let mut vec = Vec::new();
    for i in range(0, N) {
        vec.push(Mutex::new(i));
    }
    let shared_vec = Arc::new(vec);
    for seat in range(0, N) {
        let child_vec = shared_vec.clone();
        spawn(proc() { 
            let local_vec = child_vec.as_slice();
            philosopher(seat, local_vec) 
        });
    }
}

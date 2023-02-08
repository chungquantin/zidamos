use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};

unsafe fn unsafe_call_fork() {
    let pid = fork();

    //From the programmer’s point of view, a call to fork returns twice: once in the context of the running parent process, and once in the context of the running child process.
    match pid.expect("Fork Failed: Unable to create child process!") {
        Child => println!(
            "Hello from child process with pid: {} and parent pid:{}",
            getpid(),
            getppid()
        ),
        Parent { child } => {
            wait().unwrap();
            println!(
                "Hello from parent process with pid: {} and child pid:{}",
                getpid(),
                child
            );
        }
    }
}

fn main() {
    unsafe {
        unsafe_call_fork();
    }
}

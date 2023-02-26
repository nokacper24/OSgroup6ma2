# Mandatory assignment 2 Operating Systems
This repository contains the mandatory assignment 2 for the course [IDATA2305 - Operating Systems with System Programming](https://www.ntnu.edu/studies/courses/IDATA2305#tab=omEmnet) at NTNU.  

## Assignment
Link to the assignment: [Assignment 2](https://piperunner.notion.site/Assignment-2-d272273858a1411888091762c8c9361e#8ea51934bef341f887e6e069486cf99b)  

## How to run
You can either compile the code yourself or use the precompiled binaries in the [releases](https://github.com/nokacper24/OSgroup6ma2/releases).  
In order to compile you will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.  

You can run the program by executing the binary file, or by running `cargo run` in the root directory of the project.

## Observations
When running the program for the first time, already then the logs look correct.  
This is because we've used `Arc` and `Mutex` to make sure that the `TicketServer` is only accessed by one thread at a time. We were *forced* to use synchronization due to Rust's strict ownership rules. Because of that we couldn't observe the "anomaly".  
Logs from an example run:
```
Thread: main [INFO] - Starting the ticket server...
Thread: T1 [INFO] - Hi, Bob! There are 15 seats available for The Matrix and you'd like to buy 5.
Thread: T1 [INFO] - Enjoy the movie, Bob! Remaining seats: 10
Thread: T2 [INFO] - Hi, Alice! There are 10 seats available for The Matrix and you'd like to buy 3.
Thread: T2 [INFO] - Enjoy the movie, Alice! Remaining seats: 7
Thread: T4 [INFO] - Hi, Thomas! There are 7 seats available for The Matrix and you'd like to buy 5.
Thread: T4 [INFO] - Enjoy the movie, Thomas! Remaining seats: 2
Thread: T3 [INFO] - Hi, Jake! There are 2 seats available for The Matrix and you'd like to buy 1.
Thread: T3 [INFO] - Enjoy the movie, Jake! Remaining seats: 1
Thread: T6 [INFO] - Hi, Jane! There are 1 seats available for The Matrix and you'd like to buy 3.
Thread: T6 [ERROR] - Sorry Jane, we have 1 seats left.
Thread: T5 [INFO] - Hi, John! There are 1 seats available for The Matrix and you'd like to buy 2.
Thread: T5 [ERROR] - Sorry John, we have 1 seats left.
```

We can see that the `TicketServer` is accessed by multiple clients at the same time, the number of seats remains correct during the whole process.

### The anomaly
It seems that the intention of the assignment was to observe the "anomaly" that occurs when the `TicketServer` is accessed by multiple clients simultaneously. I've decided to experiment with `unsafe` code to see if I can reproduce the anomaly in Rust. The code is on the [`broken`](https://github.com/nokacper24/OSgroup6ma2/tree/broken) branch.  
Here's the output from the **unsafe** program:
```
Thread: main [INFO] - Starting the ticket server...
Thread: T1 [INFO] - Hi, Bob! There are 15 seats available for The Matrix and you'd like to buy 5.
Thread: T2 [INFO] - Hi, Alice! There are 15 seats available for The Matrix and you'd like to buy 3.
Thread: T2 [INFO] - Enjoy the movie, Alice! Remaining seats: 7
Thread: T1 [INFO] - Enjoy the movie, Bob! Remaining seats: 10
Thread: T4 [INFO] - Hi, Thomas! There are 10 seats available for The Matrix and you'd like to buy 5.
Thread: T5 [INFO] - Hi, John! There are 10 seats available for The Matrix and you'd like to buy 2.
Thread: T5 [INFO] - Enjoy the movie, John! Remaining seats: 0
Thread: T6 [INFO] - Hi, Jane! There are 7 seats available for The Matrix and you'd like to buy 3.
Thread: T4 [INFO] - Enjoy the movie, Thomas! Remaining seats: 2
Thread: T3 [INFO] - Hi, Jake! There are 15 seats available for The Matrix and you'd like to buy 1.
Thread: T6 [ERROR] - Sorry Jane, we have 0 seats left.
Thread: T3 [ERROR] - Sorry Jake, we have 0 seats left.
```
We can see that the `TicketServer` is accessed by multiple threads (clients) at the same time, the number of seats reported by the server is incorrect. For example, when Alice on T2 wants to book 3 tickets. Server first reports that there are 15 searts available, then server processes Alice's request and reports that there are 7 seats left. $15 - 3 \neq 7$  
This example demonstrates that accessing shared mutable data without proper synchronization mechanisms can lead to unexpected results.

## why Arc and Mutex solves the anomaly
The `Arc` and `Mutex` are synchronization primitives that allow us to share data between threads.
`Arc` stands for *Atomic Reference Counting* and it is a thread-safe reference counting pointer. It allows us to share data between threads.
`Mutex` stands for *Mutual Exclusion* and it is a synchronization primitive that allows us to share mutable data between threads. It allows only one thread to access the data at a time.

In our case we need to share the `TicketServer` between threads, so we use `Arc` to do that. We also need to share mutable data between threads, so we use `Mutex` to do that.

Arc and Mutex work together to make sure that the `TicketServer` is only accessed by one thread at a time. This way we can be sure that the `TicketServer` is accessed by only one client at a time, and the number of seats reported by the server is correct.


# Rust Prime Number Search Benchmark
Search prime number in a multi threaded environment 



# Running 

## Command line argument help
 ```
./PrimeNumberSearchMT --help
Usage: PrimeNumberSearchMT [OPTIONS]

Options:
  -s, --starting-number <STARTING_NUMBER>
          Starting number [default: 100000000]
  -d, --start-delta-between-thread <START_DELTA_BETWEEN_THREAD>
          Used for setting other threads Starting number [default: 1000]
  -c, --count <COUNT>
          Number of primes [default: 10]
  -t, --thread-count <THREAD_COUNT>
          Thread count [default: 4]
  -v, --verbose
          Activate verbose mode
  -h, --help
          Print help
```

## Sample output when run with default parameters

```

./PrimeNumberSearchMT       
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Command line args : CommandLineArgs {
        starting_number: 100000000,
        start_delta_between_thread: 1000,
        count: 10,
        thread_count: 4,
        verbose: false,
    }
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Starting search with 4 threads
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Staring thread with start number and count: 100000000 10
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Staring thread with start number and count: 100001000 10
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Staring thread with start number and count: 100002000 10
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Staring thread with start number and count: 100003000 10
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Thread 4 ended
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Thread 2 ended
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Thread 1 ended
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Results: [100000007, 100000037, 100000039, 100000049, 100000073, 100000081, 100000123, 100000127, 100000193, 100000213]
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Results: [100001029, 100001053, 100001059, 100001081, 100001087, 100001107, 100001119, 100001131, 100001147, 100001159]
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Thread 3 ended
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Results: [100002011, 100002013, 100002017, 100002031, 100002037, 100002059, 100002103, 100002113, 100002127, 100002139]
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Results: [100003021, 100003037, 100003063, 100003091, 100003151, 100003153, 100003157, 100003181, 100003219, 100003243]
[2025-11-30T07:30:51Z INFO  PrimeNumberSearchMT] Finished search in 571 ms 

``

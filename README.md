# BITENC

A simple send / receive client with some additional tests for
[BITE](https://github.com/alvivar/bite) to make sure that

To send and receive, just use **cargo run**

To test use **cargo test -- --test-threads=1**

Every test needs to run consecutively, that's why we use just one thread. And
you need to restart the server between runs because I need to deal with the
consecutive id asigned on the server, but maybe later.

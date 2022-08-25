# BITENC

Basically, a simple suit to test [BITE](https://github.com/alvivar/bite).

Every test needs to run consecutively, so use just one thread. And you need to
restart the server between runs because I need to deal with the consecutive id
asigned on **BITE** in the test, but maybe later.

    cargo test -- --test-threads=1

If you want a client to send simple messages, just use **cargo run**

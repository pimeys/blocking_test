# Herp derp

Multi-threaded version:

``` bash
cargo +nightly run -- --threaded
```

Single-threaded blocking version:

``` bash
cargo +nightly run -- --sync
```

Asynchronous version:

``` bash
cargo +nightly run -- --async
```

The service runs in `localhost:8080` and can be used with a web client.

To save a new query:

``` bash
curl -d 'SELECT 1 as one' localhost:8080/one
```

Then as long as the server is running, the query can be executed by executing a
`GET` request to the uri:

``` bash
curl localhost:8080/one
```

Output should be the result as JSON:

``` json
[{"one":1}]
```

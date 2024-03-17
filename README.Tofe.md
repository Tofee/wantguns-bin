Notes to myself
---------------

* To build and deploy at home:
  
```cargo build --release --target aarch64-unknown-linux-gnu && ls -lh target/aarch64-unknown-linux-gnu/release/bin```

* To release new binaries:
  * Create a proper tag like v2.x.x
  * Wait for Build-CI to finish for this tag
  * Retrieve the files in build-artefacts of the Action
  * Create a new release with the tag, and attach the binaries to the release

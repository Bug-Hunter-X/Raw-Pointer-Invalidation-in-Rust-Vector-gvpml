# Raw Pointer Invalidation in Rust
This repository demonstrates a common error in Rust when working with raw pointers and vectors. Modifying a vector's elements through a raw pointer after operations that might reallocate the vector's internal memory can lead to undefined behavior.

The `bug.rs` file contains the erroneous code, and the `bugSolution.rs` file provides a safer, more idiomatic solution.  The core issue is failing to maintain the validity of the raw pointer after potential memory reallocations within the vector.

This example highlights the importance of using safe Rust techniques when manipulating vector data to avoid such issues.
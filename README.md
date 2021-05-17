# librust
Library for using Rust functions and data structures from C.


Have to noodle on generics in C without templates. Might have to do compile time Rust to generate the bindings.

One application would be adding Rust backed data structues to NGINX JS, cruby, and cpython - along with several C and C++ projects.


```c
#include<rust/HashMap.h>

int main(){

hashmap_type t;
assert(hasmap_allocate(t));

assert(hashmap_insert(t,"string", "string2"));
assert(hashmap_free(t));
return 0;
}
```

https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html


https://doc.rust-lang.org/std/collections/index.html

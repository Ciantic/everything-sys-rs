# VoidTools' Everything bindings in Rust

This just wraps the VoidTools' bindings, nothing else.

## Example

This example requires `widestring` package along with `everything-sys`. This
same example can be found from `examples` directory.

```rust
use everything_sys::*;
use widestring::U16CString;

pub fn search(query: &str) {
    unsafe {
        let query_as_wchar = U16CString::from_str(query).unwrap();
        Everything_SetSearchW(query_as_wchar.as_ptr());
        if Everything_QueryW(1) == 1 {
            let res = Everything_GetNumResults();
            for i in 0..res {
                let filename =
                    U16CString::from_ptr_str(Everything_GetResultFileNameW(i)).to_string_lossy();
                let path = U16CString::from_ptr_str(Everything_GetResultPathW(i)).to_string_lossy();
                println!("{} {}", path, filename);
            }
        }
    }
}

pub fn main() {
    search("notepad*")
}
```

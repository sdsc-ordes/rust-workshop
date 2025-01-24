// This exercise practices to implement `Drop` on a type while retaining error
// handling.
//
// We have a type `TempFile` which should simulate some resource which you acquire
// and initialize with `new` (RAII) and which implements `Drop` which will remove
// the file from the disc whenever it gets dropped.
//
// Returning errors from `fn Drop::drop(&mut self)` is not possible. We could
// ignore errors, but in this exercise we want to **leave no loose** ends and not
// just ignore errors. That is why we implement an explicit destructor `close`
// which takes ownership by taking `self` as value and returns
// `Result<(), ErrorHandle>` in case something happens.
//
// The code below compiles, but if you try to call `remove` in `close` the compiler
// will tell you that you cannot partially move out values before `Drop::drop` is
// called at function end. Also calling `remove` in `Drop` does not work because
// `Drop::drop` does not own `self` it only has a `&mut self`.
//
// The exercise is to solve these issues by wrapping `TempFile` into an `Option`
// and implementing `Drop::drop` on the wrapped type leveraging the function
// `Option<T>::take`.
//
// This exercise was inspired from advice in ch. 3, p. 46 in `Rust for Rustaceans`
// from _J. Gjenset_.

mod module {

    // Simulating a complex object, a file handle,
    // NOTE: Do not change this type.
    #[derive(Debug)]
    struct Handle(String);

    // Define our simple file resource RAII type.
    #[derive(Debug)]
    pub struct TempFile {
        file_handle: Handle,
    }

    #[derive(Debug)]
    pub enum ErrorFileHandle {
        Remove,
    }

    // A external function given from a library
    // simulating the removal of this file handle.
    // This takes ownership and removes the file on disk (virtually).
    // NOTE: Do not change the signature of this function.
    fn remove(file_handle: Handle) -> Result<(), ErrorFileHandle> {
        println!("Closing '{:?}'", &file_handle);
        if file_handle.0 == "internal" {
            Err(ErrorFileHandle::Remove)
        } else {
            Ok(())
        }
    }

    impl TempFile {
        pub fn new(file: impl AsRef<str>) -> TempFile {
            TempFile {
                file_handle: Handle(file.as_ref().to_owned() + "-handle"),
            }
        }

        // Close is an explicit destructor which takes ownership
        // and closes the temporary file. It will directly remove
        // the file from disk.
        pub fn close(self) -> Result<(), ErrorFileHandle> {
            // remove(self.file_handle)
            Ok(())
        }
    }

    impl Drop for TempFile {
        // Thats the implicit destructor for TempFile,
        // whenever this is destroyed.
        fn drop(&mut self) {
            println!("Dropping '{:?}'", &self);
            // remove(self.file_handle)
        }
    }
}

fn main() {
    use module::*;

    {
        println!("Testing temp file 'orange'.");
        TempFile::new("orange");
    }

    println!("Testing temp file 'banana'.");
    let b = TempFile::new("banana");
    match b.close() {
        Err(e) => println!("Error ocured in destructor '{:?}'", e),
        _ => (),
    }

    println!("Testing temp file 'internal'.");
    let b = TempFile::new("internal");
    match b.close() {
        Err(e) => println!("Error ocured in destructor '{:?}'", e),
        _ => (),
    }
}

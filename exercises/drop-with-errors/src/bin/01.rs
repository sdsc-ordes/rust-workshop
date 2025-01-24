// This exercise practices to implement Drop on a type
// while retaining error handling.
// Inspiration from: Ch.3, p. 46 in "Rust for Rustaceans".
//
// We have a type `TempFile` which should simulate some resource
// which you acquire and initialize with `new` (RAII)
// and which implements `Drop` which will remove the
// file from the disc whenever it gets dropped.
//
// Returning errors from `Drop::drop` is not possible.
// We could ignore errors, but in this exercise we want to leave no loose ends
// and not just ignore errors.
// Thats why we implement an explicit destructor `close` which
// takes owner ship by taking `self` as value and returns
// `Result<(), ErrorTempFile>` in case something happens.
//
// The code below compiles, but if you try to call `remove` in
// `close` the compiler will tell you that you cannot partially move
// out values before `Drop::drop` is called at function end. Also calling `
// `remove` in `Drop` does not work because `Drop::drop` does not own `self` it
// only has a `&mut self`.
//
// The exercise is to solve these issues by wrapping `TempFile` into an `Option`

mod module {
    #[derive(Debug)]
    struct Handle(String);

    // Define our simple file resource RAII type.
    #[derive(Debug)]
    struct TempFileImpl {
        file_handle: Handle,
    }

    #[derive(Debug)]
    pub enum ErrorFileHandle {
        Remove,
    }

    #[derive(Debug)]
    pub struct TempFile(Option<TempFileImpl>);

    fn remove(file_handle: Handle) -> Result<(), ErrorFileHandle> {
        println!("Closing '{:?}'", &file_handle);

        if file_handle.0 == "internal-handle" {
            Err(ErrorFileHandle::Remove)
        } else {
            Ok(())
        }
    }

    impl TempFileImpl {
        pub fn close(self) -> Result<(), ErrorFileHandle> {
            remove(self.file_handle)
        }
    }

    impl TempFile {
        pub fn new(file: impl AsRef<str>) -> TempFile {
            TempFile(Some(TempFileImpl {
                file_handle: Handle(file.as_ref().to_owned() + "-handle"),
            }))
        }

        pub fn close(mut self) -> Result<(), ErrorFileHandle> {
            self.close_internal()
        }

        // Private function not for internal use only.
        fn close_internal(&mut self) -> Result<(), ErrorFileHandle> {
            match self.0.take() {
                Some(f) => f.close(),
                _ => Ok(()),
            }
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            println!("Dropping '{:?}'", &self);
            let _ = self.close_internal();
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

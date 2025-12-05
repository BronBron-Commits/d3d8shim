use libloading::{Library, Symbol};

fn main() {
    println!("Testing dynamic loader...");

    #[cfg(target_os = "windows")]
    let lib_name = "d3d9.dll";
    #[cfg(not(target_os = "windows"))]
    let lib_name = "libm.so.6"; // Linux test

    unsafe {
        match Library::new(lib_name) {
            Ok(lib) => {
                println!("Successfully loaded: {}", lib_name);

                #[cfg(target_os = "windows")]
                {
                    let func: Result<Symbol<unsafe extern "system" fn(u32) -> isize>, _> =
                        lib.get(b"Direct3DCreate9\0");
                    match func {
                        Ok(_) => println!("Found Direct3DCreate9 symbol."),
                        Err(_) => println!("Symbol Direct3DCreate9 not found."),
                    }
                }

                #[cfg(not(target_os = "windows"))]
                {
                    let func: Result<Symbol<unsafe extern "C" fn(f64) -> f64>, _> =
                        lib.get(b"cos\0");
                    match func {
                        Ok(_) => println!("Found cos() in libc."),
                        Err(_) => println!("Symbol not found."),
                    }
                }
            }
            Err(e) => println!("Failed to load {}: {}", lib_name, e),
        }
    }
}

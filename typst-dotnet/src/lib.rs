use std::ffi::c_char;

use interoptopus::patterns::primitives::FFICChar;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_function, ffi_type};
mod render;

/// A simple type in our FFI layer.
#[ffi_type]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi_type]
#[repr(C)]
pub struct TypstInput<'a> {
    pub content: FFISlice<'a, FFICChar>,
}

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec2) -> Vec2 {
    input
}

// fn ffistring_from_string<'a>(value: String) -> FFISlice<'a, FFICChar> {
//     let c_char_slice: Vec<FFICChar> = value
//         .into_bytes()
//         .iter()
//         .map(|&x| {
//             let x: FFICChar = (x as c_char).into();
//             x
//         })
//         .collect::<Vec<FFICChar>>();
//     let leaked_slice = Box::leak(Box::new(c_char_slice));
//     FFISlice::from_slice(&leaked_slice)
// }

fn extend_lifetime(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn render_svg<'a>(input: TypstInput<'a>) -> FFISlice<'_, u8> {
    let c_char_slice = &input
        .content
        .into_iter()
        .map(|&x| {
            let x: c_char = x.into();
            x as u8
        })
        .collect::<Vec<u8>>();
    let input_string = unsafe { std::str::from_utf8_unchecked(c_char_slice) };
    let result = render::to_svg_string(input_string);
    let extended_string = extend_lifetime(result);
    let slice = FFISlice::from_slice(extended_string.as_bytes());
    slice
}

#[cfg(test)]
mod tests {
    use crate::{my_function, render_svg};
    use interoptopus::util::NamespaceMappings;
    use interoptopus::{function, Inventory, InventoryBuilder};
    use interoptopus::{Error, Interop};

    pub fn my_inventory() -> Inventory {
        InventoryBuilder::new()
            .register(function!(my_function))
            .register(function!(render_svg))
            .inventory()
    }

    #[test]
    fn bindings_csharp() -> Result<(), Error> {
        use interoptopus_backend_csharp::overloads::DotNet;
        use interoptopus_backend_csharp::{Config, Generator};

        let config = Config {
            dll_name: "typst_dotnet".to_string(),
            namespace_mappings: NamespaceMappings::new("Typst.Net"),
            ..Config::default()
        };

        Generator::new(config, my_inventory())
            .add_overload_writer(DotNet::new())
            //.add_overload_writer(Unity::new())
            .write_file("../bindings/Interop.cs")?;
        Ok(())
    }
}

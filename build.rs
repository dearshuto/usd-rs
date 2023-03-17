fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .include("ffi/Externals/tinyusdz/src")
        .files([
            // ffi
            "ffi/Sources/Libraries/usdffi/ffi.cpp",
            // tinyusdz
            "ffi/Externals/tinyusdz/src/ascii-parser-basetype.cc",
            "ffi/Externals/tinyusdz/src/ascii-parser-timesamples-array.cc",
            "ffi/Externals/tinyusdz/src/ascii-parser-timesamples.cc",
            "ffi/Externals/tinyusdz/src/ascii-parser.cc",
            "ffi/Externals/tinyusdz/src/path-util.cc",
            "ffi/Externals/tinyusdz/src/pprinter.cc",
            "ffi/Externals/tinyusdz/src/prim-composition.cc",
            "ffi/Externals/tinyusdz/src/prim-reconstruct.cc",
            "ffi/Externals/tinyusdz/src/prim-types.cc",
            "ffi/Externals/tinyusdz/src/primvar.cc",
            "ffi/Externals/tinyusdz/src/stage.cc",
            "ffi/Externals/tinyusdz/src/str-util.cc",
            "ffi/Externals/tinyusdz/src/tiny-format.cc",
            "ffi/Externals/tinyusdz/src/tinyusdz.cc",
            "ffi/Externals/tinyusdz/src/usda-reader.cc",
            "ffi/Externals/tinyusdz/src/usdGeom.cc",
            "ffi/Externals/tinyusdz/src/usdObj.cc",
            "ffi/Externals/tinyusdz/src/usdShade.cc",
            "ffi/Externals/tinyusdz/src/value-pprint.cc",
            "ffi/Externals/tinyusdz/src/value-types.cc",
            "ffi/Externals/tinyusdz/src/xform.cc",
        ])
        .compile("tinyusdz");
}

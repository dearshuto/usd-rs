cmake_minimum_required(VERSION 3.16)

# enable_language(CXX)
PROJECT(usd-ffi)

# # 実行ファイルの出力ディレクトリの設定
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)

# # ライブラリファイルの出力ディレクトリの設定
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/Libraries)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/Libraries)

# usdffi
add_library(usdffi STATIC)
target_compile_features(usdffi PUBLIC cxx_std_17)
add_subdirectory(Include)
add_subdirectory(Sources)

# tinyusdz へのヘッダへのパス
target_include_directories(usdffi
    PUBLIC ${CMAKE_SOURCE_DIR}/Include
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/Externals/tinyusdz/src
)

# tinyusdz 用にフラグを立てる必要がある
set(CMAKE_CXX_STANDARD 17)
option(TINYUSDZ_BUILD_EXAMPLES "" OFF)
option(TINYUSDZ_BUILD_TESTS "" OFF)
add_subdirectory(Externals/tinyusdz)

add_subdirectory(Examples)
install(TARGETS usdffi DESTINATION .)

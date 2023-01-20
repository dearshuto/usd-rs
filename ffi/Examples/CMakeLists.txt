add_executable(simple simple.cpp)
target_include_directories(simple PRIVATE ${CMAKE_SOURCE_DIR}/Include)
target_link_directories(simple PRIVATE ${CMAKE_SOURCE_DIR}/Libraries)
target_link_libraries(simple PRIVATE usdffi tinyusdz_static)
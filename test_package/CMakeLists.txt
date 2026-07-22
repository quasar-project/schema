cmake_minimum_required(VERSION 3.23)

project(quasar_schema_test_package LANGUAGES C CXX)

find_package(QuaSARSchema CONFIG REQUIRED)

add_executable(ffi_consumer src/ffi_consumer.c)
target_link_libraries(ffi_consumer PRIVATE quasar::schema_ffi)
set_target_properties(ffi_consumer PROPERTIES C_STANDARD 11 C_STANDARD_REQUIRED ON)

add_executable(schema_consumer src/schema_consumer.cpp)
target_link_libraries(schema_consumer PRIVATE quasar::schema)
set_target_properties(schema_consumer PROPERTIES CXX_STANDARD 20 CXX_STANDARD_REQUIRED ON)

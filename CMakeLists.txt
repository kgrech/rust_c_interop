cmake_minimum_required(VERSION 3.20)
project(rust_c_interop C)

set(CMAKE_C_STANDARD 99)

add_executable(rust_c_interop main.c)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD RUSTFLAGS=-Zsanitizer=address cargo build -Zbuild-std --target x86_64-unknown-linux-gnu)
    set(TARGET_DIR "x86_64-unknown-linux-gnu/debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

SET(LIB_FILE "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librust_lib.a")
SET(LIB_HEADER_FOLDER "${CMAKE_CURRENT_SOURCE_DIR}/rust_lib/include")

add_custom_command(OUTPUT ${LIB_FILE}
        COMMENT "Compiling rust module"
        COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib)

add_custom_target(rust_lib_target DEPENDS ${LIB_FILE})

add_library(rust_lib STATIC IMPORTED GLOBAL)
add_dependencies(rust_lib rust_lib_target)
set_target_properties(rust_lib
        PROPERTIES
        IMPORTED_LOCATION ${LIB_FILE}
        INTERFACE_INCLUDE_DIRECTORIES ${LIB_HEADER_FOLDER})

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)

target_compile_options(rust_c_interop PRIVATE -fno-omit-frame-pointer -fsanitize=address)
target_link_libraries(rust_c_interop PRIVATE Threads::Threads rust_lib ${CMAKE_DL_LIBS} -fno-omit-frame-pointer -fsanitize=address)


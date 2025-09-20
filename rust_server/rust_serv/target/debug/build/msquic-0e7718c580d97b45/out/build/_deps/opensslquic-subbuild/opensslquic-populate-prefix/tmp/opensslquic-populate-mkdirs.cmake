# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION ${CMAKE_VERSION}) # this file comes with cmake

# If CMAKE_DISABLE_SOURCE_CHANGES is set to true and the source directory is an
# existing directory in our source tree, calling file(MAKE_DIRECTORY) on it
# would cause a fatal error, even though it would be a no-op.
if(NOT EXISTS "/home/jonathan0x113c1c3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/msquic-2.5.1-beta/submodules")
  file(MAKE_DIRECTORY "/home/jonathan0x113c1c3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/msquic-2.5.1-beta/submodules")
endif()
file(MAKE_DIRECTORY
  "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-build"
  "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix"
  "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix/tmp"
  "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix/src/opensslquic-populate-stamp"
  "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix/src"
  "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix/src/opensslquic-populate-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix/src/opensslquic-populate-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/home/jonathan0x113c1c3/Documents/com_music_w_go/rust_server/rust_serv/target/debug/build/msquic-0e7718c580d97b45/out/build/_deps/opensslquic-subbuild/opensslquic-populate-prefix/src/opensslquic-populate-stamp${cfgdir}") # cfgdir has leading slash
endif()

#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "msquic::msquic" for configuration "Debug"
set_property(TARGET msquic::msquic APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(msquic::msquic PROPERTIES
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib64/libmsquic.so.2.5.1"
  IMPORTED_SONAME_DEBUG "libmsquic.so.2"
  )

list(APPEND _cmake_import_check_targets msquic::msquic )
list(APPEND _cmake_import_check_files_for_msquic::msquic "${_IMPORT_PREFIX}/lib64/libmsquic.so.2.5.1" )

# Import target "msquic::platform" for configuration "Debug"
set_property(TARGET msquic::platform APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(msquic::platform PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib64/libmsquic_platform.a"
  )

list(APPEND _cmake_import_check_targets msquic::platform )
list(APPEND _cmake_import_check_files_for_msquic::platform "${_IMPORT_PREFIX}/lib64/libmsquic_platform.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)

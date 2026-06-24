cmake_minimum_required(VERSION 3.23)

include_guard(GLOBAL)

project(quasar_schema
  HOMEPAGE_URL "quasar-sar.ru"
  LANGUAGES C CXX
)

set(PROJECT_NAMESPACE "quasar::")
set(PROJECT_FULL_NAME "${PROJECT_NAMESPACE}${PROJECT_NAME}")

if("${CMAKE_GENERATOR}" MATCHES "^Visual Studio")
  set(CMAKE_GENERATOR_PLATFORM "x64" CACHE STRING "" FORCE)
endif()

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set_property(GLOBAL PROPERTY USE_FOLDERS ON)

find_package(MMS CONFIG REQUIRED)

find_package(protobuf CONFIG QUIET)
if(NOT TARGET protobuf::protobuf AND NOT TARGET protobuf::libprotobuf)
  find_package(Protobuf REQUIRED)
endif()

if(TARGET protobuf::protobuf)
  set(QUASAR_SCHEMA_PROTOBUF_TARGET protobuf::protobuf)
elseif(TARGET protobuf::libprotobuf)
  set(QUASAR_SCHEMA_PROTOBUF_TARGET protobuf::libprotobuf)
else()
  message(FATAL_ERROR "Could not find a protobuf CMake target")
endif()

if(TARGET protobuf::protoc)
  set(QUASAR_SCHEMA_PROTOC "$<TARGET_FILE:protobuf::protoc>")
elseif(DEFINED Protobuf_PROTOC_EXECUTABLE)
  set(QUASAR_SCHEMA_PROTOC "${Protobuf_PROTOC_EXECUTABLE}")
elseif(DEFINED protobuf_PROTOC_EXECUTABLE)
  set(QUASAR_SCHEMA_PROTOC "${protobuf_PROTOC_EXECUTABLE}")
else()
  find_program(QUASAR_SCHEMA_PROTOC protoc REQUIRED)
endif()

set(QUASAR_SCHEMA_SCHEMA_DIR "${CMAKE_CURRENT_SOURCE_DIR}/schema")
set(QUASAR_SCHEMA_GENERATED_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated")
set(QUASAR_SCHEMA_MMS_PROTO_DIR "" CACHE PATH "Path to the mms_protocol schema directory containing mms/*.proto")

if(NOT QUASAR_SCHEMA_MMS_PROTO_DIR)
  set(_quasar_schema_local_mms_schema "${CMAKE_CURRENT_SOURCE_DIR}/../mms/protocol/schema")
  if(EXISTS "${_quasar_schema_local_mms_schema}/mms")
    set(QUASAR_SCHEMA_MMS_PROTO_DIR "${_quasar_schema_local_mms_schema}" CACHE PATH "" FORCE)
  endif()
endif()

if(NOT IS_DIRECTORY "${QUASAR_SCHEMA_MMS_PROTO_DIR}/mms")
  message(FATAL_ERROR "QUASAR_SCHEMA_MMS_PROTO_DIR must point to the mms_protocol schema directory containing mms/*.proto")
endif()

file(GLOB_RECURSE QUASAR_SCHEMA_PROTO_FILES CONFIGURE_DEPENDS
  "${QUASAR_SCHEMA_SCHEMA_DIR}/quasar/*.proto"
)

set(QUASAR_SCHEMA_PROTO_IMPORT_DIRS
  "${QUASAR_SCHEMA_SCHEMA_DIR}"
  "${QUASAR_SCHEMA_MMS_PROTO_DIR}"
)

set(QUASAR_SCHEMA_PROTOC_IMPORT_ARGS)
foreach(proto_import_dir IN LISTS QUASAR_SCHEMA_PROTO_IMPORT_DIRS)
  list(APPEND QUASAR_SCHEMA_PROTOC_IMPORT_ARGS "--proto_path=${proto_import_dir}")
endforeach()

set(QUASAR_SCHEMA_GENERATED_SOURCES)
set(QUASAR_SCHEMA_GENERATED_HEADERS)
set(QUASAR_SCHEMA_CPP_OUT "${QUASAR_SCHEMA_GENERATED_DIR}")
if(MSVC)
  set(QUASAR_SCHEMA_CPP_OUT "dllexport_decl=QUASAR_SCHEMA_API:${QUASAR_SCHEMA_GENERATED_DIR}")
endif()

foreach(proto_file IN LISTS QUASAR_SCHEMA_PROTO_FILES)
  file(RELATIVE_PATH proto_relative "${QUASAR_SCHEMA_SCHEMA_DIR}" "${proto_file}")
  get_filename_component(proto_relative_dir "${proto_relative}" DIRECTORY)
  get_filename_component(proto_name "${proto_relative}" NAME_WE)

  set(proto_output_dir "${QUASAR_SCHEMA_GENERATED_DIR}/${proto_relative_dir}")
  set(proto_source "${proto_output_dir}/${proto_name}.pb.cc")
  set(proto_header "${proto_output_dir}/${proto_name}.pb.h")

  add_custom_command(
    OUTPUT "${proto_source}" "${proto_header}"
    COMMAND ${CMAKE_COMMAND} -E make_directory "${proto_output_dir}"
    COMMAND "${QUASAR_SCHEMA_PROTOC}"
      ${QUASAR_SCHEMA_PROTOC_IMPORT_ARGS}
      --cpp_out "${QUASAR_SCHEMA_CPP_OUT}"
      "${proto_file}"
    DEPENDS "${proto_file}"
    VERBATIM
  )

  list(APPEND QUASAR_SCHEMA_GENERATED_SOURCES "${proto_source}")
  list(APPEND QUASAR_SCHEMA_GENERATED_HEADERS "${proto_header}")
endforeach()

add_library(${PROJECT_NAME} SHARED)
add_library(${PROJECT_FULL_NAME} ALIAS ${PROJECT_NAME})

target_sources(${PROJECT_NAME}
  PUBLIC
    FILE_SET HEADERS
    BASE_DIRS "${QUASAR_SCHEMA_GENERATED_DIR}"
    FILES ${QUASAR_SCHEMA_GENERATED_HEADERS}
  PRIVATE
    ${QUASAR_SCHEMA_GENERATED_SOURCES}
    ${QUASAR_SCHEMA_PROTO_FILES}
)

target_include_directories(${PROJECT_NAME}
  PUBLIC
    $<BUILD_INTERFACE:${QUASAR_SCHEMA_GENERATED_DIR}>
    $<INSTALL_INTERFACE:include>
)

if(MSVC)
  target_compile_options(${PROJECT_NAME}
    PRIVATE "/DQUASAR_SCHEMA_API=__declspec(dllexport)"
    INTERFACE "/DQUASAR_SCHEMA_API=__declspec(dllimport)"
  )
endif()

target_link_libraries(${PROJECT_NAME}
  PUBLIC
    mms::protocol
    ${QUASAR_SCHEMA_PROTOBUF_TARGET}
)

set_target_properties(${PROJECT_NAME} PROPERTIES
  CXX_STANDARD 20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF
  POSITION_INDEPENDENT_CODE ON
  WINDOWS_EXPORT_ALL_SYMBOLS ON
)

include(GNUInstallDirs)

install(
  TARGETS ${PROJECT_NAME}
  EXPORT ${PROJECT_NAME}
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
  INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
  FILE_SET HEADERS DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

install(
  DIRECTORY "${QUASAR_SCHEMA_SCHEMA_DIR}/"
  DESTINATION schema
  FILES_MATCHING PATTERN "*.proto"
)

install(EXPORT ${PROJECT_NAME}
  FILE ${PROJECT_NAME}Targets.cmake
  NAMESPACE quasar::
  DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}
)

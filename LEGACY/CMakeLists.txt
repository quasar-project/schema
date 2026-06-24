cmake_minimum_required(VERSION 3.15)

include_guard(GLOBAL)

project(quasar-protobufs LANGUAGES C CXX)
set(PROJECT_NAMESPACE "quasar::")
set(PROJECT_FULL_NAME "quasar::protobufs")
set(QUASAR_PROTOBUFS_IMPORT_PATH "${CMAKE_CURRENT_SOURCE_DIR}/share/proto")

set(CMAKE_POLICY_VERSION_MINIMUM 3.5) # todo: migrate to new cmake version recursively and remove this

option(BUILD_SHARED_LIBS "Build shared libraries" OFF)
option(QUASAR_PROTOBUFS_GRPC "Build with gRPC support" ON)

if ("${CMAKE_GENERATOR}" MATCHES "^Visual Studio")
  set(CMAKE_GENERATOR_PLATFORM "x64" CACHE STRING "" FORCE)
endif ()

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set_property(GLOBAL PROPERTY USE_FOLDERS ON)

# <-- library -->
add_library(${PROJECT_NAME} STATIC)
add_library(${PROJECT_FULL_NAME} ALIAS ${PROJECT_NAME})

if(QUASAR_PROTOBUFS_GRPC)
  find_package(gRPC REQUIRED)
else()
  find_package(protobuf REQUIRED)
endif()
include(cmake/GenerateProtobufs.cmake)

if(QUASAR_PROTOBUFS_GRPC)
  generate_protobufs(
    INPUT_DIR "${QUASAR_PROTOBUFS_IMPORT_PATH}"
    OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated"
    INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include/quasar/protobufs"
    TARGET_NAME "quasar-protobufs-gen"
    TARGET_ALIAS "quasar::protobufs_gen"
    GRPC
    GRPC_INPUT_DIR "${QUASAR_PROTOBUFS_IMPORT_PATH}/quasar/relay/services"
  )
  target_link_libraries(${PROJECT_NAME}
    PUBLIC
    gRPC::grpc
    gRPC::grpc++
    PRIVATE
    quasar::protobufs_gen_grpc
  )
else()
  generate_protobufs(
    INPUT_DIR "${QUASAR_PROTOBUFS_IMPORT_PATH}"
    OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated"
    INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include/quasar/protobufs"
    TARGET_NAME "quasar-protobufs-gen"
    TARGET_ALIAS "quasar::protobufs_gen"
  )
endif()

set_target_properties(${PROJECT_NAME} PROPERTIES
  CXX_STANDARD 20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF
  POSITION_INDEPENDENT_CODE ON
)

target_link_libraries(${PROJECT_NAME}
  PUBLIC
  protobuf::protobuf
  PRIVATE
  quasar::protobufs_gen
)

target_include_directories(${PROJECT_NAME}
  PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>$<INSTALL_INTERFACE:include>
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/quasar/protobufs>$<INSTALL_INTERFACE:include/quasar/protobufs>
  PRIVATE src
)

# <-- installation -->
message(STATUS "[${PROJECT_FULL_NAME}] installing ${PROJECT_NAME} in namespace ${PROJECT_NAMESPACE}")

include(GNUInstallDirs)

install(TARGETS ${PROJECT_NAME}
  EXPORT ${PROJECT_NAME}
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
  INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

install(
  DIRECTORY "${QUASAR_PROTOBUFS_IMPORT_PATH}/"
  DESTINATION "share/proto"
  FILES_MATCHING PATTERN "*.proto"
)
install(DIRECTORY $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/quasar> $<INSTALL_INTERFACE:include/quasar> ; DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})

get_target_property(TARGET_TYPE ${PROJECT_NAME} TYPE)
if (TARGET_TYPE STREQUAL "SHARED_LIBRARY" OR TARGET_TYPE STREQUAL "MODULE_LIBRARY")
  install(FILES $<TARGET_FILE:${PROJECT_NAME}> DESTINATION ${CMAKE_INSTALL_DATADIR}/plugins)
endif ()

unset(BUILD_SHARED_LIBS CACHE)

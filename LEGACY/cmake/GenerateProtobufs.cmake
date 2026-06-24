function(generate_protobufs)
  set(options GRPC)
  set(oneValueArgs INPUT_DIR GRPC_INPUT_DIR OUTPUT_DIR INCLUDE_DIR TARGET_NAME TARGET_ALIAS)
  set(multiValueArgs IMPORT_PATHS)
  cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}" "${multiValueArgs}")

  file(MAKE_DIRECTORY "${arg_OUTPUT_DIR}")
  file(GLOB_RECURSE PROTOBUF_FILES "${arg_INPUT_DIR}/*.proto")
  file(GLOB_RECURSE GRPC_PROTOBUF_FILES "${arg_GRPC_INPUT_DIR}/*.proto")
  
  add_library(${arg_TARGET_NAME} OBJECT ${PROTOBUF_FILES})
  add_library(${arg_TARGET_NAME}-grpc OBJECT ${GRPC_PROTOBUF_FILES})
  add_library(${arg_TARGET_ALIAS} ALIAS ${arg_TARGET_NAME})
  add_library(${arg_TARGET_ALIAS}_grpc ALIAS ${arg_TARGET_NAME}-grpc)

  set_target_properties(${arg_TARGET_NAME} PROPERTIES POSITION_INDEPENDENT_CODE ON)
  set_target_properties(${arg_TARGET_NAME}-grpc PROPERTIES POSITION_INDEPENDENT_CODE ON)

  target_include_directories(${arg_TARGET_NAME} PUBLIC $<BUILD_INTERFACE:${arg_OUTPUT_DIR}>)
  target_include_directories(${arg_TARGET_NAME} PUBLIC $<BUILD_INTERFACE:${arg_INCLUDE_DIR}>)
  target_include_directories(${arg_TARGET_NAME}-grpc PUBLIC $<BUILD_INTERFACE:${arg_OUTPUT_DIR}>)
  target_include_directories(${arg_TARGET_NAME}-grpc PUBLIC $<BUILD_INTERFACE:${arg_INCLUDE_DIR}>)
  target_link_libraries(${arg_TARGET_NAME} PUBLIC protobuf::protobuf)

  protobuf_generate(
    TARGET ${arg_TARGET_NAME}
    IMPORT_DIRS ${arg_INPUT_DIR} ${arg_IMPORT_PATHS}
    PROTOC_OUT_DIR "${arg_OUTPUT_DIR}"
  )

  if (arg_GRPC)
    target_link_libraries(${arg_TARGET_NAME}-grpc
      PUBLIC
      gRPC::grpc
      gRPC::grpc++
      PRIVATE
      ${arg_TARGET_NAME}
    )

    protobuf_generate(
      TARGET ${arg_TARGET_NAME}-grpc
      LANGUAGE grpc
      GENERATE_EXTENSIONS .grpc.pb.h .grpc.pb.cc
      PLUGIN "protoc-gen-grpc=\$<TARGET_FILE:gRPC::grpc_cpp_plugin>"
      IMPORT_DIRS ${arg_INPUT_DIR} ${arg_IMPORT_PATHS}
      PROTOC_OUT_DIR "${arg_OUTPUT_DIR}"
    )
  endif ()

  set(COPY_SCRIPT "${CMAKE_CURRENT_BINARY_DIR}/copy_proto_headers_${arg_TARGET_NAME}.cmake")
  file(WRITE "${COPY_SCRIPT}"
"file(GLOB_RECURSE HEADERS
  RELATIVE \"\${INPUT_DIR}\"
  \"\${INPUT_DIR}/*.pb.h\"
  \"\${INPUT_DIR}/*.grpc.pb.h\"
)
foreach(header IN LISTS HEADERS)
  get_filename_component(dir \"\${header}\" DIRECTORY)
  file(MAKE_DIRECTORY \"\${OUTPUT_DIR}/\${dir}\")
  file(COPY \"\${INPUT_DIR}/\${header}\" DESTINATION \"\${OUTPUT_DIR}/\${dir}\")
endforeach()
")

  # Create a post-generation copy target
  add_custom_target(copy_headers_${arg_TARGET_NAME} ALL
    COMMAND ${CMAKE_COMMAND} -DINPUT_DIR=${arg_OUTPUT_DIR} -DOUTPUT_DIR=${arg_INCLUDE_DIR} -P ${COPY_SCRIPT}
    DEPENDS ${arg_TARGET_NAME} ${arg_TARGET_NAME}-grpc
    COMMENT "Copying protobuf headers for target ${arg_TARGET_NAME} & ${arg_TARGET_NAME}-grpc"
  )
endfunction()
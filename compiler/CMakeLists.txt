cmake_minimum_required(VERSION 3.13)

project(civicc VERSION 1.0 LANGUAGES C)

set(TEST_DIR "${CMAKE_CURRENT_LIST_DIR}/../tests/public")
set(COMPILER "${CMAKE_CURRENT_BINARY_DIR}/civicc")
set(PALM_LINK_STATIC OFF)

option(DISABLE_ASAN "Disable address sanitizer")

# Run the supplied public suite through the compiler-aware test runner.
enable_testing()
foreach(TEST_GROUP basic nested_funs arrays)
    add_test(NAME "${TEST_GROUP}"
        COMMAND bash "${CMAKE_CURRENT_LIST_DIR}/test/run.bash" "${COMPILER}" "${TEST_GROUP}"
        WORKING_DIRECTORY "${TEST_DIR}")
endforeach()


find_package(BISON REQUIRED)
find_package(FLEX REQUIRED)
find_package(coconut QUIET CONFIG)
if(NOT coconut_FOUND)
    set(COCONUT_SOURCE_DIR "${CMAKE_CURRENT_LIST_DIR}/../vendor/coconut" CACHE PATH
        "Path to a CoCoNut source checkout with coconut.cmake")
    set(COCONUT_BUILD_DIR "${CMAKE_CURRENT_LIST_DIR}/../build/coconut" CACHE PATH
        "Path to the CoCoNut build directory")
    if(NOT EXISTS "${COCONUT_SOURCE_DIR}/coconut.cmake")
        message(FATAL_ERROR
            "CoCoNut was not found. Set coconut_DIR to an installed package, "
            "or set COCONUT_SOURCE_DIR to a built CoCoNut source checkout.")
    endif()
    message(STATUS "Using CoCoNut source checkout: ${COCONUT_SOURCE_DIR}")
    include("${COCONUT_SOURCE_DIR}/coconut.cmake")
endif()

bison_target(
    CivicParser
    src/scanparse/parser.y
    "${CMAKE_CURRENT_BINARY_DIR}/parser.c"
    DEFINES_FILE "${CMAKE_CURRENT_BINARY_DIR}/parser.h"
    VERBOSE
)

flex_target(
    CivicLexer
    src/scanparse/lexer.l
    "${CMAKE_CURRENT_BINARY_DIR}/lexer.c"
    DEFINES_FILE "${CMAKE_CURRENT_BINARY_DIR}/lexer.h"
)

ADD_FLEX_BISON_DEPENDENCY(CivicLexer CivicParser)


# Whenever you add a file, add it here too.
add_executable(
    civicc
    ${FLEX_CivicLexer_OUTPUTS}
    ${BISON_CivicParser_OUTPUTS}
    src/main.c
    src/print/print.c
    src/scanparse/scanParse.c
    src/global/globals.c

    # Youre free to remove the demo directory and these entries.
    src/demo/opt_sub.c
    src/demo/sum_ints.c
    src/demo/rename_identifiers.c
)

target_compile_options(civicc PRIVATE
    $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -pedantic -Wno-unused-function>
)

# Enable address sanitizer
if(NOT DISABLE_ASAN)
    target_compile_options(civicc PRIVATE
        $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:$<$<CONFIG:Debug>:-fsanitize=address>>
    )

    target_link_options(civicc PRIVATE
        $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:$<$<CONFIG:Debug>:-fsanitize=address>>
    )
endif()

coconut_target_generate(civicc "${CMAKE_CURRENT_LIST_DIR}/src/main.ccn" dynamic ${PALM_LINK_STATIC})
# Older source checkouts expose the runtime header as ccn/ccn.h.
if(NOT coconut_FOUND AND EXISTS "${COCONUT_SOURCE_DIR}/copra/ccn/ccn.h")
    file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/coconut-compat/ccngen/ccn.h"
        "#include <ccn/ccn.h>\n")
    target_include_directories(civicc PRIVATE "${CMAKE_CURRENT_BINARY_DIR}/coconut-compat")
endif()
target_include_directories(civicc
    PUBLIC "${CMAKE_CURRENT_LIST_DIR}/src"
)

add_custom_target(dot
    dot -Tpng ccngen/ast.dot > ast.png
    COMMENT "Generate a png of your ast based on the generated dot diagram."
)

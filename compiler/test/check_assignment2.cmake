function(run_compiler source)
    execute_process(
        COMMAND "${COMPILER}" ${ARGN} "${TEST_DIR}/${source}"
        RESULT_VARIABLE result
        OUTPUT_VARIABLE output
        ERROR_VARIABLE errors
    )
    if(NOT "${result}" STREQUAL "0")
        message(FATAL_ERROR "Compiler failed (${result}):\n${output}\n${errors}")
    endif()
    set(raw_output "${output}" PARENT_SCOPE)
    string(REGEX REPLACE "Sum of integers: [0-9-]+\n" "" output "${output}")
    string(REGEX REPLACE "// Arithmetic operators[^\n]*\n" "" output "${output}")
    string(REGEX REPLACE "\\([0-9]+:[0-9]+(-[0-9]+)?\\)" "" output "${output}")
    string(REGEX REPLACE "[ \t\r]" "" output "${output}")
    set(expressions "${output}" PARENT_SCOPE)
endfunction()

function(expect_expressions expected)
    if(NOT "${expressions}" STREQUAL "${expected}")
        message(FATAL_ERROR "Unexpected expressions.\nExpected:\n${expected}\nActual:\n${expressions}")
    endif()
endfunction()

function(expect_counts plus minus multiply divide modulo)
    set(header "// Arithmetic operators (source): +: ${plus}, -: ${minus}, *: ${multiply}, /: ${divide}, %: ${modulo}\n")
    string(FIND "${raw_output}" "${header}" index)
    if(index LESS 0)
        message(FATAL_ERROR "Missing source counts:\n${header}\nActual:\n${raw_output}")
    endif()
    string(REGEX MATCHALL "Arithmetic operators" headers "${raw_output}")
    list(LENGTH headers header_count)
    if(NOT header_count EQUAL 1)
        message(FATAL_ERROR "Expected one root summary, got ${header_count}")
    endif()
endfunction()

if(CASE STREQUAL "operator_counts")
    run_compiler(operator_counts.cvc)
    expect_counts(2 3 2 1 1)
    expect_expressions("__plus=(__a+__b);\n__minus=(__a-__b);\n__multiply=(__k+__k);\n__divide=(__a/__b);\n__modulo=(__a%__b);\n__nested=((__a+__b)*(__c-__d));\n__removed=0;\n__relation=(__a<__b);\n__logic=((__a<__b)&&(__c==__d));\n")

    # Stop after counting: values must already be on the root before optimisation.
    run_compiler(operator_counts.cvc --breakpoint CountOperators)
    expect_counts(2 3 2 1 1)
    string(FIND "${expressions}" "multiply=(2*k);" index)
    if(index LESS 0)
        message(FATAL_ERROR "Counting unexpectedly changed the source:\n${expressions}")
    endif()

    run_compiler(no_operators.cvc)
    expect_counts(0 0 0 0 0)
    expect_expressions("__value=__k;\n")
elseif(CASE STREQUAL "root_without_counts")
    # Breakpoint printing must work before the counting pass has run.
    run_compiler(no_operators.cvc --breakpoint SPdoScanParse)
    string(FIND "${raw_output}" "Arithmetic operators" index)
    if(NOT index EQUAL -1)
        message(FATAL_ERROR "Unavailable counts were printed:\n${raw_output}")
    endif()
    expect_expressions("value=k;\n")
elseif(CASE STREQUAL "super_strength_reduction")
    run_compiler(super_strength_reduction.cvc --strength-reduction-max 4)
    file(READ "${TEST_DIR}/super_strength_reduction.expected" expected)
    expect_expressions("${expected}")
    expect_counts(3 0 15 0 0)

    # Both CLI spellings include the limit and leave the next factor alone.
    run_compiler(super_strength_reduction.cvc -r 3)
    string(REPLACE "__limitLeft=(((__k+__k)+__k)+__k);" "__limitLeft=(4*__k);" expected "${expected}")
    string(REPLACE "__limitRight=(((__k+__k)+__k)+__k);" "__limitRight=(__k*4);" expected "${expected}")
    string(REPLACE "__nested=(((__k+__k)+__k)+(((__k+__k)+__k)+__k));" "__nested=(((__k+__k)+__k)+(__k*4));" expected "${expected}")
    expect_expressions("${expected}")

    run_compiler(super_strength_reduction.cvc)
    set(default_expressions "${expressions}")
    string(FIND "${expressions}" "__left=(3*__k);" index)
    if(index LESS 0)
        message(FATAL_ERROR "Default factor limit must be 2:\n${expressions}")
    endif()
    run_compiler(super_strength_reduction.cvc --strength-reduction-max=2)
    expect_expressions("${default_expressions}")
elseif(CASE STREQUAL "cli")
    foreach(limit IN ITEMS "" 0 1 -2 3.5 abc 4abc " 4" +4 2147483648 999999999999999999999999)
        execute_process(
            COMMAND "${COMPILER}" "--strength-reduction-max=${limit}" "${TEST_DIR}/no_operators.cvc"
            RESULT_VARIABLE result OUTPUT_VARIABLE output ERROR_VARIABLE errors)
        if(NOT "${result}" STREQUAL "1" OR NOT errors MATCHES "strength-reduction-max")
            message(FATAL_ERROR "Invalid limit '${limit}' was not rejected clearly (${result}):\n${output}\n${errors}")
        endif()
    endforeach()

    execute_process(COMMAND "${COMPILER}" --strength-reduction-max
        RESULT_VARIABLE result OUTPUT_VARIABLE output ERROR_VARIABLE errors)
    if(NOT "${result}" STREQUAL "1")
        message(FATAL_ERROR "Missing factor limit was not rejected: ${result}")
    endif()

    # The largest representable limit is valid; it does not allocate anything
    # unless the input actually contains a matching multiplication.
    run_compiler(no_operators.cvc --strength-reduction-max 2147483647)
    expect_expressions("__value=__k;\n")
    execute_process(COMMAND "${COMPILER}" -h
        RESULT_VARIABLE result OUTPUT_VARIABLE output ERROR_VARIABLE errors)
    if(NOT "${result}" STREQUAL "0" OR NOT output MATCHES "strength-reduction-max")
        message(FATAL_ERROR "Help does not document the new option:\n${output}\n${errors}")
    endif()
else()
    message(FATAL_ERROR "Unknown Assignment 2 test case: ${CASE}")
endif()

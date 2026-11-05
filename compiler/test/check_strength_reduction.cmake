execute_process(
    COMMAND "${COMPILER}" "${TEST_DIR}/strength_reduction.cvc"
    RESULT_VARIABLE result
    OUTPUT_VARIABLE output
    ERROR_VARIABLE errors
)
if(NOT "${result}" STREQUAL "0")
    message(FATAL_ERROR "Compiler failed (${result}):\n${output}\n${errors}")
endif()

# Compare expressions independently of the demo integer sum, locations, and spacing.
string(REGEX REPLACE "^Sum of integers: [0-9-]+\n" "" output "${output}")
string(REGEX REPLACE "\\([0-9]+:[0-9]+(-[0-9]+)?\\)" "" output "${output}")
string(REGEX REPLACE "[ \t\r]" "" output "${output}")
file(READ "${TEST_DIR}/strength_reduction.expected" expected)
if(NOT "${output}" STREQUAL "${expected}")
    message(FATAL_ERROR "Unexpected optimisation result.\nExpected:\n${expected}\nActual:\n${output}")
endif()

cmake_minimum_required(VERSION 3.13.0)

project(hedera)

include_directories(include)

# [macOS] -framework security
#find_library(SECURITY_FRAMEWORK Security)

file(GLOB examples "examples/*.c")
foreach(example ${examples})
    get_filename_component(target ${example} NAME_WE)

    add_executable(${target} ${example})

    # > build/examples
    set_target_properties(${target} PROPERTIES RUNTIME_OUTPUT_DIRECTORY build/examples)

    # [macOS] -framework security
#    target_link_libraries(${target} ${SECURITY_FRAMEWORK})

    # -l hedera
    target_link_directories(${target} PRIVATE target/release)
    target_link_libraries(${target} hedera)
endforeach()

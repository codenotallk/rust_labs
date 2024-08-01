add_library (led SHARED "")

target_sources (led 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/led.c
)

target_include_directories (led 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/
    ${CMAKE_CURRENT_LIST_DIR}/../../manager
)


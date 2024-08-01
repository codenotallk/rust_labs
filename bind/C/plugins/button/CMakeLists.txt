add_library (button SHARED "")

target_sources (button 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/button.c
)

target_include_directories (button 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/
    ${CMAKE_CURRENT_LIST_DIR}/../../manager
)


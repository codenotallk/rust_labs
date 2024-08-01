add_library (manager SHARED "")

target_sources (manager 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/plugin_manager.c
)

target_include_directories (manager 
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/
)

target_link_libraries (manager PUBLIC dl)

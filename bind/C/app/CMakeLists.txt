add_executable (app "")

target_sources (app
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/main.c
)

target_link_libraries (app PUBLIC manager)

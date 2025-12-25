cmake_minimum_required(VERSION 3.25)
project(DIL)

set (CMAKE_CXX_STANDARD 20)

set (CLIENT_SOURCES 

)

include(FetchContent)

add_executable(${PROJECT_NAME} main.cpp ${CLIENT_SOURCES})

FetchContent_Declare(
    nlohmann
    SOURCE_DIR ${CMAKE_SOURCE_DIR}/vendor/nlohmann
)
FetchContent_MakeAvailable(nlohmann)

target_link_libraries(${PROJECT_NAME} PRIVATE nlohmann)

FetchContent_Declare(
    ftxui
    SOURCE_DIR ${CMAKE_SOURCE_DIR}/vendor/ftxui
)
FetchContent_MakeAvailable(ftxui)

target_link_libraries(${PROJECT_NAME} PRIVATE 
    ftxui::component
    ftxui::dom
    ftxui::screen
)

target_include_directories(${PROJECT_NAME}
    PRIVATE
    src/

    PUBLIC
    vendor/
)

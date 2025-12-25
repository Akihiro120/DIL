#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <ftxui/component/component.hpp>
#include <ftxui/component/loop.hpp>
#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/screen/screen.hpp>
#include <iostream>

using namespace ftxui;
namespace fs = std::filesystem;

int main()
{
    auto screen = ScreenInteractive::Fullscreen();

    const char *homeDir = std::getenv("HOME");
    if (!homeDir)
    {
        return 1;
    }

    fs::path fullPath = fs::path(homeDir) / ".config/DIL/tasks.json";
    std::ifstream dataFile(fullPath);
    if (dataFile)
    {
        std::cout << "Task file found: Loading Tasks" << std::endl;
    }
    else {
        std::cout << "Task file not found" << std::endl;
    }

    auto taskPanel = Renderer([&]
        {
            return text("Task");
        });
    taskPanel |= flex;
    taskPanel |= borderHeavy;

    auto editPanel = Renderer([&]
        {
            return text("Edit");
        });
    editPanel |= flex;
    editPanel |= borderHeavy;

    auto component = Renderer(
        [&]
        {
            return hbox({taskPanel->Render(), editPanel->Render()});
        });

    bool running = true;
    component |= CatchEvent([&](Event event) -> bool
        {
            if (event == Event::Character('q'))
            {
                running = false;
                return true;
            }
            return false;
        });

    Loop loop(&screen, component);
    while (!loop.HasQuitted() && running)
    {
        loop.RunOnce();
    }

    return 0;
}

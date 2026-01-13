#include <iostream>
#include <fstream>
#include <filesystem>
#include <string>
#include <vector>
#include <algorithm>

namespace fs = std::filesystem;

void create_gitkeep(const fs::path& path, const std::string& content, bool empty) {
    std::ofstream file(path);
    if (!empty) {
        file << content;
    }
}

void process_path(const fs::path& path, const std::string& content, bool let_go, bool empty) {
    fs::path target = path / ".gitkeep";
    if (let_go) {
        if (fs::exists(target)) {
            fs::remove(target);
            std::cout << "Removed: " << target << std::endl;
        }
    } else {
        create_gitkeep(target, content, empty);
        std::cout << "Created: " << target << std::endl;
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Usage: gitkeep <path> [-m message] [-l] [-r] [-e]" << std::endl;
        return 0;
    }

    std::string path_str;
    std::string message = "Created by GitKeep";
    bool let_go = false;
    bool recursive = false;
    bool empty = false;

    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];
        if (arg == "-m" && i + 1 < argc) {
            message = argv[++i];
        } else if (arg == "-l" || arg == "--let-go") {
            let_go = true;
        } else if (arg == "-r" || arg == "--recursive") {
            recursive = true;
        } else if (arg == "-e") {
            empty = true;
        } else if (path_str.empty() && arg[0] != '-') {
            path_str = arg;
        }
    }

    if (path_str.empty()) {
        std::cerr << "Error: No path specified." << std::endl;
        return 1;
    }

    fs::path root(path_str);
    if (!fs::exists(root) || !fs::is_directory(root)) {
        std::cerr << "Error: Invalid directory." << std::endl;
        return 1;
    }

    if (recursive) {
        // Process the root directory itself
        process_path(root, message, let_go, empty);
        
        for (auto& entry : fs::recursive_directory_iterator(root)) {
            if (entry.is_directory()) {
                if (entry.path().filename() == ".git") continue;
                process_path(entry.path(), message, let_go, empty);
            }
        }
    } else {
        process_path(root, message, let_go, empty);
    }

    return 0;
}

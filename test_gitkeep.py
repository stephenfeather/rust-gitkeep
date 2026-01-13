import os
import subprocess
import shutil

BINARY_PATH = "./target/debug/gitkeep"
TEST_DIR = "test_env"

def setup():
    if os.path.exists(TEST_DIR):
        shutil.rmtree(TEST_DIR)
    os.makedirs(os.path.join(TEST_DIR, "subdir"))

def run_gitkeep(args):
    result = subprocess.run([BINARY_PATH] + args, capture_output=True, text=True)
    return result

def test_basic_creation():
    setup()
    run_gitkeep([TEST_DIR])
    file_path = os.path.join(TEST_DIR, ".gitkeep")
    assert os.path.exists(file_path), "File not created"
    with open(file_path, "r") as f:
        content = f.read()
    assert content == "Created by GitKeep", f"Wrong content: {content}"
    print("Basic creation passed")

def test_custom_message():
    setup()
    run_gitkeep([TEST_DIR, "-m", "Custom Msg"])
    file_path = os.path.join(TEST_DIR, ".gitkeep")
    with open(file_path, "r") as f:
        content = f.read()
    assert content == "Custom Msg", f"Wrong content: {content}"
    print("Custom message passed")

def test_empty_file():
    setup()
    run_gitkeep([TEST_DIR, "-e"])
    file_path = os.path.join(TEST_DIR, ".gitkeep")
    assert os.path.getsize(file_path) == 0, "File not empty"
    print("Empty file passed")

def test_recursive_creation():
    setup()
    run_gitkeep([TEST_DIR, "-r"])
    assert os.path.exists(os.path.join(TEST_DIR, ".gitkeep")), "Root file missing"
    assert os.path.exists(os.path.join(TEST_DIR, "subdir", ".gitkeep")), "Subdir file missing"
    print("Recursive creation passed")

def test_removal():
    setup()
    # Create first
    run_gitkeep([TEST_DIR])
    assert os.path.exists(os.path.join(TEST_DIR, ".gitkeep"))
    # Remove
    run_gitkeep([TEST_DIR, "-l"])
    assert not os.path.exists(os.path.join(TEST_DIR, ".gitkeep")), "File not removed"
    print("Removal passed")

def test_recursive_removal():
    setup()
    # Create recursively
    run_gitkeep([TEST_DIR, "-r"])
    assert os.path.exists(os.path.join(TEST_DIR, "subdir", ".gitkeep"))
    
    # Remove recursively
    # Note: Using -l and -r together
    run_gitkeep([TEST_DIR, "-l", "-r"])
    assert not os.path.exists(os.path.join(TEST_DIR, ".gitkeep")), "Root file not removed"
    assert not os.path.exists(os.path.join(TEST_DIR, "subdir", ".gitkeep")), "Subdir file not removed"
    print("Recursive removal passed")

def test_no_args_help():
    result = run_gitkeep([])
    # Check if help text is displayed (look for 'Usage' or 'Options')
    assert "Usage:" in result.stdout or "Options:" in result.stdout, "Help text not shown"
    print("No args help passed")

if __name__ == "__main__":
    try:
        test_basic_creation()
        test_custom_message()
        test_empty_file()
        test_recursive_creation()
        test_removal()
        test_recursive_removal()
        test_no_args_help()
        print("ALL TESTS PASSED")
    finally:
        if os.path.exists(TEST_DIR):
            shutil.rmtree(TEST_DIR)

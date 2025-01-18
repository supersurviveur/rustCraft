# This file export built file from target and copy it
# to build folderwith a possible index
import os
import shutil


# Export built file from target and copy it to build folder
def exportBuiltFile(filename: str, targetFolder: str, buildFolder: str):
    # Get absolute paths
    targetFolder = os.path.abspath(targetFolder)
    buildFolder = os.path.abspath(buildFolder)

    shutil.copyfile(
        os.path.join(targetFolder, filename),
        os.path.join(buildFolder, filename),
    )


def main():
    # TODO better paths, config or auto detect, auto create out dir
    filename = "librustcraft_test.so"
    targetFolder = "./tests/target/debug"
    buildFolder = "build/out"
    exportBuiltFile(filename, targetFolder, buildFolder)


main()

@echo off
cd ..

rmdir build /S /Q
mkdir build
cd build
mkdir jar
cd jar

copy /Y ..\..\rustCraftMod\build\libs\testMod-1.0-SNAPSHOT.jar rustCraft.jar
xcopy  /E "..\..\rust_lib\assets\" "assets\"

jar -uf rustCraft.jar assets/
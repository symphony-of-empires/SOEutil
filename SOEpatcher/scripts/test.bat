cargo build
cd ..
cd target/debug
if not exist "test" mkdir test
copy soe_patcher.exe test\soe_patcher.exe
cd test
soe_patcher.exe --update-game --update-launcher --allow-prereleases
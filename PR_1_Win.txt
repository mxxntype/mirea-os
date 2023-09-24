@echo off

SET SURNAME="Vartanyan"
SET NAME="Artem"
SET FNAME="Aleksandrovich"
SET GROUP="BSBO_01_22"

IF "%~1" == "--clean" (
  rd /s /q %SURNAME%
  rd /s /q %FNAME%
  rd /s /q %GROUP%
  exit /b 0
)

SET LS=dir /A /Q /O:GN

cd
mkdir %FNAME% %SURNAME% %GROUP%
%LS%

cd %SURNAME%
echo. > %NAME%_1
echo. > %NAME%_2
echo. > %NAME%_3
%LS%

SET WORDS="some\nsort\nof\nword\nthat\ni\nhad\nto\nput\ninto\nthis\nfile..."
for %%f in (*) do (
  echo.%WORDS% > %%f
)

copy %NAME%_1 ..\%FNAME%
move %NAME%_2 ..\%GROUP%

cd ..
type *.*

cd %GROUP%
move %NAME%_2 "%SURNAME%_%NAME%"
type "%SURNAME%_%NAME%"
cd ..

copy /E /I %FNAME% %SURNAME%\
type %SURNAME%\*

copy /E /I %GROUP% %SURNAME%\%FNAME%\
%LS% %SURNAME%\%FNAME%

cd %FNAME%
copy %NAME%_1 %NAME%_2
copy %NAME%_1 %NAME%_3
%LS%
move %NAME%_1 "Сентябрь"
move %NAME%_2 "Октябрь"
move %NAME%_3 "Ноябрь"
cd ..

del /s /q %SURNAME%\*
%LS%
%0 --clean
%LS%

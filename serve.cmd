@echo off
cls
IF [%1] == [] (
    SET folder="dist"
) ELSE (
    SET folder="release"
)
php -S localhost:80 -t %folder% router.php
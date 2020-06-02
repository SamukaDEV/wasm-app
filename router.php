<?php


//$_SERVER["REQUEST_URI"]

// $req_uri = $_SERVER['REQUEST_URI'];
// echo $req_uri;
// echo file_get_contents("static/" . $req_uri);


$path = pathinfo($_SERVER["SCRIPT_FILENAME"]);
if ($path["extension"] == "wasm") {
    header("Content-Type: application/wasm");
    readfile($_SERVER["SCRIPT_FILENAME"]);
} else {
    return FALSE;
}

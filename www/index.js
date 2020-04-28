import * as wasm from "vulexia";

let colors = {
    "noun": "red",
    "verb": "green",
    "adjective": "blue",
    "conjunction": "orange",
    "preposition": "pink",
    "interjection": "black",
    "article": "black",
    "pronoun": "yellow",
};

document.getElementById("submit").addEventListener("click", function(event) {
    let input = document.getElementById("input").value;
    let output = wasm.color(input);
    document.getElementById("output").innerHTML = output;
});

console.log(colors);

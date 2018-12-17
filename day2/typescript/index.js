"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const fs = require("fs");
function main() {
    const ids = loadIds();
    console.log("Checksum: %s", checksum(ids));
}
function checksum(ids) {
    let twice = 0, thrice = 0;
    ids.forEach(id => {
        const [hasTwice, hasThrice] = hasRepeats(id);
        if (hasTwice)
            twice += 1;
        if (hasThrice)
            thrice += 1;
    });
    return twice * thrice;
}
function hasRepeats(id) {
    const counts = countLetters(id);
    return [
        counts.some((c) => c == 2),
        counts.some((c) => c == 3),
    ];
}
function countLetters(string) {
    const counts = {};
    string.split("").forEach((char) => {
        counts[char] = (counts[char] || 0) + 1;
    });
    return Object.values(counts);
}
function loadIds() {
    const data = fs.readFileSync(0, "utf8");
    return data.split("\n").filter(Boolean);
}
main();


var enabled = [];
var str = "";
function toggle(mode_name, self, elementID) {
    const text = document.getElementById(elementID).innerHTML;
    if (enabled.includes(mode_name)) {
        enabled.splice(enabled.indexOf(mode_name), 1);
    } else {
        enabled.push(mode_name);
    }
    var x = document.getElementById(elementID);
    if (x.style.display === "none") {
        x.style.display = "block";
    } else {
        x.style.display = "none";
    }
    self.blur();
    self.focus();
    str = enabled.join('-');
}

function textFocus(text) {
    document.getElementById("focus-text").innerHTML = text;
}

function saveAndExit() {
    if (enabled === undefined || enabled.length == 0) {
        location.href = "http://localhost/";
    }
    str = enabled.join('-');
    location.href = "http://localhost/" + str;
}
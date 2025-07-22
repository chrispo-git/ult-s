
var enabled = [];
var str = "";
function toggle(mode_name, self, original, altered, elementID) {
    const text = document.getElementById(elementID).innerHTML;
    if (enabled.includes(mode_name)) {
        enabled.splice(enabled.indexOf(mode_name), 1);
        document.getElementById(elementID).innerHTML = text.replace("("+altered+")", "("+original+")");
    } else {
        enabled.push(mode_name);
        document.getElementById(elementID).innerHTML = text.replace("("+original+")", "("+altered+")");
    }
    self.blur();
    self.focus();
    str = enabled.join('-');
}

function saveAndExit() {
    if (enabled === undefined || enabled.length == 0) {
        location.href = "http://localhost/";
    }
    str = enabled.join('-');
    location.href = "http://localhost/" + str;
}
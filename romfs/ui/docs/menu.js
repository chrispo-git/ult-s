var currentCategoryIndex = 0;
var currentButtonIndex = 0;

// Retrieve all buttons inside the currently active category
function getActiveButtons() {
    var activeCol = document.querySelector('.category-column.active');
    if (!activeCol) return [];
    return activeCol.querySelectorAll('button');
}

// Custom scroll check: Only scrolls when the element is about to move off-screen
function ensureVisible(targetBtn) {
    if (!targetBtn) return;

    // Find the scrollable container (.category-column or parent)
    var container = targetBtn.parentElement;
    while (container && container !== document.body) {
        var style = window.getComputedStyle(container);
        if (style.overflowY === 'auto' || style.overflowY === 'scroll' || container.classList.contains('category-column') || container.classList.contains('main')) {
            break;
        }
        container = container.parentElement;
    }

    if (!container) return;

    var padding = 20; // Margin from edge
    var btnTop = targetBtn.offsetTop;
    var btnBottom = btnTop + targetBtn.offsetHeight;

    var viewTop = container.scrollTop;
    var viewBottom = viewTop + container.clientHeight;

    // If button is below the visible frame, scroll down JUST enough
    if (btnBottom > viewBottom) {
        container.scrollTop = btnBottom - container.clientHeight + padding;
    } 
    // If button is above the visible frame, scroll up JUST enough
    else if (btnTop < viewTop) {
        container.scrollTop = btnTop - padding;
    }
}

// Move focus to a specific button index
function updateFocus(index) {
    var buttons = getActiveButtons();
    if (!buttons.length) return;

    // Clamp index within bounds
    if (index < 0) index = 0;
    if (index >= buttons.length) index = buttons.length - 1;

    currentButtonIndex = index;

    // Clear .is-focused from ALL buttons
    var allButtons = document.querySelectorAll('button');
    for (var i = 0; i < allButtons.length; i++) {
        allButtons[i].classList.remove('is-focused');
    }

    var targetBtn = buttons[currentButtonIndex];
    if (targetBtn) {
        targetBtn.classList.add('is-focused');
        
        // Native focus with preventScroll fallback
        try {
            targetBtn.focus({ preventScroll: true });
        } catch(e) {
            targetBtn.focus();
        }

        // Smooth boundary scroll check
        ensureVisible(targetBtn);
    }
}

function showCategory(targetIndex) {
    var columns = document.querySelectorAll('.category-column');
    if (!columns.length) return;

    currentCategoryIndex = (targetIndex + columns.length) % columns.length;

    for (var i = 0; i < columns.length; i++) {
        var col = columns[i];
        if (i === currentCategoryIndex) {
            col.classList.add('active');
            
            // Reset scroll position to top when entering new category
            col.scrollTop = 0;

            // Update Header Title
            var headerTitle = col.getAttribute('data-title') || ('Category ' + (i + 1));
            var headerElement = document.getElementById('test');
            if (headerElement) headerElement.innerText = headerTitle;
        } else {
            col.classList.remove('active');
        }
    }

    // Reset focus to top button on page swap
    updateFocus(0);
}

function nextCategory() {
    showCategory(currentCategoryIndex + 1);
}

function prevCategory() {
    showCategory(currentCategoryIndex - 1);
}

window.addEventListener("DOMContentLoaded", function() {
    var buttons = document.querySelectorAll('button');

    // Sync JavaScript index whenever ANY button receives focus
    for (var i = 0; i < buttons.length; i++) {
        (function(btn) {
            btn.addEventListener('focus', function() {
                var all = document.querySelectorAll('button');
                for (var j = 0; j < all.length; j++) {
                    all[j].classList.remove('is-focused');
                }

                btn.classList.add('is-focused');

                var activeBtns = getActiveButtons();
                for (var k = 0; k < activeBtns.length; k++) {
                    if (activeBtns[k] === btn) {
                        currentButtonIndex = k;
                        break;
                    }
                }
            });
        })(buttons[i]);
    }

    // D-Pad Navigation
    window.addEventListener('keydown', function(e) {
        var keyUp = typeof UP !== 'undefined' ? UP : 38;
        var keyDown = typeof DOWN !== 'undefined' ? DOWN : 40;

        var buttons = getActiveButtons();
        if (!buttons.length) return;

        if (e.keyCode == keyUp) {
            if (currentButtonIndex > 0) {
                updateFocus(currentButtonIndex - 1);
            }
        } else if (e.keyCode == keyDown) {
            if (currentButtonIndex < buttons.length - 1) {
                updateFocus(currentButtonIndex + 1);
            }
        }
    });

    // Native Switch Footer Buttons
    if (window.nx && window.nx.footer) {
        window.nx.footer.setAssign("A", "", function() {
            var activeBtn = document.querySelector('.category-column.active button.is-focused') || document.activeElement;
            if (activeBtn && typeof activeBtn.click === 'function') {
                activeBtn.click();
            }
        });
        window.nx.footer.setAssign("B", "", function() {
            window.location.href = "http://localhost/";
        });
        window.nx.footer.setAssign("X", "", nextCategory);
        window.nx.footer.setAssign("Y", "", prevCategory);
        window.nx.footer.setAssign("L", "", prevCategory);
        window.nx.footer.setAssign("R", "", nextCategory);
    }

    // Boot Category 0
    showCategory(0);
});
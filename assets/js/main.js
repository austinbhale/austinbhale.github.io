$(function () {

    var isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

    // Remove the scrolling effect for non-mobile devices.
    for (var i = 0; i < 5; i++) {
        (isMobile) ? $('#parallax-mobile' + i).css("display", "block"): $('#parallax' + i).css("display", "block");
        if (isMobile) {
            $('#parallax-ctr' + i).removeClass("parallax-container" + i).addClass("noparallax-container");
        }
    }

    // Set visited cookie for returning to displayed text.
    if (getCookie('visited')) {
        $('.hideme').css("opacity", "1.0");
        deleteCookie('visited');
    } else {
        // Cookie expires in 999 days.
        setCookie('visited', 'true', 999);
        $(window).scroll(function () {
            $('.hideme').each(function (i) {
                var top_half_of_object = $(this).position().top + ($(this).outerHeight() / 4);
                var bottom_of_window = $(window).scrollTop() + $(window).height();
                // If the object is completely visible in the window, fade it in.
                if (bottom_of_window > top_half_of_object) {
                    $(this).animate({
                        'opacity': '1'
                    }, 500);
                }
            });
        });
    }
});

// Set the cookie for seeing if the page has been visited.
var setCookie = function (name, value, expire_days) {
    var date = new Date();
    date.setDate(date.getDate() + expire_days);
    var expire_date = date.toUTCString();
    // IE8 bug fix with documentMode.
    var isIE8 = (document.documentMode !== undefined);
    if (expire_days == 0) {
        expire_date = (isIE8 == true) ? "" : "0";
    }
    var cookie_value = escape(value) + ((expire_days == null) ? "" : "; expire_date=" + expire_date);
    document.cookie = name + "=" + cookie_value;
}

var getCookie = function (cookie_name) {
    var name = cookie_name + "=";
    var ca = document.cookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') c = c.substring(1);
        if (c.indexOf(name) != -1) return c.substring(name.length, c.length);
    }
    return "";
}

// Delete the visited cookie on session end.
var deleteCookie = function (name) {
    document.cookie = name + '=; expire_date=Thu, 01 Jan 1970 00:00:01 GMT;';
}

$('#gform').on('submit', function (e) {

    // Boolean variables for checking valid form responses.
    var valids = false;
    var invalids = false;
    var received = false;
    var elementsValid = document.getElementsByClassName('valid');
    var elementsInvalid = document.getElementsByClassName('invalid');

    (elementsValid.length > 3) ? valids = true: valids = false;
    (elementsInvalid.length == 0) ? invalids = true: invalids = false;

    if (valids && invalids) {
        $('#gform *').fadeOut(2000, function () {
            if (!received) {
                $('#gform').prepend('Thanks! I\'ll be in touch with you soon.');
                received = true;
            }
        });
    } else {
        alert("Please fill in all fields.");
    }
});

// Adjusts fixed navbar height for hashed sections.
var heightSlider = $('#nav').height();
var shiftWindow = function () {
    (location.hash == "#contact") ? scrollBy(0, -heightSlider): scrollBy(0, -heightSlider);
};
if (location.hash) shiftWindow();
window.addEventListener("hashchange", shiftWindow);

// Smooth scroll to hash -- selects each used link with hashes.
var SCROLLSPY_TIME = 700;
$('a[href*="#"]')
    .not('[href="#"]')
    .not('[href="#0"]')
    .click(function (event) {
        if (location.pathname.replace(/^\//, '') == this.pathname.replace(/^\//, '') &&
            location.hostname == this.hostname) {
            var target = $(this.hash);
            target = target.length ? target : $('[name=' + this.hash.slice(1) + ']');
            if (target.length) {
                event.preventDefault();
                $('html, body').animate({
                    scrollTop: target.offset().top - heightSlider
                }, SCROLLSPY_TIME, function () {
                    var $target = $(target);
                    $target.focus();
                    if ($target.is(":focus")) {
                        return false;
                    } else {
                        $target.focus();
                    };
                });
            }
        }
    });
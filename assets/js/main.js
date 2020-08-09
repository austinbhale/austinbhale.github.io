$(function () {

    var isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

    // Remove the scrolling effect for non-mobile devices.
    for (var i = 1; i < 4; i++) {
        (isMobile) ? $('#parallax-mobile' + i).css("display", "block") : $('#parallax' + i).css("display", "block");
        if (isMobile) {
            $('#parallax-ctr' + i).removeClass("parallax-container" + i).addClass("noparallax-container");
        }
    }

    // Set visited cookie for returning to displayed text.
    if (getCookie('visited')) {
        $('.hideme').css("opacity", "1.0");
        (window.innerWidth <= 600) ? $('.moving-line').css("width", "50px") : $('.moving-line').css("width", "100px");
        $("body").removeClass("hideme");
        deleteCookie('visited');
    } else {
        $(window).scroll(function () {

            $('.hideme').each(function (i) {
                var top_half_of_object = $(this).position().top + ($(this).outerHeight() / 4);
                var bottom_of_window = $(window).scrollTop() + $(window).height();
                // If the object is completely visible in the window, fade it in.

                if (bottom_of_window > top_half_of_object) {
                    $(this).animate({
                        'opacity': '1'
                    }, 500);
                    lines();
                }
            });
        });
        // Cookie expires in 999 days.
        setCookie('visited', 'true', 999);
    }

    // Horizontal languages/technologies area.
    $('.customer-logos').slick({
        slidesToShow: 4,
        slidesToScroll: 1,
        draggable: true,
        autoplay: true,
        autoplaySpeed: 1000,
        arrows: false,
        swipeToSlide: true,
        dots: false,
        pauseOnHover: false,
        responsive: [{
            breakpoint: 768,
            settings: {
                slidesToShow: 4
            }
        }, {
            breakpoint: 600,
            settings: {
                slidesToShow: 3
            }
        }]
    });

    anime.timeline({
        loop: false
    })
        .add({
            targets: '.h',
            translateY: (window.innerWidth <= 600) ? '14vh' : '25vh',
            easing: 'easeInOutExpo',
            delay: 600,
            endDelay: 300,
            duration: 2000
        })
        .add({
            targets: ['.fname', '.lname'],
            opacity: 1
        })
        .add({
            targets: '.animate-ctr .first-letter',
            scale: [0.3, 1],
            opacity: [0, 1],
            translateZ: 0,
            easing: "easeOutExpo",
            duration: 600,
            delay: (el, i) => 70 * (i + 1)
        })
        .add({
            targets: '.animate-ctr .last-letter',
            scale: [0.3, 1],
            opacity: [0, 1],
            translateZ: 0,
            easing: "easeOutExpo",
            duration: 600,
            delay: (el, i) => 70 * (i + 1)
        })
        .add({
            targets: '.animate-ctr',
            translateX: '-25%',
            easing: "easeInOutExpo",
            duration: 1000
        })
        .add({
            targets: ['.links', '#nav'],
            opacity: 1,
            duration: 1600
        })
        .add({
            targets: '.down-arr',
            opacity: 1,
            easing: 'easeInOutExpo',
            duration: 700,
            translateY: '90vh'
        });
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

var fNameWrapper = document.querySelector('.animate-ctr .fname');
fNameWrapper.innerHTML = fNameWrapper.textContent.replace(/\S/g, "<span class='first-letter'>$&</span>");

var lNameWrapper = document.querySelector('.animate-ctr .lname');
lNameWrapper.innerHTML = lNameWrapper.textContent.replace(/\S/g, "<span class='last-letter'>$&</span>");

function lines() {
    anime({
        targets: '.moving-line',
        width: (window.innerWidth <= 600) ? '50px' : '100px',
        easing: 'easeOutExpo',
        duration: 2000
    });
}


$('#gform').on('submit', function (e) {

    // Boolean variables for checking valid form responses.
    var valids = false;
    var invalids = false;
    var received = false;
    var elementsValid = document.getElementsByClassName('valid');
    var elementsInvalid = document.getElementsByClassName('invalid');

    (elementsValid.length > 3) ? valids = true : valids = false;
    (elementsInvalid.length == 0) ? invalids = true : invalids = false;

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
    (location.hash == "#contact") ? scrollBy(0, -heightSlider) : scrollBy(0, -heightSlider);
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

// Project triangle animation
var btn = document.getElementById('project0');
var oldHeight = $('.projects').height();
var active = false;
var activeSection = '';
$(".project-links").click(function (event) {
    if (active) {
        return;
    }
    var section = '.' + event.target.id;

    if (/\.back/i.test(section)) {
        $(".project-name").css("display", "none");
        $(".folders").css("display", "block");
        return;
    }

    // Prevent rapid clicking bugs.
    if (!(/\.project[0-9]+/i.test(section) || /\.folder[0-9]+/i.test(section)) || active) {
        return;
    }

    if (/\.folder[0-9]+/i.test(section)) {
        $(".folders").css("display", "none");
        $(".back").css("display", "block");
        $(section).css("display", "block");
        return;
    }

    $(section).css("display", "block");
    active = true;
    activeSection = section;
    var projectsHeight = false;
    $('.projects').animate({
        height: $(section).height() + 15
    }, 200, function () {
        projectsHeight = true;
    });
    var morphing = anime({
        targets: '.polymorph',
        points: [{
            value: '0,40 0, 110 0, 0 47.7, 0 67, 76'
        },
        {
            value: '0,80 0, 110 50, 100 0, 0 0, 76'
        }
        ],
        easing: 'easeOutQuad',
        duration: 800,
        loop: false
    });

    var sectionFadeIn = false;
    var fadeIn = anime({
        targets: section,
        opacity: 1,
        duration: 500,
        easing: 'easeInQuad',
    });
    fadeIn.finished.then(function () {
        sectionFadeIn = true;
    });

    var linksFadeOut = false;
    $('.project-links').fadeOut(500, function () {
        linksFadeOut = true;
    });

    var promise = morphing.finished.then(() => {
        $(".cta2").click(function () {
            if (sectionFadeIn && linksFadeOut && projectsHeight) {
                activeSection = '';
                var morphingBack = anime({
                    targets: '.polymorph',
                    points: [{
                        value: '0,80 0, 110 0, 0 47.7, 0 67, 76'
                    },
                    {
                        value: '0,40 0,110 0,0 49.3,0 215,0'
                    }
                    ],
                    easing: 'easeOutQuad',
                    duration: 800,
                    loop: false
                });
                var fadeOut = anime({
                    targets: section,
                    opacity: 0,
                    duration: 500,
                    easing: 'easeOutQuad'
                });
                fadeOut.finished.then(function () {
                    sectionFadeIn = false;
                });

                $('.project-links').fadeIn(500, function () {
                    linksFadeOut = false;
                });
                $('.projects').animate({
                    height: oldHeight
                }, 300, function () {
                    projectsHeight = false;
                });
                $(section).toggle();
                var finished = morphingBack.finished.then(() => {
                    if (!(sectionFadeIn && linksFadeOut && projectsHeight)) {
                        active = false;
                    }
                });
            }
        });
    });
});

window.addEventListener('resize', function (event) {
    if (activeSection !== '') {
        $('.projects').height($(activeSection).height() + 15)
    }

    $('.h').css("transform", `translateY(${$('.lname').css("margin-top")})`);
});
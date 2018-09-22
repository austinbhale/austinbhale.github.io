var SCROLLSPY_TIME = 700;
var heightSlider = $('#nav').height();
var isMobile = false;

$(window).load(function() {
    if ( /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {isMobile = true;}
   
    for (var i=0; i<5; i++) {
        (isMobile) ? $('#parallax-mobile' + i).css("display", "block") : $('#parallax' + i).css("display", "block");
        if (isMobile) {$('#parallax-ctr'+ i).removeClass("parallax-container"+i).addClass("noparallax-container");}
    }
});

var elementsValid = document.getElementsByClassName('valid');
var elementsInvalid = document.getElementsByClassName('invalid');
var valids = false; var invalids = false; var received = false;
$('#gform').on('submit', function(e) {
     
    (elementsValid.length > 3) ? valids = true : valids = false;
    (elementsInvalid.length == 0) ? invalids = true : invalids = false;
    
    if (valids && invalids) {
        $('#gform *').fadeOut(2000, function() {
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
var shiftWindow = function() { 
    (location.hash == "#contact") ? scrollBy(0, -heightSlider) : scrollBy(0, -heightSlider); };
if (location.hash) shiftWindow();
window.addEventListener("hashchange", shiftWindow);

// Smooth scroll to hash -- selects each used link with hashes.
$('a[href*="#"]') 
    .not('[href="#"]')
    .not('[href="#0"]')
    .click(function(event) {
        if (location.pathname.replace(/^\//, '') == this.pathname.replace(/^\//, '') 
            && location.hostname == this.hostname) {
            var target = $(this.hash);
            target = target.length ? target : $('[name=' + this.hash.slice(1) + ']');
            if (target.length) {
                event.preventDefault();
                $('html, body').animate({
                    scrollTop: target.offset().top - heightSlider
                }, SCROLLSPY_TIME, function() {
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
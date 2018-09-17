var SCROLLSPY_TIME = 500;
var heightSlider = $('#nav').height();
var isMobile = false;

$(window).load(function() {
    if ( /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {isMobile = true;}
   
   for (var i=0; i<4; i++) {
        (isMobile) ? $('#parallax' + i).css("display", "none") : $('#parallax-mobile' + i).css("display", "none");
    }
});

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
                    // $target.attr('tabindex','-1');
                    $target.focus();
                };
                });
            }
        }
});
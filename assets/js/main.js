var SCROLLSPY_TIME = 500;
var heightSlider = $('#nav').height();
var isMobile = false;

$(window).load(function() {
    if ( /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {isMobile = true;}
   
   for (var i=0; i<5; i++) {
        (isMobile) ? $('#parallax' + i).css("display", "none") : $('#parallax-mobile' + i).css("display", "none");
    }
});

// Contact form implementation
function submitForm() {
    if ((   $('#first_name').hasClass("validate valid") && 
            $('#last_name').hasClass("validate valid")) && 
        (   $('#email_inline').hasClass("validate valid") && 
            $('#textarea1').hasClass("validate valid")
        )) {
            event.preventDefault();
            var first_name = $('#first_name').val();
            var last_name = $('#last_name').val();
            var email = $('#email_inline').val();
            var message = $('#textarea1').val();

    $.ajax({
      type: "POST",
      url: "mail.php",
      data: { first_name: first_name, last_name: last_name,
              email: email, message: message }
      })
      .done(function() {
          alert("data saved");
      });  
    }
}

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
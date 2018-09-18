<?php
if ($_POST) {
    $first_name = $_POST['first'];
    $last_name = $_POST['last'];
    $email = $_POST['email'];
    $message = $_POST['message'];

    $to = "haleau@live.unc.edu";
    $subject = "Personal Website Contact Form";
    $body = "Name: ".$first_name." ".$last_name."\nEmail: ".$email."\nMessage: ".$message;
    $headers = "From: " . $email;

    //send email
    mail($to, $subject, $body, $headers) or die("Error!");
    echo "Thank you"  . " -" . "<a href='http://austinbhale.com' style='text-decoration:none;color:#ff0099;'> Return Home</a>";
} else {
    echo "ERROR reading PHP file!";
}
?>
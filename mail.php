<?php
$first_name = $_POST['first'];
$last_name = $_POST['last'];
$email = $_POST['email'];
$message = $_POST['message'];

$to = "haleau@live.unc.edu";
$subject = "Personal Website Contact Form";
$body = "Name: ".$first_name." ".$last_name."\nEmail: ".$email."\nMessage: ".$message;
$headers = "From: " . $email;

//send email
mail($to, $subject, $body, $headers);
?>
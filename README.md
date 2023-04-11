Simple app that encrypt string to image, and decrypt such image to string. Each iteration consists of two stages - encryption part, and decryption part. 

* Iteration 1:

  App reads input from termial, and changes each character into proper ascii number that will be used as a coolour of the pixel. Each letter is one pixel in grayscale image. Image is a square with size n*n, where (n-1)^2 < string_legth <= n^2. String is filled up with ascii 0 to the size of an image. Iamge is saved with provided path.

  App reads image path from termial, and loads said iamge. Each pixel is transalted to proper ascii character. As output app prints string in terminal.

* Iteration 2:
  
  Saved image is a rectangle with size n*h, where n is calculated as in Iteration 1, and h is n*(h-1) < string_length <= n*h.

* Iteration 3:
  
  App checks if input string is path to text file, and if it is, loads text file contents as input.

  App beside printing to termial, can save decoded string to text file, with provided path.

* Iteration 4:
  
  Saved image is in rgb format instead of grayscale. Each pixel contains 3 characters.

* Iteration 5:
  
  App can use RSA encryption when encrypting image. User provides public RSA key as input. 

  The same can be used when decrypting. User provides private RSA key, and image is decrypted.


Following this book series: https://www.amazon.com/gp/product/B01B5AODD8

In windows, you'll have to specify ascii encoding when piping to a file:

```
PS F:\workspace\raytracer> .\target\debug\raytracer.exe | out-file test.ppm -encoding ascii
```

# lowpoly

This is an implementation of the algorithm described in [this paper](http://cjqian.github.io/docs/tri_iw_paper.pdf) by Crystal J. Qian

It uses rust and the following algorithms:

* [Canny edge detection ](https://en.wikipedia.org/wiki/Canny_edge_detector)
* [Delaunay_triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation)

## Usage

Run

    cargo run --release -- <input_image_path> <output_image_path>

Example:

    cargo run --release -- examples/test2_original.jpeg output.png
    
## Examples 

Original             |  Lowpoly version
:-------------------------:|:-------------------------:
![](https://github.com/yabirgb/lowpoly/blob/master/examples/test1_original.jpeg)  |  ![](https://github.com/yabirgb/lowpoly/blob/master/examples/test1.png)
![](https://github.com/yabirgb/lowpoly/blob/master/examples/test2_original.jpeg)  |  ![](https://github.com/yabirgb/lowpoly/blob/master/examples/test2.png)
![](https://github.com/yabirgb/lowpoly/blob/master/examples/test3_original.jpeg)  |  ![](https://github.com/yabirgb/lowpoly/blob/master/examples/test3.png)

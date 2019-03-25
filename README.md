# lowpoly

This is an implementation of the algorithm described in [this paper](http://cjqian.github.io/docs/tri_iw_paper.pdf) by Crystal J. Qian

It uses rust and the following algorithms:

* [Canny edge detection ](https://en.wikipedia.org/wiki/Canny_edge_detector)
* [Delaunay_triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation)

## Usage

Put the image under the folder `resource` with name `test2.jpeg` (`resources/test2.jpeg`)

Then run

    cargo run
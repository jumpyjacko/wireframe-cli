# wireframe-cli
A quick rust implementation of a wireframe renderer for the terminal inspired by [mattbatwings's Minecraft wireframe renderer](https://youtu.be/hFRlnNci3Rs).
Also probably not a good idea to call it a cli as it's not even an interface you can interact with, you have to change the source code.

Written in Rust because I need to learn more of it. Definitely not optimized as I am not experienced with Rust.

### About the wireframe renderer
This wireframe renderer uses [weak projection](https://en.wikipedia.org/wiki/3D_projection#Weak_perspective_projection) as well as the [digital differential analyzer](https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)) to draw the lines. Lots of translation of the Wikipedia page's psuedo- and C++ code.

![rotating_square_based_pyramid](./wireframe-cli.gif)
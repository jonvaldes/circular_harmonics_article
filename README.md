# Circular Harmonics

The final article can be found at [https://valdes.cc/articles/ch.html](https://valdes.cc/articles/ch.html)

To run the source code that creates the animations, you'll just need a relatively recent Rust installation, and to run `cargo run --release`.
Should work on Linux and Windows out of the box, and I expect it should work on other platforms too.

Compiling the article itself requires having the [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) 
CLI on your path, as well as a python3 install with the [watchdog](https://pypi.org/project/watchdog/) package 
(you can install it with `pip install watchdog`).

Once you have that, you can run `serve.py`, which will generate 2 versions of the article:
- ch.html: Final version ready to upload
- ch_dev.html: Development version, with an autoreload mechanism.

`serve.py` will then run a web server and serve the article on `localhost:8000/ch_dev.html`

# License information

Source code licensed under MIT license. See LICENSE.txt


Article contents licensed under Creative Commons Attribution-ShareAlike license: https://creativecommons.org/licenses/by-sa/4.0/ 

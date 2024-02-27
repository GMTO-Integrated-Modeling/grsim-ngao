# Getting Started

Some of the prerequisites for the the NGAO model are [CUDA] and [Rust].
[CUDA] is the programming language that [CEO] is written into, meaning that a machine with a recent NVIDIA GPU  is also mandatory.
[Rust] is the programming language for DOS Actors.

The location of the [CUDA] compiler `nvcc` must be set with the environment variable `CUDACXX`, e.g.
```bash
export CUDACXX=/usr/local/cuda/bin/nvcc
```
If you do not want to set `CUDACXX` each time you log in, the line above can be written in the shell config file.
For the `bash` shell, this is `.bashrc`.

Once both, [CUDA] and [Rust], are installed, we can start writing our first optical model.


Note that in order to produce model flowcharts, [Graphviz] is needed but the models will run anyway without it.

[CUDA]: https://developer.nvidia.com/cuda-downloads
[CEO]: https://github.com/rconan/CEO
[Rust]: https://www.rust-lang.org/
[Graphviz]: https://www.graphviz.org/
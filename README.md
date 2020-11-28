# qURL (quick URL)

Quick command-line HTTP request utility written in Rust.
The goal of this project is to provide a secure, reliable and quick command-line utility utilising the features Rust language.
Using the verbose mode, it guides you through the process of making a http request, starting from parsed request data and finishing on response containing souce data, headers, ip address etc.


### Note

qURL is a long way from completion, it may be unstable, unsafe and lack many features. It should only be used for testing purposes for now.


### Installation

If you have cargo installed on your system:

.. code-block:: bash

    $ cargo install qurl

You can also head over to [releases tab](https://github.com/Zeerooth/qurl/releases/) and download the latest binary for your OS.


### Usage

Simplest GET request:

```bash
    $ qurl get https://httpbin.org/get
```


To enable verbose output and get more information about requests and responses, add ``-v`` flag (which will from now on be used in examples to provide some better understading about what's going on under the hood):

```bash
    $ qurl get https://httpbin.org/get -v
```


Let's add some headers (note that headers are **not** case-sensitive):

```bash
    $ qurl get https://httpbin.org/get -v --header accept:application/json
```


What about posting some json data?:

```bash
    $ qurl post https://httpbin.org/post -v --json '{example:"json"}'
```


See ``qurl --help`` for more options.


### Special thanks

* Rust community - for creating an awesome language with well-maintained documentation and excellent environment for learning and programming
* [clap](https://github.com/clap-rs/clap) and [reqwest](https://github.com/seanmonstar/reqwest) teams for amazing packages that this project heavily relies on

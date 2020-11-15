# biotools # 

[![Build Status](https://travis-ci.org/wckdouglas/biotools.svg?branch=main)](https://travis-ci.org/wckdouglas/biotools)
[![codecov](https://codecov.io/gh/wckdouglas/biotools/branch/main/graph/badge.svg)](https://codecov.io/gh/wckdouglas/biotools)


A playground for implementing functions in Rust and porting into python

## Setting up environment ##

```
$ conda create -n pyrust python=3.6 ipython pytest
```

Apparently, installing *maturin* via *conda-forge* channel will break it.

```
$ pip install maturin
```

## Installing ##

```
$ maturin develop
```


## Loading python package ##

```
$ ipython
> import biotools_lib
```

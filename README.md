# stupidf


`stupidf` is a library for limited parsing of STDF files. The `STDF` structure can be used
directly in rust, or alternatively sent out to Python using the `parse_stdf` function.

STDF is the [Standard Test Data Format](https://en.wikipedia.org/wiki/Standard_Test_Data_Format) and is commonly used for high-volume test of semiconductors in Automated Test Equipment (ATE) systems. 

The purpose of the library is to quickly and efficiently parse STDF files (which are a fairly unfriendly binary linked list-based format) into more friendly [polars](https://pola.rs/) [DataFrame](https://docs.pola.rs/user-guide/concepts/data-types-and-structures/#dataframe) format. 

# Example

```
let verbose = false;
if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
    let df: DataFrame = (&stdf.test_data).into();
    let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
    println!("{df:#?}");
    println!("{df_fmti}");
    }
```

Also contains Python bindings to this functionality, e.g.

```
   import stupidf as sf
   stdf = sf.parse_stdf("my_stdf.stdf")
   stdf['df']
````

# Installation

The rust library can be compiled simply wtih

```cargo build --deveop```

Docs can be built and viewed with

```
cargo docs
cargo docs --open
```

The Python bindings can be made using [`maturin`](https://www.maturin.rs/). Activate the desired virtualenv, then run

```
maturin develop
```

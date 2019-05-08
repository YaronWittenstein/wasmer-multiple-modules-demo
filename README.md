# wasmer-multiple-modules-demo

A small demo that showcases the following scenario:
* Given two wasm modules to be used each as a library, each one exporting a function
* Instantiates each module with an empty import object
* A *glue* wasm module the imports both *lib* modules
* Instantiates the *glue* wasm with an import object containing the instances of the *lib* modules
* Calling *glue* exported function, will trigger calls to the *lib* modules


The demo is using the awesome [wasmer runtime](https://github.com/wasmerio/wasmer)


<a id="0x1_debug"></a>

# Module `0x1::debug`

Module providing debug functionality.


-  [Constants](#@Constants_0)
-  [Function `print`](#0x1_debug_print)
-  [Function `print_stack_trace`](#0x1_debug_print_stack_trace)
-  [Function `format`](#0x1_debug_format)
-  [Function `native_print`](#0x1_debug_native_print)
-  [Function `native_stack_trace`](#0x1_debug_native_stack_trace)
-  [Specification](#@Specification_1)
    -  [Function `print`](#@Specification_1_print)
    -  [Function `print_stack_trace`](#@Specification_1_print_stack_trace)
    -  [Function `native_print`](#@Specification_1_native_print)
    -  [Function `native_stack_trace`](#@Specification_1_native_stack_trace)


```move
module 0x1::debug {
    use 0x1::string;
    use 0x1::string_utils;
}
```


<a id="@Constants_0"></a>

## Constants


<a id="0x1_debug_MSG_1"></a>



```move
module 0x1::debug {
    const MSG_1: vector<u8> = [97, 98, 99, 100, 101, 102];
}
```


<a id="0x1_debug_MSG_2"></a>



```move
module 0x1::debug {
    const MSG_2: vector<u8> = [49, 50, 51, 52, 53, 54];
}
```


<a id="0x1_debug_print"></a>

## Function `print`



```move
module 0x1::debug {
    public fun print<T>(x: &T)
}
```


##### Implementation


```move
module 0x1::debug {
    public fun print<T>(x: &T) {
        native_print(format(x));
    }
}
```


<a id="0x1_debug_print_stack_trace"></a>

## Function `print_stack_trace`



```move
module 0x1::debug {
    public fun print_stack_trace()
}
```


##### Implementation


```move
module 0x1::debug {
    public fun print_stack_trace() {
        native_print(native_stack_trace());
    }
}
```


<a id="0x1_debug_format"></a>

## Function `format`



```move
module 0x1::debug {
    fun format<T>(x: &T): string::String
}
```


##### Implementation


```move
module 0x1::debug {
    inline fun format<T>(x: &T): String {
        aptos_std::string_utils::debug_string(x)
    }
}
```


<a id="0x1_debug_native_print"></a>

## Function `native_print`



```move
module 0x1::debug {
    fun native_print(x: string::String)
}
```


##### Implementation


```move
module 0x1::debug {
    native fun native_print(x: String);
}
```


<a id="0x1_debug_native_stack_trace"></a>

## Function `native_stack_trace`



```move
module 0x1::debug {
    fun native_stack_trace(): string::String
}
```


##### Implementation


```move
module 0x1::debug {
    native fun native_stack_trace(): String;
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_print"></a>

### Function `print`


```move
module 0x1::debug {
    public fun print<T>(x: &T)
}
```



```move
module 0x1::debug {
    aborts_if false;
}
```


<a id="@Specification_1_print_stack_trace"></a>

### Function `print_stack_trace`


```move
module 0x1::debug {
    public fun print_stack_trace()
}
```



```move
module 0x1::debug {
    aborts_if false;
}
```


<a id="@Specification_1_native_print"></a>

### Function `native_print`


```move
module 0x1::debug {
    fun native_print(x: string::String)
}
```



```move
module 0x1::debug {
    pragma opaque;
    aborts_if false;
}
```


<a id="@Specification_1_native_stack_trace"></a>

### Function `native_stack_trace`


```move
module 0x1::debug {
    fun native_stack_trace(): string::String
}
```



```move
module 0x1::debug {
    pragma opaque;
    aborts_if false;
}
```

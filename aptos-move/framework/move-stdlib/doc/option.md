
<a id="0x1_option"></a>

# Module `0x1::option`

This module defines the Option type and its methods to represent and handle an optional value.


-  [Struct `Option`](#0x1_option_Option)
-  [Constants](#@Constants_0)
-  [Function `none`](#0x1_option_none)
-  [Function `some`](#0x1_option_some)
-  [Function `from_vec`](#0x1_option_from_vec)
-  [Function `is_none`](#0x1_option_is_none)
-  [Function `is_some`](#0x1_option_is_some)
-  [Function `contains`](#0x1_option_contains)
-  [Function `borrow`](#0x1_option_borrow)
-  [Function `borrow_with_default`](#0x1_option_borrow_with_default)
-  [Function `get_with_default`](#0x1_option_get_with_default)
-  [Function `fill`](#0x1_option_fill)
-  [Function `extract`](#0x1_option_extract)
-  [Function `borrow_mut`](#0x1_option_borrow_mut)
-  [Function `swap`](#0x1_option_swap)
-  [Function `swap_or_fill`](#0x1_option_swap_or_fill)
-  [Function `destroy_with_default`](#0x1_option_destroy_with_default)
-  [Function `destroy_some`](#0x1_option_destroy_some)
-  [Function `destroy_none`](#0x1_option_destroy_none)
-  [Function `to_vec`](#0x1_option_to_vec)
-  [Function `for_each`](#0x1_option_for_each)
-  [Function `for_each_ref`](#0x1_option_for_each_ref)
-  [Function `for_each_mut`](#0x1_option_for_each_mut)
-  [Function `fold`](#0x1_option_fold)
-  [Function `map`](#0x1_option_map)
-  [Function `map_ref`](#0x1_option_map_ref)
-  [Function `filter`](#0x1_option_filter)
-  [Function `any`](#0x1_option_any)
-  [Function `destroy`](#0x1_option_destroy)
-  [Specification](#@Specification_1)
    -  [Helper Schema](#@Helper_Schema_2)
    -  [Struct `Option`](#@Specification_1_Option)
    -  [Function `none`](#@Specification_1_none)
    -  [Function `some`](#@Specification_1_some)
    -  [Function `from_vec`](#@Specification_1_from_vec)
    -  [Function `is_none`](#@Specification_1_is_none)
    -  [Function `is_some`](#@Specification_1_is_some)
    -  [Function `contains`](#@Specification_1_contains)
    -  [Function `borrow`](#@Specification_1_borrow)
    -  [Function `borrow_with_default`](#@Specification_1_borrow_with_default)
    -  [Function `get_with_default`](#@Specification_1_get_with_default)
    -  [Function `fill`](#@Specification_1_fill)
    -  [Function `extract`](#@Specification_1_extract)
    -  [Function `borrow_mut`](#@Specification_1_borrow_mut)
    -  [Function `swap`](#@Specification_1_swap)
    -  [Function `swap_or_fill`](#@Specification_1_swap_or_fill)
    -  [Function `destroy_with_default`](#@Specification_1_destroy_with_default)
    -  [Function `destroy_some`](#@Specification_1_destroy_some)
    -  [Function `destroy_none`](#@Specification_1_destroy_none)
    -  [Function `to_vec`](#@Specification_1_to_vec)


```move
module 0x1::option {
    use 0x1::vector;
}
```


<a id="0x1_option_Option"></a>

## Struct `Option`

Abstraction of a value that may or may not be present. Implemented with a vector of size
zero or one because Move bytecode does not have ADTs.


```move
module 0x1::option {
    struct Option<Element> has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`vec: vector<Element>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_option_EOPTION_IS_SET"></a>

The `Option` is in an invalid state for the operation attempted.
The `Option` is `Some` while it should be `None`.


```move
module 0x1::option {
    const EOPTION_IS_SET: u64 = 262144;
}
```


<a id="0x1_option_EOPTION_NOT_SET"></a>

The `Option` is in an invalid state for the operation attempted.
The `Option` is `None` while it should be `Some`.


```move
module 0x1::option {
    const EOPTION_NOT_SET: u64 = 262145;
}
```


<a id="0x1_option_EOPTION_VEC_TOO_LONG"></a>

Cannot construct an option from a vector with 2 or more elements.


```move
module 0x1::option {
    const EOPTION_VEC_TOO_LONG: u64 = 262146;
}
```


<a id="0x1_option_none"></a>

## Function `none`

Return an empty `Option`


```move
module 0x1::option {
    public fun none<Element>(): option::Option<Element>
}
```


##### Implementation


```move
module 0x1::option {
    public fun none<Element>(): Option<Element> {
        Option { vec: vector::empty() }
    }
}
```


<a id="0x1_option_some"></a>

## Function `some`

Return an `Option` containing `e`


```move
module 0x1::option {
    public fun some<Element>(e: Element): option::Option<Element>
}
```


##### Implementation


```move
module 0x1::option {
    public fun some<Element>(e: Element): Option<Element> {
        Option { vec: vector::singleton(e) }
    }
}
```


<a id="0x1_option_from_vec"></a>

## Function `from_vec`



```move
module 0x1::option {
    public fun from_vec<Element>(vec: vector<Element>): option::Option<Element>
}
```


##### Implementation


```move
module 0x1::option {
    public fun from_vec<Element>(vec: vector<Element>): Option<Element> {
        assert!(vector::length(&vec) <= 1, EOPTION_VEC_TOO_LONG);
        Option { vec }
    }
}
```


<a id="0x1_option_is_none"></a>

## Function `is_none`

Return true if `t` does not hold a value


```move
module 0x1::option {
    public fun is_none<Element>(t: &option::Option<Element>): bool
}
```


##### Implementation


```move
module 0x1::option {
    public fun is_none<Element>(t: &Option<Element>): bool {
        vector::is_empty(&t.vec)
    }
}
```


<a id="0x1_option_is_some"></a>

## Function `is_some`

Return true if `t` holds a value


```move
module 0x1::option {
    public fun is_some<Element>(t: &option::Option<Element>): bool
}
```


##### Implementation


```move
module 0x1::option {
    public fun is_some<Element>(t: &Option<Element>): bool {
        !vector::is_empty(&t.vec)
    }
}
```


<a id="0x1_option_contains"></a>

## Function `contains`

Return true if the value in `t` is equal to `e_ref`
Always returns `false` if `t` does not hold a value


```move
module 0x1::option {
    public fun contains<Element>(t: &option::Option<Element>, e_ref: &Element): bool
}
```


##### Implementation


```move
module 0x1::option {
    public fun contains<Element>(t: &Option<Element>, e_ref: &Element): bool {
        vector::contains(&t.vec, e_ref)
    }
}
```


<a id="0x1_option_borrow"></a>

## Function `borrow`

Return an immutable reference to the value inside `t`
Aborts if `t` does not hold a value


```move
module 0x1::option {
    public fun borrow<Element>(t: &option::Option<Element>): &Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun borrow<Element>(t: &Option<Element>): &Element {
        assert!(is_some(t), EOPTION_NOT_SET);
        vector::borrow(&t.vec, 0)
    }
}
```


<a id="0x1_option_borrow_with_default"></a>

## Function `borrow_with_default`

Return a reference to the value inside `t` if it holds one
Return `default_ref` if `t` does not hold a value


```move
module 0x1::option {
    public fun borrow_with_default<Element>(t: &option::Option<Element>, default_ref: &Element): &Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun borrow_with_default<Element>(t: &Option<Element>, default_ref: &Element): &Element {
        let vec_ref = &t.vec;
        if (vector::is_empty(vec_ref)) default_ref
        else vector::borrow(vec_ref, 0)
    }
}
```


<a id="0x1_option_get_with_default"></a>

## Function `get_with_default`

Return the value inside `t` if it holds one
Return `default` if `t` does not hold a value


```move
module 0x1::option {
    public fun get_with_default<Element: copy, drop>(t: &option::Option<Element>, default: Element): Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun get_with_default<Element: copy + drop>(
        t: &Option<Element>,
        default: Element,
    ): Element {
        let vec_ref = &t.vec;
        if (vector::is_empty(vec_ref)) default
        else *vector::borrow(vec_ref, 0)
    }
}
```


<a id="0x1_option_fill"></a>

## Function `fill`

Convert the none option `t` to a some option by adding `e`.
Aborts if `t` already holds a value


```move
module 0x1::option {
    public fun fill<Element>(t: &mut option::Option<Element>, e: Element)
}
```


##### Implementation


```move
module 0x1::option {
    public fun fill<Element>(t: &mut Option<Element>, e: Element) {
        let vec_ref = &mut t.vec;
        if (vector::is_empty(vec_ref)) vector::push_back(vec_ref, e)
        else abort EOPTION_IS_SET
    }
}
```


<a id="0x1_option_extract"></a>

## Function `extract`

Convert a `some` option to a `none` by removing and returning the value stored inside `t`
Aborts if `t` does not hold a value


```move
module 0x1::option {
    public fun extract<Element>(t: &mut option::Option<Element>): Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun extract<Element>(t: &mut Option<Element>): Element {
        assert!(is_some(t), EOPTION_NOT_SET);
        vector::pop_back(&mut t.vec)
    }
}
```


<a id="0x1_option_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the value inside `t`
Aborts if `t` does not hold a value


```move
module 0x1::option {
    public fun borrow_mut<Element>(t: &mut option::Option<Element>): &mut Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun borrow_mut<Element>(t: &mut Option<Element>): &mut Element {
        assert!(is_some(t), EOPTION_NOT_SET);
        vector::borrow_mut(&mut t.vec, 0)
    }
}
```


<a id="0x1_option_swap"></a>

## Function `swap`

Swap the old value inside `t` with `e` and return the old value
Aborts if `t` does not hold a value


```move
module 0x1::option {
    public fun swap<Element>(t: &mut option::Option<Element>, e: Element): Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun swap<Element>(t: &mut Option<Element>, e: Element): Element {
        assert!(is_some(t), EOPTION_NOT_SET);
        let vec_ref = &mut t.vec;
        let old_value = vector::pop_back(vec_ref);
        vector::push_back(vec_ref, e);
        old_value
    }
}
```


<a id="0x1_option_swap_or_fill"></a>

## Function `swap_or_fill`

Swap the old value inside `t` with `e` and return the old value;
or if there is no old value, fill it with `e`.
Different from swap(), swap_or_fill() allows for `t` not holding a value.


```move
module 0x1::option {
    public fun swap_or_fill<Element>(t: &mut option::Option<Element>, e: Element): option::Option<Element>
}
```


##### Implementation


```move
module 0x1::option {
    public fun swap_or_fill<Element>(t: &mut Option<Element>, e: Element): Option<Element> {
        let vec_ref = &mut t.vec;
        let old_value = if (vector::is_empty(vec_ref)) none()
            else some(vector::pop_back(vec_ref));
        vector::push_back(vec_ref, e);
        old_value
    }
}
```


<a id="0x1_option_destroy_with_default"></a>

## Function `destroy_with_default`

Destroys `t.` If `t` holds a value, return it. Returns `default` otherwise


```move
module 0x1::option {
    public fun destroy_with_default<Element: drop>(t: option::Option<Element>, default: Element): Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun destroy_with_default<Element: drop>(t: Option<Element>, default: Element): Element {
        let Option { vec } = t;
        if (vector::is_empty(&mut vec)) default
        else vector::pop_back(&mut vec)
    }
}
```


<a id="0x1_option_destroy_some"></a>

## Function `destroy_some`

Unpack `t` and return its contents
Aborts if `t` does not hold a value


```move
module 0x1::option {
    public fun destroy_some<Element>(t: option::Option<Element>): Element
}
```


##### Implementation


```move
module 0x1::option {
    public fun destroy_some<Element>(t: Option<Element>): Element {
        assert!(is_some(&t), EOPTION_NOT_SET);
        let Option { vec } = t;
        let elem = vector::pop_back(&mut vec);
        vector::destroy_empty(vec);
        elem
    }
}
```


<a id="0x1_option_destroy_none"></a>

## Function `destroy_none`

Unpack `t`
Aborts if `t` holds a value


```move
module 0x1::option {
    public fun destroy_none<Element>(t: option::Option<Element>)
}
```


##### Implementation


```move
module 0x1::option {
    public fun destroy_none<Element>(t: Option<Element>) {
        assert!(is_none(&t), EOPTION_IS_SET);
        let Option { vec } = t;
        vector::destroy_empty(vec)
    }
}
```


<a id="0x1_option_to_vec"></a>

## Function `to_vec`

Convert `t` into a vector of length 1 if it is `Some`,
and an empty vector otherwise


```move
module 0x1::option {
    public fun to_vec<Element>(t: option::Option<Element>): vector<Element>
}
```


##### Implementation


```move
module 0x1::option {
    public fun to_vec<Element>(t: Option<Element>): vector<Element> {
        let Option { vec } = t;
        vec
    }
}
```


<a id="0x1_option_for_each"></a>

## Function `for_each`

Apply the function to the optional element, consuming it. Does nothing if no value present.


```move
module 0x1::option {
    public fun for_each<Element>(o: option::Option<Element>, f: |Element|)
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun for_each<Element>(o: Option<Element>, f: |Element|) {
        if (is_some(&o)) {
            f(destroy_some(o))
        } else {
            destroy_none(o)
        }
    }
}
```


<a id="0x1_option_for_each_ref"></a>

## Function `for_each_ref`

Apply the function to the optional element reference. Does nothing if no value present.


```move
module 0x1::option {
    public fun for_each_ref<Element>(o: &option::Option<Element>, f: |&Element|)
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun for_each_ref<Element>(o: &Option<Element>, f: |&Element|) {
        if (is_some(o)) {
            f(borrow(o))
        }
    }
}
```


<a id="0x1_option_for_each_mut"></a>

## Function `for_each_mut`

Apply the function to the optional element reference. Does nothing if no value present.


```move
module 0x1::option {
    public fun for_each_mut<Element>(o: &mut option::Option<Element>, f: |&mut Element|)
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun for_each_mut<Element>(o: &mut Option<Element>, f: |&mut Element|) {
        if (is_some(o)) {
            f(borrow_mut(o))
        }
    }
}
```


<a id="0x1_option_fold"></a>

## Function `fold`

Folds the function over the optional element.


```move
module 0x1::option {
    public fun fold<Accumulator, Element>(o: option::Option<Element>, init: Accumulator, f: |(Accumulator, Element)|Accumulator): Accumulator
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun fold<Accumulator, Element>(
        o: Option<Element>,
        init: Accumulator,
        f: |Accumulator,Element|Accumulator
    ): Accumulator {
        if (is_some(&o)) {
            f(init, destroy_some(o))
        } else {
            destroy_none(o);
            init
        }
    }
}
```


<a id="0x1_option_map"></a>

## Function `map`

Maps the content of an option.


```move
module 0x1::option {
    public fun map<Element, OtherElement>(o: option::Option<Element>, f: |Element|OtherElement): option::Option<OtherElement>
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun map<Element, OtherElement>(o: Option<Element>, f: |Element|OtherElement): Option<OtherElement> {
        if (is_some(&o)) {
            some(f(destroy_some(o)))
        } else {
            destroy_none(o);
            none()
        }
    }
}
```


<a id="0x1_option_map_ref"></a>

## Function `map_ref`

Maps the content of an option without destroying the original option.


```move
module 0x1::option {
    public fun map_ref<Element, OtherElement>(o: &option::Option<Element>, f: |&Element|OtherElement): option::Option<OtherElement>
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun map_ref<Element, OtherElement>(
        o: &Option<Element>, f: |&Element|OtherElement): Option<OtherElement> {
        if (is_some(o)) {
            some(f(borrow(o)))
        } else {
            none()
        }
    }
}
```


<a id="0x1_option_filter"></a>

## Function `filter`

Filters the content of an option


```move
module 0x1::option {
    public fun filter<Element: drop>(o: option::Option<Element>, f: |&Element|bool): option::Option<Element>
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun filter<Element:drop>(o: Option<Element>, f: |&Element|bool): Option<Element> {
        if (is_some(&o) && f(borrow(&o))) {
            o
        } else {
            none()
        }
    }
}
```


<a id="0x1_option_any"></a>

## Function `any`

Returns true if the option contains an element which satisfies predicate.


```move
module 0x1::option {
    public fun any<Element>(o: &option::Option<Element>, p: |&Element|bool): bool
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun any<Element>(o: &Option<Element>, p: |&Element|bool): bool {
        is_some(o) && p(borrow(o))
    }
}
```


<a id="0x1_option_destroy"></a>

## Function `destroy`

Utility function to destroy an option that is not droppable.


```move
module 0x1::option {
    public fun destroy<Element>(o: option::Option<Element>, d: |Element|)
}
```


##### Implementation


```move
module 0x1::option {
    public inline fun destroy<Element>(o: Option<Element>, d: |Element|) {
        let vec = to_vec(o);
        vector::destroy(vec, |e| d(e));
    }
}
```


<a id="@Specification_1"></a>

## Specification




```move
module 0x1::option {
    pragma aborts_if_is_strict;
}
```


<a id="@Helper_Schema_2"></a>

### Helper Schema



<a id="0x1_option_AbortsIfNone"></a>


```move
module 0x1::option {
    schema AbortsIfNone<Element> {
        t: Option<Element>;
        aborts_if spec_is_none(t) with EOPTION_NOT_SET;
    }
}
```


<a id="@Specification_1_Option"></a>

### Struct `Option`


```move
module 0x1::option {
    struct Option<Element> has copy, drop, store
}
```


<dl>
<dt>
`vec: vector<Element>`
</dt>
<dd>

</dd>
</dl>


The size of vector is always less than equal to 1
because it&apos;s 0 for &quot;none&quot; or 1 for &quot;some&quot;.


```move
module 0x1::option {
    invariant len(vec) <= 1;
}
```


<a id="@Specification_1_none"></a>

### Function `none`


```move
module 0x1::option {
    public fun none<Element>(): option::Option<Element>
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == spec_none<Element>();
}
```



<a id="0x1_option_spec_none"></a>


```move
module 0x1::option {
    fun spec_none<Element>(): Option<Element> {
       Option{ vec: vec() }
    }
}
```


<a id="@Specification_1_some"></a>

### Function `some`


```move
module 0x1::option {
    public fun some<Element>(e: Element): option::Option<Element>
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == spec_some(e);
}
```



<a id="0x1_option_spec_some"></a>


```move
module 0x1::option {
    fun spec_some<Element>(e: Element): Option<Element> {
       Option{ vec: vec(e) }
    }
}
```


<a id="@Specification_1_from_vec"></a>

### Function `from_vec`


```move
module 0x1::option {
    public fun from_vec<Element>(vec: vector<Element>): option::Option<Element>
}
```



```move
module 0x1::option {
    aborts_if vector::length(vec) > 1;
}
```


<a id="@Specification_1_is_none"></a>

### Function `is_none`


```move
module 0x1::option {
    public fun is_none<Element>(t: &option::Option<Element>): bool
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == spec_is_none(t);
}
```



<a id="0x1_option_spec_is_none"></a>


```move
module 0x1::option {
    fun spec_is_none<Element>(t: Option<Element>): bool {
       vector::is_empty(t.vec)
    }
}
```


<a id="@Specification_1_is_some"></a>

### Function `is_some`


```move
module 0x1::option {
    public fun is_some<Element>(t: &option::Option<Element>): bool
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == spec_is_some(t);
}
```



<a id="0x1_option_spec_is_some"></a>


```move
module 0x1::option {
    fun spec_is_some<Element>(t: Option<Element>): bool {
       !vector::is_empty(t.vec)
    }
}
```


<a id="@Specification_1_contains"></a>

### Function `contains`


```move
module 0x1::option {
    public fun contains<Element>(t: &option::Option<Element>, e_ref: &Element): bool
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == spec_contains(t, e_ref);
}
```



<a id="0x1_option_spec_contains"></a>


```move
module 0x1::option {
    fun spec_contains<Element>(t: Option<Element>, e: Element): bool {
       is_some(t) && borrow(t) == e
    }
}
```


<a id="@Specification_1_borrow"></a>

### Function `borrow`


```move
module 0x1::option {
    public fun borrow<Element>(t: &option::Option<Element>): &Element
}
```



```move
module 0x1::option {
    pragma opaque;
    include AbortsIfNone<Element>;
    ensures result == spec_borrow(t);
}
```



<a id="0x1_option_spec_borrow"></a>


```move
module 0x1::option {
    fun spec_borrow<Element>(t: Option<Element>): Element {
       t.vec[0]
    }
}
```


<a id="@Specification_1_borrow_with_default"></a>

### Function `borrow_with_default`


```move
module 0x1::option {
    public fun borrow_with_default<Element>(t: &option::Option<Element>, default_ref: &Element): &Element
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == (if (spec_is_some(t)) spec_borrow(t) else default_ref);
}
```


<a id="@Specification_1_get_with_default"></a>

### Function `get_with_default`


```move
module 0x1::option {
    public fun get_with_default<Element: copy, drop>(t: &option::Option<Element>, default: Element): Element
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == (if (spec_is_some(t)) spec_borrow(t) else default);
}
```


<a id="@Specification_1_fill"></a>

### Function `fill`


```move
module 0x1::option {
    public fun fill<Element>(t: &mut option::Option<Element>, e: Element)
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if spec_is_some(t) with EOPTION_IS_SET;
    ensures spec_is_some(t);
    ensures spec_borrow(t) == e;
}
```


<a id="@Specification_1_extract"></a>

### Function `extract`


```move
module 0x1::option {
    public fun extract<Element>(t: &mut option::Option<Element>): Element
}
```



```move
module 0x1::option {
    pragma opaque;
    include AbortsIfNone<Element>;
    ensures result == spec_borrow(old(t));
    ensures spec_is_none(t);
}
```


<a id="@Specification_1_borrow_mut"></a>

### Function `borrow_mut`


```move
module 0x1::option {
    public fun borrow_mut<Element>(t: &mut option::Option<Element>): &mut Element
}
```



```move
module 0x1::option {
    include AbortsIfNone<Element>;
    ensures result == spec_borrow(t);
    ensures t == old(t);
}
```


<a id="@Specification_1_swap"></a>

### Function `swap`


```move
module 0x1::option {
    public fun swap<Element>(t: &mut option::Option<Element>, e: Element): Element
}
```



```move
module 0x1::option {
    pragma opaque;
    include AbortsIfNone<Element>;
    ensures result == spec_borrow(old(t));
    ensures spec_is_some(t);
    ensures spec_borrow(t) == e;
}
```


<a id="@Specification_1_swap_or_fill"></a>

### Function `swap_or_fill`


```move
module 0x1::option {
    public fun swap_or_fill<Element>(t: &mut option::Option<Element>, e: Element): option::Option<Element>
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == old(t);
    ensures spec_borrow(t) == e;
}
```


<a id="@Specification_1_destroy_with_default"></a>

### Function `destroy_with_default`


```move
module 0x1::option {
    public fun destroy_with_default<Element: drop>(t: option::Option<Element>, default: Element): Element
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == (if (spec_is_some(t)) spec_borrow(t) else default);
}
```


<a id="@Specification_1_destroy_some"></a>

### Function `destroy_some`


```move
module 0x1::option {
    public fun destroy_some<Element>(t: option::Option<Element>): Element
}
```



```move
module 0x1::option {
    pragma opaque;
    include AbortsIfNone<Element>;
    ensures result == spec_borrow(t);
}
```


<a id="@Specification_1_destroy_none"></a>

### Function `destroy_none`


```move
module 0x1::option {
    public fun destroy_none<Element>(t: option::Option<Element>)
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if spec_is_some(t) with EOPTION_IS_SET;
}
```


<a id="@Specification_1_to_vec"></a>

### Function `to_vec`


```move
module 0x1::option {
    public fun to_vec<Element>(t: option::Option<Element>): vector<Element>
}
```



```move
module 0x1::option {
    pragma opaque;
    aborts_if false;
    ensures result == t.vec;
}
```

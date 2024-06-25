
<a id="0x1_acl"></a>

# Module `0x1::acl`

Access control list (acl) module. An acl is a list of account addresses who
have the access permission to a certain object.
This module uses a `vector` to represent the list, but can be refactored to
use a &quot;set&quot; instead when it&apos;s available in the language in the future.


-  [Struct `ACL`](#0x1_acl_ACL)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_acl_empty)
-  [Function `add`](#0x1_acl_add)
-  [Function `remove`](#0x1_acl_remove)
-  [Function `contains`](#0x1_acl_contains)
-  [Function `assert_contains`](#0x1_acl_assert_contains)
-  [Specification](#@Specification_1)
    -  [Struct `ACL`](#@Specification_1_ACL)
    -  [Function `add`](#@Specification_1_add)
    -  [Function `remove`](#@Specification_1_remove)
    -  [Function `contains`](#@Specification_1_contains)
    -  [Function `assert_contains`](#@Specification_1_assert_contains)


```move
module 0x1::acl {
    use 0x1::error;
    use 0x1::vector;
}
```


<a id="0x1_acl_ACL"></a>

## Struct `ACL`



```move
module 0x1::acl {
    struct ACL has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`list: vector<address>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_acl_ECONTAIN"></a>

The ACL already contains the address.


```move
module 0x1::acl {
    const ECONTAIN: u64 = 0;
}
```


<a id="0x1_acl_ENOT_CONTAIN"></a>

The ACL does not contain the address.


```move
module 0x1::acl {
    const ENOT_CONTAIN: u64 = 1;
}
```


<a id="0x1_acl_empty"></a>

## Function `empty`

Return an empty ACL.


```move
module 0x1::acl {
    public fun empty(): acl::ACL
}
```


##### Implementation


```move
module 0x1::acl {
    public fun empty(): ACL {
        ACL{ list: vector::empty<address>() }
    }
}
```


<a id="0x1_acl_add"></a>

## Function `add`

Add the address to the ACL.


```move
module 0x1::acl {
    public fun add(acl: &mut acl::ACL, addr: address)
}
```


##### Implementation


```move
module 0x1::acl {
    public fun add(acl: &mut ACL, addr: address) {
        assert!(!vector::contains(&mut acl.list, &addr), error::invalid_argument(ECONTAIN));
        vector::push_back(&mut acl.list, addr);
    }
}
```


<a id="0x1_acl_remove"></a>

## Function `remove`

Remove the address from the ACL.


```move
module 0x1::acl {
    public fun remove(acl: &mut acl::ACL, addr: address)
}
```


##### Implementation


```move
module 0x1::acl {
    public fun remove(acl: &mut ACL, addr: address) {
        let (found, index) = vector::index_of(&mut acl.list, &addr);
        assert!(found, error::invalid_argument(ENOT_CONTAIN));
        vector::remove(&mut acl.list, index);
    }
}
```


<a id="0x1_acl_contains"></a>

## Function `contains`

Return true iff the ACL contains the address.


```move
module 0x1::acl {
    public fun contains(acl: &acl::ACL, addr: address): bool
}
```


##### Implementation


```move
module 0x1::acl {
    public fun contains(acl: &ACL, addr: address): bool {
        vector::contains(&acl.list, &addr)
    }
}
```


<a id="0x1_acl_assert_contains"></a>

## Function `assert_contains`

assert! that the ACL has the address.


```move
module 0x1::acl {
    public fun assert_contains(acl: &acl::ACL, addr: address)
}
```


##### Implementation


```move
module 0x1::acl {
    public fun assert_contains(acl: &ACL, addr: address) {
        assert!(contains(acl, addr), error::invalid_argument(ENOT_CONTAIN));
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_ACL"></a>

### Struct `ACL`


```move
module 0x1::acl {
    struct ACL has copy, drop, store
}
```


<dl>
<dt>
`list: vector<address>`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::acl {
    invariant forall i in 0..len(list), j in 0..len(list): list[i] == list[j] ==> i == j;
}
```



<a id="0x1_acl_spec_contains"></a>


```move
module 0x1::acl {
    fun spec_contains(acl: ACL, addr: address): bool {
       exists a in acl.list: a == addr
    }
}
```


<a id="@Specification_1_add"></a>

### Function `add`


```move
module 0x1::acl {
    public fun add(acl: &mut acl::ACL, addr: address)
}
```



```move
module 0x1::acl {
    aborts_if spec_contains(acl, addr) with error::INVALID_ARGUMENT;
    ensures spec_contains(acl, addr);
}
```


<a id="@Specification_1_remove"></a>

### Function `remove`


```move
module 0x1::acl {
    public fun remove(acl: &mut acl::ACL, addr: address)
}
```



```move
module 0x1::acl {
    aborts_if !spec_contains(acl, addr) with error::INVALID_ARGUMENT;
    ensures !spec_contains(acl, addr);
}
```


<a id="@Specification_1_contains"></a>

### Function `contains`


```move
module 0x1::acl {
    public fun contains(acl: &acl::ACL, addr: address): bool
}
```



```move
module 0x1::acl {
    ensures result == spec_contains(acl, addr);
}
```


<a id="@Specification_1_assert_contains"></a>

### Function `assert_contains`


```move
module 0x1::acl {
    public fun assert_contains(acl: &acl::ACL, addr: address)
}
```



```move
module 0x1::acl {
    aborts_if !spec_contains(acl, addr) with error::INVALID_ARGUMENT;
}
```

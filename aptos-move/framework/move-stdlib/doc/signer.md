
<a id="0x1_signer"></a>

# Module `0x1::signer`



-  [Function `borrow_address`](#0x1_signer_borrow_address)
-  [Function `address_of`](#0x1_signer_address_of)
-  [Specification](#@Specification_0)


```move
module 0x1::signer {
}
```


<a id="0x1_signer_borrow_address"></a>

## Function `borrow_address`

Borrows the address of the signer
Conceptually, you can think of the `signer` as being a struct wrapper around an
address
```
struct signer has drop &#123; addr: address &#125;
```
`borrow_address` borrows this inner field


```move
module 0x1::signer {
    public fun borrow_address(s: &signer): &address
}
```


##### Implementation


```move
module 0x1::signer {
    native public fun borrow_address(s: &signer): &address;
}
```


<a id="0x1_signer_address_of"></a>

## Function `address_of`



```move
module 0x1::signer {
    public fun address_of(s: &signer): address
}
```


##### Implementation


```move
module 0x1::signer {
    public fun address_of(s: &signer): address {
        *borrow_address(s)
    }
}
```


<a id="@Specification_0"></a>

## Specification

Return true only if `s` is a transaction signer. This is a spec function only available in spec.


<a id="0x1_signer_is_txn_signer"></a>


```move
module 0x1::signer {
    native fun is_txn_signer(s: signer): bool;
}
```

Return true only if `a` is a transaction signer address. This is a spec function only available in spec.


<a id="0x1_signer_is_txn_signer_addr"></a>


```move
module 0x1::signer {
    native fun is_txn_signer_addr(a: address): bool;
}
```

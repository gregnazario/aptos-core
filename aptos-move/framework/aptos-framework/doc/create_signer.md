
<a id="0x1_create_signer"></a>

# Module `0x1::create_signer`

Provides a common place for exporting `create_signer` across the Aptos Framework.

To use create_signer, add the module below, such that:
`friend aptos_framework::friend_wants_create_signer`
where `friend_wants_create_signer` is the module that needs `create_signer`.

Note, that this is only available within the Aptos Framework.

This exists to make auditing straight forward and to limit the need to depend
on account to have access to this.


-  [Function `create_signer`](#0x1_create_signer_create_signer)
-  [Specification](#@Specification_0)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `create_signer`](#@Specification_0_create_signer)


```move
module 0x1::create_signer {
}
```


<a id="0x1_create_signer_create_signer"></a>

## Function `create_signer`



```move
module 0x1::create_signer {
    public(friend) fun create_signer(addr: address): signer
}
```


##### Implementation


```move
module 0x1::create_signer {
    public(friend) native fun create_signer(addr: address): signer;
}
```


<a id="@Specification_0"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>Obtaining a signer for an arbitrary account should only be available within the Aptos Framework.</td>
<td>Critical</td>
<td>The create_signer::create_signer function only allows friend modules to retrieve the signer for an arbitrarily address.</td>
<td>Enforced through function visibility.</td>
</tr>

<tr>
<td>2</td>
<td>The account owner should have the ability to create a signer for their account.</td>
<td>Medium</td>
<td>Before an Account resource is created, a signer is created for the specified new_address, and later, the Account resource is assigned to this signer.</td>
<td>Enforced by the [https://github.com/aptos&#45;labs/aptos&#45;core/blob/main/third_party/move/move&#45;vm/types/src/values/values_impl.rs#L1129](move vm).</td>
</tr>

<tr>
<td>3</td>
<td>An account should only be able to create a signer for another account if that account has granted it signing capabilities.</td>
<td>Critical</td>
<td>The Account resource holds a signer_capability_offer field which allows the owner to share the signer capability with other accounts.</td>
<td>Formally verified via [account.md#high&#45;level&#45;spec&#45;3](AccountContainsAddr).</td>
</tr>

<tr>
<td>4</td>
<td>A signer should be returned for addresses that are not registered as accounts.</td>
<td>Low</td>
<td>The signer is just a struct that wraps an address, allows for non&#45;accounts to have a signer.</td>
<td>Formally verified via [#0x1_create_signer_create_signer](create_signer).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::create_signer {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_0_create_signer"></a>

### Function `create_signer`


```move
module 0x1::create_signer {
    public(friend) fun create_signer(addr: address): signer
}
```

Convert address to singer and return.


```move
module 0x1::create_signer {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] signer::address_of(result) == addr;
}
```


<a id="0x1_system_addresses"></a>

# Module `0x1::system_addresses`



-  [Constants](#@Constants_0)
-  [Function `assert_core_resource`](#0x1_system_addresses_assert_core_resource)
-  [Function `assert_core_resource_address`](#0x1_system_addresses_assert_core_resource_address)
-  [Function `is_core_resource_address`](#0x1_system_addresses_is_core_resource_address)
-  [Function `assert_aptos_framework`](#0x1_system_addresses_assert_aptos_framework)
-  [Function `assert_framework_reserved_address`](#0x1_system_addresses_assert_framework_reserved_address)
-  [Function `assert_framework_reserved`](#0x1_system_addresses_assert_framework_reserved)
-  [Function `is_framework_reserved_address`](#0x1_system_addresses_is_framework_reserved_address)
-  [Function `is_aptos_framework_address`](#0x1_system_addresses_is_aptos_framework_address)
-  [Function `assert_vm`](#0x1_system_addresses_assert_vm)
-  [Function `is_vm`](#0x1_system_addresses_is_vm)
-  [Function `is_vm_address`](#0x1_system_addresses_is_vm_address)
-  [Function `is_reserved_address`](#0x1_system_addresses_is_reserved_address)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `assert_core_resource`](#@Specification_1_assert_core_resource)
    -  [Function `assert_core_resource_address`](#@Specification_1_assert_core_resource_address)
    -  [Function `is_core_resource_address`](#@Specification_1_is_core_resource_address)
    -  [Function `assert_aptos_framework`](#@Specification_1_assert_aptos_framework)
    -  [Function `assert_framework_reserved_address`](#@Specification_1_assert_framework_reserved_address)
    -  [Function `assert_framework_reserved`](#@Specification_1_assert_framework_reserved)
    -  [Function `assert_vm`](#@Specification_1_assert_vm)


```move
module 0x1::system_addresses {
    use 0x1::error;
    use 0x1::signer;
}
```


<a id="@Constants_0"></a>

## Constants


<a id="0x1_system_addresses_ENOT_APTOS_FRAMEWORK_ADDRESS"></a>

The address/account did not correspond to the core framework address


```move
module 0x1::system_addresses {
    const ENOT_APTOS_FRAMEWORK_ADDRESS: u64 = 3;
}
```


<a id="0x1_system_addresses_ENOT_CORE_RESOURCE_ADDRESS"></a>

The address/account did not correspond to the core resource address


```move
module 0x1::system_addresses {
    const ENOT_CORE_RESOURCE_ADDRESS: u64 = 1;
}
```


<a id="0x1_system_addresses_ENOT_FRAMEWORK_RESERVED_ADDRESS"></a>

The address is not framework reserved address


```move
module 0x1::system_addresses {
    const ENOT_FRAMEWORK_RESERVED_ADDRESS: u64 = 4;
}
```


<a id="0x1_system_addresses_EVM"></a>

The operation can only be performed by the VM


```move
module 0x1::system_addresses {
    const EVM: u64 = 2;
}
```


<a id="0x1_system_addresses_assert_core_resource"></a>

## Function `assert_core_resource`



```move
module 0x1::system_addresses {
    public fun assert_core_resource(account: &signer)
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun assert_core_resource(account: &signer) {
        assert_core_resource_address(signer::address_of(account))
    }
}
```


<a id="0x1_system_addresses_assert_core_resource_address"></a>

## Function `assert_core_resource_address`



```move
module 0x1::system_addresses {
    public fun assert_core_resource_address(addr: address)
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun assert_core_resource_address(addr: address) {
        assert!(is_core_resource_address(addr), error::permission_denied(ENOT_CORE_RESOURCE_ADDRESS))
    }
}
```


<a id="0x1_system_addresses_is_core_resource_address"></a>

## Function `is_core_resource_address`



```move
module 0x1::system_addresses {
    public fun is_core_resource_address(addr: address): bool
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun is_core_resource_address(addr: address): bool {
        addr == @core_resources
    }
}
```


<a id="0x1_system_addresses_assert_aptos_framework"></a>

## Function `assert_aptos_framework`



```move
module 0x1::system_addresses {
    public fun assert_aptos_framework(account: &signer)
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun assert_aptos_framework(account: &signer) {
        assert!(
            is_aptos_framework_address(signer::address_of(account)),
            error::permission_denied(ENOT_APTOS_FRAMEWORK_ADDRESS),
        )
    }
}
```


<a id="0x1_system_addresses_assert_framework_reserved_address"></a>

## Function `assert_framework_reserved_address`



```move
module 0x1::system_addresses {
    public fun assert_framework_reserved_address(account: &signer)
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun assert_framework_reserved_address(account: &signer) {
        assert_framework_reserved(signer::address_of(account));
    }
}
```


<a id="0x1_system_addresses_assert_framework_reserved"></a>

## Function `assert_framework_reserved`



```move
module 0x1::system_addresses {
    public fun assert_framework_reserved(addr: address)
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun assert_framework_reserved(addr: address) {
        assert!(
            is_framework_reserved_address(addr),
            error::permission_denied(ENOT_FRAMEWORK_RESERVED_ADDRESS),
        )
    }
}
```


<a id="0x1_system_addresses_is_framework_reserved_address"></a>

## Function `is_framework_reserved_address`

Return true if `addr` is 0x0 or under the on chain governance&apos;s control.


```move
module 0x1::system_addresses {
    public fun is_framework_reserved_address(addr: address): bool
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun is_framework_reserved_address(addr: address): bool {
        is_aptos_framework_address(addr) ||
            addr == @0x2 ||
            addr == @0x3 ||
            addr == @0x4 ||
            addr == @0x5 ||
            addr == @0x6 ||
            addr == @0x7 ||
            addr == @0x8 ||
            addr == @0x9 ||
            addr == @0xa
    }
}
```


<a id="0x1_system_addresses_is_aptos_framework_address"></a>

## Function `is_aptos_framework_address`

Return true if `addr` is 0x1.


```move
module 0x1::system_addresses {
    public fun is_aptos_framework_address(addr: address): bool
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun is_aptos_framework_address(addr: address): bool {
        addr == @aptos_framework
    }
}
```


<a id="0x1_system_addresses_assert_vm"></a>

## Function `assert_vm`

Assert that the signer has the VM reserved address.


```move
module 0x1::system_addresses {
    public fun assert_vm(account: &signer)
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun assert_vm(account: &signer) {
        assert!(is_vm(account), error::permission_denied(EVM))
    }
}
```


<a id="0x1_system_addresses_is_vm"></a>

## Function `is_vm`

Return true if `addr` is a reserved address for the VM to call system modules.


```move
module 0x1::system_addresses {
    public fun is_vm(account: &signer): bool
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun is_vm(account: &signer): bool {
        is_vm_address(signer::address_of(account))
    }
}
```


<a id="0x1_system_addresses_is_vm_address"></a>

## Function `is_vm_address`

Return true if `addr` is a reserved address for the VM to call system modules.


```move
module 0x1::system_addresses {
    public fun is_vm_address(addr: address): bool
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun is_vm_address(addr: address): bool {
        addr == @vm_reserved
    }
}
```


<a id="0x1_system_addresses_is_reserved_address"></a>

## Function `is_reserved_address`

Return true if `addr` is either the VM address or an Aptos Framework address.


```move
module 0x1::system_addresses {
    public fun is_reserved_address(addr: address): bool
}
```


##### Implementation


```move
module 0x1::system_addresses {
    public fun is_reserved_address(addr: address): bool {
        is_aptos_framework_address(addr) || is_vm_address(addr)
    }
}
```


<a id="@Specification_1"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>Asserting that a provided address corresponds to the Core Resources address should always yield a true result when matched.</td>
<td>Low</td>
<td>The assert_core_resource and assert_core_resource_address functions ensure that the provided signer or address belong to the @core_resources account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](AbortsIfNotCoreResource).</td>
</tr>

<tr>
<td>2</td>
<td>Asserting that a provided address corresponds to the Aptos Framework Resources address should always yield a true result when matched.</td>
<td>High</td>
<td>The assert_aptos_framework function ensures that the provided signer belongs to the @aptos_framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](AbortsIfNotAptosFramework).</td>
</tr>

<tr>
<td>3</td>
<td>Asserting that a provided address corresponds to the VM address should always yield a true result when matched.</td>
<td>High</td>
<td>The assert_vm function ensure that the provided signer belongs to the @vm_reserved account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](AbortsIfNotVM).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::system_addresses {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_assert_core_resource"></a>

### Function `assert_core_resource`


```move
module 0x1::system_addresses {
    public fun assert_core_resource(account: &signer)
}
```



```move
module 0x1::system_addresses {
    pragma opaque;
    include AbortsIfNotCoreResource { addr: signer::address_of(account) };
}
```


<a id="@Specification_1_assert_core_resource_address"></a>

### Function `assert_core_resource_address`


```move
module 0x1::system_addresses {
    public fun assert_core_resource_address(addr: address)
}
```



```move
module 0x1::system_addresses {
    pragma opaque;
    include AbortsIfNotCoreResource;
}
```


<a id="@Specification_1_is_core_resource_address"></a>

### Function `is_core_resource_address`


```move
module 0x1::system_addresses {
    public fun is_core_resource_address(addr: address): bool
}
```



```move
module 0x1::system_addresses {
    pragma opaque;
    aborts_if false;
    ensures result == (addr == @core_resources);
}
```


<a id="@Specification_1_assert_aptos_framework"></a>

### Function `assert_aptos_framework`


```move
module 0x1::system_addresses {
    public fun assert_aptos_framework(account: &signer)
}
```



```move
module 0x1::system_addresses {
    pragma opaque;
    include AbortsIfNotAptosFramework;
}
```


<a id="@Specification_1_assert_framework_reserved_address"></a>

### Function `assert_framework_reserved_address`


```move
module 0x1::system_addresses {
    public fun assert_framework_reserved_address(account: &signer)
}
```



```move
module 0x1::system_addresses {
    aborts_if !is_framework_reserved_address(signer::address_of(account));
}
```


<a id="@Specification_1_assert_framework_reserved"></a>

### Function `assert_framework_reserved`


```move
module 0x1::system_addresses {
    public fun assert_framework_reserved(addr: address)
}
```



```move
module 0x1::system_addresses {
    aborts_if !is_framework_reserved_address(addr);
}
```

Specifies that a function aborts if the account does not have the aptos framework address.


<a id="0x1_system_addresses_AbortsIfNotAptosFramework"></a>


```move
module 0x1::system_addresses {
    schema AbortsIfNotAptosFramework {
        account: signer;
    // This enforces ### high&#45;level&#45;req&#45;2
    [#high&#45;level&#45;req](high&#45;level requirement 2):
        aborts_if signer::address_of(account) != @aptos_framework with error::PERMISSION_DENIED;
    }
}
```


<a id="@Specification_1_assert_vm"></a>

### Function `assert_vm`


```move
module 0x1::system_addresses {
    public fun assert_vm(account: &signer)
}
```



```move
module 0x1::system_addresses {
    pragma opaque;
    include AbortsIfNotVM;
}
```

Specifies that a function aborts if the account does not have the VM reserved address.


<a id="0x1_system_addresses_AbortsIfNotVM"></a>


```move
module 0x1::system_addresses {
    schema AbortsIfNotVM {
        account: signer;
    // This enforces ### high&#45;level&#45;req&#45;3
    [#high&#45;level&#45;req](high&#45;level requirement 3):
        aborts_if signer::address_of(account) != @vm_reserved with error::PERMISSION_DENIED;
    }
}
```

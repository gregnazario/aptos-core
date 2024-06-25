
<a id="0x1_aggregator_factory"></a>

# Module `0x1::aggregator_factory`

This module provides foundations to create aggregators. Currently only
Aptos Framework (0x1) can create them, so this module helps to wrap
the constructor of `Aggregator` struct so that only a system account
can initialize one. In the future, this might change and aggregators
can be enabled for the public.


-  [Resource `AggregatorFactory`](#0x1_aggregator_factory_AggregatorFactory)
-  [Constants](#@Constants_0)
-  [Function `initialize_aggregator_factory`](#0x1_aggregator_factory_initialize_aggregator_factory)
-  [Function `create_aggregator_internal`](#0x1_aggregator_factory_create_aggregator_internal)
-  [Function `create_aggregator`](#0x1_aggregator_factory_create_aggregator)
-  [Function `new_aggregator`](#0x1_aggregator_factory_new_aggregator)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize_aggregator_factory`](#@Specification_1_initialize_aggregator_factory)
    -  [Function `create_aggregator_internal`](#@Specification_1_create_aggregator_internal)
    -  [Function `create_aggregator`](#@Specification_1_create_aggregator)
    -  [Function `new_aggregator`](#@Specification_1_new_aggregator)


```move
module 0x1::aggregator_factory {
    use 0x1::aggregator;
    use 0x1::error;
    use 0x1::system_addresses;
    use 0x1::table;
}
```


<a id="0x1_aggregator_factory_AggregatorFactory"></a>

## Resource `AggregatorFactory`

Creates new aggregators. Used to control the numbers of aggregators in the
system and who can create them. At the moment, only Aptos Framework (0x1)
account can.


```move
module 0x1::aggregator_factory {
    struct AggregatorFactory has key
}
```


##### Fields


<dl>
<dt>
`phantom_table: table::Table<address, u128>`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_aggregator_factory_EAGGREGATOR_FACTORY_NOT_FOUND"></a>

Aggregator factory is not published yet.


```move
module 0x1::aggregator_factory {
    const EAGGREGATOR_FACTORY_NOT_FOUND: u64 = 1;
}
```


<a id="0x1_aggregator_factory_initialize_aggregator_factory"></a>

## Function `initialize_aggregator_factory`

Creates a new factory for aggregators. Can only be called during genesis.


```move
module 0x1::aggregator_factory {
    public(friend) fun initialize_aggregator_factory(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::aggregator_factory {
    public(friend) fun initialize_aggregator_factory(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        let aggregator_factory = AggregatorFactory {
            phantom_table: table::new()
        };
        move_to(aptos_framework, aggregator_factory);
    }
}
```


<a id="0x1_aggregator_factory_create_aggregator_internal"></a>

## Function `create_aggregator_internal`

Creates a new aggregator instance which overflows on exceeding a `limit`.


```move
module 0x1::aggregator_factory {
    public(friend) fun create_aggregator_internal(limit: u128): aggregator::Aggregator
}
```


##### Implementation


```move
module 0x1::aggregator_factory {
    public(friend) fun create_aggregator_internal(limit: u128): Aggregator acquires AggregatorFactory {
        assert!(
            exists<AggregatorFactory>(@aptos_framework),
            error::not_found(EAGGREGATOR_FACTORY_NOT_FOUND)
        );

        let aggregator_factory = borrow_global_mut<AggregatorFactory>(@aptos_framework);
        new_aggregator(aggregator_factory, limit)
    }
}
```


<a id="0x1_aggregator_factory_create_aggregator"></a>

## Function `create_aggregator`

This is currently a function closed for public. This can be updated in the future by on&#45;chain governance
to allow any signer to call.


```move
module 0x1::aggregator_factory {
    public fun create_aggregator(account: &signer, limit: u128): aggregator::Aggregator
}
```


##### Implementation


```move
module 0x1::aggregator_factory {
    public fun create_aggregator(account: &signer, limit: u128): Aggregator acquires AggregatorFactory {
        // Only Aptos Framework (0x1) account can call this for now.
        system_addresses::assert_aptos_framework(account);
        create_aggregator_internal(limit)
    }
}
```


<a id="0x1_aggregator_factory_new_aggregator"></a>

## Function `new_aggregator`

Returns a new aggregator.


```move
module 0x1::aggregator_factory {
    fun new_aggregator(aggregator_factory: &mut aggregator_factory::AggregatorFactory, limit: u128): aggregator::Aggregator
}
```


##### Implementation


```move
module 0x1::aggregator_factory {
    native fun new_aggregator(aggregator_factory: &mut AggregatorFactory, limit: u128): Aggregator;
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
<td>During the module&apos;s initialization, it guarantees that the Aptos framework is the caller and that the AggregatorFactory resource will move under the Aptos framework account.</td>
<td>High</td>
<td>The initialize function is responsible for establishing the initial state of the module by creating the AggregatorFactory resource, indicating its presence within the module&apos;s context. Subsequently, the resource transfers to the Aptos framework account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize_aggregator_factory).</td>
</tr>

<tr>
<td>2</td>
<td>To create a new aggregator instance, the aggregator factory must already be initialized and exist under the Aptos account.</td>
<td>High</td>
<td>The create_aggregator_internal function asserts that AggregatorFactory exists for the Aptos account.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](CreateAggregatorInternalAbortsIf).</td>
</tr>

<tr>
<td>3</td>
<td>Only the Aptos framework address may create an aggregator instance currently.</td>
<td>Low</td>
<td>The create_aggregator function ensures that the address calling it is the Aptos framework address.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](create_aggregator).</td>
</tr>

<tr>
<td>4</td>
<td>The creation of new aggregators should be done correctly.</td>
<td>High</td>
<td>The native new_aggregator function correctly creates a new aggregator.</td>
<td>The new_aggregator native function has been manually audited.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::aggregator_factory {
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize_aggregator_factory"></a>

### Function `initialize_aggregator_factory`


```move
module 0x1::aggregator_factory {
    public(friend) fun initialize_aggregator_factory(aptos_framework: &signer)
}
```

Make sure the caller is @aptos_framework.
AggregatorFactory is not under the caller before creating the resource.


```move
module 0x1::aggregator_factory {
    let addr = signer::address_of(aptos_framework);
    aborts_if addr != @aptos_framework;
    aborts_if exists<AggregatorFactory>(addr);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures exists<AggregatorFactory>(addr);
}
```


<a id="@Specification_1_create_aggregator_internal"></a>

### Function `create_aggregator_internal`


```move
module 0x1::aggregator_factory {
    public(friend) fun create_aggregator_internal(limit: u128): aggregator::Aggregator
}
```



```move
module 0x1::aggregator_factory {
// This enforces ### high&#45;level&#45;req&#45;2
[#high&#45;level&#45;req](high&#45;level requirement 2):
    include CreateAggregatorInternalAbortsIf;
    ensures aggregator::spec_get_limit(result) == limit;
    ensures aggregator::spec_aggregator_get_val(result) == 0;
}
```



<a id="0x1_aggregator_factory_CreateAggregatorInternalAbortsIf"></a>


```move
module 0x1::aggregator_factory {
    schema CreateAggregatorInternalAbortsIf {
        aborts_if !exists<AggregatorFactory>(@aptos_framework);
    }
}
```


<a id="@Specification_1_create_aggregator"></a>

### Function `create_aggregator`


```move
module 0x1::aggregator_factory {
    public fun create_aggregator(account: &signer, limit: u128): aggregator::Aggregator
}
```

Make sure the caller is @aptos_framework.
AggregatorFactory existed under the @aptos_framework when Creating a new aggregator.


```move
module 0x1::aggregator_factory {
    let addr = signer::address_of(account);
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    aborts_if addr != @aptos_framework;
    aborts_if !exists<AggregatorFactory>(@aptos_framework);
}
```



<a id="0x1_aggregator_factory_spec_new_aggregator"></a>


```move
module 0x1::aggregator_factory {
    native fun spec_new_aggregator(limit: u128): Aggregator;
}
```


<a id="@Specification_1_new_aggregator"></a>

### Function `new_aggregator`


```move
module 0x1::aggregator_factory {
    fun new_aggregator(aggregator_factory: &mut aggregator_factory::AggregatorFactory, limit: u128): aggregator::Aggregator
}
```



```move
module 0x1::aggregator_factory {
    pragma opaque;
    aborts_if false;
    ensures result == spec_new_aggregator(limit);
    ensures aggregator::spec_get_limit(result) == limit;
}
```

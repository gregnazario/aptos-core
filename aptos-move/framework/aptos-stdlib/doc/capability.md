
<a id="0x1_capability"></a>

# Module `0x1::capability`

A module which defines the basic concept of
[&#42;capabilities&#42;](https://en.wikipedia.org/wiki/Capability&#45;based_security) for managing access control.

EXPERIMENTAL


<a id="@Overview_0"></a>

## Overview


A capability is a unforgeable token which testifies that a signer has authorized a certain operation.
The token is valid during the transaction where it is obtained. Since the type `capability::Cap` has
no ability to be stored in global memory, capabilities cannot leak out of a transaction. For every function
called within a transaction which has a capability as a parameter, it is guaranteed that the capability
has been obtained via a proper signer&#45;based authorization step previously in the transaction&apos;s execution.


<a id="@Usage_1"></a>

### Usage


Initializing and acquiring capabilities is usually encapsulated in a module with a type
tag which can only be constructed by this module.

```
module Pkg::Feature &#123;
use std::capability::Cap;

/// A type tag used in Cap&lt;Feature&gt;. Only this module can create an instance,
/// and there is no public function other than Self::acquire which returns a value of this type.
/// This way, this module has full control how Cap&lt;Feature&gt; is given out.
struct Feature has drop &#123;&#125;

/// Initializes this module.
public fun initialize(s: &amp;signer) &#123;
// Create capability. This happens once at module initialization time.
// One needs to provide a witness for being the owner of Feature
// in the 2nd parameter.
&lt;&lt;additional conditions allowing to initialize this capability&gt;&gt;
capability::create&lt;Feature&gt;(s, &amp;Feature&#123;&#125;);
&#125;

/// Acquires the capability to work with this feature.
public fun acquire(s: &amp;signer): Cap&lt;Feature&gt; &#123;
&lt;&lt;additional conditions allowing to acquire this capability&gt;&gt;
capability::acquire&lt;Feature&gt;(s, &amp;Feature&#123;&#125;);
&#125;

/// Does something related to the feature. The caller must pass a Cap&lt;Feature&gt;.
public fun do_something(_cap: Cap&lt;Feature&gt;) &#123; ... &#125;
&#125;
```


<a id="@Delegation_2"></a>

### Delegation


Capabilities come with the optional feature of &#42;delegation&#42;. Via `Self::delegate`, an owner of a capability
can designate another signer to be also capable of acquiring the capability. Like the original creator,
the delegate needs to present his signer to obtain the capability in his transactions. Delegation can
be revoked via `Self::revoke`, removing this access right from the delegate.

While the basic authorization mechanism for delegates is the same as with core capabilities, the
target of delegation might be subject of restrictions which need to be specified and verified. This can
be done via global invariants in the specification language. For example, in order to prevent delegation
all together for a capability, one can use the following invariant:

```
invariant forall a: address where capability::spec_has_cap&lt;Feature&gt;(a):
len(capability::spec_delegates&lt;Feature&gt;(a)) &#61;&#61; 0;
```

Similarly, the following invariant would enforce that delegates, if existent, must satisfy a certain
predicate:

```
invariant forall a: address where capability::spec_has_cap&lt;Feature&gt;(a):
forall d in capability::spec_delegates&lt;Feature&gt;(a):
is_valid_delegate_for_feature(d);
```


-  [Overview](#@Overview_0)
    -  [Usage](#@Usage_1)
    -  [Delegation](#@Delegation_2)
-  [Struct `Cap`](#0x1_capability_Cap)
-  [Struct `LinearCap`](#0x1_capability_LinearCap)
-  [Resource `CapState`](#0x1_capability_CapState)
-  [Resource `CapDelegateState`](#0x1_capability_CapDelegateState)
-  [Constants](#@Constants_3)
-  [Function `create`](#0x1_capability_create)
-  [Function `acquire`](#0x1_capability_acquire)
-  [Function `acquire_linear`](#0x1_capability_acquire_linear)
-  [Function `validate_acquire`](#0x1_capability_validate_acquire)
-  [Function `root_addr`](#0x1_capability_root_addr)
-  [Function `linear_root_addr`](#0x1_capability_linear_root_addr)
-  [Function `delegate`](#0x1_capability_delegate)
-  [Function `revoke`](#0x1_capability_revoke)
-  [Function `remove_element`](#0x1_capability_remove_element)
-  [Function `add_element`](#0x1_capability_add_element)
-  [Specification](#@Specification_4)
    -  [Function `create`](#@Specification_4_create)
    -  [Function `acquire`](#@Specification_4_acquire)
    -  [Function `acquire_linear`](#@Specification_4_acquire_linear)
    -  [Function `delegate`](#@Specification_4_delegate)
    -  [Function `revoke`](#@Specification_4_revoke)
    -  [Function `remove_element`](#@Specification_4_remove_element)
    -  [Function `add_element`](#@Specification_4_add_element)


```move
module 0x1::capability {
    use 0x1::error;
    use 0x1::signer;
    use 0x1::vector;
}
```


<a id="0x1_capability_Cap"></a>

## Struct `Cap`

The token representing an acquired capability. Cannot be stored in memory, but copied and dropped freely.


```move
module 0x1::capability {
    struct Cap<Feature> has copy, drop
}
```


##### Fields


<dl>
<dt>
`root: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_capability_LinearCap"></a>

## Struct `LinearCap`

A linear version of a capability token. This can be used if an acquired capability should be enforced
to be used only once for an authorization.


```move
module 0x1::capability {
    struct LinearCap<Feature> has drop
}
```


##### Fields


<dl>
<dt>
`root: address`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_capability_CapState"></a>

## Resource `CapState`

An internal data structure for representing a configured capability.


```move
module 0x1::capability {
    struct CapState<Feature> has key
}
```


##### Fields


<dl>
<dt>
`delegates: vector<address>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_capability_CapDelegateState"></a>

## Resource `CapDelegateState`

An internal data structure for representing a configured delegated capability.


```move
module 0x1::capability {
    struct CapDelegateState<Feature> has key
}
```


##### Fields


<dl>
<dt>
`root: address`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_3"></a>

## Constants


<a id="0x1_capability_ECAPABILITY_ALREADY_EXISTS"></a>

Capability resource already exists on the specified account


```move
module 0x1::capability {
    const ECAPABILITY_ALREADY_EXISTS: u64 = 1;
}
```


<a id="0x1_capability_ECAPABILITY_NOT_FOUND"></a>

Capability resource not found


```move
module 0x1::capability {
    const ECAPABILITY_NOT_FOUND: u64 = 2;
}
```


<a id="0x1_capability_EDELEGATE"></a>

Account does not have delegated permissions


```move
module 0x1::capability {
    const EDELEGATE: u64 = 3;
}
```


<a id="0x1_capability_create"></a>

## Function `create`

Creates a new capability class, owned by the passed signer. A caller must pass a witness that
they own the `Feature` type parameter.


```move
module 0x1::capability {
    public fun create<Feature>(owner: &signer, _feature_witness: &Feature)
}
```


##### Implementation


```move
module 0x1::capability {
    public fun create<Feature>(owner: &signer, _feature_witness: &Feature) {
        let addr = signer::address_of(owner);
        assert!(!exists<CapState<Feature>>(addr), error::already_exists(ECAPABILITY_ALREADY_EXISTS));
        move_to<CapState<Feature>>(owner, CapState { delegates: vector::empty() });
    }
}
```


<a id="0x1_capability_acquire"></a>

## Function `acquire`

Acquires a capability token. Only the owner of the capability class, or an authorized delegate,
can succeed with this operation. A caller must pass a witness that they own the `Feature` type
parameter.


```move
module 0x1::capability {
    public fun acquire<Feature>(requester: &signer, _feature_witness: &Feature): capability::Cap<Feature>
}
```


##### Implementation


```move
module 0x1::capability {
    public fun acquire<Feature>(requester: &signer, _feature_witness: &Feature): Cap<Feature>
    acquires CapState, CapDelegateState {
        Cap<Feature> { root: validate_acquire<Feature>(requester) }
    }
}
```


<a id="0x1_capability_acquire_linear"></a>

## Function `acquire_linear`

Acquires a linear capability token. It is up to the module which owns `Feature` to decide
whether to expose a linear or non&#45;linear capability.


```move
module 0x1::capability {
    public fun acquire_linear<Feature>(requester: &signer, _feature_witness: &Feature): capability::LinearCap<Feature>
}
```


##### Implementation


```move
module 0x1::capability {
    public fun acquire_linear<Feature>(requester: &signer, _feature_witness: &Feature): LinearCap<Feature>
    acquires CapState, CapDelegateState {
        LinearCap<Feature> { root: validate_acquire<Feature>(requester) }
    }
}
```


<a id="0x1_capability_validate_acquire"></a>

## Function `validate_acquire`

Helper to validate an acquire. Returns the root address of the capability.


```move
module 0x1::capability {
    fun validate_acquire<Feature>(requester: &signer): address
}
```


##### Implementation


```move
module 0x1::capability {
    fun validate_acquire<Feature>(requester: &signer): address
    acquires CapState, CapDelegateState {
        let addr = signer::address_of(requester);
        if (exists<CapDelegateState<Feature>>(addr)) {
            let root_addr = borrow_global<CapDelegateState<Feature>>(addr).root;
            // double check that requester is actually registered as a delegate
            assert!(exists<CapState<Feature>>(root_addr), error::invalid_state(EDELEGATE));
            assert!(vector::contains(&borrow_global<CapState<Feature>>(root_addr).delegates, &addr),
                error::invalid_state(EDELEGATE));
            root_addr
        } else {
            assert!(exists<CapState<Feature>>(addr), error::not_found(ECAPABILITY_NOT_FOUND));
            addr
        }
    }
}
```


<a id="0x1_capability_root_addr"></a>

## Function `root_addr`

Returns the root address associated with the given capability token. Only the owner
of the feature can do this.


```move
module 0x1::capability {
    public fun root_addr<Feature>(cap: capability::Cap<Feature>, _feature_witness: &Feature): address
}
```


##### Implementation


```move
module 0x1::capability {
    public fun root_addr<Feature>(cap: Cap<Feature>, _feature_witness: &Feature): address {
        cap.root
    }
}
```


<a id="0x1_capability_linear_root_addr"></a>

## Function `linear_root_addr`

Returns the root address associated with the given linear capability token.


```move
module 0x1::capability {
    public fun linear_root_addr<Feature>(cap: capability::LinearCap<Feature>, _feature_witness: &Feature): address
}
```


##### Implementation


```move
module 0x1::capability {
    public fun linear_root_addr<Feature>(cap: LinearCap<Feature>, _feature_witness: &Feature): address {
        cap.root
    }
}
```


<a id="0x1_capability_delegate"></a>

## Function `delegate`

Registers a delegation relation. If the relation already exists, this function does
nothing.


```move
module 0x1::capability {
    public fun delegate<Feature>(cap: capability::Cap<Feature>, _feature_witness: &Feature, to: &signer)
}
```


##### Implementation


```move
module 0x1::capability {
    public fun delegate<Feature>(cap: Cap<Feature>, _feature_witness: &Feature, to: &signer)
    acquires CapState {
        let addr = signer::address_of(to);
        if (exists<CapDelegateState<Feature>>(addr)) return;
        move_to(to, CapDelegateState<Feature> { root: cap.root });
        add_element(&mut borrow_global_mut<CapState<Feature>>(cap.root).delegates, addr);
    }
}
```


<a id="0x1_capability_revoke"></a>

## Function `revoke`

Revokes a delegation relation. If no relation exists, this function does nothing.


```move
module 0x1::capability {
    public fun revoke<Feature>(cap: capability::Cap<Feature>, _feature_witness: &Feature, from: address)
}
```


##### Implementation


```move
module 0x1::capability {
    public fun revoke<Feature>(cap: Cap<Feature>, _feature_witness: &Feature, from: address)
    acquires CapState, CapDelegateState
    {
        if (!exists<CapDelegateState<Feature>>(from)) return;
        let CapDelegateState { root: _root } = move_from<CapDelegateState<Feature>>(from);
        remove_element(&mut borrow_global_mut<CapState<Feature>>(cap.root).delegates, &from);
    }
}
```


<a id="0x1_capability_remove_element"></a>

## Function `remove_element`

Helper to remove an element from a vector.


```move
module 0x1::capability {
    fun remove_element<E: drop>(v: &mut vector<E>, x: &E)
}
```


##### Implementation


```move
module 0x1::capability {
    fun remove_element<E: drop>(v: &mut vector<E>, x: &E) {
        let (found, index) = vector::index_of(v, x);
        if (found) {
            vector::remove(v, index);
        }
    }
}
```


<a id="0x1_capability_add_element"></a>

## Function `add_element`

Helper to add an element to a vector.


```move
module 0x1::capability {
    fun add_element<E: drop>(v: &mut vector<E>, x: E)
}
```


##### Implementation


```move
module 0x1::capability {
    fun add_element<E: drop>(v: &mut vector<E>, x: E) {
        if (!vector::contains(v, &x)) {
            vector::push_back(v, x)
        }
    }
}
```


<a id="@Specification_4"></a>

## Specification

Helper specification function to check whether a capability exists at address.


<a id="0x1_capability_spec_has_cap"></a>


```move
module 0x1::capability {
    fun spec_has_cap<Feature>(addr: address): bool {
       exists<CapState<Feature>>(addr)
    }
}
```

Helper specification function to obtain the delegates of a capability.


<a id="0x1_capability_spec_delegates"></a>


```move
module 0x1::capability {
    fun spec_delegates<Feature>(addr: address): vector<address> {
       global<CapState<Feature>>(addr).delegates
    }
}
```

Helper specification function to check whether a delegated capability exists at address.


<a id="0x1_capability_spec_has_delegate_cap"></a>


```move
module 0x1::capability {
    fun spec_has_delegate_cap<Feature>(addr: address): bool {
       exists<CapDelegateState<Feature>>(addr)
    }
}
```


<a id="@Specification_4_create"></a>

### Function `create`


```move
module 0x1::capability {
    public fun create<Feature>(owner: &signer, _feature_witness: &Feature)
}
```



```move
module 0x1::capability {
    let addr = signer::address_of(owner);
    aborts_if spec_has_cap<Feature>(addr);
    ensures spec_has_cap<Feature>(addr);
}
```


<a id="@Specification_4_acquire"></a>

### Function `acquire`


```move
module 0x1::capability {
    public fun acquire<Feature>(requester: &signer, _feature_witness: &Feature): capability::Cap<Feature>
}
```



```move
module 0x1::capability {
    let addr = signer::address_of(requester);
    let root_addr = global<CapDelegateState<Feature>>(addr).root;
    include AcquireSchema<Feature>;
    ensures spec_has_delegate_cap<Feature>(addr) ==> result.root == root_addr;
    ensures !spec_has_delegate_cap<Feature>(addr) ==> result.root == addr;
}
```


<a id="@Specification_4_acquire_linear"></a>

### Function `acquire_linear`


```move
module 0x1::capability {
    public fun acquire_linear<Feature>(requester: &signer, _feature_witness: &Feature): capability::LinearCap<Feature>
}
```



```move
module 0x1::capability {
    let addr = signer::address_of(requester);
    let root_addr = global<CapDelegateState<Feature>>(addr).root;
    include AcquireSchema<Feature>;
    ensures spec_has_delegate_cap<Feature>(addr) ==> result.root == root_addr;
    ensures !spec_has_delegate_cap<Feature>(addr) ==> result.root == addr;
}
```



<a id="0x1_capability_AcquireSchema"></a>


```move
module 0x1::capability {
    schema AcquireSchema<Feature> {
        addr: address;
        root_addr: address;
        aborts_if spec_has_delegate_cap<Feature>(addr) && !spec_has_cap<Feature>(root_addr);
        aborts_if spec_has_delegate_cap<Feature>(addr) && !vector::spec_contains(spec_delegates<Feature>(root_addr), addr);
        aborts_if !spec_has_delegate_cap<Feature>(addr) && !spec_has_cap<Feature>(addr);
    }
}
```


<a id="@Specification_4_delegate"></a>

### Function `delegate`


```move
module 0x1::capability {
    public fun delegate<Feature>(cap: capability::Cap<Feature>, _feature_witness: &Feature, to: &signer)
}
```



```move
module 0x1::capability {
    let addr = signer::address_of(to);
    ensures spec_has_delegate_cap<Feature>(addr);
    ensures !old(spec_has_delegate_cap<Feature>(addr)) ==> global<CapDelegateState<Feature>>(addr).root == cap.root;
    ensures !old(spec_has_delegate_cap<Feature>(addr)) ==> vector::spec_contains(spec_delegates<Feature>(cap.root), addr);
}
```


<a id="@Specification_4_revoke"></a>

### Function `revoke`


```move
module 0x1::capability {
    public fun revoke<Feature>(cap: capability::Cap<Feature>, _feature_witness: &Feature, from: address)
}
```



```move
module 0x1::capability {
    ensures !spec_has_delegate_cap<Feature>(from);
}
```


<a id="@Specification_4_remove_element"></a>

### Function `remove_element`


```move
module 0x1::capability {
    fun remove_element<E: drop>(v: &mut vector<E>, x: &E)
}
```



<a id="@Specification_4_add_element"></a>

### Function `add_element`


```move
module 0x1::capability {
    fun add_element<E: drop>(v: &mut vector<E>, x: E)
}
```



```move
module 0x1::capability {
    ensures vector::spec_contains(v, x);
}
```

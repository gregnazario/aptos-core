
<a id="0x1_jwks"></a>

# Module `0x1::jwks`

JWK functions and structs.

Note: An important design constraint for this module is that the JWK consensus Rust code is unable to
spawn a VM and make a Move function call. Instead, the JWK consensus Rust code will have to directly
write some of the resources in this file. As a result, the structs in this file are declared so as to
have a simple layout which is easily accessible in Rust.


-  [Struct `OIDCProvider`](#0x1_jwks_OIDCProvider)
-  [Resource `SupportedOIDCProviders`](#0x1_jwks_SupportedOIDCProviders)
-  [Struct `UnsupportedJWK`](#0x1_jwks_UnsupportedJWK)
-  [Struct `RSA_JWK`](#0x1_jwks_RSA_JWK)
-  [Struct `JWK`](#0x1_jwks_JWK)
-  [Struct `ProviderJWKs`](#0x1_jwks_ProviderJWKs)
-  [Struct `AllProvidersJWKs`](#0x1_jwks_AllProvidersJWKs)
-  [Resource `ObservedJWKs`](#0x1_jwks_ObservedJWKs)
-  [Struct `ObservedJWKsUpdated`](#0x1_jwks_ObservedJWKsUpdated)
-  [Struct `Patch`](#0x1_jwks_Patch)
-  [Struct `PatchRemoveAll`](#0x1_jwks_PatchRemoveAll)
-  [Struct `PatchRemoveIssuer`](#0x1_jwks_PatchRemoveIssuer)
-  [Struct `PatchRemoveJWK`](#0x1_jwks_PatchRemoveJWK)
-  [Struct `PatchUpsertJWK`](#0x1_jwks_PatchUpsertJWK)
-  [Resource `Patches`](#0x1_jwks_Patches)
-  [Resource `PatchedJWKs`](#0x1_jwks_PatchedJWKs)
-  [Constants](#@Constants_0)
-  [Function `get_patched_jwk`](#0x1_jwks_get_patched_jwk)
-  [Function `try_get_patched_jwk`](#0x1_jwks_try_get_patched_jwk)
-  [Function `upsert_oidc_provider`](#0x1_jwks_upsert_oidc_provider)
-  [Function `upsert_oidc_provider_for_next_epoch`](#0x1_jwks_upsert_oidc_provider_for_next_epoch)
-  [Function `remove_oidc_provider`](#0x1_jwks_remove_oidc_provider)
-  [Function `remove_oidc_provider_for_next_epoch`](#0x1_jwks_remove_oidc_provider_for_next_epoch)
-  [Function `on_new_epoch`](#0x1_jwks_on_new_epoch)
-  [Function `set_patches`](#0x1_jwks_set_patches)
-  [Function `new_patch_remove_all`](#0x1_jwks_new_patch_remove_all)
-  [Function `new_patch_remove_issuer`](#0x1_jwks_new_patch_remove_issuer)
-  [Function `new_patch_remove_jwk`](#0x1_jwks_new_patch_remove_jwk)
-  [Function `new_patch_upsert_jwk`](#0x1_jwks_new_patch_upsert_jwk)
-  [Function `new_rsa_jwk`](#0x1_jwks_new_rsa_jwk)
-  [Function `new_unsupported_jwk`](#0x1_jwks_new_unsupported_jwk)
-  [Function `initialize`](#0x1_jwks_initialize)
-  [Function `remove_oidc_provider_internal`](#0x1_jwks_remove_oidc_provider_internal)
-  [Function `upsert_into_observed_jwks`](#0x1_jwks_upsert_into_observed_jwks)
-  [Function `remove_issuer_from_observed_jwks`](#0x1_jwks_remove_issuer_from_observed_jwks)
-  [Function `regenerate_patched_jwks`](#0x1_jwks_regenerate_patched_jwks)
-  [Function `try_get_jwk_by_issuer`](#0x1_jwks_try_get_jwk_by_issuer)
-  [Function `try_get_jwk_by_id`](#0x1_jwks_try_get_jwk_by_id)
-  [Function `get_jwk_id`](#0x1_jwks_get_jwk_id)
-  [Function `upsert_provider_jwks`](#0x1_jwks_upsert_provider_jwks)
-  [Function `remove_issuer`](#0x1_jwks_remove_issuer)
-  [Function `upsert_jwk`](#0x1_jwks_upsert_jwk)
-  [Function `remove_jwk`](#0x1_jwks_remove_jwk)
-  [Function `apply_patch`](#0x1_jwks_apply_patch)
-  [Specification](#@Specification_1)
    -  [Function `on_new_epoch`](#@Specification_1_on_new_epoch)


```move
module 0x1::jwks {
    use 0x1::chain_status;
    use 0x1::comparator;
    use 0x1::config_buffer;
    use 0x1::copyable_any;
    use 0x1::error;
    use 0x1::event;
    use 0x1::option;
    use 0x1::reconfiguration;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::vector;
}
```


<a id="0x1_jwks_OIDCProvider"></a>

## Struct `OIDCProvider`

An OIDC provider.


```move
module 0x1::jwks {
    struct OIDCProvider has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`name: vector<u8>`
</dt>
<dd>
 The utf&#45;8 encoded issuer string. E.g., b&quot;https://www.facebook.com&quot;.
</dd>
<dt>
`config_url: vector<u8>`
</dt>
<dd>
 The ut8&#45;8 encoded OpenID configuration URL of the provider.
 E.g., b&quot;https://www.facebook.com/.well&#45;known/openid&#45;configuration/&quot;.
</dd>
</dl>


<a id="0x1_jwks_SupportedOIDCProviders"></a>

## Resource `SupportedOIDCProviders`

A list of OIDC providers whose JWKs should be watched by validators. Maintained by governance proposals.


```move
module 0x1::jwks {
    struct SupportedOIDCProviders has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`providers: vector<jwks::OIDCProvider>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_UnsupportedJWK"></a>

## Struct `UnsupportedJWK`

An JWK variant that represents the JWKs which were observed but not yet supported by Aptos.
Observing `UnsupportedJWK`s means the providers adopted a new key type/format, and the system should be updated.


```move
module 0x1::jwks {
    struct UnsupportedJWK has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`id: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`payload: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_RSA_JWK"></a>

## Struct `RSA_JWK`

A JWK variant where `kty` is `RSA`.


```move
module 0x1::jwks {
    struct RSA_JWK has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`kid: string::String`
</dt>
<dd>

</dd>
<dt>
`kty: string::String`
</dt>
<dd>

</dd>
<dt>
`alg: string::String`
</dt>
<dd>

</dd>
<dt>
`e: string::String`
</dt>
<dd>

</dd>
<dt>
`n: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_JWK"></a>

## Struct `JWK`

A JSON web key.


```move
module 0x1::jwks {
    struct JWK has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`variant: copyable_any::Any`
</dt>
<dd>
 A `JWK` variant packed as an `Any`.
 Currently the variant type is one of the following.
 &#45; `RSA_JWK`
 &#45; `UnsupportedJWK`
</dd>
</dl>


<a id="0x1_jwks_ProviderJWKs"></a>

## Struct `ProviderJWKs`

A provider and its `JWK`s.


```move
module 0x1::jwks {
    struct ProviderJWKs has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`issuer: vector<u8>`
</dt>
<dd>
 The utf&#45;8 encoding of the issuer string (e.g., &quot;https://www.facebook.com&quot;).
</dd>
<dt>
`version: u64`
</dt>
<dd>
 A version number is needed by JWK consensus to dedup the updates.
 e.g, when on chain version &#61; 5, multiple nodes can propose an update with version &#61; 6.
 Bumped every time the JWKs for the current issuer is updated.
 The Rust authenticator only uses the latest version.
</dd>
<dt>
`jwks: vector<jwks::JWK>`
</dt>
<dd>
 Vector of `JWK`&apos;s sorted by their unique ID (from `get_jwk_id`) in dictionary order.
</dd>
</dl>


<a id="0x1_jwks_AllProvidersJWKs"></a>

## Struct `AllProvidersJWKs`

Multiple `ProviderJWKs` objects, indexed by issuer and key ID.


```move
module 0x1::jwks {
    struct AllProvidersJWKs has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`entries: vector<jwks::ProviderJWKs>`
</dt>
<dd>
 Vector of `ProviderJWKs` sorted by `ProviderJWKs::issuer` in dictionary order.
</dd>
</dl>


<a id="0x1_jwks_ObservedJWKs"></a>

## Resource `ObservedJWKs`

The `AllProvidersJWKs` that validators observed and agreed on.


```move
module 0x1::jwks {
    struct ObservedJWKs has copy, drop, store, key
}
```


##### Fields


<dl>
<dt>
`jwks: jwks::AllProvidersJWKs`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_ObservedJWKsUpdated"></a>

## Struct `ObservedJWKsUpdated`

When `ObservedJWKs` is updated, this event is sent to resync the JWK consensus state in all validators.


```move
module 0x1::jwks {
    #[event]
    struct ObservedJWKsUpdated has drop, store
}
```


##### Fields


<dl>
<dt>
`epoch: u64`
</dt>
<dd>

</dd>
<dt>
`jwks: jwks::AllProvidersJWKs`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_Patch"></a>

## Struct `Patch`

A small edit or patch that is applied to a `AllProvidersJWKs` to obtain `PatchedJWKs`.


```move
module 0x1::jwks {
    struct Patch has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`variant: copyable_any::Any`
</dt>
<dd>
 A `Patch` variant packed as an `Any`.
 Currently the variant type is one of the following.
 &#45; `PatchRemoveAll`
 &#45; `PatchRemoveIssuer`
 &#45; `PatchRemoveJWK`
 &#45; `PatchUpsertJWK`
</dd>
</dl>


<a id="0x1_jwks_PatchRemoveAll"></a>

## Struct `PatchRemoveAll`

A `Patch` variant to remove all JWKs.


```move
module 0x1::jwks {
    struct PatchRemoveAll has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`dummy_field: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_PatchRemoveIssuer"></a>

## Struct `PatchRemoveIssuer`

A `Patch` variant to remove an issuer and all its JWKs.


```move
module 0x1::jwks {
    struct PatchRemoveIssuer has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`issuer: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_PatchRemoveJWK"></a>

## Struct `PatchRemoveJWK`

A `Patch` variant to remove a specific JWK of an issuer.


```move
module 0x1::jwks {
    struct PatchRemoveJWK has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`issuer: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`jwk_id: vector<u8>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_PatchUpsertJWK"></a>

## Struct `PatchUpsertJWK`

A `Patch` variant to upsert a JWK for an issuer.


```move
module 0x1::jwks {
    struct PatchUpsertJWK has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`issuer: vector<u8>`
</dt>
<dd>

</dd>
<dt>
`jwk: jwks::JWK`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_Patches"></a>

## Resource `Patches`

A sequence of `Patch` objects that are applied &#42;one by one&#42; to the `ObservedJWKs`.

Maintained by governance proposals.


```move
module 0x1::jwks {
    struct Patches has key
}
```


##### Fields


<dl>
<dt>
`patches: vector<jwks::Patch>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_jwks_PatchedJWKs"></a>

## Resource `PatchedJWKs`

The result of applying the `Patches` to the `ObservedJWKs`.
This is what applications should consume.


```move
module 0x1::jwks {
    struct PatchedJWKs has drop, key
}
```


##### Fields


<dl>
<dt>
`jwks: jwks::AllProvidersJWKs`
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_jwks_EISSUER_NOT_FOUND"></a>



```move
module 0x1::jwks {
    const EISSUER_NOT_FOUND: u64 = 5;
}
```


<a id="0x1_jwks_EJWK_ID_NOT_FOUND"></a>



```move
module 0x1::jwks {
    const EJWK_ID_NOT_FOUND: u64 = 6;
}
```


<a id="0x1_jwks_ENATIVE_INCORRECT_VERSION"></a>



```move
module 0x1::jwks {
    const ENATIVE_INCORRECT_VERSION: u64 = 259;
}
```


<a id="0x1_jwks_ENATIVE_MISSING_RESOURCE_OBSERVED_JWKS"></a>



```move
module 0x1::jwks {
    const ENATIVE_MISSING_RESOURCE_OBSERVED_JWKS: u64 = 258;
}
```


<a id="0x1_jwks_ENATIVE_MISSING_RESOURCE_VALIDATOR_SET"></a>



```move
module 0x1::jwks {
    const ENATIVE_MISSING_RESOURCE_VALIDATOR_SET: u64 = 257;
}
```


<a id="0x1_jwks_ENATIVE_MULTISIG_VERIFICATION_FAILED"></a>



```move
module 0x1::jwks {
    const ENATIVE_MULTISIG_VERIFICATION_FAILED: u64 = 260;
}
```


<a id="0x1_jwks_ENATIVE_NOT_ENOUGH_VOTING_POWER"></a>



```move
module 0x1::jwks {
    const ENATIVE_NOT_ENOUGH_VOTING_POWER: u64 = 261;
}
```


<a id="0x1_jwks_EUNEXPECTED_EPOCH"></a>



```move
module 0x1::jwks {
    const EUNEXPECTED_EPOCH: u64 = 1;
}
```


<a id="0x1_jwks_EUNEXPECTED_VERSION"></a>



```move
module 0x1::jwks {
    const EUNEXPECTED_VERSION: u64 = 2;
}
```


<a id="0x1_jwks_EUNKNOWN_JWK_VARIANT"></a>



```move
module 0x1::jwks {
    const EUNKNOWN_JWK_VARIANT: u64 = 4;
}
```


<a id="0x1_jwks_EUNKNOWN_PATCH_VARIANT"></a>



```move
module 0x1::jwks {
    const EUNKNOWN_PATCH_VARIANT: u64 = 3;
}
```


<a id="0x1_jwks_get_patched_jwk"></a>

## Function `get_patched_jwk`

Get a JWK by issuer and key ID from the `PatchedJWKs`.
Abort if such a JWK does not exist.
More convenient to call from Rust, since it does not wrap the JWK in an `Option`.


```move
module 0x1::jwks {
    public fun get_patched_jwk(issuer: vector<u8>, jwk_id: vector<u8>): jwks::JWK
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun get_patched_jwk(issuer: vector<u8>, jwk_id: vector<u8>): JWK acquires PatchedJWKs {
        option::extract(&mut try_get_patched_jwk(issuer, jwk_id))
    }
}
```


<a id="0x1_jwks_try_get_patched_jwk"></a>

## Function `try_get_patched_jwk`

Get a JWK by issuer and key ID from the `PatchedJWKs`, if it exists.
More convenient to call from Move, since it does not abort.


```move
module 0x1::jwks {
    public fun try_get_patched_jwk(issuer: vector<u8>, jwk_id: vector<u8>): option::Option<jwks::JWK>
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun try_get_patched_jwk(issuer: vector<u8>, jwk_id: vector<u8>): Option<JWK> acquires PatchedJWKs {
        let jwks = &borrow_global<PatchedJWKs>(@aptos_framework).jwks;
        try_get_jwk_by_issuer(jwks, issuer, jwk_id)
    }
}
```


<a id="0x1_jwks_upsert_oidc_provider"></a>

## Function `upsert_oidc_provider`

Deprecated by `upsert_oidc_provider_for_next_epoch()`.

TODO: update all the tests that reference this function, then disable this function.


```move
module 0x1::jwks {
    public fun upsert_oidc_provider(fx: &signer, name: vector<u8>, config_url: vector<u8>): option::Option<vector<u8>>
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun upsert_oidc_provider(fx: &signer, name: vector<u8>, config_url: vector<u8>): Option<vector<u8>> acquires SupportedOIDCProviders {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();

        let provider_set = borrow_global_mut<SupportedOIDCProviders>(@aptos_framework);

        let old_config_url= remove_oidc_provider_internal(provider_set, name);
        vector::push_back(&mut provider_set.providers, OIDCProvider { name, config_url });
        old_config_url
    }
}
```


<a id="0x1_jwks_upsert_oidc_provider_for_next_epoch"></a>

## Function `upsert_oidc_provider_for_next_epoch`

Used in on&#45;chain governances to update the supported OIDC providers, effective starting next epoch.
Example usage:
```
aptos_framework::jwks::upsert_oidc_provider_for_next_epoch(
&amp;framework_signer,
b&quot;https://accounts.google.com&quot;,
b&quot;https://accounts.google.com/.well&#45;known/openid&#45;configuration&quot;
);
aptos_framework::aptos_governance::reconfigure(&amp;framework_signer);
```


```move
module 0x1::jwks {
    public fun upsert_oidc_provider_for_next_epoch(fx: &signer, name: vector<u8>, config_url: vector<u8>): option::Option<vector<u8>>
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun upsert_oidc_provider_for_next_epoch(fx: &signer, name: vector<u8>, config_url: vector<u8>): Option<vector<u8>> acquires SupportedOIDCProviders {
        system_addresses::assert_aptos_framework(fx);

        let provider_set = if (config_buffer::does_exist<SupportedOIDCProviders>()) {
            config_buffer::extract<SupportedOIDCProviders>()
        } else {
            *borrow_global_mut<SupportedOIDCProviders>(@aptos_framework)
        };

        let old_config_url = remove_oidc_provider_internal(&mut provider_set, name);
        vector::push_back(&mut provider_set.providers, OIDCProvider { name, config_url });
        config_buffer::upsert(provider_set);
        old_config_url
    }
}
```


<a id="0x1_jwks_remove_oidc_provider"></a>

## Function `remove_oidc_provider`

Deprecated by `remove_oidc_provider_for_next_epoch()`.

TODO: update all the tests that reference this function, then disable this function.


```move
module 0x1::jwks {
    public fun remove_oidc_provider(fx: &signer, name: vector<u8>): option::Option<vector<u8>>
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun remove_oidc_provider(fx: &signer, name: vector<u8>): Option<vector<u8>> acquires SupportedOIDCProviders {
        system_addresses::assert_aptos_framework(fx);
        chain_status::assert_genesis();

        let provider_set = borrow_global_mut<SupportedOIDCProviders>(@aptos_framework);
        remove_oidc_provider_internal(provider_set, name)
    }
}
```


<a id="0x1_jwks_remove_oidc_provider_for_next_epoch"></a>

## Function `remove_oidc_provider_for_next_epoch`

Used in on&#45;chain governances to update the supported OIDC providers, effective starting next epoch.
Example usage:
```
aptos_framework::jwks::remove_oidc_provider_for_next_epoch(
&amp;framework_signer,
b&quot;https://accounts.google.com&quot;,
);
aptos_framework::aptos_governance::reconfigure(&amp;framework_signer);
```


```move
module 0x1::jwks {
    public fun remove_oidc_provider_for_next_epoch(fx: &signer, name: vector<u8>): option::Option<vector<u8>>
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun remove_oidc_provider_for_next_epoch(fx: &signer, name: vector<u8>): Option<vector<u8>> acquires SupportedOIDCProviders {
        system_addresses::assert_aptos_framework(fx);

        let provider_set = if (config_buffer::does_exist<SupportedOIDCProviders>()) {
            config_buffer::extract<SupportedOIDCProviders>()
        } else {
            *borrow_global_mut<SupportedOIDCProviders>(@aptos_framework)
        };
        let ret = remove_oidc_provider_internal(&mut provider_set, name);
        config_buffer::upsert(provider_set);
        ret
    }
}
```


<a id="0x1_jwks_on_new_epoch"></a>

## Function `on_new_epoch`

Only used in reconfigurations to apply the pending `SupportedOIDCProviders`, if there is any.


```move
module 0x1::jwks {
    public(friend) fun on_new_epoch(framework: &signer)
}
```


##### Implementation


```move
module 0x1::jwks {
    public(friend) fun on_new_epoch(framework: &signer) acquires SupportedOIDCProviders {
        system_addresses::assert_aptos_framework(framework);
        if (config_buffer::does_exist<SupportedOIDCProviders>()) {
            let new_config = config_buffer::extract<SupportedOIDCProviders>();
            if (exists<SupportedOIDCProviders>(@aptos_framework)) {
                *borrow_global_mut<SupportedOIDCProviders>(@aptos_framework) = new_config;
            } else {
                move_to(framework, new_config);
            }
        }
    }
}
```


<a id="0x1_jwks_set_patches"></a>

## Function `set_patches`

Set the `Patches`. Only called in governance proposals.


```move
module 0x1::jwks {
    public fun set_patches(fx: &signer, patches: vector<jwks::Patch>)
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun set_patches(fx: &signer, patches: vector<Patch>) acquires Patches, PatchedJWKs, ObservedJWKs {
        system_addresses::assert_aptos_framework(fx);
        borrow_global_mut<Patches>(@aptos_framework).patches = patches;
        regenerate_patched_jwks();
    }
}
```


<a id="0x1_jwks_new_patch_remove_all"></a>

## Function `new_patch_remove_all`

Create a `Patch` that removes all entries.


```move
module 0x1::jwks {
    public fun new_patch_remove_all(): jwks::Patch
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun new_patch_remove_all(): Patch {
        Patch {
            variant: copyable_any::pack(PatchRemoveAll {}),
        }
    }
}
```


<a id="0x1_jwks_new_patch_remove_issuer"></a>

## Function `new_patch_remove_issuer`

Create a `Patch` that removes the entry of a given issuer, if exists.


```move
module 0x1::jwks {
    public fun new_patch_remove_issuer(issuer: vector<u8>): jwks::Patch
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun new_patch_remove_issuer(issuer: vector<u8>): Patch {
        Patch {
            variant: copyable_any::pack(PatchRemoveIssuer { issuer }),
        }
    }
}
```


<a id="0x1_jwks_new_patch_remove_jwk"></a>

## Function `new_patch_remove_jwk`

Create a `Patch` that removes the entry of a given issuer, if exists.


```move
module 0x1::jwks {
    public fun new_patch_remove_jwk(issuer: vector<u8>, jwk_id: vector<u8>): jwks::Patch
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun new_patch_remove_jwk(issuer: vector<u8>, jwk_id: vector<u8>): Patch {
        Patch {
            variant: copyable_any::pack(PatchRemoveJWK { issuer, jwk_id })
        }
    }
}
```


<a id="0x1_jwks_new_patch_upsert_jwk"></a>

## Function `new_patch_upsert_jwk`

Create a `Patch` that upserts a JWK into an issuer&apos;s JWK set.


```move
module 0x1::jwks {
    public fun new_patch_upsert_jwk(issuer: vector<u8>, jwk: jwks::JWK): jwks::Patch
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun new_patch_upsert_jwk(issuer: vector<u8>, jwk: JWK): Patch {
        Patch {
            variant: copyable_any::pack(PatchUpsertJWK { issuer, jwk })
        }
    }
}
```


<a id="0x1_jwks_new_rsa_jwk"></a>

## Function `new_rsa_jwk`

Create a `JWK` of variant `RSA_JWK`.


```move
module 0x1::jwks {
    public fun new_rsa_jwk(kid: string::String, alg: string::String, e: string::String, n: string::String): jwks::JWK
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun new_rsa_jwk(kid: String, alg: String, e: String, n: String): JWK {
        JWK {
            variant: copyable_any::pack(RSA_JWK {
                kid,
                kty: utf8(b"RSA"),
                e,
                n,
                alg,
            }),
        }
    }
}
```


<a id="0x1_jwks_new_unsupported_jwk"></a>

## Function `new_unsupported_jwk`

Create a `JWK` of variant `UnsupportedJWK`.


```move
module 0x1::jwks {
    public fun new_unsupported_jwk(id: vector<u8>, payload: vector<u8>): jwks::JWK
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun new_unsupported_jwk(id: vector<u8>, payload: vector<u8>): JWK {
        JWK {
            variant: copyable_any::pack(UnsupportedJWK { id, payload })
        }
    }
}
```


<a id="0x1_jwks_initialize"></a>

## Function `initialize`

Initialize some JWK resources. Should only be invoked by genesis.


```move
module 0x1::jwks {
    public fun initialize(fx: &signer)
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun initialize(fx: &signer) {
        system_addresses::assert_aptos_framework(fx);
        move_to(fx, SupportedOIDCProviders { providers: vector[] });
        move_to(fx, ObservedJWKs { jwks: AllProvidersJWKs { entries: vector[] } });
        move_to(fx, Patches { patches: vector[] });
        move_to(fx, PatchedJWKs { jwks: AllProvidersJWKs { entries: vector[] } });
    }
}
```


<a id="0x1_jwks_remove_oidc_provider_internal"></a>

## Function `remove_oidc_provider_internal`

Helper function that removes an OIDC provider from the `SupportedOIDCProviders`.
Returns the old config URL of the provider, if any, as an `Option`.


```move
module 0x1::jwks {
    fun remove_oidc_provider_internal(provider_set: &mut jwks::SupportedOIDCProviders, name: vector<u8>): option::Option<vector<u8>>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun remove_oidc_provider_internal(provider_set: &mut SupportedOIDCProviders, name: vector<u8>): Option<vector<u8>> {
        let (name_exists, idx) = vector::find(&provider_set.providers, |obj| {
            let provider: &OIDCProvider = obj;
            provider.name == name
        });

        if (name_exists) {
            let old_provider = vector::swap_remove(&mut provider_set.providers, idx);
            option::some(old_provider.config_url)
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_jwks_upsert_into_observed_jwks"></a>

## Function `upsert_into_observed_jwks`

Only used by validators to publish their observed JWK update.

NOTE: It is assumed verification has been done to ensure each update is quorum&#45;certified,
and its `version` equals to the on&#45;chain version &#43; 1.


```move
module 0x1::jwks {
    public fun upsert_into_observed_jwks(fx: &signer, provider_jwks_vec: vector<jwks::ProviderJWKs>)
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun upsert_into_observed_jwks(fx: &signer, provider_jwks_vec: vector<ProviderJWKs>) acquires ObservedJWKs, PatchedJWKs, Patches {
        system_addresses::assert_aptos_framework(fx);
        let observed_jwks = borrow_global_mut<ObservedJWKs>(@aptos_framework);
        vector::for_each(provider_jwks_vec, |obj| {
            let provider_jwks: ProviderJWKs = obj;
            upsert_provider_jwks(&mut observed_jwks.jwks, provider_jwks);
        });

        let epoch = reconfiguration::current_epoch();
        emit(ObservedJWKsUpdated { epoch, jwks: observed_jwks.jwks });
        regenerate_patched_jwks();
    }
}
```


<a id="0x1_jwks_remove_issuer_from_observed_jwks"></a>

## Function `remove_issuer_from_observed_jwks`

Only used by governance to delete an issuer from `ObservedJWKs`, if it exists.

Return the potentially existing `ProviderJWKs` of the given issuer.


```move
module 0x1::jwks {
    public fun remove_issuer_from_observed_jwks(fx: &signer, issuer: vector<u8>): option::Option<jwks::ProviderJWKs>
}
```


##### Implementation


```move
module 0x1::jwks {
    public fun remove_issuer_from_observed_jwks(fx: &signer, issuer: vector<u8>): Option<ProviderJWKs> acquires ObservedJWKs, PatchedJWKs, Patches {
        system_addresses::assert_aptos_framework(fx);
        let observed_jwks = borrow_global_mut<ObservedJWKs>(@aptos_framework);
        let old_value = remove_issuer(&mut observed_jwks.jwks, issuer);

        let epoch = reconfiguration::current_epoch();
        emit(ObservedJWKsUpdated { epoch, jwks: observed_jwks.jwks });
        regenerate_patched_jwks();

        old_value
    }
}
```


<a id="0x1_jwks_regenerate_patched_jwks"></a>

## Function `regenerate_patched_jwks`

Regenerate `PatchedJWKs` from `ObservedJWKs` and `Patches` and save the result.


```move
module 0x1::jwks {
    fun regenerate_patched_jwks()
}
```


##### Implementation


```move
module 0x1::jwks {
    fun regenerate_patched_jwks() acquires PatchedJWKs, Patches, ObservedJWKs {
        let jwks = borrow_global<ObservedJWKs>(@aptos_framework).jwks;
        let patches = borrow_global<Patches>(@aptos_framework);
        vector::for_each_ref(&patches.patches, |obj|{
            let patch: &Patch = obj;
            apply_patch(&mut jwks, *patch);
        });
        *borrow_global_mut<PatchedJWKs>(@aptos_framework) = PatchedJWKs { jwks };
    }
}
```


<a id="0x1_jwks_try_get_jwk_by_issuer"></a>

## Function `try_get_jwk_by_issuer`

Get a JWK by issuer and key ID from a `AllProvidersJWKs`, if it exists.


```move
module 0x1::jwks {
    fun try_get_jwk_by_issuer(jwks: &jwks::AllProvidersJWKs, issuer: vector<u8>, jwk_id: vector<u8>): option::Option<jwks::JWK>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun try_get_jwk_by_issuer(jwks: &AllProvidersJWKs, issuer: vector<u8>, jwk_id: vector<u8>): Option<JWK> {
        let (issuer_found, index) = vector::find(&jwks.entries, |obj| {
            let provider_jwks: &ProviderJWKs = obj;
            issuer == provider_jwks.issuer
        });

        if (issuer_found) {
            try_get_jwk_by_id(vector::borrow(&jwks.entries, index), jwk_id)
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_jwks_try_get_jwk_by_id"></a>

## Function `try_get_jwk_by_id`

Get a JWK by key ID from a `ProviderJWKs`, if it exists.


```move
module 0x1::jwks {
    fun try_get_jwk_by_id(provider_jwks: &jwks::ProviderJWKs, jwk_id: vector<u8>): option::Option<jwks::JWK>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun try_get_jwk_by_id(provider_jwks: &ProviderJWKs, jwk_id: vector<u8>): Option<JWK> {
        let (jwk_id_found, index) = vector::find(&provider_jwks.jwks, |obj|{
            let jwk: &JWK = obj;
            jwk_id == get_jwk_id(jwk)
        });

        if (jwk_id_found) {
            option::some(*vector::borrow(&provider_jwks.jwks, index))
        } else {
            option::none()
        }
    }
}
```


<a id="0x1_jwks_get_jwk_id"></a>

## Function `get_jwk_id`

Get the ID of a JWK.


```move
module 0x1::jwks {
    fun get_jwk_id(jwk: &jwks::JWK): vector<u8>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun get_jwk_id(jwk: &JWK): vector<u8> {
        let variant_type_name = *string::bytes(copyable_any::type_name(&jwk.variant));
        if (variant_type_name == b"0x1::jwks::RSA_JWK") {
            let rsa = copyable_any::unpack<RSA_JWK>(jwk.variant);
            *string::bytes(&rsa.kid)
        } else if (variant_type_name == b"0x1::jwks::UnsupportedJWK") {
            let unsupported = copyable_any::unpack<UnsupportedJWK>(jwk.variant);
            unsupported.id
        } else {
            abort(error::invalid_argument(EUNKNOWN_JWK_VARIANT))
        }
    }
}
```


<a id="0x1_jwks_upsert_provider_jwks"></a>

## Function `upsert_provider_jwks`

Upsert a `ProviderJWKs` into an `AllProvidersJWKs`. If this upsert replaced an existing entry, return it.
Maintains the sorted&#45;by&#45;issuer invariant in `AllProvidersJWKs`.


```move
module 0x1::jwks {
    fun upsert_provider_jwks(jwks: &mut jwks::AllProvidersJWKs, provider_jwks: jwks::ProviderJWKs): option::Option<jwks::ProviderJWKs>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun upsert_provider_jwks(jwks: &mut AllProvidersJWKs, provider_jwks: ProviderJWKs): Option<ProviderJWKs> {
        // NOTE: Using a linear-time search here because we do not expect too many providers.
        let found = false;
        let index = 0;
        let num_entries = vector::length(&jwks.entries);
        while (index < num_entries) {
            let cur_entry = vector::borrow(&jwks.entries, index);
            let comparison = compare_u8_vector(provider_jwks.issuer, cur_entry.issuer);
            if (is_greater_than(&comparison)) {
                index = index + 1;
            } else {
                found = is_equal(&comparison);
                break
            }
        };

        // Now if `found == true`, `index` points to the JWK we want to update/remove; otherwise, `index` points to
        // where we want to insert.
        let ret = if (found) {
            let entry = vector::borrow_mut(&mut jwks.entries, index);
            let old_entry = option::some(*entry);
            *entry = provider_jwks;
            old_entry
        } else {
            vector::insert(&mut jwks.entries, index, provider_jwks);
            option::none()
        };

        ret
    }
}
```


<a id="0x1_jwks_remove_issuer"></a>

## Function `remove_issuer`

Remove the entry of an issuer from a `AllProvidersJWKs` and return the entry, if exists.
Maintains the sorted&#45;by&#45;issuer invariant in `AllProvidersJWKs`.


```move
module 0x1::jwks {
    fun remove_issuer(jwks: &mut jwks::AllProvidersJWKs, issuer: vector<u8>): option::Option<jwks::ProviderJWKs>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun remove_issuer(jwks: &mut AllProvidersJWKs, issuer: vector<u8>): Option<ProviderJWKs> {
        let (found, index) = vector::find(&jwks.entries, |obj| {
            let provider_jwk_set: &ProviderJWKs = obj;
            provider_jwk_set.issuer == issuer
        });

        let ret = if (found) {
            option::some(vector::remove(&mut jwks.entries, index))
        } else {
            option::none()
        };

        ret
    }
}
```


<a id="0x1_jwks_upsert_jwk"></a>

## Function `upsert_jwk`

Upsert a `JWK` into a `ProviderJWKs`. If this upsert replaced an existing entry, return it.


```move
module 0x1::jwks {
    fun upsert_jwk(set: &mut jwks::ProviderJWKs, jwk: jwks::JWK): option::Option<jwks::JWK>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun upsert_jwk(set: &mut ProviderJWKs, jwk: JWK): Option<JWK> {
        let found = false;
        let index = 0;
        let num_entries = vector::length(&set.jwks);
        while (index < num_entries) {
            let cur_entry = vector::borrow(&set.jwks, index);
            let comparison = compare_u8_vector(get_jwk_id(&jwk), get_jwk_id(cur_entry));
            if (is_greater_than(&comparison)) {
                index = index + 1;
            } else {
                found = is_equal(&comparison);
                break
            }
        };

        // Now if `found == true`, `index` points to the JWK we want to update/remove; otherwise, `index` points to
        // where we want to insert.
        let ret = if (found) {
            let entry = vector::borrow_mut(&mut set.jwks, index);
            let old_entry = option::some(*entry);
            *entry = jwk;
            old_entry
        } else {
            vector::insert(&mut set.jwks, index, jwk);
            option::none()
        };

        ret
    }
}
```


<a id="0x1_jwks_remove_jwk"></a>

## Function `remove_jwk`

Remove the entry of a key ID from a `ProviderJWKs` and return the entry, if exists.


```move
module 0x1::jwks {
    fun remove_jwk(jwks: &mut jwks::ProviderJWKs, jwk_id: vector<u8>): option::Option<jwks::JWK>
}
```


##### Implementation


```move
module 0x1::jwks {
    fun remove_jwk(jwks: &mut ProviderJWKs, jwk_id: vector<u8>): Option<JWK> {
        let (found, index) = vector::find(&jwks.jwks, |obj| {
            let jwk: &JWK = obj;
            jwk_id == get_jwk_id(jwk)
        });

        let ret = if (found) {
            option::some(vector::remove(&mut jwks.jwks, index))
        } else {
            option::none()
        };

        ret
    }
}
```


<a id="0x1_jwks_apply_patch"></a>

## Function `apply_patch`

Modify an `AllProvidersJWKs` object with a `Patch`.
Maintains the sorted&#45;by&#45;issuer invariant in `AllProvidersJWKs`.


```move
module 0x1::jwks {
    fun apply_patch(jwks: &mut jwks::AllProvidersJWKs, patch: jwks::Patch)
}
```


##### Implementation


```move
module 0x1::jwks {
    fun apply_patch(jwks: &mut AllProvidersJWKs, patch: Patch) {
        let variant_type_name = *string::bytes(copyable_any::type_name(&patch.variant));
        if (variant_type_name == b"0x1::jwks::PatchRemoveAll") {
            jwks.entries = vector[];
        } else if (variant_type_name == b"0x1::jwks::PatchRemoveIssuer") {
            let cmd = copyable_any::unpack<PatchRemoveIssuer>(patch.variant);
            remove_issuer(jwks, cmd.issuer);
        } else if (variant_type_name == b"0x1::jwks::PatchRemoveJWK") {
            let cmd = copyable_any::unpack<PatchRemoveJWK>(patch.variant);
            // TODO: This is inefficient: we remove the issuer, modify its JWKs & and reinsert the updated issuer. Why
            // not just update it in place?
            let existing_jwk_set = remove_issuer(jwks, cmd.issuer);
            if (option::is_some(&existing_jwk_set)) {
                let jwk_set = option::extract(&mut existing_jwk_set);
                remove_jwk(&mut jwk_set, cmd.jwk_id);
                upsert_provider_jwks(jwks, jwk_set);
            };
        } else if (variant_type_name == b"0x1::jwks::PatchUpsertJWK") {
            let cmd = copyable_any::unpack<PatchUpsertJWK>(patch.variant);
            // TODO: This is inefficient: we remove the issuer, modify its JWKs & and reinsert the updated issuer. Why
            // not just update it in place?
            let existing_jwk_set = remove_issuer(jwks, cmd.issuer);
            let jwk_set = if (option::is_some(&existing_jwk_set)) {
                option::extract(&mut existing_jwk_set)
            } else {
                ProviderJWKs {
                    version: 0,
                    issuer: cmd.issuer,
                    jwks: vector[],
                }
            };
            upsert_jwk(&mut jwk_set, cmd.jwk);
            upsert_provider_jwks(jwks, jwk_set);
        } else {
            abort(std::error::invalid_argument(EUNKNOWN_PATCH_VARIANT))
        }
    }
}
```


<a id="@Specification_1"></a>

## Specification


<a id="@Specification_1_on_new_epoch"></a>

### Function `on_new_epoch`


```move
module 0x1::jwks {
    public(friend) fun on_new_epoch(framework: &signer)
}
```



```move
module 0x1::jwks {
    requires @aptos_framework == std::signer::address_of(framework);
    include config_buffer::OnNewEpochRequirement<SupportedOIDCProviders>;
    aborts_if false;
}
```

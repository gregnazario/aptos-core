
<a id="0x1_code"></a>

# Module `0x1::code`

This module supports functionality related to code management.


-  [Resource `PackageRegistry`](#0x1_code_PackageRegistry)
-  [Struct `PackageMetadata`](#0x1_code_PackageMetadata)
-  [Struct `PackageDep`](#0x1_code_PackageDep)
-  [Struct `ModuleMetadata`](#0x1_code_ModuleMetadata)
-  [Struct `UpgradePolicy`](#0x1_code_UpgradePolicy)
-  [Struct `PublishPackage`](#0x1_code_PublishPackage)
-  [Struct `AllowedDep`](#0x1_code_AllowedDep)
-  [Constants](#@Constants_0)
-  [Function `upgrade_policy_arbitrary`](#0x1_code_upgrade_policy_arbitrary)
-  [Function `upgrade_policy_compat`](#0x1_code_upgrade_policy_compat)
-  [Function `upgrade_policy_immutable`](#0x1_code_upgrade_policy_immutable)
-  [Function `can_change_upgrade_policy_to`](#0x1_code_can_change_upgrade_policy_to)
-  [Function `initialize`](#0x1_code_initialize)
-  [Function `publish_package`](#0x1_code_publish_package)
-  [Function `freeze_code_object`](#0x1_code_freeze_code_object)
-  [Function `publish_package_txn`](#0x1_code_publish_package_txn)
-  [Function `check_upgradability`](#0x1_code_check_upgradability)
-  [Function `check_coexistence`](#0x1_code_check_coexistence)
-  [Function `check_dependencies`](#0x1_code_check_dependencies)
-  [Function `is_policy_exempted_address`](#0x1_code_is_policy_exempted_address)
-  [Function `get_module_names`](#0x1_code_get_module_names)
-  [Function `request_publish`](#0x1_code_request_publish)
-  [Function `request_publish_with_allowed_deps`](#0x1_code_request_publish_with_allowed_deps)
-  [Specification](#@Specification_1)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `initialize`](#@Specification_1_initialize)
    -  [Function `publish_package`](#@Specification_1_publish_package)
    -  [Function `freeze_code_object`](#@Specification_1_freeze_code_object)
    -  [Function `publish_package_txn`](#@Specification_1_publish_package_txn)
    -  [Function `check_upgradability`](#@Specification_1_check_upgradability)
    -  [Function `check_coexistence`](#@Specification_1_check_coexistence)
    -  [Function `check_dependencies`](#@Specification_1_check_dependencies)
    -  [Function `get_module_names`](#@Specification_1_get_module_names)
    -  [Function `request_publish`](#@Specification_1_request_publish)
    -  [Function `request_publish_with_allowed_deps`](#@Specification_1_request_publish_with_allowed_deps)


```move
module 0x1::code {
    use 0x1::copyable_any;
    use 0x1::error;
    use 0x1::event;
    use 0x1::features;
    use 0x1::object;
    use 0x1::option;
    use 0x1::signer;
    use 0x1::string;
    use 0x1::system_addresses;
    use 0x1::util;
    use 0x1::vector;
}
```


<a id="0x1_code_PackageRegistry"></a>

## Resource `PackageRegistry`

The package registry at the given address.


```move
module 0x1::code {
    struct PackageRegistry has drop, store, key
}
```


##### Fields


<dl>
<dt>
`packages: vector<code::PackageMetadata>`
</dt>
<dd>
 Packages installed at this address.
</dd>
</dl>


<a id="0x1_code_PackageMetadata"></a>

## Struct `PackageMetadata`

Metadata for a package. All byte blobs are represented as base64&#45;of&#45;gzipped&#45;bytes


```move
module 0x1::code {
    struct PackageMetadata has drop, store
}
```


##### Fields


<dl>
<dt>
`name: string::String`
</dt>
<dd>
 Name of this package.
</dd>
<dt>
`upgrade_policy: code::UpgradePolicy`
</dt>
<dd>
 The upgrade policy of this package.
</dd>
<dt>
`upgrade_number: u64`
</dt>
<dd>
 The numbers of times this module has been upgraded. Also serves as the on&#45;chain version.
 This field will be automatically assigned on successful upgrade.
</dd>
<dt>
`source_digest: string::String`
</dt>
<dd>
 The source digest of the sources in the package. This is constructed by first building the
 sha256 of each individual source, than sorting them alphabetically, and sha256 them again.
</dd>
<dt>
`manifest: vector<u8>`
</dt>
<dd>
 The package manifest, in the Move.toml format. Gzipped text.
</dd>
<dt>
`modules: vector<code::ModuleMetadata>`
</dt>
<dd>
 The list of modules installed by this package.
</dd>
<dt>
`deps: vector<code::PackageDep>`
</dt>
<dd>
 Holds PackageDeps.
</dd>
<dt>
`extension: option::Option<copyable_any::Any>`
</dt>
<dd>
 For future extension
</dd>
</dl>


<a id="0x1_code_PackageDep"></a>

## Struct `PackageDep`

A dependency to a package published at address


```move
module 0x1::code {
    struct PackageDep has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>

</dd>
<dt>
`package_name: string::String`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_code_ModuleMetadata"></a>

## Struct `ModuleMetadata`

Metadata about a module in a package.


```move
module 0x1::code {
    struct ModuleMetadata has drop, store
}
```


##### Fields


<dl>
<dt>
`name: string::String`
</dt>
<dd>
 Name of the module.
</dd>
<dt>
`source: vector<u8>`
</dt>
<dd>
 Source text, gzipped String. Empty if not provided.
</dd>
<dt>
`source_map: vector<u8>`
</dt>
<dd>
 Source map, in compressed BCS. Empty if not provided.
</dd>
<dt>
`extension: option::Option<copyable_any::Any>`
</dt>
<dd>
 For future extensions.
</dd>
</dl>


<a id="0x1_code_UpgradePolicy"></a>

## Struct `UpgradePolicy`

Describes an upgrade policy


```move
module 0x1::code {
    struct UpgradePolicy has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`policy: u8`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_code_PublishPackage"></a>

## Struct `PublishPackage`

Event emitted when code is published to an address.


```move
module 0x1::code {
    #[event]
    struct PublishPackage has drop, store
}
```


##### Fields


<dl>
<dt>
`code_address: address`
</dt>
<dd>

</dd>
<dt>
`is_upgrade: bool`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_code_AllowedDep"></a>

## Struct `AllowedDep`

A helper type for request_publish_with_allowed_deps


```move
module 0x1::code {
    struct AllowedDep has drop
}
```


##### Fields


<dl>
<dt>
`account: address`
</dt>
<dd>
 Address of the module.
</dd>
<dt>
`module_name: string::String`
</dt>
<dd>
 Name of the module. If this is the empty string, then this serves as a wildcard for
 all modules from this address. This is used for speeding up dependency checking for packages from
 well&#45;known framework addresses, where we can assume that there are no malicious packages.
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_code_ECODE_OBJECT_DOES_NOT_EXIST"></a>

`code_object` does not exist.


```move
module 0x1::code {
    const ECODE_OBJECT_DOES_NOT_EXIST: u64 = 10;
}
```


<a id="0x1_code_EDEP_ARBITRARY_NOT_SAME_ADDRESS"></a>

A dependency to an `arbitrary` package must be on the same address.


```move
module 0x1::code {
    const EDEP_ARBITRARY_NOT_SAME_ADDRESS: u64 = 7;
}
```


<a id="0x1_code_EDEP_WEAKER_POLICY"></a>

A dependency cannot have a weaker upgrade policy.


```move
module 0x1::code {
    const EDEP_WEAKER_POLICY: u64 = 6;
}
```


<a id="0x1_code_EINCOMPATIBLE_POLICY_DISABLED"></a>

Creating a package with incompatible upgrade policy is disabled.


```move
module 0x1::code {
    const EINCOMPATIBLE_POLICY_DISABLED: u64 = 8;
}
```


<a id="0x1_code_EMODULE_MISSING"></a>

Cannot delete a module that was published in the same package


```move
module 0x1::code {
    const EMODULE_MISSING: u64 = 4;
}
```


<a id="0x1_code_EMODULE_NAME_CLASH"></a>

Package contains duplicate module names with existing modules publised in other packages on this address


```move
module 0x1::code {
    const EMODULE_NAME_CLASH: u64 = 1;
}
```


<a id="0x1_code_ENOT_PACKAGE_OWNER"></a>

Not the owner of the package registry.


```move
module 0x1::code {
    const ENOT_PACKAGE_OWNER: u64 = 9;
}
```


<a id="0x1_code_EPACKAGE_DEP_MISSING"></a>

Dependency could not be resolved to any published package.


```move
module 0x1::code {
    const EPACKAGE_DEP_MISSING: u64 = 5;
}
```


<a id="0x1_code_EUPGRADE_IMMUTABLE"></a>

Cannot upgrade an immutable package


```move
module 0x1::code {
    const EUPGRADE_IMMUTABLE: u64 = 2;
}
```


<a id="0x1_code_EUPGRADE_WEAKER_POLICY"></a>

Cannot downgrade a package&apos;s upgradability policy


```move
module 0x1::code {
    const EUPGRADE_WEAKER_POLICY: u64 = 3;
}
```


<a id="0x1_code_upgrade_policy_arbitrary"></a>

## Function `upgrade_policy_arbitrary`

Whether unconditional code upgrade with no compatibility check is allowed. This
publication mode should only be used for modules which aren&apos;t shared with user others.
The developer is responsible for not breaking memory layout of any resources he already
stored on chain.


```move
module 0x1::code {
    public fun upgrade_policy_arbitrary(): code::UpgradePolicy
}
```


##### Implementation


```move
module 0x1::code {
    public fun upgrade_policy_arbitrary(): UpgradePolicy {
        UpgradePolicy { policy: 0 }
    }
}
```


<a id="0x1_code_upgrade_policy_compat"></a>

## Function `upgrade_policy_compat`

Whether a compatibility check should be performed for upgrades. The check only passes if
a new module has (a) the same public functions (b) for existing resources, no layout change.


```move
module 0x1::code {
    public fun upgrade_policy_compat(): code::UpgradePolicy
}
```


##### Implementation


```move
module 0x1::code {
    public fun upgrade_policy_compat(): UpgradePolicy {
        UpgradePolicy { policy: 1 }
    }
}
```


<a id="0x1_code_upgrade_policy_immutable"></a>

## Function `upgrade_policy_immutable`

Whether the modules in the package are immutable and cannot be upgraded.


```move
module 0x1::code {
    public fun upgrade_policy_immutable(): code::UpgradePolicy
}
```


##### Implementation


```move
module 0x1::code {
    public fun upgrade_policy_immutable(): UpgradePolicy {
        UpgradePolicy { policy: 2 }
    }
}
```


<a id="0x1_code_can_change_upgrade_policy_to"></a>

## Function `can_change_upgrade_policy_to`

Whether the upgrade policy can be changed. In general, the policy can be only
strengthened but not weakened.


```move
module 0x1::code {
    public fun can_change_upgrade_policy_to(from: code::UpgradePolicy, to: code::UpgradePolicy): bool
}
```


##### Implementation


```move
module 0x1::code {
    public fun can_change_upgrade_policy_to(from: UpgradePolicy, to: UpgradePolicy): bool {
        from.policy <= to.policy
    }
}
```


<a id="0x1_code_initialize"></a>

## Function `initialize`

Initialize package metadata for Genesis.


```move
module 0x1::code {
    fun initialize(aptos_framework: &signer, package_owner: &signer, metadata: code::PackageMetadata)
}
```


##### Implementation


```move
module 0x1::code {
    fun initialize(aptos_framework: &signer, package_owner: &signer, metadata: PackageMetadata)
    acquires PackageRegistry {
        system_addresses::assert_aptos_framework(aptos_framework);
        let addr = signer::address_of(package_owner);
        if (!exists<PackageRegistry>(addr)) {
            move_to(package_owner, PackageRegistry { packages: vector[metadata] })
        } else {
            vector::push_back(&mut borrow_global_mut<PackageRegistry>(addr).packages, metadata)
        }
    }
}
```


<a id="0x1_code_publish_package"></a>

## Function `publish_package`

Publishes a package at the given signer&apos;s address. The caller must provide package metadata describing the
package.


```move
module 0x1::code {
    public fun publish_package(owner: &signer, pack: code::PackageMetadata, code: vector<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::code {
    public fun publish_package(owner: &signer, pack: PackageMetadata, code: vector<vector<u8>>) acquires PackageRegistry {
        // Disallow incompatible upgrade mode. Governance can decide later if this should be reconsidered.
        assert!(
            pack.upgrade_policy.policy > upgrade_policy_arbitrary().policy,
            error::invalid_argument(EINCOMPATIBLE_POLICY_DISABLED),
        );

        let addr = signer::address_of(owner);
        if (!exists<PackageRegistry>(addr)) {
            move_to(owner, PackageRegistry { packages: vector::empty() })
        };

        // Checks for valid dependencies to other packages
        let allowed_deps = check_dependencies(addr, &pack);

        // Check package against conflicts
        // To avoid prover compiler error on spec
        // the package need to be an immutable variable
        let module_names = get_module_names(&pack);
        let package_immutable = &borrow_global<PackageRegistry>(addr).packages;
        let len = vector::length(package_immutable);
        let index = len;
        let upgrade_number = 0;
        vector::enumerate_ref(package_immutable
        , |i, old| {
            let old: &PackageMetadata = old;
            if (old.name == pack.name) {
                upgrade_number = old.upgrade_number + 1;
                check_upgradability(old, &pack, &module_names);
                index = i;
            } else {
                check_coexistence(old, &module_names)
            };
        });

        // Assign the upgrade counter.
        pack.upgrade_number = upgrade_number;

        let packages = &mut borrow_global_mut<PackageRegistry>(addr).packages;
        // Update registry
        let policy = pack.upgrade_policy;
        if (index < len) {
            *vector::borrow_mut(packages, index) = pack
        } else {
            vector::push_back(packages, pack)
        };

        event::emit(PublishPackage {
            code_address: addr,
            is_upgrade: upgrade_number > 0
        });

        // Request publish
        if (features::code_dependency_check_enabled())
            request_publish_with_allowed_deps(addr, module_names, allowed_deps, code, policy.policy)
        else
        // The new `request_publish_with_allowed_deps` has not yet rolled out, so call downwards
        // compatible code.
            request_publish(addr, module_names, code, policy.policy)
    }
}
```


<a id="0x1_code_freeze_code_object"></a>

## Function `freeze_code_object`



```move
module 0x1::code {
    public fun freeze_code_object(publisher: &signer, code_object: object::Object<code::PackageRegistry>)
}
```


##### Implementation


```move
module 0x1::code {
    public fun freeze_code_object(publisher: &signer, code_object: Object<PackageRegistry>) acquires PackageRegistry {
        let code_object_addr = object::object_address(&code_object);
        assert!(exists<PackageRegistry>(code_object_addr), error::not_found(ECODE_OBJECT_DOES_NOT_EXIST));
        assert!(
            object::is_owner(code_object, signer::address_of(publisher)),
            error::permission_denied(ENOT_PACKAGE_OWNER)
        );

        let registry = borrow_global_mut<PackageRegistry>(code_object_addr);
        vector::for_each_mut<PackageMetadata>(&mut registry.packages, |pack| {
            let package: &mut PackageMetadata = pack;
            package.upgrade_policy = upgrade_policy_immutable();
        });
    }
}
```


<a id="0x1_code_publish_package_txn"></a>

## Function `publish_package_txn`

Same as `publish_package` but as an entry function which can be called as a transaction. Because
of current restrictions for txn parameters, the metadata needs to be passed in serialized form.


```move
module 0x1::code {
    public entry fun publish_package_txn(owner: &signer, metadata_serialized: vector<u8>, code: vector<vector<u8>>)
}
```


##### Implementation


```move
module 0x1::code {
    public entry fun publish_package_txn(owner: &signer, metadata_serialized: vector<u8>, code: vector<vector<u8>>)
    acquires PackageRegistry {
        publish_package(owner, util::from_bytes<PackageMetadata>(metadata_serialized), code)
    }
}
```


<a id="0x1_code_check_upgradability"></a>

## Function `check_upgradability`

Checks whether the given package is upgradable, and returns true if a compatibility check is needed.


```move
module 0x1::code {
    fun check_upgradability(old_pack: &code::PackageMetadata, new_pack: &code::PackageMetadata, new_modules: &vector<string::String>)
}
```


##### Implementation


```move
module 0x1::code {
    fun check_upgradability(
        old_pack: &PackageMetadata, new_pack: &PackageMetadata, new_modules: &vector<String>) {
        assert!(old_pack.upgrade_policy.policy < upgrade_policy_immutable().policy,
            error::invalid_argument(EUPGRADE_IMMUTABLE));
        assert!(can_change_upgrade_policy_to(old_pack.upgrade_policy, new_pack.upgrade_policy),
            error::invalid_argument(EUPGRADE_WEAKER_POLICY));
        let old_modules = get_module_names(old_pack);

        vector::for_each_ref(&old_modules, |old_module| {
            assert!(
                vector::contains(new_modules, old_module),
                EMODULE_MISSING
            );
        });
    }
}
```


<a id="0x1_code_check_coexistence"></a>

## Function `check_coexistence`

Checks whether a new package with given names can co&#45;exist with old package.


```move
module 0x1::code {
    fun check_coexistence(old_pack: &code::PackageMetadata, new_modules: &vector<string::String>)
}
```


##### Implementation


```move
module 0x1::code {
    fun check_coexistence(old_pack: &PackageMetadata, new_modules: &vector<String>) {
        // The modules introduced by each package must not overlap with `names`.
        vector::for_each_ref(&old_pack.modules, |old_mod| {
            let old_mod: &ModuleMetadata = old_mod;
            let j = 0;
            while (j < vector::length(new_modules)) {
                let name = vector::borrow(new_modules, j);
                assert!(&old_mod.name != name, error::already_exists(EMODULE_NAME_CLASH));
                j = j + 1;
            };
        });
    }
}
```


<a id="0x1_code_check_dependencies"></a>

## Function `check_dependencies`

Check that the upgrade policies of all packages are equal or higher quality than this package. Also
compute the list of module dependencies which are allowed by the package metadata. The later
is passed on to the native layer to verify that bytecode dependencies are actually what is pretended here.


```move
module 0x1::code {
    fun check_dependencies(publish_address: address, pack: &code::PackageMetadata): vector<code::AllowedDep>
}
```


##### Implementation


```move
module 0x1::code {
    fun check_dependencies(publish_address: address, pack: &PackageMetadata): vector<AllowedDep>
    acquires PackageRegistry {
        let allowed_module_deps = vector::empty();
        let deps = &pack.deps;
        vector::for_each_ref(deps, |dep| {
            let dep: &PackageDep = dep;
            assert!(exists<PackageRegistry>(dep.account), error::not_found(EPACKAGE_DEP_MISSING));
            if (is_policy_exempted_address(dep.account)) {
                // Allow all modules from this address, by using "" as a wildcard in the AllowedDep
                let account: address = dep.account;
                let module_name = string::utf8(b"");
                vector::push_back(&mut allowed_module_deps, AllowedDep { account, module_name });
            } else {
                let registry = borrow_global<PackageRegistry>(dep.account);
                let found = vector::any(&registry.packages, |dep_pack| {
                    let dep_pack: &PackageMetadata = dep_pack;
                    if (dep_pack.name == dep.package_name) {
                        // Check policy
                        assert!(
                            dep_pack.upgrade_policy.policy >= pack.upgrade_policy.policy,
                            error::invalid_argument(EDEP_WEAKER_POLICY)
                        );
                        if (dep_pack.upgrade_policy == upgrade_policy_arbitrary()) {
                            assert!(
                                dep.account == publish_address,
                                error::invalid_argument(EDEP_ARBITRARY_NOT_SAME_ADDRESS)
                            )
                        };
                        // Add allowed deps
                        let account = dep.account;
                        let k = 0;
                        let r = vector::length(&dep_pack.modules);
                        while (k < r) {
                            let module_name = vector::borrow(&dep_pack.modules, k).name;
                            vector::push_back(&mut allowed_module_deps, AllowedDep { account, module_name });
                            k = k + 1;
                        };
                        true
                    } else {
                        false
                    }
                });
                assert!(found, error::not_found(EPACKAGE_DEP_MISSING));
            };
        });
        allowed_module_deps
    }
}
```


<a id="0x1_code_is_policy_exempted_address"></a>

## Function `is_policy_exempted_address`

Core addresses which are exempted from the check that their policy matches the referring package. Without
this exemption, it would not be possible to define an immutable package based on the core system, which
requires to be upgradable for maintenance and evolution, and is configured to be `compatible`.


```move
module 0x1::code {
    fun is_policy_exempted_address(addr: address): bool
}
```


##### Implementation


```move
module 0x1::code {
    fun is_policy_exempted_address(addr: address): bool {
        addr == @1 || addr == @2 || addr == @3 || addr == @4 || addr == @5 ||
            addr == @6 || addr == @7 || addr == @8 || addr == @9 || addr == @10
    }
}
```


<a id="0x1_code_get_module_names"></a>

## Function `get_module_names`

Get the names of the modules in a package.


```move
module 0x1::code {
    fun get_module_names(pack: &code::PackageMetadata): vector<string::String>
}
```


##### Implementation


```move
module 0x1::code {
    fun get_module_names(pack: &PackageMetadata): vector<String> {
        let module_names = vector::empty();
        vector::for_each_ref(&pack.modules, |pack_module| {
            let pack_module: &ModuleMetadata = pack_module;
            vector::push_back(&mut module_names, pack_module.name);
        });
        module_names
    }
}
```


<a id="0x1_code_request_publish"></a>

## Function `request_publish`

Native function to initiate module loading


```move
module 0x1::code {
    fun request_publish(owner: address, expected_modules: vector<string::String>, bundle: vector<vector<u8>>, policy: u8)
}
```


##### Implementation


```move
module 0x1::code {
    native fun request_publish(
        owner: address,
        expected_modules: vector<String>,
        bundle: vector<vector<u8>>,
        policy: u8
    );
}
```


<a id="0x1_code_request_publish_with_allowed_deps"></a>

## Function `request_publish_with_allowed_deps`

Native function to initiate module loading, including a list of allowed dependencies.


```move
module 0x1::code {
    fun request_publish_with_allowed_deps(owner: address, expected_modules: vector<string::String>, allowed_deps: vector<code::AllowedDep>, bundle: vector<vector<u8>>, policy: u8)
}
```


##### Implementation


```move
module 0x1::code {
    native fun request_publish_with_allowed_deps(
        owner: address,
        expected_modules: vector<String>,
        allowed_deps: vector<AllowedDep>,
        bundle: vector<vector<u8>>,
        policy: u8
    );
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
<td>Updating a package should fail if the user is not the owner of it.</td>
<td>Critical</td>
<td>The publish_package function may only be able to update the package if the signer is the actual owner of the package.</td>
<td>The Aptos upgrade native functions have been manually audited.</td>
</tr>

<tr>
<td>2</td>
<td>The arbitrary upgrade policy should never be used.</td>
<td>Critical</td>
<td>There should never be a pass of an arbitrary upgrade policy to the request_publish native function.</td>
<td>Manually audited that it aborts if package.upgrade_policy.policy &#61;&#61; 0.</td>
</tr>

<tr>
<td>3</td>
<td>Should perform accurate compatibility checks when the policy indicates compatibility, ensuring it meets the required conditions.</td>
<td>Critical</td>
<td>Specifies if it should perform compatibility checks for upgrades. The check only passes if a new module has (a) the same public functions, and (b) for existing resources, no layout change.</td>
<td>The Move upgradability patterns have been manually audited.</td>
</tr>

<tr>
<td>4</td>
<td>Package upgrades should abide by policy change rules. In particular, The new upgrade policy must be equal to or stricter when compared to the old one. The original upgrade policy must not be immutable. The new package must contain all modules contained in the old package.</td>
<td>Medium</td>
<td>A package may only be updated using the publish_package function when the check_upgradability function returns true.</td>
<td>This is audited by a manual review of the check_upgradability patterns.</td>
</tr>

<tr>
<td>5</td>
<td>The upgrade policy of a package must not exceed the strictness level imposed by its dependencies.</td>
<td>Medium</td>
<td>The upgrade_policy of a package may only be less than its dependencies throughout the upgrades. In addition, the native code properly restricts the use of dependencies outside the passed&#45;in metadata.</td>
<td>This has been manually audited.</td>
</tr>

<tr>
<td>6</td>
<td>The extension for package metadata is currently unused.</td>
<td>Medium</td>
<td>The extension field in PackageMetadata should be unused.</td>
<td>Data invariant on the extension field has been manually audited.</td>
</tr>

<tr>
<td>7</td>
<td>The upgrade number of a package increases incrementally in a monotonic manner with each subsequent upgrade.</td>
<td>Low</td>
<td>On each upgrade of a particular package, the publish_package function updates the upgrade_number for that package.</td>
<td>Post condition on upgrade_number has been manually audited.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::code {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_1_initialize"></a>

### Function `initialize`


```move
module 0x1::code {
    fun initialize(aptos_framework: &signer, package_owner: &signer, metadata: code::PackageMetadata)
}
```



```move
module 0x1::code {
    let aptos_addr = signer::address_of(aptos_framework);
    let owner_addr = signer::address_of(package_owner);
    aborts_if !system_addresses::is_aptos_framework_address(aptos_addr);
    ensures exists<PackageRegistry>(owner_addr);
}
```


<a id="@Specification_1_publish_package"></a>

### Function `publish_package`


```move
module 0x1::code {
    public fun publish_package(owner: &signer, pack: code::PackageMetadata, code: vector<vector<u8>>)
}
```



```move
module 0x1::code {
    pragma aborts_if_is_partial;
    let addr = signer::address_of(owner);
    modifies global<PackageRegistry>(addr);
    aborts_if pack.upgrade_policy.policy <= upgrade_policy_arbitrary().policy;
}
```


<a id="@Specification_1_freeze_code_object"></a>

### Function `freeze_code_object`


```move
module 0x1::code {
    public fun freeze_code_object(publisher: &signer, code_object: object::Object<code::PackageRegistry>)
}
```



```move
module 0x1::code {
    pragma aborts_if_is_partial;
    let code_object_addr = code_object.inner;
    aborts_if !exists<object::ObjectCore>(code_object_addr);
    aborts_if !exists<PackageRegistry>(code_object_addr);
    aborts_if !object::is_owner(code_object, signer::address_of(publisher));
    modifies global<PackageRegistry>(code_object_addr);
}
```


<a id="@Specification_1_publish_package_txn"></a>

### Function `publish_package_txn`


```move
module 0x1::code {
    public entry fun publish_package_txn(owner: &signer, metadata_serialized: vector<u8>, code: vector<vector<u8>>)
}
```



```move
module 0x1::code {
    pragma verify = false;
}
```


<a id="@Specification_1_check_upgradability"></a>

### Function `check_upgradability`


```move
module 0x1::code {
    fun check_upgradability(old_pack: &code::PackageMetadata, new_pack: &code::PackageMetadata, new_modules: &vector<string::String>)
}
```



```move
module 0x1::code {
    pragma aborts_if_is_partial;
    aborts_if old_pack.upgrade_policy.policy >= upgrade_policy_immutable().policy;
    aborts_if !can_change_upgrade_policy_to(old_pack.upgrade_policy, new_pack.upgrade_policy);
}
```


<a id="@Specification_1_check_coexistence"></a>

### Function `check_coexistence`


```move
module 0x1::code {
    fun check_coexistence(old_pack: &code::PackageMetadata, new_modules: &vector<string::String>)
}
```



```move
module 0x1::code {
    pragma verify = false;
}
```


<a id="@Specification_1_check_dependencies"></a>

### Function `check_dependencies`


```move
module 0x1::code {
    fun check_dependencies(publish_address: address, pack: &code::PackageMetadata): vector<code::AllowedDep>
}
```



```move
module 0x1::code {
    pragma verify = false;
}
```


<a id="@Specification_1_get_module_names"></a>

### Function `get_module_names`


```move
module 0x1::code {
    fun get_module_names(pack: &code::PackageMetadata): vector<string::String>
}
```



```move
module 0x1::code {
    pragma opaque;
    aborts_if [abstract] false;
    ensures [abstract] len(result) == len(pack.modules);
    ensures [abstract] forall i in 0..len(result): result[i] == pack.modules[i].name;
}
```


<a id="@Specification_1_request_publish"></a>

### Function `request_publish`


```move
module 0x1::code {
    fun request_publish(owner: address, expected_modules: vector<string::String>, bundle: vector<vector<u8>>, policy: u8)
}
```



```move
module 0x1::code {
    pragma opaque;
}
```


<a id="@Specification_1_request_publish_with_allowed_deps"></a>

### Function `request_publish_with_allowed_deps`


```move
module 0x1::code {
    fun request_publish_with_allowed_deps(owner: address, expected_modules: vector<string::String>, allowed_deps: vector<code::AllowedDep>, bundle: vector<vector<u8>>, policy: u8)
}
```



```move
module 0x1::code {
    pragma opaque;
}
```

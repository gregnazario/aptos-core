
<a id="0x1_storage_gas"></a>

# Module `0x1::storage_gas`

Gas parameters for global storage.


<a id="@General_overview_sections_0"></a>

## General overview sections


[Definitions](#definitions)

&#42; [Utilization dimensions](#utilization&#45;dimensions)
&#42; [Utilization ratios](#utilization&#45;ratios)
&#42; [Gas curve lookup](#gas&#45;curve&#45;lookup)
&#42; [Item&#45;wise operations](#item&#45;wise&#45;operations)
&#42; [Byte&#45;wise operations](#byte&#45;wise&#45;operations)

[Function dependencies](#function&#45;dependencies)

&#42; [Initialization](#initialization)
&#42; [Reconfiguration](#reconfiguration)
&#42; [Setting configurations](#setting&#45;configurations)


<a id="@Definitions_1"></a>

## Definitions



<a id="@Utilization_dimensions_2"></a>

### Utilization dimensions


Global storage gas fluctuates each epoch based on total utilization,
which is defined across two dimensions:

1. The number of &quot;items&quot; in global storage.
2. The number of bytes in global storage.

&quot;Items&quot; include:

1. Resources having the `key` attribute, which have been moved into
global storage via a `move_to()` operation.
2.  Table entries.


<a id="@Utilization_ratios_3"></a>

### Utilization ratios


`initialize()` sets an arbitrary &quot;target&quot; utilization for both
item&#45;wise and byte&#45;wise storage, then each epoch, gas parameters are
reconfigured based on the &quot;utilization ratio&quot; for each of the two
utilization dimensions. The utilization ratio for a given dimension,
either item&#45;wise or byte&#45;wise, is taken as the quotient of actual
utilization and target utilization. For example, given a 500 GB
target and 250 GB actual utilization, the byte&#45;wise utilization
ratio is 50%.

See `base_8192_exponential_curve()` for mathematical definitions.


<a id="@Gas_curve_lookup_4"></a>

### Gas curve lookup


The utilization ratio in a given epoch is used as a lookup value in
a Eulerian approximation to an exponential curve, known as a
`GasCurve`, which is defined in `base_8192_exponential_curve()`,
based on a minimum gas charge and a maximum gas charge.

The minimum gas charge and maximum gas charge at the endpoints of
the curve are set in `initialize()`, and correspond to the following
operations defined in `StorageGas`:

1. Per&#45;item read
2. Per&#45;item create
3. Per&#45;item write
4. Per&#45;byte read
5. Per&#45;byte create
6. Per&#45;byte write

For example, if the byte&#45;wise utilization ratio is 50%, then
per&#45;byte reads will charge the minimum per&#45;byte gas cost, plus
1.09% of the difference between the maximum and the minimum cost.
See `base_8192_exponential_curve()` for a supporting calculation.


<a id="@Item&#45;wise_operations_5"></a>

### Item&#45;wise operations


1. Per&#45;item read gas is assessed whenever an item is read from
global storage via `borrow_global<T>()` or via a table entry read
operation.
2. Per&#45;item create gas is assessed whenever an item is created in
global storage via `move_to<T>()` or via a table entry creation
operation.
3. Per&#45;item write gas is assessed whenever an item is overwritten in
global storage via `borrow_global_mut<T>` or via a table entry
mutation operation.


<a id="@Byte&#45;wise_operations_6"></a>

### Byte&#45;wise operations


Byte&#45;wise operations are assessed in a manner similar to per&#45;item
operations, but account for the number of bytes affected by the
given operation. Notably, this number denotes the total number of
bytes in an &#42;entire item&#42;.

For example, if an operation mutates a `u8` field in a resource that
has 5 other `u128` fields, the per&#45;byte gas write cost will account
for $(5 &#42; 128) / 8 &#43; 1 &#61; 81$ bytes. Vectors are similarly treated
as fields.


<a id="@Function_dependencies_7"></a>

## Function dependencies


The below dependency chart uses `mermaid.js` syntax, which can be
automatically rendered into a diagram (depending on the browser)
when viewing the documentation file generated from source code. If
a browser renders the diagrams with coloring that makes it difficult
to read, try a different browser.


<a id="@Initialization_8"></a>

### Initialization


```mermaid

flowchart LR

initialize &#45;&#45;&gt; base_8192_exponential_curve
base_8192_exponential_curve &#45;&#45;&gt; new_gas_curve
base_8192_exponential_curve &#45;&#45;&gt; new_point
new_gas_curve &#45;&#45;&gt; validate_points

```


<a id="@Reconfiguration_9"></a>

### Reconfiguration


```mermaid

flowchart LR

calculate_gas &#45;&#45;&gt; Interpolate %% capitalized
calculate_read_gas &#45;&#45;&gt; calculate_gas
calculate_create_gas &#45;&#45;&gt; calculate_gas
calculate_write_gas &#45;&#45;&gt; calculate_gas
on_reconfig &#45;&#45;&gt; calculate_read_gas
on_reconfig &#45;&#45;&gt; calculate_create_gas
on_reconfig &#45;&#45;&gt; calculate_write_gas
reconfiguration::reconfigure &#45;&#45;&gt; on_reconfig

```

Here, the function `interpolate()` is spelled `Interpolate` because
`interpolate` is a reserved word in `mermaid.js`.


<a id="@Setting_configurations_10"></a>

### Setting configurations


```mermaid

flowchart LR

gas_schedule::set_storage_gas_config &#45;&#45;&gt; set_config

```


<a id="@Complete_docgen_index_11"></a>

## Complete docgen index


The below index is automatically generated from source code:


-  [General overview sections](#@General_overview_sections_0)
-  [Definitions](#@Definitions_1)
    -  [Utilization dimensions](#@Utilization_dimensions_2)
    -  [Utilization ratios](#@Utilization_ratios_3)
    -  [Gas curve lookup](#@Gas_curve_lookup_4)
    -  [Item&#45;wise operations](#@Item&#45;wise_operations_5)
    -  [Byte&#45;wise operations](#@Byte&#45;wise_operations_6)
-  [Function dependencies](#@Function_dependencies_7)
    -  [Initialization](#@Initialization_8)
    -  [Reconfiguration](#@Reconfiguration_9)
    -  [Setting configurations](#@Setting_configurations_10)
-  [Complete docgen index](#@Complete_docgen_index_11)
-  [Resource `StorageGas`](#0x1_storage_gas_StorageGas)
-  [Struct `Point`](#0x1_storage_gas_Point)
-  [Struct `UsageGasConfig`](#0x1_storage_gas_UsageGasConfig)
-  [Struct `GasCurve`](#0x1_storage_gas_GasCurve)
-  [Resource `StorageGasConfig`](#0x1_storage_gas_StorageGasConfig)
-  [Constants](#@Constants_12)
-  [Function `base_8192_exponential_curve`](#0x1_storage_gas_base_8192_exponential_curve)
    -  [Function definition](#@Function_definition_13)
    -  [Example](#@Example_14)
    -  [Utilization multipliers](#@Utilization_multipliers_15)
-  [Function `new_point`](#0x1_storage_gas_new_point)
-  [Function `new_gas_curve`](#0x1_storage_gas_new_gas_curve)
-  [Function `new_usage_gas_config`](#0x1_storage_gas_new_usage_gas_config)
-  [Function `new_storage_gas_config`](#0x1_storage_gas_new_storage_gas_config)
-  [Function `set_config`](#0x1_storage_gas_set_config)
-  [Function `initialize`](#0x1_storage_gas_initialize)
-  [Function `validate_points`](#0x1_storage_gas_validate_points)
-  [Function `calculate_gas`](#0x1_storage_gas_calculate_gas)
-  [Function `interpolate`](#0x1_storage_gas_interpolate)
-  [Function `calculate_read_gas`](#0x1_storage_gas_calculate_read_gas)
-  [Function `calculate_create_gas`](#0x1_storage_gas_calculate_create_gas)
-  [Function `calculate_write_gas`](#0x1_storage_gas_calculate_write_gas)
-  [Function `on_reconfig`](#0x1_storage_gas_on_reconfig)
-  [Specification](#@Specification_16)
    -  [Struct `Point`](#@Specification_16_Point)
    -  [Struct `UsageGasConfig`](#@Specification_16_UsageGasConfig)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Struct `GasCurve`](#@Specification_16_GasCurve)
    -  [Function `base_8192_exponential_curve`](#@Specification_16_base_8192_exponential_curve)
    -  [Function `new_point`](#@Specification_16_new_point)
    -  [Function `new_gas_curve`](#@Specification_16_new_gas_curve)
    -  [Function `new_usage_gas_config`](#@Specification_16_new_usage_gas_config)
    -  [Function `new_storage_gas_config`](#@Specification_16_new_storage_gas_config)
    -  [Function `set_config`](#@Specification_16_set_config)
    -  [Function `initialize`](#@Specification_16_initialize)
    -  [Function `validate_points`](#@Specification_16_validate_points)
    -  [Function `calculate_gas`](#@Specification_16_calculate_gas)
    -  [Function `interpolate`](#@Specification_16_interpolate)
    -  [Function `on_reconfig`](#@Specification_16_on_reconfig)


```move
module 0x1::storage_gas {
    use 0x1::error;
    use 0x1::state_storage;
    use 0x1::system_addresses;
}
```


<a id="0x1_storage_gas_StorageGas"></a>

## Resource `StorageGas`

Storage parameters, reconfigured each epoch.

Parameters are updated during reconfiguration via
`on_reconfig()`, based on storage utilization at the beginning
of the epoch in which the reconfiguration transaction is
executed. The gas schedule derived from these parameters will
then be used to calculate gas for the entirety of the
following epoch, such that the data is one epoch older than
ideal. Notably, however, per this approach, the virtual machine
does not need to reload gas parameters after the
first transaction of an epoch.


```move
module 0x1::storage_gas {
    struct StorageGas has key
}
```


##### Fields


<dl>
<dt>
`per_item_read: u64`
</dt>
<dd>
 Cost to read an item from global storage.
</dd>
<dt>
`per_item_create: u64`
</dt>
<dd>
 Cost to create an item in global storage.
</dd>
<dt>
`per_item_write: u64`
</dt>
<dd>
 Cost to overwrite an item in global storage.
</dd>
<dt>
`per_byte_read: u64`
</dt>
<dd>
 Cost to read a byte from global storage.
</dd>
<dt>
`per_byte_create: u64`
</dt>
<dd>
 Cost to create a byte in global storage.
</dd>
<dt>
`per_byte_write: u64`
</dt>
<dd>
 Cost to overwrite a byte in global storage.
</dd>
</dl>


<a id="0x1_storage_gas_Point"></a>

## Struct `Point`

A point in a Eulerian curve approximation, with each coordinate
given in basis points:

&#124; Field value &#124; Percentage &#124;
&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;
&#124; `1`         &#124; 00.01 %    &#124;
&#124; `10`        &#124; 00.10 %    &#124;
&#124; `100`       &#124; 01.00 %    &#124;
&#124; `1000`      &#124; 10.00 %    &#124;


```move
module 0x1::storage_gas {
    struct Point has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`x: u64`
</dt>
<dd>
 x&#45;coordinate basis points, corresponding to utilization
 ratio in `base_8192_exponential_curve()`.
</dd>
<dt>
`y: u64`
</dt>
<dd>
 y&#45;coordinate basis points, corresponding to utilization
 multiplier in `base_8192_exponential_curve()`.
</dd>
</dl>


<a id="0x1_storage_gas_UsageGasConfig"></a>

## Struct `UsageGasConfig`

A gas configuration for either per&#45;item or per&#45;byte costs.

Contains a target usage amount, as well as a Eulerian
approximation of an exponential curve for reads, creations, and
overwrites. See `StorageGasConfig`.


```move
module 0x1::storage_gas {
    struct UsageGasConfig has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`target_usage: u64`
</dt>
<dd>

</dd>
<dt>
`read_curve: storage_gas::GasCurve`
</dt>
<dd>

</dd>
<dt>
`create_curve: storage_gas::GasCurve`
</dt>
<dd>

</dd>
<dt>
`write_curve: storage_gas::GasCurve`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_storage_gas_GasCurve"></a>

## Struct `GasCurve`

Eulerian approximation of an exponential curve.

Assumes the following endpoints:

&#42; $(x_0, y_0) &#61; (0, 0)$
&#42; $(x_f, y_f) &#61; (10000, 10000)$

Intermediate points must satisfy:

1. $x_i &gt; x_&#123;i &#45; 1&#125;$ ( $x$ is strictly increasing).
2. $0 \leq x_i \leq 10000$ ( $x$ is between 0 and 10000).
3. $y_i \geq y_&#123;i &#45; 1&#125;$ ( $y$ is non&#45;decreasing).
4. $0 \leq y_i \leq 10000$ ( $y$ is between 0 and 10000).

Lookup between two successive points is calculated via linear
interpolation, e.g., as if there were a straight line between
them.

See `base_8192_exponential_curve()`.


```move
module 0x1::storage_gas {
    struct GasCurve has copy, drop, store
}
```


##### Fields


<dl>
<dt>
`min_gas: u64`
</dt>
<dd>

</dd>
<dt>
`max_gas: u64`
</dt>
<dd>

</dd>
<dt>
`points: vector<storage_gas::Point>`
</dt>
<dd>

</dd>
</dl>


<a id="0x1_storage_gas_StorageGasConfig"></a>

## Resource `StorageGasConfig`

Gas configurations for per&#45;item and per&#45;byte prices.


```move
module 0x1::storage_gas {
    struct StorageGasConfig has copy, drop, key
}
```


##### Fields


<dl>
<dt>
`item_config: storage_gas::UsageGasConfig`
</dt>
<dd>
 Per&#45;item gas configuration.
</dd>
<dt>
`byte_config: storage_gas::UsageGasConfig`
</dt>
<dd>
 Per&#45;byte gas configuration.
</dd>
</dl>


<a id="@Constants_12"></a>

## Constants


<a id="0x1_storage_gas_MAX_U64"></a>



```move
module 0x1::storage_gas {
    const MAX_U64: u64 = 18446744073709551615;
}
```


<a id="0x1_storage_gas_BASIS_POINT_DENOMINATION"></a>



```move
module 0x1::storage_gas {
    const BASIS_POINT_DENOMINATION: u64 = 10000;
}
```


<a id="0x1_storage_gas_EINVALID_GAS_RANGE"></a>



```move
module 0x1::storage_gas {
    const EINVALID_GAS_RANGE: u64 = 2;
}
```


<a id="0x1_storage_gas_EINVALID_MONOTONICALLY_NON_DECREASING_CURVE"></a>



```move
module 0x1::storage_gas {
    const EINVALID_MONOTONICALLY_NON_DECREASING_CURVE: u64 = 5;
}
```


<a id="0x1_storage_gas_EINVALID_POINT_RANGE"></a>



```move
module 0x1::storage_gas {
    const EINVALID_POINT_RANGE: u64 = 6;
}
```


<a id="0x1_storage_gas_ESTORAGE_GAS"></a>



```move
module 0x1::storage_gas {
    const ESTORAGE_GAS: u64 = 1;
}
```


<a id="0x1_storage_gas_ESTORAGE_GAS_CONFIG"></a>



```move
module 0x1::storage_gas {
    const ESTORAGE_GAS_CONFIG: u64 = 0;
}
```


<a id="0x1_storage_gas_ETARGET_USAGE_TOO_BIG"></a>



```move
module 0x1::storage_gas {
    const ETARGET_USAGE_TOO_BIG: u64 = 4;
}
```


<a id="0x1_storage_gas_EZERO_TARGET_USAGE"></a>



```move
module 0x1::storage_gas {
    const EZERO_TARGET_USAGE: u64 = 3;
}
```


<a id="0x1_storage_gas_base_8192_exponential_curve"></a>

## Function `base_8192_exponential_curve`

Default exponential curve having base 8192.


<a id="@Function_definition_13"></a>

### Function definition


Gas price as a function of utilization ratio is defined as:

$$g(u_r) &#61; g_&#123;min&#125; &#43; \frac&#123;(b^&#123;u_r&#125; &#45; 1)&#125;&#123;b &#45; 1&#125; \Delta_g$$

$$g(u_r) &#61; g_&#123;min&#125; &#43; u_m \Delta_g$$

&#124; Variable                            &#124; Description            &#124;
&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;
&#124; $g_&#123;min&#125;$                           &#124; `min_gas`              &#124;
&#124; $g_&#123;max&#125;$                           &#124; `max_gas`              &#124;
&#124; $\Delta_&#123;g&#125; &#61; g_&#123;max&#125; &#45; g_&#123;min&#125;$    &#124; Gas delta              &#124;
&#124; $u$                                 &#124; Utilization            &#124;
&#124; $u_t$                               &#124; Target utilization     &#124;
&#124; $u_r &#61; u / u_t$                     &#124; Utilization ratio      &#124;
&#124; $u_m &#61; \frac&#123;(b^&#123;u_r&#125; &#45; 1)&#125;&#123;b &#45; 1&#125;$ &#124; Utilization multiplier &#124;
&#124; $b &#61; 8192$                          &#124; Exponent base          &#124;


<a id="@Example_14"></a>

### Example


Hence for a utilization ratio of 50% ( $u_r &#61; 0.5$ ):

$$g(0.5) &#61; g_&#123;min&#125; &#43; \frac&#123;8192^&#123;0.5&#125; &#45; 1&#125;&#123;8192 &#45; 1&#125; \Delta_g$$

$$g(0.5) \approx g_&#123;min&#125; &#43; 0.0109 \Delta_g$$

Which means that the price above `min_gas` is approximately
1.09% of the difference between `max_gas` and `min_gas`.


<a id="@Utilization_multipliers_15"></a>

### Utilization multipliers


&#124; $u_r$ &#124; $u_m$ (approximate) &#124;
&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;
&#124; 10%   &#124; 0.02%               &#124;
&#124; 20%   &#124; 0.06%               &#124;
&#124; 30%   &#124; 0.17%               &#124;
&#124; 40%   &#124; 0.44%               &#124;
&#124; 50%   &#124; 1.09%               &#124;
&#124; 60%   &#124; 2.71%               &#124;
&#124; 70%   &#124; 6.69%               &#124;
&#124; 80%   &#124; 16.48%              &#124;
&#124; 90%   &#124; 40.61%              &#124;
&#124; 95%   &#124; 63.72%              &#124;
&#124; 99%   &#124; 91.38%              &#124;


```move
module 0x1::storage_gas {
    public fun base_8192_exponential_curve(min_gas: u64, max_gas: u64): storage_gas::GasCurve
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public fun base_8192_exponential_curve(min_gas: u64, max_gas: u64): GasCurve {
        new_gas_curve(min_gas, max_gas,
            vector[
                new_point(1000, 2),
                new_point(2000, 6),
                new_point(3000, 17),
                new_point(4000, 44),
                new_point(5000, 109),
                new_point(6000, 271),
                new_point(7000, 669),
                new_point(8000, 1648),
                new_point(9000, 4061),
                new_point(9500, 6372),
                new_point(9900, 9138),
            ]
        )
    }
}
```


<a id="0x1_storage_gas_new_point"></a>

## Function `new_point`



```move
module 0x1::storage_gas {
    public fun new_point(x: u64, y: u64): storage_gas::Point
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public fun new_point(x: u64, y: u64): Point {
        assert!(
            x <= BASIS_POINT_DENOMINATION && y <= BASIS_POINT_DENOMINATION,
            error::invalid_argument(EINVALID_POINT_RANGE)
        );
        Point { x, y }
    }
}
```


<a id="0x1_storage_gas_new_gas_curve"></a>

## Function `new_gas_curve`



```move
module 0x1::storage_gas {
    public fun new_gas_curve(min_gas: u64, max_gas: u64, points: vector<storage_gas::Point>): storage_gas::GasCurve
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public fun new_gas_curve(min_gas: u64, max_gas: u64, points: vector<Point>): GasCurve {
        assert!(max_gas >= min_gas, error::invalid_argument(EINVALID_GAS_RANGE));
        assert!(max_gas <= MAX_U64 / BASIS_POINT_DENOMINATION, error::invalid_argument(EINVALID_GAS_RANGE));
        validate_points(&points);
        GasCurve {
            min_gas,
            max_gas,
            points
        }
    }
}
```


<a id="0x1_storage_gas_new_usage_gas_config"></a>

## Function `new_usage_gas_config`



```move
module 0x1::storage_gas {
    public fun new_usage_gas_config(target_usage: u64, read_curve: storage_gas::GasCurve, create_curve: storage_gas::GasCurve, write_curve: storage_gas::GasCurve): storage_gas::UsageGasConfig
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public fun new_usage_gas_config(target_usage: u64, read_curve: GasCurve, create_curve: GasCurve, write_curve: GasCurve): UsageGasConfig {
        assert!(target_usage > 0, error::invalid_argument(EZERO_TARGET_USAGE));
        assert!(target_usage <= MAX_U64 / BASIS_POINT_DENOMINATION, error::invalid_argument(ETARGET_USAGE_TOO_BIG));
        UsageGasConfig {
            target_usage,
            read_curve,
            create_curve,
            write_curve,
        }
    }
}
```


<a id="0x1_storage_gas_new_storage_gas_config"></a>

## Function `new_storage_gas_config`



```move
module 0x1::storage_gas {
    public fun new_storage_gas_config(item_config: storage_gas::UsageGasConfig, byte_config: storage_gas::UsageGasConfig): storage_gas::StorageGasConfig
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public fun new_storage_gas_config(item_config: UsageGasConfig, byte_config: UsageGasConfig): StorageGasConfig {
        StorageGasConfig {
            item_config,
            byte_config
        }
    }
}
```


<a id="0x1_storage_gas_set_config"></a>

## Function `set_config`



```move
module 0x1::storage_gas {
    public(friend) fun set_config(aptos_framework: &signer, config: storage_gas::StorageGasConfig)
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public(friend) fun set_config(aptos_framework: &signer, config: StorageGasConfig) acquires StorageGasConfig {
        system_addresses::assert_aptos_framework(aptos_framework);
        *borrow_global_mut<StorageGasConfig>(@aptos_framework) = config;
    }
}
```


<a id="0x1_storage_gas_initialize"></a>

## Function `initialize`

Initialize per&#45;item and per&#45;byte gas prices.

Target utilization is set to 2 billion items and 1 TB.

`GasCurve` endpoints are initialized as follows:

&#124; Data style &#124; Operation &#124; Minimum gas &#124; Maximum gas &#124;
&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#45;&#124;
&#124; Per item   &#124; Read      &#124; 300K        &#124; 300K &#42; 100  &#124;
&#124; Per item   &#124; Create    &#124; 300k        &#124; 300k &#42; 100    &#124;
&#124; Per item   &#124; Write     &#124; 300K        &#124; 300K &#42; 100  &#124;
&#124; Per byte   &#124; Read      &#124; 300         &#124; 300 &#42; 100   &#124;
&#124; Per byte   &#124; Create    &#124; 5K          &#124; 5K &#42; 100    &#124;
&#124; Per byte   &#124; Write     &#124; 5K          &#124; 5K &#42; 100    &#124;

`StorageGas` values are additionally initialized, but per
`on_reconfig()`, they will be reconfigured for each subsequent
epoch after initialization.

See `base_8192_exponential_curve()` fore more information on
target utilization.


```move
module 0x1::storage_gas {
    public fun initialize(aptos_framework: &signer)
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public fun initialize(aptos_framework: &signer) {
        system_addresses::assert_aptos_framework(aptos_framework);
        assert!(
            !exists<StorageGasConfig>(@aptos_framework),
            error::already_exists(ESTORAGE_GAS_CONFIG)
        );

        let k: u64 = 1000;
        let m: u64 = 1000 * 1000;

        let item_config = UsageGasConfig {
            target_usage: 2 * k * m, // 2 billion
            read_curve: base_8192_exponential_curve(300 * k, 300 * k * 100),
            create_curve: base_8192_exponential_curve(300 * k, 300 * k * 100),
            write_curve: base_8192_exponential_curve(300 * k, 300 * k * 100),
        };
        let byte_config = UsageGasConfig {
            target_usage: 1 * m * m, // 1TB
            read_curve: base_8192_exponential_curve(300, 300 * 100),
            create_curve: base_8192_exponential_curve(5 * k,  5 * k * 100),
            write_curve: base_8192_exponential_curve(5 * k,  5 * k * 100),
        };
        move_to(aptos_framework, StorageGasConfig {
            item_config,
            byte_config,
        });

        assert!(
            !exists<StorageGas>(@aptos_framework),
            error::already_exists(ESTORAGE_GAS)
        );
        move_to(aptos_framework, StorageGas {
            per_item_read: 300 * k,
            per_item_create: 5 * m,
            per_item_write: 300 * k,
            per_byte_read: 300,
            per_byte_create: 5 * k,
            per_byte_write: 5 * k,
        });
    }
}
```


<a id="0x1_storage_gas_validate_points"></a>

## Function `validate_points`



```move
module 0x1::storage_gas {
    fun validate_points(points: &vector<storage_gas::Point>)
}
```


##### Implementation


```move
module 0x1::storage_gas {
    fun validate_points(points: &vector<Point>) {
        let len = vector::length(points);
        spec {
            assume len < MAX_U64;
        };
        let i = 0;
        while ({
            spec {
                invariant forall j in 0..i: {
                    let cur = if (j == 0) { Point { x: 0, y: 0 } } else { points[j - 1] };
                    let next = if (j == len) { Point { x: BASIS_POINT_DENOMINATION, y: BASIS_POINT_DENOMINATION } } else { points[j] };
                    cur.x < next.x && cur.y <= next.y
                };
            };
            i <= len
        }) {
            let cur = if (i == 0) { &Point { x: 0, y: 0 } } else { vector::borrow(points, i - 1) };
            let next = if (i == len) { &Point { x: BASIS_POINT_DENOMINATION, y: BASIS_POINT_DENOMINATION } } else { vector::borrow(points, i) };
            assert!(cur.x < next.x && cur.y <= next.y, error::invalid_argument(EINVALID_MONOTONICALLY_NON_DECREASING_CURVE));
            i = i + 1;
        }
    }
}
```


<a id="0x1_storage_gas_calculate_gas"></a>

## Function `calculate_gas`



```move
module 0x1::storage_gas {
    fun calculate_gas(max_usage: u64, current_usage: u64, curve: &storage_gas::GasCurve): u64
}
```


##### Implementation


```move
module 0x1::storage_gas {
    fun calculate_gas(max_usage: u64, current_usage: u64, curve: &GasCurve): u64 {
        let capped_current_usage = if (current_usage > max_usage) max_usage else current_usage;
        let points = &curve.points;
        let num_points = vector::length(points);
        let current_usage_bps = capped_current_usage * BASIS_POINT_DENOMINATION / max_usage;

        // Check the corner case that current_usage_bps drops before the first point.
        let (left, right) = if (num_points == 0) {
            (&Point { x: 0, y: 0 }, &Point { x: BASIS_POINT_DENOMINATION, y: BASIS_POINT_DENOMINATION })
        } else if (current_usage_bps < vector::borrow(points, 0).x) {
            (&Point { x: 0, y: 0 }, vector::borrow(points, 0))
        } else if (vector::borrow(points, num_points - 1).x <= current_usage_bps) {
            (vector::borrow(points, num_points - 1), &Point { x: BASIS_POINT_DENOMINATION, y: BASIS_POINT_DENOMINATION })
        } else {
            let (i, j) = (0, num_points - 2);
            while ({
                spec {
                    invariant i <= j;
                    invariant j < num_points - 1;
                    invariant points[i].x <= current_usage_bps;
                    invariant current_usage_bps < points[j + 1].x;
                };
                i < j
            }) {
                let mid = j - (j - i) / 2;
                if (current_usage_bps < vector::borrow(points, mid).x) {
                    spec {
                        // j is strictly decreasing.
                        assert mid - 1 < j;
                    };
                    j = mid - 1;
                } else {
                    spec {
                        // i is strictly increasing.
                        assert i < mid;
                    };
                    i = mid;
                };
            };
            (vector::borrow(points, i), vector::borrow(points, i + 1))
        };
        let y_interpolated = interpolate(left.x, right.x, left.y, right.y, current_usage_bps);
        interpolate(0, BASIS_POINT_DENOMINATION, curve.min_gas, curve.max_gas, y_interpolated)
    }
}
```


<a id="0x1_storage_gas_interpolate"></a>

## Function `interpolate`



```move
module 0x1::storage_gas {
    fun interpolate(x0: u64, x1: u64, y0: u64, y1: u64, x: u64): u64
}
```


##### Implementation


```move
module 0x1::storage_gas {
    fun interpolate(x0: u64, x1: u64, y0: u64, y1: u64, x: u64): u64 {
        y0 + (x - x0) * (y1 - y0) / (x1 - x0)
    }
}
```


<a id="0x1_storage_gas_calculate_read_gas"></a>

## Function `calculate_read_gas`



```move
module 0x1::storage_gas {
    fun calculate_read_gas(config: &storage_gas::UsageGasConfig, usage: u64): u64
}
```


##### Implementation


```move
module 0x1::storage_gas {
    fun calculate_read_gas(config: &UsageGasConfig, usage: u64): u64 {
        calculate_gas(config.target_usage, usage, &config.read_curve)
    }
}
```


<a id="0x1_storage_gas_calculate_create_gas"></a>

## Function `calculate_create_gas`



```move
module 0x1::storage_gas {
    fun calculate_create_gas(config: &storage_gas::UsageGasConfig, usage: u64): u64
}
```


##### Implementation


```move
module 0x1::storage_gas {
    fun calculate_create_gas(config: &UsageGasConfig, usage: u64): u64 {
        calculate_gas(config.target_usage, usage, &config.create_curve)
    }
}
```


<a id="0x1_storage_gas_calculate_write_gas"></a>

## Function `calculate_write_gas`



```move
module 0x1::storage_gas {
    fun calculate_write_gas(config: &storage_gas::UsageGasConfig, usage: u64): u64
}
```


##### Implementation


```move
module 0x1::storage_gas {
    fun calculate_write_gas(config: &UsageGasConfig, usage: u64): u64 {
        calculate_gas(config.target_usage, usage, &config.write_curve)
    }
}
```


<a id="0x1_storage_gas_on_reconfig"></a>

## Function `on_reconfig`



```move
module 0x1::storage_gas {
    public(friend) fun on_reconfig()
}
```


##### Implementation


```move
module 0x1::storage_gas {
    public(friend) fun on_reconfig() acquires StorageGas, StorageGasConfig {
        assert!(
            exists<StorageGasConfig>(@aptos_framework),
            error::not_found(ESTORAGE_GAS_CONFIG)
        );
        assert!(
            exists<StorageGas>(@aptos_framework),
            error::not_found(ESTORAGE_GAS)
        );
        let (items, bytes) = state_storage::current_items_and_bytes();
        let gas_config = borrow_global<StorageGasConfig>(@aptos_framework);
        let gas = borrow_global_mut<StorageGas>(@aptos_framework);
        gas.per_item_read = calculate_read_gas(&gas_config.item_config, items);
        gas.per_item_create = calculate_create_gas(&gas_config.item_config, items);
        gas.per_item_write = calculate_write_gas(&gas_config.item_config, items);
        gas.per_byte_read = calculate_read_gas(&gas_config.byte_config, bytes);
        gas.per_byte_create = calculate_create_gas(&gas_config.byte_config, bytes);
        gas.per_byte_write = calculate_write_gas(&gas_config.byte_config, bytes);
    }
}
```


<a id="@Specification_16"></a>

## Specification



<a id="0x1_storage_gas_spec_calculate_gas"></a>


```move
module 0x1::storage_gas {
    fun spec_calculate_gas(max_usage: u64, current_usage: u64, curve: GasCurve): u64;
}
```



<a id="0x1_storage_gas_NewGasCurveAbortsIf"></a>


```move
module 0x1::storage_gas {
    schema NewGasCurveAbortsIf {
        min_gas: u64;
        max_gas: u64;
        aborts_if max_gas < min_gas;
        aborts_if max_gas > MAX_U64 / BASIS_POINT_DENOMINATION;
    }
}
```

A non decreasing curve must ensure that next is greater than cur.


<a id="0x1_storage_gas_ValidatePointsAbortsIf"></a>


```move
module 0x1::storage_gas {
    schema ValidatePointsAbortsIf {
        points: vector<Point>;
    // This enforces ### high&#45;level&#45;req&#45;2
    [#high&#45;level&#45;req](high&#45;level requirement 2):
        aborts_if exists i in 0..len(points) - 1: (
            points[i].x >= points[i + 1].x || points[i].y > points[i + 1].y
        );
        aborts_if len(points) > 0 && points[0].x == 0;
        aborts_if len(points) > 0 && points[len(points) - 1].x == BASIS_POINT_DENOMINATION;
    }
}
```


<a id="@Specification_16_Point"></a>

### Struct `Point`


```move
module 0x1::storage_gas {
    struct Point has copy, drop, store
}
```


<dl>
<dt>
`x: u64`
</dt>
<dd>
 x&#45;coordinate basis points, corresponding to utilization
 ratio in `base_8192_exponential_curve()`.
</dd>
<dt>
`y: u64`
</dt>
<dd>
 y&#45;coordinate basis points, corresponding to utilization
 multiplier in `base_8192_exponential_curve()`.
</dd>
</dl>



```move
module 0x1::storage_gas {
    invariant x <= BASIS_POINT_DENOMINATION;
    invariant y <= BASIS_POINT_DENOMINATION;
}
```


<a id="@Specification_16_UsageGasConfig"></a>

### Struct `UsageGasConfig`


```move
module 0x1::storage_gas {
    struct UsageGasConfig has copy, drop, store
}
```


<dl>
<dt>
`target_usage: u64`
</dt>
<dd>

</dd>
<dt>
`read_curve: storage_gas::GasCurve`
</dt>
<dd>

</dd>
<dt>
`create_curve: storage_gas::GasCurve`
</dt>
<dd>

</dd>
<dt>
`write_curve: storage_gas::GasCurve`
</dt>
<dd>

</dd>
</dl>



```move
module 0x1::storage_gas {
    invariant target_usage > 0;
    invariant target_usage <= MAX_U64 / BASIS_POINT_DENOMINATION;
}
```




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>The module&apos;s initialization guarantees the creation of the StorageGasConfig resource with a precise configuration, including accurate gas curves for per&#45;item and per&#45;byte operations.</td>
<td>Medium</td>
<td>The initialize function is responsible for setting up the initial state of the module, ensuring the fulfillment of the following conditions: (1) the creation of the StorageGasConfig resource, indicating its existence witqhin the module&apos;s context, and (2) the configuration of the StorageGasConfig resource includes the precise gas curves that define the behavior of per&#45;item and per&#45;byte operations.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;1](initialize). Moreover, the native gas logic has been manually audited.</td>
</tr>

<tr>
<td>2</td>
<td>The gas curve approximates an exponential curve based on a minimum and maximum gas charge.</td>
<td>High</td>
<td>The validate_points function ensures that the provided vector of points represents a monotonically non&#45;decreasing curve.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;2](validate_points). Moreover, the configuration logic has been manually audited.</td>
</tr>

<tr>
<td>3</td>
<td>The initialized gas curve structure has values set according to the provided parameters.</td>
<td>Low</td>
<td>The new_gas_curve function initializes the GasCurve structure with values provided as parameters.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;3](new_gas_curve).</td>
</tr>

<tr>
<td>4</td>
<td>The initialized usage gas configuration structure has values set according to the provided parameters.</td>
<td>Low</td>
<td>The new_usage_gas_config function initializes the UsageGasConfig structure with values provided as parameters.</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;4](new_usage_gas_config).</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::storage_gas {
    pragma verify = true;
    pragma aborts_if_is_strict;
    invariant [suspendable] chain_status::is_operating() ==> exists<StorageGasConfig>(@aptos_framework);
    invariant [suspendable] chain_status::is_operating() ==> exists<StorageGas>(@aptos_framework);
}
```


<a id="@Specification_16_GasCurve"></a>

### Struct `GasCurve`


```move
module 0x1::storage_gas {
    struct GasCurve has copy, drop, store
}
```


<dl>
<dt>
`min_gas: u64`
</dt>
<dd>

</dd>
<dt>
`max_gas: u64`
</dt>
<dd>

</dd>
<dt>
`points: vector<storage_gas::Point>`
</dt>
<dd>

</dd>
</dl>


Invariant 1: The minimum gas charge does not exceed the maximum gas charge.


```move
module 0x1::storage_gas {
    invariant min_gas <= max_gas;
}
```

Invariant 2: The maximum gas charge is capped by MAX_U64 scaled down by the basis point denomination.


```move
module 0x1::storage_gas {
    invariant max_gas <= MAX_U64 / BASIS_POINT_DENOMINATION;
}
```

Invariant 3: The x&#45;coordinate increases monotonically and the y&#45;coordinate increasing strictly monotonically,
that is, the gas&#45;curve is a monotonically increasing function.


```move
module 0x1::storage_gas {
    invariant (len(points) > 0 ==> points[0].x > 0)
        && (len(points) > 0 ==> points[len(points) - 1].x < BASIS_POINT_DENOMINATION)
        && (forall i in 0..len(points) - 1: (points[i].x < points[i + 1].x && points[i].y <= points[i + 1].y));
}
```


<a id="@Specification_16_base_8192_exponential_curve"></a>

### Function `base_8192_exponential_curve`


```move
module 0x1::storage_gas {
    public fun base_8192_exponential_curve(min_gas: u64, max_gas: u64): storage_gas::GasCurve
}
```



```move
module 0x1::storage_gas {
    include NewGasCurveAbortsIf;
}
```


<a id="@Specification_16_new_point"></a>

### Function `new_point`


```move
module 0x1::storage_gas {
    public fun new_point(x: u64, y: u64): storage_gas::Point
}
```



```move
module 0x1::storage_gas {
    aborts_if x > BASIS_POINT_DENOMINATION || y > BASIS_POINT_DENOMINATION;
    ensures result.x == x;
    ensures result.y == y;
}
```


<a id="@Specification_16_new_gas_curve"></a>

### Function `new_gas_curve`


```move
module 0x1::storage_gas {
    public fun new_gas_curve(min_gas: u64, max_gas: u64, points: vector<storage_gas::Point>): storage_gas::GasCurve
}
```

A non decreasing curve must ensure that next is greater than cur.


```move
module 0x1::storage_gas {
    pragma verify_duration_estimate = 120;
    include NewGasCurveAbortsIf;
    include ValidatePointsAbortsIf;
// This enforces ### high&#45;level&#45;req&#45;3
[#high&#45;level&#45;req](high&#45;level requirement 3):
    ensures result == GasCurve {
        min_gas,
        max_gas,
        points
    };
}
```


<a id="@Specification_16_new_usage_gas_config"></a>

### Function `new_usage_gas_config`


```move
module 0x1::storage_gas {
    public fun new_usage_gas_config(target_usage: u64, read_curve: storage_gas::GasCurve, create_curve: storage_gas::GasCurve, write_curve: storage_gas::GasCurve): storage_gas::UsageGasConfig
}
```



```move
module 0x1::storage_gas {
    aborts_if target_usage == 0;
    aborts_if target_usage > MAX_U64 / BASIS_POINT_DENOMINATION;
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    ensures result == UsageGasConfig {
        target_usage,
        read_curve,
        create_curve,
        write_curve,
    };
}
```


<a id="@Specification_16_new_storage_gas_config"></a>

### Function `new_storage_gas_config`


```move
module 0x1::storage_gas {
    public fun new_storage_gas_config(item_config: storage_gas::UsageGasConfig, byte_config: storage_gas::UsageGasConfig): storage_gas::StorageGasConfig
}
```



```move
module 0x1::storage_gas {
    aborts_if false;
    ensures result.item_config == item_config;
    ensures result.byte_config == byte_config;
}
```


<a id="@Specification_16_set_config"></a>

### Function `set_config`


```move
module 0x1::storage_gas {
    public(friend) fun set_config(aptos_framework: &signer, config: storage_gas::StorageGasConfig)
}
```

Signer address must be @aptos_framework and StorageGasConfig exists.


```move
module 0x1::storage_gas {
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    aborts_if !exists<StorageGasConfig>(@aptos_framework);
}
```


<a id="@Specification_16_initialize"></a>

### Function `initialize`


```move
module 0x1::storage_gas {
    public fun initialize(aptos_framework: &signer)
}
```

Signer address must be @aptos_framework.
Address @aptos_framework does not exist StorageGasConfig and StorageGas before the function call is restricted
and exists after the function is executed.


```move
module 0x1::storage_gas {
    include system_addresses::AbortsIfNotAptosFramework{ account: aptos_framework };
    pragma verify_duration_estimate = 120;
    aborts_if exists<StorageGasConfig>(@aptos_framework);
    aborts_if exists<StorageGas>(@aptos_framework);
// This enforces ### high&#45;level&#45;req&#45;1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures exists<StorageGasConfig>(@aptos_framework);
    ensures exists<StorageGas>(@aptos_framework);
}
```


<a id="@Specification_16_validate_points"></a>

### Function `validate_points`


```move
module 0x1::storage_gas {
    fun validate_points(points: &vector<storage_gas::Point>)
}
```

A non decreasing curve must ensure that next is greater than cur.


```move
module 0x1::storage_gas {
    pragma aborts_if_is_strict = false;
    pragma verify = false;
    pragma opaque;
    include ValidatePointsAbortsIf;
}
```


<a id="@Specification_16_calculate_gas"></a>

### Function `calculate_gas`


```move
module 0x1::storage_gas {
    fun calculate_gas(max_usage: u64, current_usage: u64, curve: &storage_gas::GasCurve): u64
}
```



```move
module 0x1::storage_gas {
    pragma opaque;
    pragma verify_duration_estimate = 120;
    requires max_usage > 0;
    requires max_usage <= MAX_U64 / BASIS_POINT_DENOMINATION;
    aborts_if false;
    ensures [abstract] result == spec_calculate_gas(max_usage, current_usage, curve);
}
```


<a id="@Specification_16_interpolate"></a>

### Function `interpolate`


```move
module 0x1::storage_gas {
    fun interpolate(x0: u64, x1: u64, y0: u64, y1: u64, x: u64): u64
}
```



```move
module 0x1::storage_gas {
    pragma opaque;
    pragma intrinsic;
    aborts_if false;
}
```


<a id="@Specification_16_on_reconfig"></a>

### Function `on_reconfig`


```move
module 0x1::storage_gas {
    public(friend) fun on_reconfig()
}
```

Address @aptos_framework must exist StorageGasConfig and StorageGas and StateStorageUsage.


```move
module 0x1::storage_gas {
    requires chain_status::is_operating();
    aborts_if !exists<StorageGasConfig>(@aptos_framework);
    aborts_if !exists<StorageGas>(@aptos_framework);
    aborts_if !exists<state_storage::StateStorageUsage>(@aptos_framework);
}
```

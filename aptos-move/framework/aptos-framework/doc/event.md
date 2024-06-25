
<a id="0x1_event"></a>

# Module `0x1::event`

The Event module defines an `EventHandleGenerator` that is used to create
`EventHandle`s with unique GUIDs. It contains a counter for the number
of `EventHandle`s it generates. An `EventHandle` is used to count the number of
events emitted to a handle and emit events to the event store.


-  [Struct `EventHandle`](#0x1_event_EventHandle)
-  [Function `emit`](#0x1_event_emit)
-  [Function `write_module_event_to_store`](#0x1_event_write_module_event_to_store)
-  [Function `new_event_handle`](#0x1_event_new_event_handle)
-  [Function `emit_event`](#0x1_event_emit_event)
-  [Function `guid`](#0x1_event_guid)
-  [Function `counter`](#0x1_event_counter)
-  [Function `write_to_event_store`](#0x1_event_write_to_event_store)
-  [Function `destroy_handle`](#0x1_event_destroy_handle)
-  [Specification](#@Specification_0)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `emit`](#@Specification_0_emit)
    -  [Function `write_module_event_to_store`](#@Specification_0_write_module_event_to_store)
    -  [Function `emit_event`](#@Specification_0_emit_event)
    -  [Function `guid`](#@Specification_0_guid)
    -  [Function `counter`](#@Specification_0_counter)
    -  [Function `write_to_event_store`](#@Specification_0_write_to_event_store)
    -  [Function `destroy_handle`](#@Specification_0_destroy_handle)


```move
module 0x1::event {
    use 0x1::bcs;
    use 0x1::guid;
}
```


<a id="0x1_event_EventHandle"></a>

## Struct `EventHandle`

A handle for an event such that:
1. Other modules can emit events to this handle.
2. Storage can use this handle to prove the total number of events that happened in the past.


```move
module 0x1::event {
    #[deprecated]
    struct EventHandle<T: drop, store> has store
}
```


##### Fields


<dl>
<dt>
`counter: u64`
</dt>
<dd>
 Total number of events emitted to this event stream.
</dd>
<dt>
`guid: guid::GUID`
</dt>
<dd>
 A globally unique ID for this event stream.
</dd>
</dl>


<a id="0x1_event_emit"></a>

## Function `emit`

Emit a module event with payload `msg`.


```move
module 0x1::event {
    public fun emit<T: drop, store>(msg: T)
}
```


##### Implementation


```move
module 0x1::event {
    public fun emit<T: store + drop>(msg: T) {
        write_module_event_to_store<T>(msg);
    }
}
```


<a id="0x1_event_write_module_event_to_store"></a>

## Function `write_module_event_to_store`

Log `msg` with the event stream identified by `T`


```move
module 0x1::event {
    fun write_module_event_to_store<T: drop, store>(msg: T)
}
```


##### Implementation


```move
module 0x1::event {
    native fun write_module_event_to_store<T: drop + store>(msg: T);
}
```


<a id="0x1_event_new_event_handle"></a>

## Function `new_event_handle`

Use EventHandleGenerator to generate a unique event handle for `sig`


```move
module 0x1::event {
    #[deprecated]
    public(friend) fun new_event_handle<T: drop, store>(guid: guid::GUID): event::EventHandle<T>
}
```


##### Implementation


```move
module 0x1::event {
    public(friend) fun new_event_handle<T: drop + store>(guid: GUID): EventHandle<T> {
        EventHandle<T> {
            counter: 0,
            guid,
        }
    }
}
```


<a id="0x1_event_emit_event"></a>

## Function `emit_event`

Emit an event with payload `msg` by using `handle_ref`&apos;s key and counter.


```move
module 0x1::event {
    #[deprecated]
    public fun emit_event<T: drop, store>(handle_ref: &mut event::EventHandle<T>, msg: T)
}
```


##### Implementation


```move
module 0x1::event {
    public fun emit_event<T: drop + store>(handle_ref: &mut EventHandle<T>, msg: T) {
        write_to_event_store<T>(bcs::to_bytes(&handle_ref.guid), handle_ref.counter, msg);
        spec {
            assume handle_ref.counter + 1 <= MAX_U64;
        };
        handle_ref.counter = handle_ref.counter + 1;
    }
}
```


<a id="0x1_event_guid"></a>

## Function `guid`

Return the GUID associated with this EventHandle


```move
module 0x1::event {
    #[deprecated]
    public fun guid<T: drop, store>(handle_ref: &event::EventHandle<T>): &guid::GUID
}
```


##### Implementation


```move
module 0x1::event {
    public fun guid<T: drop + store>(handle_ref: &EventHandle<T>): &GUID {
        &handle_ref.guid
    }
}
```


<a id="0x1_event_counter"></a>

## Function `counter`

Return the current counter associated with this EventHandle


```move
module 0x1::event {
    #[deprecated]
    public fun counter<T: drop, store>(handle_ref: &event::EventHandle<T>): u64
}
```


##### Implementation


```move
module 0x1::event {
    public fun counter<T: drop + store>(handle_ref: &EventHandle<T>): u64 {
        handle_ref.counter
    }
}
```


<a id="0x1_event_write_to_event_store"></a>

## Function `write_to_event_store`

Log `msg` as the `count`th event associated with the event stream identified by `guid`


```move
module 0x1::event {
    #[deprecated]
    fun write_to_event_store<T: drop, store>(guid: vector<u8>, count: u64, msg: T)
}
```


##### Implementation


```move
module 0x1::event {
    native fun write_to_event_store<T: drop + store>(guid: vector<u8>, count: u64, msg: T);
}
```


<a id="0x1_event_destroy_handle"></a>

## Function `destroy_handle`

Destroy a unique handle.


```move
module 0x1::event {
    #[deprecated]
    public fun destroy_handle<T: drop, store>(handle: event::EventHandle<T>)
}
```


##### Implementation


```move
module 0x1::event {
    public fun destroy_handle<T: drop + store>(handle: EventHandle<T>) {
        EventHandle<T> { counter: _, guid: _ } = handle;
    }
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
<td>Each event handle possesses a distinct and unique GUID.</td>
<td>Critical</td>
<td>The new_event_handle function creates an EventHandle object with a unique GUID, ensuring distinct identification.</td>
<td>Audited: GUIDs are created in guid::create. Each time the function is called, it increments creation_num_ref. Multiple calls to the function will result in distinct GUID values.</td>
</tr>

<tr>
<td>2</td>
<td>Unable to publish two events with the same GUID &amp; sequence number.</td>
<td>Critical</td>
<td>Two events may either have the same GUID with a different counter or the same counter with a different GUID.</td>
<td>This is implied by [#high&#45;level&#45;req](high&#45;level requirement 1).</td>
</tr>

<tr>
<td>3</td>
<td>Event native functions respect normal Move rules around object creation and destruction.</td>
<td>Critical</td>
<td>Must follow the same rules and principles that apply to object creation and destruction in Move when using event native functions.</td>
<td>The native functions of this module have been manually audited.</td>
</tr>

<tr>
<td>4</td>
<td>Counter increases monotonically between event emissions</td>
<td>Medium</td>
<td>With each event emission, the emit_event function increments the counter of the EventHandle by one.</td>
<td>Formally verified in the post condition of [#high&#45;level&#45;req&#45;4](emit_event).</td>
</tr>

<tr>
<td>5</td>
<td>For a given EventHandle, it should always be possible to: (1) return the GUID associated with this EventHandle, (2) return the current counter associated with this EventHandle, and (3) destroy the handle.</td>
<td>Low</td>
<td>The following functions should not abort if EventHandle exists: guid(), counter(), destroy_handle().</td>
<td>Formally verified via [#high&#45;level&#45;req&#45;5.1](guid), [#high&#45;level&#45;req&#45;5.2](counter) and [#high&#45;level&#45;req&#45;5.3](destroy_handle).</td>
</tr>

</table>



<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::event {
    pragma verify = true;
    pragma aborts_if_is_strict;
}
```


<a id="@Specification_0_emit"></a>

### Function `emit`


```move
module 0x1::event {
    public fun emit<T: drop, store>(msg: T)
}
```



```move
module 0x1::event {
    pragma opaque;
}
```


<a id="@Specification_0_write_module_event_to_store"></a>

### Function `write_module_event_to_store`


```move
module 0x1::event {
    fun write_module_event_to_store<T: drop, store>(msg: T)
}
```

Native function use opaque.


```move
module 0x1::event {
    pragma opaque;
}
```


<a id="@Specification_0_emit_event"></a>

### Function `emit_event`


```move
module 0x1::event {
    #[deprecated]
    public fun emit_event<T: drop, store>(handle_ref: &mut event::EventHandle<T>, msg: T)
}
```



```move
module 0x1::event {
    pragma opaque;
    aborts_if [abstract] false;
// This enforces ### high&#45;level&#45;req&#45;4
[#high&#45;level&#45;req](high&#45;level requirement 4):
    ensures [concrete] handle_ref.counter == old(handle_ref.counter) + 1;
}
```


<a id="@Specification_0_guid"></a>

### Function `guid`


```move
module 0x1::event {
    #[deprecated]
    public fun guid<T: drop, store>(handle_ref: &event::EventHandle<T>): &guid::GUID
}
```



```move
module 0x1::event {
// This enforces ### high&#45;level&#45;req&#45;5.1
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if false;
}
```


<a id="@Specification_0_counter"></a>

### Function `counter`


```move
module 0x1::event {
    #[deprecated]
    public fun counter<T: drop, store>(handle_ref: &event::EventHandle<T>): u64
}
```



```move
module 0x1::event {
// This enforces ### high&#45;level&#45;req&#45;5.2
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if false;
}
```


<a id="@Specification_0_write_to_event_store"></a>

### Function `write_to_event_store`


```move
module 0x1::event {
    #[deprecated]
    fun write_to_event_store<T: drop, store>(guid: vector<u8>, count: u64, msg: T)
}
```

Native function use opaque.


```move
module 0x1::event {
    pragma opaque;
}
```


<a id="@Specification_0_destroy_handle"></a>

### Function `destroy_handle`


```move
module 0x1::event {
    #[deprecated]
    public fun destroy_handle<T: drop, store>(handle: event::EventHandle<T>)
}
```



```move
module 0x1::event {
// This enforces ### high&#45;level&#45;req&#45;5.3
[#high&#45;level&#45;req](high&#45;level requirement 5):
    aborts_if false;
}
```

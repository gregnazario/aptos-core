# Multisig V2 Timelock — Design & Implementation Spec

## Overview

Two coupled features for `multisig_account.move`:

1. **Opt-in timelock**: Once a proposed transaction reaches approval quorum (`num_signatures_required`), it cannot execute until `timelock_secs` seconds have elapsed. There is no way around this except feature 2.
2. **Timelock bypass via higher threshold**: A configurable higher signature count (`bypass_num_signatures`) that, if met, allows immediate execution without waiting. Must be configured at the same time as the timelock.

All changes are in one file: `multisig_account.move` (this directory).

---

## Key Design Constraint: Struct Immutability

Move structs already stored on-chain cannot have new fields added. Both `MultisigAccount` and `MultisigTransaction` are deployed and have existing on-chain instances. Therefore:

- **No changes to `MultisigAccount` or `MultisigTransaction` struct definitions.**
- All new state lives in a **separate resource**: `MultisigAccountTimelock`.
- All timelock logic is encapsulated in **non-inline helper functions** that `acquires MultisigAccountTimelock`, so that existing function signatures (and their `acquires` clauses) do not need to change.

---

## New Resource

```move
/// Opt-in timelock configuration for a multisig account.
/// Stored at the multisig account address alongside MultisigAccount.
/// Absence of this resource means no timelock is configured (default for all existing accounts).
struct MultisigAccountTimelock has key {
    // How many seconds must elapse after quorum is reached before execution is allowed.
    // 0 means the timelock is disabled (resource kept around so the Table doesn't need to be destroyed).
    timelock_secs: u64,
    // Number of approvals that bypass the timelock and allow immediate execution.
    // Must be > num_signatures_required and <= num_owners when timelock_secs > 0.
    bypass_num_signatures: u64,
    // Maps transaction sequence_number -> timestamp (seconds) when that transaction first reached
    // approval quorum. Entry is absent if quorum has not been reached or was lost.
    // Entries are cleaned up when transactions are executed or rejected.
    approval_timestamps: Table<u64, u64>,
}
```

**Lifecycle:**
- Created by `update_timelock` (via `move_to`) when first configured
- Updated in place for subsequent config changes
- "Disabled" by setting `timelock_secs = 0` (resource stays because `Table` cannot be dropped)
- `approval_timestamps` entries are added/removed on every vote, cleaned up on transaction execution/rejection

---

## New Error Codes

```move
/// Prologue error: transaction has reached quorum but the timelock period has not yet elapsed.
const ETIMELOCK_NOT_EXPIRED: u64 = 2011;

/// timelock_secs and bypass_num_signatures must both be positive, or both be zero (to disable).
const EINVALID_TIMELOCK_CONFIG: u64 = 21;

/// timelock_secs must be > 0 when enabling a timelock.
const EINVALID_TIMELOCK_DURATION: u64 = 22;

/// bypass_num_signatures must be > num_signatures_required and <= num_owners.
const EINVALID_TIMELOCK_BYPASS_THRESHOLD: u64 = 23;
```

`ETIMELOCK_NOT_EXPIRED` is >2000 because it is thrown during the transaction prologue (`validate_multisig_transaction`). Error codes >2000 are reserved for prologue errors in this module.

---

## New Event

```move
#[event]
struct TimelockUpdated has drop, store {
    multisig_account: address,
    old_timelock_secs: u64,
    new_timelock_secs: u64,
    old_bypass_num_signatures: u64,
    new_bypass_num_signatures: u64,
}
```

Old values are 0 when the resource didn't previously exist.

---

## New View Functions

```move
#[view]
/// Return the timelock duration in seconds, or 0 if no timelock is configured.
public fun timelock_secs(multisig_account: address): u64 acquires MultisigAccountTimelock {
    if (!exists<MultisigAccountTimelock>(multisig_account)) return 0;
    borrow_global<MultisigAccountTimelock>(multisig_account).timelock_secs
}

#[view]
/// Return the number of approvals required to bypass the timelock, or 0 if no timelock.
public fun timelock_bypass_num_signatures(
    multisig_account: address
): u64 acquires MultisigAccountTimelock {
    if (!exists<MultisigAccountTimelock>(multisig_account)) return 0;
    borrow_global<MultisigAccountTimelock>(multisig_account).bypass_num_signatures
}
```

---

## New Entry Function: `update_timelock`

```move
/// Configure or disable the timelock for this multisig account.
/// To enable: provide timelock_secs > 0 and bypass_num_signatures > num_signatures_required.
/// To disable: provide timelock_secs = 0 and bypass_num_signatures = 0.
///
/// Not public — can only be invoked directly as a multisig transaction payload.
/// This prevents another module from obtaining the multisig signer and altering
/// the timelock configuration.
entry fun update_timelock(
    multisig_account: &signer,
    new_timelock_secs: u64,
    new_bypass_num_signatures: u64,
) acquires MultisigAccount, MultisigAccountTimelock {
    let multisig_address = address_of(multisig_account);
    assert_multisig_account_exists(multisig_address);
    let multisig_account_resource = borrow_global<MultisigAccount>(multisig_address);

    // Both must be positive (enable) or both must be zero (disable).
    assert!(
        (new_timelock_secs > 0) == (new_bypass_num_signatures > 0),
        error::invalid_argument(EINVALID_TIMELOCK_CONFIG)
    );

    if (new_timelock_secs > 0) {
        assert!(
            new_bypass_num_signatures > multisig_account_resource.num_signatures_required,
            error::invalid_argument(EINVALID_TIMELOCK_BYPASS_THRESHOLD)
        );
        assert!(
            new_bypass_num_signatures <= multisig_account_resource.owners.length(),
            error::invalid_argument(EINVALID_TIMELOCK_BYPASS_THRESHOLD)
        );
    };

    let (old_timelock_secs, old_bypass) = if (exists<MultisigAccountTimelock>(multisig_address)) {
        let timelock = borrow_global_mut<MultisigAccountTimelock>(multisig_address);
        let old = (timelock.timelock_secs, timelock.bypass_num_signatures);
        timelock.timelock_secs = new_timelock_secs;
        timelock.bypass_num_signatures = new_bypass_num_signatures;
        old
    } else {
        move_to(multisig_account, MultisigAccountTimelock {
            timelock_secs: new_timelock_secs,
            bypass_num_signatures: new_bypass_num_signatures,
            approval_timestamps: table::new<u64, u64>(),
        });
        (0, 0)
    };

    emit(TimelockUpdated {
        multisig_account: multisig_address,
        old_timelock_secs,
        new_timelock_secs,
        old_bypass_num_signatures: old_bypass,
        new_bypass_num_signatures,
    });
}
```

---

## Non-Inline Helper Functions

These encapsulate all `MultisigAccountTimelock` access so that **no existing function signatures need their `acquires` clause changed**. In Move, only inline functions propagate their `acquires` requirements to the caller — regular function calls do not.

### `maybe_update_approval_timestamp`

Called after every vote and after transaction creation. Updates the approval timestamp table based on whether quorum is currently met.

```move
/// Track when a transaction first reaches approval quorum (for timelock enforcement).
/// If timelock is not configured or not active, this is a no-op.
fun maybe_update_approval_timestamp(
    multisig_account: address,
    sequence_number: u64,
    num_approvals: u64,
    num_signatures_required: u64,
) acquires MultisigAccountTimelock {
    if (!exists<MultisigAccountTimelock>(multisig_account)) return;
    let timelock = borrow_global_mut<MultisigAccountTimelock>(multisig_account);
    if (timelock.timelock_secs == 0) return;

    if (num_approvals >= num_signatures_required) {
        // Quorum met: record the timestamp if not already recorded
        if (!timelock.approval_timestamps.contains(sequence_number)) {
            timelock.approval_timestamps.add(sequence_number, now_seconds());
        };
        // If already present, preserve the original timestamp (don't restart the clock)
    } else {
        // Quorum lost: remove the timestamp (clock resets)
        if (timelock.approval_timestamps.contains(sequence_number)) {
            timelock.approval_timestamps.remove(sequence_number);
        };
    };
}
```

### `assert_timelock_expired`

Called during `validate_multisig_transaction` (prologue). Enforces the timelock unless the bypass threshold is met.

```move
/// Assert that the timelock has expired for the given transaction, or that the bypass
/// threshold is met. No-op if no timelock is configured.
fun assert_timelock_expired(
    multisig_account: address,
    sequence_number: u64,
    effective_num_approvals: u64,
) acquires MultisigAccountTimelock {
    if (!exists<MultisigAccountTimelock>(multisig_account)) return;
    let timelock = borrow_global<MultisigAccountTimelock>(multisig_account);
    if (timelock.timelock_secs == 0) return;

    // If bypass threshold met, skip the timelock entirely
    if (effective_num_approvals >= timelock.bypass_num_signatures) return;

    // Otherwise enforce the timelock
    assert!(
        timelock.approval_timestamps.contains(sequence_number),
        error::invalid_state(ETIMELOCK_NOT_EXPIRED)
    );
    let approval_time = *timelock.approval_timestamps.borrow(sequence_number);
    assert!(
        now_seconds() >= approval_time + timelock.timelock_secs,
        error::invalid_state(ETIMELOCK_NOT_EXPIRED)
    );
}
```

### `maybe_remove_approval_timestamp`

Called when a transaction is executed or rejected. Cleans up the entry from the approval timestamps table.

```move
/// Remove the approval timestamp for a resolved transaction.
/// No-op if no timelock is configured or no timestamp recorded.
fun maybe_remove_approval_timestamp(
    multisig_account: address,
    sequence_number: u64,
) acquires MultisigAccountTimelock {
    if (!exists<MultisigAccountTimelock>(multisig_account)) return;
    let timelock = borrow_global_mut<MultisigAccountTimelock>(multisig_account);
    if (timelock.approval_timestamps.contains(sequence_number)) {
        timelock.approval_timestamps.remove(sequence_number);
    };
}
```

### `validate_timelock_invariants`

Called at the end of `update_owner_schema` to ensure owner/threshold changes don't break the timelock bypass invariants.

```move
/// After owner or threshold changes, verify the timelock bypass threshold is still valid.
/// No-op if no timelock is configured or timelock is disabled.
fun validate_timelock_invariants(
    multisig_account: address,
    num_owners: u64,
    num_signatures_required: u64,
) acquires MultisigAccountTimelock {
    if (!exists<MultisigAccountTimelock>(multisig_account)) return;
    let timelock = borrow_global<MultisigAccountTimelock>(multisig_account);
    if (timelock.timelock_secs == 0) return;
    assert!(
        timelock.bypass_num_signatures <= num_owners,
        error::invalid_state(EINVALID_TIMELOCK_BYPASS_THRESHOLD)
    );
    assert!(
        timelock.bypass_num_signatures > num_signatures_required,
        error::invalid_state(EINVALID_TIMELOCK_BYPASS_THRESHOLD)
    );
}
```

---

## Changes to Existing Functions

Because all helpers are non-inline, **no existing function needs its `acquires` clause changed**. The changes are minimal insertions at specific points.

### `vote_transanction` (line 1005)

After recording the vote (line 1023), before the `Vote` event emission (line 1025), insert:

```move
        // After vote is recorded in transaction.votes:

        // Track approval quorum time for timelock enforcement.
        let (num_approvals, _) = num_approvals_and_rejections_internal(
            &multisig_account_resource.owners, transaction);
        maybe_update_approval_timestamp(
            multisig_account,
            sequence_number,
            num_approvals,
            multisig_account_resource.num_signatures_required,
        );
```

**Borrow safety:** `transaction` is mutably borrowed from `multisig_account_resource.transactions` (line 1015). `num_approvals_and_rejections_internal` reads `&owners` (different field) and `&MultisigTransaction` (`&mut` coerces to `&`). `maybe_update_approval_timestamp` accesses `MultisigAccountTimelock` — a completely different resource from `MultisigAccount`. No borrow conflicts.

**`acquires` unchanged:** `maybe_update_approval_timestamp` is non-inline, so `vote_transanction` keeps `acquires MultisigAccount` only.

### `add_transaction` (inline fun, line 1227)

After the creator's auto-approval (line 1242), before inserting into the table (line 1246), insert:

```move
        // The transaction creator also automatically votes for the transaction.
        transaction.votes.add(creator, true);

        // Track approval quorum time (handles 1-of-N case where quorum is immediate).
        let sequence_number = multisig_account_resource.next_sequence_number;
        let (num_approvals, _) = num_approvals_and_rejections_internal(
            &multisig_account_resource.owners, &transaction);
        maybe_update_approval_timestamp(
            multisig_account,
            sequence_number,
            num_approvals,
            multisig_account_resource.num_signatures_required,
        );

        multisig_account_resource.next_sequence_number = sequence_number + 1;
        multisig_account_resource.transactions.add(sequence_number, transaction);
```

Note: `sequence_number` is moved up from its original position (line 1244) because we need it before the table insert. This is a reordering only — the value is the same.

**`acquires` unchanged:** `add_transaction` is inline, but `maybe_update_approval_timestamp` is non-inline, so the inline expansion does not pull in `acquires MultisigAccountTimelock`.

### `validate_multisig_transaction` (line 1107)

Insert between the quorum check (line 1125) and the payload hash check (line 1129):

```move
        // ... existing quorum check above (line 1114-1125) ...

        // Timelock enforcement.
        // Compute effective approvals including implicit executor vote (same as can_execute).
        let (approvals, _) = num_approvals_and_rejections(multisig_account, sequence_number);
        let voted_approve = has_voted_for_approval(multisig_account, sequence_number, address_of(owner));
        let effective_approvals = if (features::multisig_v2_enhancement_feature_enabled() && !voted_approve) {
            approvals + 1
        } else {
            approvals
        };
        assert_timelock_expired(multisig_account, sequence_number, effective_approvals);

        // If the transaction payload is not stored on chain, verify ...
        let multisig_account_resource = borrow_global<MultisigAccount>(multisig_account);
```

**Borrow safety:** `num_approvals_and_rejections` (inline) and `has_voted_for_approval` (inline) each borrow `MultisigAccount` temporarily and release it. `assert_timelock_expired` (non-inline) borrows `MultisigAccountTimelock` only. The existing payload check borrow at line 1129 comes after all of these.

**`acquires` unchanged:** The inline calls access `MultisigAccount` (already in `acquires`). `assert_timelock_expired` is non-inline and accesses only `MultisigAccountTimelock`.

### `transaction_execution_cleanup_common` (inline fun, line 1197)

After `remove_executed_transaction` (line 1202), insert cleanup:

```move
        let multisig_account_resource = borrow_global_mut<MultisigAccount>(multisig_account);
        let (num_approvals, _) = remove_executed_transaction(multisig_account_resource);

        // Clean up approval timestamp for the resolved transaction.
        maybe_remove_approval_timestamp(multisig_account, sequence_number);
```

**`acquires` unchanged:** `maybe_remove_approval_timestamp` is non-inline. The callers (`successful_transaction_execution_cleanup` and `failed_transaction_execution_cleanup`) don't need new `acquires`.

### `execute_rejected_transaction` (line 1054)

After `remove_executed_transaction` (line 1071), before the assertion and event emission, insert:

```move
        let multisig_account_resource = borrow_global_mut<MultisigAccount>(multisig_account);
        let (_, num_rejections) = remove_executed_transaction(multisig_account_resource);

        // Clean up approval timestamp for the rejected transaction.
        maybe_remove_approval_timestamp(multisig_account, sequence_number);

        assert!(
            num_rejections >= multisig_account_resource.num_signatures_required,
            ...
```

Wait — there's a borrow conflict here. `multisig_account_resource` is mutably borrowed (line 1070). Then we call `maybe_remove_approval_timestamp` which accesses `MultisigAccountTimelock` (different resource — no conflict). Then we use `multisig_account_resource.num_signatures_required` (line 1073). This is fine — the mutable borrow of `MultisigAccount` is still alive, and `maybe_remove_approval_timestamp` only touches `MultisigAccountTimelock`.

**`acquires` unchanged.**

### `update_owner_schema` (line 1352)

At the end of the function, after the existing `num_owners >= num_signatures_required` assertion (line 1424), insert:

```move
        // Verify number of owners.
        let num_owners = multisig_account_ref_mut.owners.length();
        assert!(
            num_owners >= multisig_account_ref_mut.num_signatures_required,
            error::invalid_state(ENOT_ENOUGH_OWNERS)
        );

        // Validate timelock bypass invariants after owner/threshold changes.
        validate_timelock_invariants(
            multisig_address,
            num_owners,
            multisig_account_ref_mut.num_signatures_required,
        );
    }
```

**Borrow safety:** `multisig_account_ref_mut` is a mutable borrow of `MultisigAccount`. `validate_timelock_invariants` accesses only `MultisigAccountTimelock`. No conflict.

**`acquires` unchanged.**

---

## Why No Existing `acquires` Changes

In Move, the `acquires` annotation lists resources that a function **directly** accesses via `borrow_global`, `borrow_global_mut`, `move_from`, `move_to`, or `exists`. Calling a non-inline function that has its own `acquires` does **not** require the caller to list those resources. Only inline functions propagate their accesses to the caller.

All timelock resource access is wrapped in the non-inline helpers: `maybe_update_approval_timestamp`, `assert_timelock_expired`, `maybe_remove_approval_timestamp`, and `validate_timelock_invariants`. These are the only functions (plus `update_timelock` and the view functions) with `acquires MultisigAccountTimelock`.

This means every existing public function (`vote_transanction`, `execute_rejected_transaction`, `create_transaction`, etc.) keeps its original signature unchanged.

---

## Behavioral Implications

### Implicit executor vote + timelock

When the v2 enhancement feature is enabled, the executor's implicit approval counts toward both the regular threshold and the bypass threshold. However:

- If the executor's implicit vote is what brings the count to the regular threshold for the first time, there will be no entry in `approval_timestamps` (quorum was never reached during explicit voting).
- `assert_timelock_expired` will fail with `ETIMELOCK_NOT_EXPIRED` because no timestamp was recorded.
- **The executor must first explicitly call `approve_transaction`** (which triggers `maybe_update_approval_timestamp` to record the time), wait for the timelock, then execute.

This is correct: the timelock cannot be circumvented by relying on implicit votes.

### Enabling the timelock on an existing account

When `update_timelock` is first called, the `MultisigAccountTimelock` resource is created. Existing pending transactions that are already at quorum do NOT have entries in `approval_timestamps` yet. They cannot execute until someone casts a vote (even a redundant re-approval by an existing approver) to trigger `maybe_update_approval_timestamp`, which starts the clock.

This is acceptable: enabling a timelock is a security hardening step, and requiring a fresh vote cycle for existing pending transactions is a reasonable trade-off.

### Disabling the timelock

Calling `update_timelock(0, 0)` through the multisig flow sets `timelock_secs = 0`. Since this is itself a multisig transaction, it must go through the currently-active timelock to execute. Once it executes, subsequent transactions can execute immediately (the helpers no-op when `timelock_secs == 0`). The resource stays because `Table` cannot be dropped, but stale `approval_timestamps` entries are cleaned up naturally as transactions resolve.

### Threshold/owner changes and stale timestamps

`approval_timestamps` entries are only updated when votes are cast, not when the threshold or owner list changes. This is safe because:

- The quorum check always runs first. If quorum is not currently met, execution is blocked regardless of timestamps.
- If the threshold increases such that a pending transaction drops below quorum, the quorum check blocks execution.
- `validate_timelock_invariants` prevents owner/threshold changes that would break the bypass invariant (e.g., removing an owner such that `bypass_num_signatures > num_owners`).

### Rejections are unaffected

The timelock does **not** apply to rejections. `execute_rejected_transaction` operates independently. If enough owners reject, the transaction can be removed immediately. The timelock protects against hasty execution, not hasty cancellation.

### Clock behavior

| Scenario | `approval_timestamps` behavior |
|----------|--------------------------------|
| Creator proposes, 1-of-N multisig | Entry added in `add_transaction` (creator auto-approves, meets quorum) |
| k-th explicit approval cast | Entry added with `now_seconds()` in `vote_transanction` |
| Voter switches approve -> reject, quorum drops | Entry removed (clock resets) |
| Voter re-approves, quorum restored | New entry with `now_seconds()` (clock restarts from now) |
| Quorum already met, additional approval | Entry preserved (clock does NOT restart) |
| Owner removed, quorum drops | Entry is stale but quorum check blocks execution |
| Transaction executed/rejected | Entry removed (cleanup) |
| Timelock not configured | No resource, all helpers no-op |

---

## Invariants

When timelock is active (`exists<MultisigAccountTimelock>` and `timelock_secs > 0`):

1. `bypass_num_signatures > num_signatures_required`
2. `bypass_num_signatures <= owners.length()`

Enforced in both `update_timelock` and `validate_timelock_invariants` (called from `update_owner_schema`).

---

## Test Cases

| Test | What it validates |
|------|-------------------|
| `test_update_timelock` | Configure timelock, verify via view functions, then disable |
| `test_update_timelock_invalid_config_should_fail` | One positive and one zero -> `EINVALID_TIMELOCK_CONFIG` |
| `test_update_timelock_bypass_not_greater_than_threshold_should_fail` | `bypass <= num_signatures_required` -> `EINVALID_TIMELOCK_BYPASS_THRESHOLD` |
| `test_update_timelock_bypass_exceeds_owners_should_fail` | `bypass > num_owners` -> `EINVALID_TIMELOCK_BYPASS_THRESHOLD` |
| `test_execute_before_timelock_expires_should_fail` | Quorum reached, but `now < approval_time + timelock` -> `ETIMELOCK_NOT_EXPIRED` |
| `test_execute_after_timelock_expires` | Quorum reached, `now >= approval_time + timelock` -> success |
| `test_execute_with_bypass_skips_timelock` | Enough approvals to bypass -> immediate execution |
| `test_timelock_resets_when_quorum_lost` | Approve -> quorum -> reject (quorum lost) -> re-approve -> new timestamp |
| `test_timelock_set_immediately_for_single_sig` | 1-of-N: timestamp set at creation (creator auto-approval meets quorum) |
| `test_remove_owners_invalidates_bypass_should_fail` | Remove owner such that `bypass > num_owners` -> fails |
| `test_update_threshold_invalidates_bypass_should_fail` | Raise `num_signatures_required` to equal bypass -> fails |
| `test_no_timelock_executes_normally` | No timelock configured -> existing behavior unchanged |
| `test_disable_timelock_allows_immediate_execution` | Enable then disable -> next transaction executes immediately |

Tests use `timestamp::set_time_has_started_for_testing` and `timestamp::fast_forward_seconds` to control time.

---

## Change Summary

| Location | Change |
|----------|--------|
| After error codes (line ~101) | Add 4 new error constants |
| After events (line ~332) | Add `MultisigAccountTimelock` resource, `TimelockUpdated` event |
| After view functions (line ~496) | Add `timelock_secs()` and `timelock_bypass_num_signatures()` views |
| After self-updates (line ~897) | Add `update_timelock` entry function |
| After `update_timelock` | Add 4 non-inline helpers: `maybe_update_approval_timestamp`, `assert_timelock_expired`, `maybe_remove_approval_timestamp`, `validate_timelock_invariants` |
| `vote_transanction` (line ~1023) | Insert call to `maybe_update_approval_timestamp` after vote recording |
| `validate_multisig_transaction` (line ~1125) | Insert timelock check: compute effective approvals, call `assert_timelock_expired` |
| `add_transaction` (line ~1242) | Insert call to `maybe_update_approval_timestamp` after creator auto-approval |
| `transaction_execution_cleanup_common` (line ~1202) | Insert call to `maybe_remove_approval_timestamp` |
| `execute_rejected_transaction` (line ~1071) | Insert call to `maybe_remove_approval_timestamp` |
| `update_owner_schema` (line ~1424) | Insert call to `validate_timelock_invariants` |
| Tests section (line ~1429) | Add 13 new test functions |

**No existing struct definitions change. No existing function signatures change.**

---

## Verification

```bash
cargo check -p aptos-framework              # compile check
cargo test -p aptos-framework -- multisig    # Move unit tests
cargo test -p aptos-api -- multisig          # E2E API tests
cargo build -p aptos-cached-packages         # required after framework changes
```

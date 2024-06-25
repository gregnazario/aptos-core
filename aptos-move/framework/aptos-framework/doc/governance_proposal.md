
<a id="0x1_governance_proposal"></a>

# Module `0x1::governance_proposal`

Define the GovernanceProposal that will be used as part of on&#45;chain governance by AptosGovernance.

This is separate from the AptosGovernance module to avoid circular dependency between AptosGovernance and Stake.


-  [Struct `GovernanceProposal`](#0x1_governance_proposal_GovernanceProposal)
-  [Function `create_proposal`](#0x1_governance_proposal_create_proposal)
-  [Function `create_empty_proposal`](#0x1_governance_proposal_create_empty_proposal)
-  [Specification](#@Specification_0)
    -  [Function `create_proposal`](#@Specification_0_create_proposal)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `create_empty_proposal`](#@Specification_0_create_empty_proposal)


```move
module 0x1::governance_proposal {
}
```


<a id="0x1_governance_proposal_GovernanceProposal"></a>

## Struct `GovernanceProposal`



```move
module 0x1::governance_proposal {
    struct GovernanceProposal has drop, store
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


<a id="0x1_governance_proposal_create_proposal"></a>

## Function `create_proposal`

Create and return a GovernanceProposal resource. Can only be called by AptosGovernance


```move
module 0x1::governance_proposal {
    public(friend) fun create_proposal(): governance_proposal::GovernanceProposal
}
```


##### Implementation


```move
module 0x1::governance_proposal {
    public(friend) fun create_proposal(): GovernanceProposal {
        GovernanceProposal {}
    }
}
```


<a id="0x1_governance_proposal_create_empty_proposal"></a>

## Function `create_empty_proposal`

Useful for AptosGovernance to create an empty proposal as proof.


```move
module 0x1::governance_proposal {
    public(friend) fun create_empty_proposal(): governance_proposal::GovernanceProposal
}
```


##### Implementation


```move
module 0x1::governance_proposal {
    public(friend) fun create_empty_proposal(): GovernanceProposal {
        create_proposal()
    }
}
```


<a id="@Specification_0"></a>

## Specification


<a id="@Specification_0_create_proposal"></a>

### Function `create_proposal`


```move
module 0x1::governance_proposal {
    public(friend) fun create_proposal(): governance_proposal::GovernanceProposal
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
<td>Creating a proposal should never abort but should always return a governance proposal resource.</td>
<td>Medium</td>
<td>Both create_proposal and create_empty_proposal functions return a GovernanceProposal resource.</td>
<td>Enforced via [#high&#45;level&#45;req&#45;1.1](create_proposal) and [#high&#45;level&#45;req&#45;1.2](create_empty_proposal).</td>
</tr>

<tr>
<td>2</td>
<td>The governance proposal module should only be accessible to the aptos governance.</td>
<td>Medium</td>
<td>Both create_proposal and create_empty_proposal functions are only available to the friend module aptos_framework::aptos_governance.</td>
<td>Enforced via friend module relationship.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


```move
module 0x1::governance_proposal {
    aborts_if false;
// This enforces ### high&#45;level&#45;req&#45;1.1
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures result == GovernanceProposal {};
}
```


<a id="@Specification_0_create_empty_proposal"></a>

### Function `create_empty_proposal`


```move
module 0x1::governance_proposal {
    public(friend) fun create_empty_proposal(): governance_proposal::GovernanceProposal
}
```



```move
module 0x1::governance_proposal {
    aborts_if false;
// This enforces ### high&#45;level&#45;req&#45;1.2
[#high&#45;level&#45;req](high&#45;level requirement 1):
    ensures result == GovernanceProposal {};
}
```

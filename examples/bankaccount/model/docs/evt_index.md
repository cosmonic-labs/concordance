# Events
The following is a list of events within this system.


## Account Created
Indicates that a bank account has been created

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Funds Deposited
Indicates that funds have been deposited into a bank account

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Funds Withdrawn
Indicates that funds have been withdrawn from a bank account

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Interbank Transfer Completed
Indicates that an interbank transfer has been completed

### Emitted By
_No inbound entities declared_

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |


## Interbank Transfer Failed
Indicates that an interbank transfer has failed

### Emitted By
_No inbound entities declared_

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |


## Interbank Transfer Initiated
Indicates that an interbank transfer has been initiated

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Reserved Funds Withdrawn
Indicates that reserved funds have been withdrawn from a bank account

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Wire Funds Released
Indicates that funds have been released from a wire transfer

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Wire Funds Reserved
Indicates that funds have been reserved for a wire transfer

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |


## Wire Transfer Requested
Indicates that a wire transfer has been requested

### Emitted By
This is a list of entities that emit this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |

### Handled By
This is a list of entities that handle this event

| Entity | Description |
|---|:--|
| [bankaccount](./agg_index.md#bankaccount) | The aggregate for a bank account |
| [bankaccount](./pm_index.md#bankaccount) | The process manager for a bank account |
| [bankaccount](./proj_index.md#bankaccount) | The projector for a bank account |



---
_This file is automatically generated by a tool. Do not modify its contents manually_
export CONCORDANCE_PROVIDER_ID=VAUL6FF47DZIMDOGNWZBCPVABWDK3UEZ5U7DGAM4HCKCHOEPTBZHGTAM
export KVREDIS_PROVIDER_ID=VAZVC4RX54J2NVCMCW7BPCAHGGG5XZXDBXFUMDUXGESTMQEJLC3YVZWB

export PROJECTOR_ACTOR_ID=MC5D3GHCW3FN6UWHJDH63VQI36L66YN73OIBFVSM3EXPIC6ZG3AEVTE3
export PROCESS_MANAGER_ACTOR_ID=MC5EQZ6NZY2T5US5JJTCJVAWHETCIIZLLVFUPERSTO2T3AR2NF62JWKI
export AGGREGATE_ACTOR_ID=MCZ2V2VTF4S4QAYKHJTGARIGWFMQXS2FDHKNNI3H7ZHHYAWE6IVCTD7M

# Link projector <-> concordance
wash ctl link put $PROJECTOR_ACTOR_ID $CONCORDANCE_PROVIDER_ID \
    cosmonic:eventsourcing \
    ROLE=projector INTEREST=account_created,funds_deposited,funds_withdrawn,wire_funds_reserved,wire_funds_released NAME=bankaccount_projector

# Link projector <-> keyvalue
wash ctl link put $PROJECTOR_ACTOR_ID $KVREDIS_PROVIDER_ID wasmcloud:keyvalue URL='redis://0.0.0.0:6379/'

# Link process manager <-> concordance
wash ctl link put $PROCESS_MANAGER_ACTOR_ID $CONCORDANCE_PROVIDER_ID \
    cosmonic:eventsourcing \
    ROLE=process_manager KEY=wire_transfer_id NAME=interbankxfer INTEREST='{"start":"wire_transfer_requested","advance":["wire_funds_reserved","interbank_transfer_initiated"],"stop":["interbank_transfer_completed","interbank_transfer_failed"]}'

# Link aggregate <-> concodrance
wash ctl link put $AGGREGATE_ACTOR_ID $CONCORDANCE_PROVIDER_ID \
    cosmonic:eventsourcing \
    ROLE=aggregate KEY=account_number INTEREST=bankaccount NAME=bankaccount
# PEM_FILE="../wallet/wallet-owner.pem"
# NFT_MINTER_CONTRACT="output/nft-minter.wasm"

# PROXY_ARGUMENT="--proxy=https://devnet-api.elrond.com"
# CHAIN_ARGUMENT="--chain=D"
# CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqxxgm0gwvqj8xmwg6zph5tcke6x558uk6ap7qwz54ef"

# build_nft_minter() {
#     (set -x; erdpy --verbose contract build "$NFT_MINTER_CONTRACT")
# }

# deploy_nft_minter() {
#     # local TOKEN_ID=0x45474c44 # "EGLD"
#     local PING_AMOUNT=1500000000000000000 # 1.5 EGLD
#     local DURATION=86400 # 1 day in seconds
#     # local ACTIVATION_TIMESTAMP= # skipped
#     # local MAX_FUNDS= #skipped
    
#     local OUTFILE="out.json"
#     (set -x; erdpy contract deploy --bytecode="$NFT_MINTER_CONTRACT" \
#         --pem="$PEM_FILE" \
#         $PROXY_ARGUMENT $CHAIN_ARGUMENT \
#         --outfile="$OUTFILE" --recall-nonce --gas-limit=60000000 \
#         --arguments ${PING_AMOUNT} ${DURATION} --send \
#         || return)

#     local RESULT_ADDRESS=$(erdpy data parse --file="$OUTFILE" --expression="data['emitted_tx']['address']")
#     local RESULT_TRANSACTION=$(erdpy data parse --file="$OUTFILE" --expression="data['emitted_tx']['hash']")

#     echo ""
#     echo "Deployed contract with:"
#     echo "  \$RESULT_ADDRESS == ${RESULT_ADDRESS}"
#     echo "  \$RESULT_TRANSACTION == ${RESULT_TRANSACTION}"
#     echo ""
# }

# number_to_u64() {
#     local NUMBER=$1
#     printf "%016x" $NUMBER
# }



# ######nft calls for the smart contract.


# # NOTES:
# # erd1qqqqqqqqqqqqqpgqfduzss9v5qev29nf240kz7vf32j0atpsap7q6n37kw is special system contract address
# # issueNonFungible is the command to issue the NFT
# # collection name is 0x52696e6750617373 (= MintPass)
# # ticker name is 0x52494e4750415353 (=MINTPASS)

# issueToken(){
#     erdpy contract call erd1dzurql3men9v5h8kkvq9343q79wqszg95aw2fukjwvvu0wlsap7qj5flf7  --function issue_token --arguments 0x52696e6750617373 0x52494e4750415353 --recall-nonce --gas-limit 60000000 --pem=${PEM_FILE} --send --value 50000000000000000
# }



# # NOTES:
# # erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u is a special system contract
# # setSpecialRole is the command to set a role
# # 0x52494e47504153532d656537376234 is our token/collection identifier (RINGPASS-ee77b4 in hex)
# # 0xd8858ff5bd57c878624f10f1f6b1295a7782f3155095ec1448e9a85b1f4080d4 is our wallet address decoded to hex
# # 0x45534454526f6c654e4654437265617465 is the role we are giving our wallet (ESDTRoleNFTCreate)

# setSpecialRole(){
#    erdpy contract call erd1dzurql3men9v5h8kkvq9343q79wqszg95aw2fukjwvvu0wlsap7qj5flf7 --function setSpecialRole --arguments 0x52494e47504153532d656537376234 0xd8858ff5bd57c878624f10f1f6b1295a7782f3155095ec1448e9a85b1f4080d4 0x45534454526f6c654e4654437265617465 --recall-nonce --gas-limit 60000000 --pem=${PEM_FILE} --send
# }

# mintNft(){
#     erdpy contract call erd1qqqqqqqqqqqqqpgqxxgm0gwvqj8xmwg6zph5tcke6x558uk6ap7qwz54ef --function mint --arguments 0x52494e47504153532d656537376234 1 0x52696e672050617373 0 0x00ad12b8600c09a844551018255763831173488f33804a461f6009418c34cf07 0 0x68747470733a2f2f676174657761792e70696e6174612e636c6f75642f697066732f516d6231645563726e724d346953566535357339744b5372344177706931624c693759484b34587848586d623344  --recall-nonce --gas-limit 70000000 --pem=${PEM_FILE} --send
# }
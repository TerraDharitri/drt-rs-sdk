ALICE="${USERS}/alice.pem"
ADDRESS=$(drtpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(drtpy data load --key=deployTransaction-testnet)
DEPLOY_ARGUMENTS="12 42000000"
DEPLOY_GAS="80000000"
PROXY=https://testnet-api.numbat.com

deploy() {
    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} --arguments ${DEPLOY_ARGUMENTS} \
          --outfile="deploy-testnet.interaction.json" --send --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(drtpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(drtpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    drtpy data store --key=address-testnet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

checkDeployment() {
    drtpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']" --proxy=${PROXY}
    drtpy account get --address=$ADDRESS --omit-fields="['code']" --proxy=${PROXY}
}

status() {
    drtpy --verbose contract query ${ADDRESS} --function="status" --proxy=${PROXY}
}

currentFunds() {
    drtpy --verbose contract query ${ADDRESS} --function="currentFunds" --proxy=${PROXY}
}

sendFunds() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="fund" --value=3\
        --send --proxy=${PROXY} --chain=T
}

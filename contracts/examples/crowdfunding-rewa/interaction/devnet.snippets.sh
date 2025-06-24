ALICE="${USERS}/alice.pem"
ADDRESS=$(drtpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(drtpy data load --key=deployTransaction-devnet)
DEPLOY_ARGUMENTS="12 4096"
DEPLOY_GAS="80000000"

deploy() {
    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} --arguments ${DEPLOY_ARGUMENTS} \
          --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(drtpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(drtpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    drtpy data store --key=address-devnet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deploySimulate() {
    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} --arguments ${DEPLOY_ARGUMENTS} \
          --outfile="simulate-devnet.interaction.json" --simulate || return

    TRANSACTION=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['hash']")
    ADDRESS=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['emitted_tx']['address']")
    RETCODE=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnCode']")
    RETMSG=$(drtpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnMessage']")

    echo ""
    echo "Simulated transaction: ${TRANSACTION}"
    echo "Smart contract address: ${ADDRESS}"
    echo "Deployment return code: ${RETCODE}"
    echo "Deployment return message: ${RETMSG}"
}

checkDeployment() {
    drtpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    drtpy account get --address=$ADDRESS --omit-fields="['code']"
}

status() {
    drtpy --verbose contract query ${ADDRESS} --function="status"
}

currentFunds() {
    drtpy --verbose contract query ${ADDRESS} --function="currentFunds"
}

sendFunds() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="fund" --value=3\
        --send
}

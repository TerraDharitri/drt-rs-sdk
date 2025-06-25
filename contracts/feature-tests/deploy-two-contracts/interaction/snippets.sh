ALICE="/home/numbat/MySandbox/testnet/wallets/users/alice.pem"
ADDRESS=$(drtpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(drtpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.numbat.com

deploy() {
    drtpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(drtpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(drtpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    drtpy data store --key=address-testnet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deployContract() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="deployContract" --send --proxy=${PROXY} --chain=T
}

deployTwoContracts() {
    drtpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="deployTwoContracts" --send --proxy=${PROXY} --chain=T
}
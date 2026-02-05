use dharitri_sc_scenario::imports::ReturnCode;
use dharitri_sc_snippets::network_response;
use dharitri_sdk::data::transaction::{TransactionInfo, TransactionOnNetwork};

#[test]
fn test_tx_payable_features_rewa() {
    let data = r#"
{
  "data": {
      "transaction": {
          "type": "normal",
          "processingTypeOnSource": "BuiltInFunctionCall",
          "processingTypeOnDestination": "SCInvoking",
          "hash": "29c295934b5419dc10756c6ddf45468f4de3ce4a1f7ba236aeee0e7275784a89",
          "nonce": 5,
          "round": 1297249,
          "epoch": 1080,
          "value": "0",
          "receiver": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
          "sender": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
          "gasPrice": 1000000000,
          "gasLimit": 6000000,
          "gasUsed": 6000000,
          "data": "TXVsdGlFU0RUTkZUVHJhbnNmZXJAMDAwMDAwMDAwMDAwMDAwMDA1MDA0NjY1ZWRiYWNhNmI3NjMxMWJkYjA3ZWNjMjlhMDNiYzBlMDk5YzlkMGZkNkAwMkA0NTQ3NGM0NDJkMzAzMDMwMzAzMDMwQEAyNzEwQDUyNGY1MzQ1NTQ1NDQxMmQzMTM1MzkzNzYyMzhAQDAxQDcwNjE3OTYxNjI2YzY1NWY2MTZjNmM1Zjc0NzI2MTZlNzM2NjY1NzI3Mw==",
          "signature": "2facfcf63bab6d48fa4d0f9fbe710fddccd43e6dd1907264f3ee824a61fa91a06e8cd40e1e0eb71d3703241c4c4cbb645ba4de8bfcfb9ab38bed8fccf33b6b0b",
          "sourceShard": 1,
          "destinationShard": 1,
          "blockNonce": 1295602,
          "blockHash": "7097493c311ec6733c87cff78c12abbea294e57904ba40c93d1f95a2017646b4",
          "notarizedAtSourceInMetaNonce": 1296516,
          "NotarizedAtSourceInMetaHash": "cebb239cb97d17e83ca4a0eb114e123bf3deed1e5726c090382590a49d32342a",
          "notarizedAtDestinationInMetaNonce": 1296516,
          "notarizedAtDestinationInMetaHash": "cebb239cb97d17e83ca4a0eb114e123bf3deed1e5726c090382590a49d32342a",
          "miniblockType": "TxBlock",
          "miniblockHash": "64d77ede30339030f8b4211528ac12380e6765af607414cc41d86db67cf210a2",
          "hyperblockNonce": 1296516,
          "hyperblockHash": "cebb239cb97d17e83ca4a0eb114e123bf3deed1e5726c090382590a49d32342a",
          "timestamp": 1753198494,
          "timestampMs": 1753198494000,
          "smartContractResults": [
              {
                  "hash": "df1842706d9014a4000b0a18c8d8a19a7d468e2d969564e1665d9bd6947b8630",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                  "sender": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
                  "data": "MultiDCDTNFTTransfer@02@524557412d303030303030@00@2710@524f53455454412d313539376238@00@01@70617961626c655f616c6c5f7472616e7366657273",
                  "prevTxHash": "29c295934b5419dc10756c6ddf45468f4de3ce4a1f7ba236aeee0e7275784a89",
                  "originalTxHash": "29c295934b5419dc10756c6ddf45468f4de3ce4a1f7ba236aeee0e7275784a89",
                  "gasLimit": 5260500,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
                  "logs": {
                      "address": "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                      "events": [
                          {
                              "address": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
                              "identifier": "MultiDCDTNFTTransfer",
                              "topics": [
                                  "RUdMRC0wMDAwMDA=",
                                  "",
                                  "JxA=",
                                  "Uk9TRVRUQS0xNTk3Yjg=",
                                  "",
                                  "AQ==",
                                  "AAAAAAAAAAAFAEZl7brKa3YxG9sH7MKaA7wOCZydD9Y="
                              ],
                              "data": null,
                              "additionalData": [
                                  "",
                                  "TXVsdGlFU0RUTkZUVHJhbnNmZXI=",
                                  "Ag==",
                                  "RUdMRC0wMDAwMDA=",
                                  "AA==",
                                  "JxA=",
                                  "Uk9TRVRUQS0xNTk3Yjg=",
                                  "AA==",
                                  "AQ==",
                                  "cGF5YWJsZV9hbGxfdHJhbnNmZXJz"
                              ]
                          },
                          {
                              "address": "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                              "identifier": "writeLog",
                              "topics": [
                                  "iZRRs2GoPonXO0CW08kMIJsnh0ycfLAbsIsLtNwVaT0=",
                                  "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNTI2MDUwMCwgZ2FzIHVzZWQgPSAxNjU1NDI1"
                              ],
                              "data": "QDZmNmJAMDAwMDAwMDQ0NTQ3NGM0NDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMjI3MTAwMDAwMDAwZTUyNGY1MzQ1NTQ1NDQxMmQzMTM1MzkzNzYyMzgwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDEwMQ==",
                              "additionalData": [
                                  "QDZmNmJAMDAwMDAwMDQ0NTQ3NGM0NDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMjI3MTAwMDAwMDAwZTUyNGY1MzQ1NTQ1NDQxMmQzMTM1MzkzNzYyMzgwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDEwMQ=="
                              ]
                          },
                          {
                              "address": "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                              "identifier": "completedTxEvent",
                              "topics": [
                                  "KcKVk0tUGdwQdWxt30VGj03jzkofe6I2ru4OcnV4Sok="
                              ],
                              "data": null,
                              "additionalData": null
                          }
                      ]
                  },
                  "tokens": [
                      "REWA-000000",
                      "ROSETTA-1597b8"
                  ],
                  "dcdtValues": [
                      "10000",
                      "1"
                  ],
                  "receivers": [
                      "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                      "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7"
                  ],
                  "receiversShardIDs": [
                      2,
                      2
                  ],
                  "operation": "MultiDCDTNFTTransfer",
                  "function": "payable_all_transfers"
              },
              {
                  "hash": "2b6a15aef02cbe2329e9a3fc590493bce3ad5fbfa75e96e2086b90aec63f1e73",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                  "sender": "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
                  "data": "payable_all_transfers",
                  "prevTxHash": "df1842706d9014a4000b0a18c8d8a19a7d468e2d969564e1665d9bd6947b8630",
                  "originalTxHash": "29c295934b5419dc10756c6ddf45468f4de3ce4a1f7ba236aeee0e7275784a89",
                  "gasLimit": 5260500,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
                  "operation": "transfer",
                  "function": "payable_all_transfers"
              }
          ],
          "logs": {
              "address": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
              "events": [
                  {
                      "address": "drt13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7s2pgrqc",
                      "identifier": "MultiDCDTNFTTransfer",
                      "topics": [
                          "RUdMRC0wMDAwMDA=",
                          "",
                          "JxA=",
                          "Uk9TRVRUQS0xNTk3Yjg=",
                          "",
                          "AQ==",
                          "AAAAAAAAAAAFAEZl7brKa3YxG9sH7MKaA7wOCZydD9Y="
                      ],
                      "data": null,
                      "additionalData": [
                          "",
                          "TXVsdGlFU0RUTkZUVHJhbnNmZXI=",
                          "AAAAAAAAAAAFAEZl7brKa3YxG9sH7MKaA7wOCZydD9Y=",
                          "Ag==",
                          "RUdMRC0wMDAwMDA=",
                          "",
                          "JxA=",
                          "Uk9TRVRUQS0xNTk3Yjg=",
                          "",
                          "AQ==",
                          "cGF5YWJsZV9hbGxfdHJhbnNmZXJz"
                      ]
                  }
              ]
          },
          "status": "success",
          "tokens": [
              "REWA-000000",
              "ROSETTA-1597b8"
          ],
          "dcdtValues": [
              "10000",
              "1"
          ],
          "receivers": [
              "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7",
              "drt1qqqqqqqqqqqqqpgqgej7mwk2ddmrzx7mqlkv9xsrhs8qn8yapltqdm2fr7"
          ],
          "receiversShardIDs": [
              2,
              2
          ],
          "operation": "MultiDCDTNFTTransfer",
          "function": "payable_all_transfers",
          "initiallyPaidFee": "396105000000000",
          "fee": "396105000000000",
          "chainID": "T",
          "version": 1,
          "options": 0
      }
  },
  "error": "",
  "code": "successful"
}"#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network, ReturnCode::Success);

    let expected: Vec<u8> = vec![
        0, 0, 0, 4, 69, 71, 76, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 39, 16, 0, 0, 0, 14, 82,
        79, 83, 69, 84, 84, 65, 45, 49, 53, 57, 55, 98, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
    ];
    assert_eq!(tx_response.out, vec![expected]);
}

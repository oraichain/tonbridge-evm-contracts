/**
* This file was automatically generated by @oraichain/ts-codegen@0.35.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @oraichain/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import {Uint128, Addr, AssetInfo, InstantiateMsg, ExecuteMsg, HexBinary, Binary, UpdatePairMsg, DeletePairMsg, BridgeToTonMsg, Cw20ReceiveMsg, TokenFee, Ratio, RegisterDenomMsg, Metadata, DenomUnit, QueryMsg, MigrateMsg, Uint256, Amount, ChannelResponse, Coin, Cw20Coin, RouterController, Config, Boolean, String, PairQuery, MappingMetadata} from "./TonbridgeBridge.types";
export interface TonbridgeBridgeReadOnlyInterface {
  contractAddress: string;
  owner: () => Promise<String>;
  config: () => Promise<Config>;
  isTxProcessed: ({
    txHash
  }: {
    txHash: HexBinary;
  }) => Promise<Boolean>;
  channelStateData: () => Promise<ChannelResponse>;
  tokenFee: ({
    remoteTokenDenom
  }: {
    remoteTokenDenom: string;
  }) => Promise<Ratio>;
  pairMapping: ({
    key
  }: {
    key: string;
  }) => Promise<PairQuery>;
  sendPacketCommitment: ({
    seq
  }: {
    seq: number;
  }) => Promise<Uint256>;
  ackCommitment: ({
    seq
  }: {
    seq: number;
  }) => Promise<Uint256>;
}
export class TonbridgeBridgeQueryClient implements TonbridgeBridgeReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.owner = this.owner.bind(this);
    this.config = this.config.bind(this);
    this.isTxProcessed = this.isTxProcessed.bind(this);
    this.channelStateData = this.channelStateData.bind(this);
    this.tokenFee = this.tokenFee.bind(this);
    this.pairMapping = this.pairMapping.bind(this);
    this.sendPacketCommitment = this.sendPacketCommitment.bind(this);
    this.ackCommitment = this.ackCommitment.bind(this);
  }

  owner = async (): Promise<String> => {
    return this.client.queryContractSmart(this.contractAddress, {
      owner: {}
    });
  };
  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  isTxProcessed = async ({
    txHash
  }: {
    txHash: HexBinary;
  }): Promise<Boolean> => {
    return this.client.queryContractSmart(this.contractAddress, {
      is_tx_processed: {
        tx_hash: txHash
      }
    });
  };
  channelStateData = async (): Promise<ChannelResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      channel_state_data: {}
    });
  };
  tokenFee = async ({
    remoteTokenDenom
  }: {
    remoteTokenDenom: string;
  }): Promise<Ratio> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_fee: {
        remote_token_denom: remoteTokenDenom
      }
    });
  };
  pairMapping = async ({
    key
  }: {
    key: string;
  }): Promise<PairQuery> => {
    return this.client.queryContractSmart(this.contractAddress, {
      pair_mapping: {
        key
      }
    });
  };
  sendPacketCommitment = async ({
    seq
  }: {
    seq: number;
  }): Promise<Uint256> => {
    return this.client.queryContractSmart(this.contractAddress, {
      send_packet_commitment: {
        seq
      }
    });
  };
  ackCommitment = async ({
    seq
  }: {
    seq: number;
  }): Promise<Uint256> => {
    return this.client.queryContractSmart(this.contractAddress, {
      ack_commitment: {
        seq
      }
    });
  };
}
export interface TonbridgeBridgeInterface extends TonbridgeBridgeReadOnlyInterface {
  contractAddress: string;
  sender: string;
  readTransaction: ({
    txBoc,
    txProof
  }: {
    txBoc: HexBinary;
    txProof: HexBinary;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateMappingPair: ({
    denom,
    localAssetInfo,
    localAssetInfoDecimals,
    opcode,
    remoteDecimals,
    tokenOrigin
  }: {
    denom: string;
    localAssetInfo: AssetInfo;
    localAssetInfoDecimals: number;
    opcode: HexBinary;
    remoteDecimals: number;
    tokenOrigin: number;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  deleteMappingPair: ({
    denom
  }: {
    denom: string;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  bridgeToTon: ({
    denom,
    timeout,
    to
  }: {
    denom: string;
    timeout?: number;
    to: string;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  receive: ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateOwner: ({
    newOwner
  }: {
    newOwner: Addr;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateConfig: ({
    bridgeAdapter,
    relayerFee,
    relayerFeeReceiver,
    relayerFeeToken,
    swapRouterContract,
    tokenFee,
    tokenFeeReceiver,
    validatorContractAddr
  }: {
    bridgeAdapter?: string;
    relayerFee?: Uint128;
    relayerFeeReceiver?: Addr;
    relayerFeeToken?: AssetInfo;
    swapRouterContract?: string;
    tokenFee?: TokenFee[];
    tokenFeeReceiver?: Addr;
    validatorContractAddr?: Addr;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  registerDenom: ({
    metadata,
    subdenom
  }: {
    metadata?: Metadata;
    subdenom: string;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class TonbridgeBridgeClient extends TonbridgeBridgeQueryClient implements TonbridgeBridgeInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.readTransaction = this.readTransaction.bind(this);
    this.updateMappingPair = this.updateMappingPair.bind(this);
    this.deleteMappingPair = this.deleteMappingPair.bind(this);
    this.bridgeToTon = this.bridgeToTon.bind(this);
    this.receive = this.receive.bind(this);
    this.updateOwner = this.updateOwner.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.registerDenom = this.registerDenom.bind(this);
  }

  readTransaction = async ({
    txBoc,
    txProof
  }: {
    txBoc: HexBinary;
    txProof: HexBinary;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      read_transaction: {
        tx_boc: txBoc,
        tx_proof: txProof
      }
    }, _fee, _memo, _funds);
  };
  updateMappingPair = async ({
    denom,
    localAssetInfo,
    localAssetInfoDecimals,
    opcode,
    remoteDecimals,
    tokenOrigin
  }: {
    denom: string;
    localAssetInfo: AssetInfo;
    localAssetInfoDecimals: number;
    opcode: HexBinary;
    remoteDecimals: number;
    tokenOrigin: number;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_mapping_pair: {
        denom,
        local_asset_info: localAssetInfo,
        local_asset_info_decimals: localAssetInfoDecimals,
        opcode,
        remote_decimals: remoteDecimals,
        token_origin: tokenOrigin
      }
    }, _fee, _memo, _funds);
  };
  deleteMappingPair = async ({
    denom
  }: {
    denom: string;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      delete_mapping_pair: {
        denom
      }
    }, _fee, _memo, _funds);
  };
  bridgeToTon = async ({
    denom,
    timeout,
    to
  }: {
    denom: string;
    timeout?: number;
    to: string;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      bridge_to_ton: {
        denom,
        timeout,
        to
      }
    }, _fee, _memo, _funds);
  };
  receive = async ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      receive: {
        amount,
        msg,
        sender
      }
    }, _fee, _memo, _funds);
  };
  updateOwner = async ({
    newOwner
  }: {
    newOwner: Addr;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_owner: {
        new_owner: newOwner
      }
    }, _fee, _memo, _funds);
  };
  updateConfig = async ({
    bridgeAdapter,
    relayerFee,
    relayerFeeReceiver,
    relayerFeeToken,
    swapRouterContract,
    tokenFee,
    tokenFeeReceiver,
    validatorContractAddr
  }: {
    bridgeAdapter?: string;
    relayerFee?: Uint128;
    relayerFeeReceiver?: Addr;
    relayerFeeToken?: AssetInfo;
    swapRouterContract?: string;
    tokenFee?: TokenFee[];
    tokenFeeReceiver?: Addr;
    validatorContractAddr?: Addr;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_config: {
        bridge_adapter: bridgeAdapter,
        relayer_fee: relayerFee,
        relayer_fee_receiver: relayerFeeReceiver,
        relayer_fee_token: relayerFeeToken,
        swap_router_contract: swapRouterContract,
        token_fee: tokenFee,
        token_fee_receiver: tokenFeeReceiver,
        validator_contract_addr: validatorContractAddr
      }
    }, _fee, _memo, _funds);
  };
  registerDenom = async ({
    metadata,
    subdenom
  }: {
    metadata?: Metadata;
    subdenom: string;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register_denom: {
        metadata,
        subdenom
      }
    }, _fee, _memo, _funds);
  };
}
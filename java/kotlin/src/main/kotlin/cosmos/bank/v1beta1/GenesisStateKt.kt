//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/bank/v1beta1/genesis.proto

package cosmos.bank.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun genesisState(block: cosmos.bank.v1beta1.GenesisStateKt.Dsl.() -> Unit): cosmos.bank.v1beta1.Genesis.GenesisState =
  cosmos.bank.v1beta1.GenesisStateKt.Dsl._create(cosmos.bank.v1beta1.Genesis.GenesisState.newBuilder()).apply { block() }._build()
object GenesisStateKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.bank.v1beta1.Genesis.GenesisState.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.bank.v1beta1.Genesis.GenesisState.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.bank.v1beta1.Genesis.GenesisState = _builder.build()

    /**
     * <pre>
     * params defines all the paramaters of the module.
     * </pre>
     *
     * <code>.cosmos.bank.v1beta1.Params params = 1 [(.gogoproto.nullable) = false];</code>
     */
    var params: cosmos.bank.v1beta1.Bank.Params
      @JvmName("getParams")
      get() = _builder.getParams()
      @JvmName("setParams")
      set(value) {
        _builder.setParams(value)
      }
    /**
     * <pre>
     * params defines all the paramaters of the module.
     * </pre>
     *
     * <code>.cosmos.bank.v1beta1.Params params = 1 [(.gogoproto.nullable) = false];</code>
     */
    fun clearParams() {
      _builder.clearParams()
    }
    /**
     * <pre>
     * params defines all the paramaters of the module.
     * </pre>
     *
     * <code>.cosmos.bank.v1beta1.Params params = 1 [(.gogoproto.nullable) = false];</code>
     * @return Whether the params field is set.
     */
    fun hasParams(): kotlin.Boolean {
      return _builder.hasParams()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    class BalancesProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     */
     val balances: com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getBalancesList()
      )
    /**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     * @param value The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addBalances")
    fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>.add(value: cosmos.bank.v1beta1.Genesis.Balance) {
      _builder.addBalances(value)
    }/**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     * @param value The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignBalances")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>.plusAssign(value: cosmos.bank.v1beta1.Genesis.Balance) {
      add(value)
    }/**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     * @param values The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllBalances")
    fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>.addAll(values: kotlin.collections.Iterable<cosmos.bank.v1beta1.Genesis.Balance>) {
      _builder.addAllBalances(values)
    }/**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     * @param values The balances to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllBalances")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>.plusAssign(values: kotlin.collections.Iterable<cosmos.bank.v1beta1.Genesis.Balance>) {
      addAll(values)
    }/**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     * @param index The index to set the value at.
     * @param value The balances to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setBalances")
    operator fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>.set(index: kotlin.Int, value: cosmos.bank.v1beta1.Genesis.Balance) {
      _builder.setBalances(index, value)
    }/**
     * <pre>
     * balances is an array containing the balances of all the accounts.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Balance balances = 2 [(.gogoproto.nullable) = false];</code>
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearBalances")
    fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Genesis.Balance, BalancesProxy>.clear() {
      _builder.clearBalances()
    }
    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    class SupplyProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     */
     val supply: com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getSupplyList()
      )
    /**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param value The supply to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addSupply")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>.add(value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      _builder.addSupply(value)
    }/**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param value The supply to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignSupply")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>.plusAssign(value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      add(value)
    }/**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param values The supply to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllSupply")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>.addAll(values: kotlin.collections.Iterable<cosmos.base.v1beta1.CoinOuterClass.Coin>) {
      _builder.addAllSupply(values)
    }/**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param values The supply to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllSupply")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>.plusAssign(values: kotlin.collections.Iterable<cosmos.base.v1beta1.CoinOuterClass.Coin>) {
      addAll(values)
    }/**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param index The index to set the value at.
     * @param value The supply to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setSupply")
    operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>.set(index: kotlin.Int, value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      _builder.setSupply(index, value)
    }/**
     * <pre>
     * supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
     * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
     * </pre>
     *
     * <code>repeated .cosmos.base.v1beta1.Coin supply = 3 [(.gogoproto.nullable) = false, (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearSupply")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, SupplyProxy>.clear() {
      _builder.clearSupply()
    }
    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    class DenomMetadataProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     */
     val denomMetadata: com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getDenomMetadataList()
      )
    /**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     * @param value The denomMetadata to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addDenomMetadata")
    fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>.add(value: cosmos.bank.v1beta1.Bank.Metadata) {
      _builder.addDenomMetadata(value)
    }/**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     * @param value The denomMetadata to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignDenomMetadata")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>.plusAssign(value: cosmos.bank.v1beta1.Bank.Metadata) {
      add(value)
    }/**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     * @param values The denomMetadata to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllDenomMetadata")
    fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>.addAll(values: kotlin.collections.Iterable<cosmos.bank.v1beta1.Bank.Metadata>) {
      _builder.addAllDenomMetadata(values)
    }/**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     * @param values The denomMetadata to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllDenomMetadata")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>.plusAssign(values: kotlin.collections.Iterable<cosmos.bank.v1beta1.Bank.Metadata>) {
      addAll(values)
    }/**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     * @param index The index to set the value at.
     * @param value The denomMetadata to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setDenomMetadata")
    operator fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>.set(index: kotlin.Int, value: cosmos.bank.v1beta1.Bank.Metadata) {
      _builder.setDenomMetadata(index, value)
    }/**
     * <pre>
     * denom_metadata defines the metadata of the differents coins.
     * </pre>
     *
     * <code>repeated .cosmos.bank.v1beta1.Metadata denom_metadata = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"denom_metadata&#92;""];</code>
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearDenomMetadata")
    fun com.google.protobuf.kotlin.DslList<cosmos.bank.v1beta1.Bank.Metadata, DenomMetadataProxy>.clear() {
      _builder.clearDenomMetadata()
    }}
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.bank.v1beta1.Genesis.GenesisState.copy(block: cosmos.bank.v1beta1.GenesisStateKt.Dsl.() -> Unit): cosmos.bank.v1beta1.Genesis.GenesisState =
  cosmos.bank.v1beta1.GenesisStateKt.Dsl._create(this.toBuilder()).apply { block() }._build()

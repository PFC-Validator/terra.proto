//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: terra/treasury/v1beta1/treasury.proto

package terra.treasury.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun epochInitialIssuance(block: terra.treasury.v1beta1.EpochInitialIssuanceKt.Dsl.() -> Unit): terra.treasury.v1beta1.Treasury.EpochInitialIssuance =
  terra.treasury.v1beta1.EpochInitialIssuanceKt.Dsl._create(terra.treasury.v1beta1.Treasury.EpochInitialIssuance.newBuilder()).apply { block() }._build()
object EpochInitialIssuanceKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: terra.treasury.v1beta1.Treasury.EpochInitialIssuance.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: terra.treasury.v1beta1.Treasury.EpochInitialIssuance.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): terra.treasury.v1beta1.Treasury.EpochInitialIssuance = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    class IssuanceProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     */
     val issuance: com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getIssuanceList()
      )
    /**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param value The issuance to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addIssuance")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>.add(value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      _builder.addIssuance(value)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param value The issuance to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignIssuance")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>.plusAssign(value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      add(value)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param values The issuance to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllIssuance")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>.addAll(values: kotlin.collections.Iterable<cosmos.base.v1beta1.CoinOuterClass.Coin>) {
      _builder.addAllIssuance(values)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param values The issuance to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllIssuance")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>.plusAssign(values: kotlin.collections.Iterable<cosmos.base.v1beta1.CoinOuterClass.Coin>) {
      addAll(values)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param index The index to set the value at.
     * @param value The issuance to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setIssuance")
    operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>.set(index: kotlin.Int, value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      _builder.setIssuance(index, value)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin issuance = 1 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"issuance&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearIssuance")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, IssuanceProxy>.clear() {
      _builder.clearIssuance()
    }}
}
@kotlin.jvm.JvmSynthetic
inline fun terra.treasury.v1beta1.Treasury.EpochInitialIssuance.copy(block: terra.treasury.v1beta1.EpochInitialIssuanceKt.Dsl.() -> Unit): terra.treasury.v1beta1.Treasury.EpochInitialIssuance =
  terra.treasury.v1beta1.EpochInitialIssuanceKt.Dsl._create(this.toBuilder()).apply { block() }._build()

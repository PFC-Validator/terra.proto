//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: tendermint/abci/types.proto

package tendermint.abci;

@kotlin.jvm.JvmSynthetic
inline fun responseListSnapshots(block: tendermint.abci.ResponseListSnapshotsKt.Dsl.() -> Unit): tendermint.abci.Types.ResponseListSnapshots =
  tendermint.abci.ResponseListSnapshotsKt.Dsl._create(tendermint.abci.Types.ResponseListSnapshots.newBuilder()).apply { block() }._build()
object ResponseListSnapshotsKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: tendermint.abci.Types.ResponseListSnapshots.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: tendermint.abci.Types.ResponseListSnapshots.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): tendermint.abci.Types.ResponseListSnapshots = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    class SnapshotsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     */
     val snapshots: com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getSnapshotsList()
      )
    /**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     * @param value The snapshots to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addSnapshots")
    fun com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>.add(value: tendermint.abci.Types.Snapshot) {
      _builder.addSnapshots(value)
    }/**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     * @param value The snapshots to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignSnapshots")
    inline operator fun com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>.plusAssign(value: tendermint.abci.Types.Snapshot) {
      add(value)
    }/**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     * @param values The snapshots to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllSnapshots")
    fun com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>.addAll(values: kotlin.collections.Iterable<tendermint.abci.Types.Snapshot>) {
      _builder.addAllSnapshots(values)
    }/**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     * @param values The snapshots to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllSnapshots")
    inline operator fun com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>.plusAssign(values: kotlin.collections.Iterable<tendermint.abci.Types.Snapshot>) {
      addAll(values)
    }/**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     * @param index The index to set the value at.
     * @param value The snapshots to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setSnapshots")
    operator fun com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>.set(index: kotlin.Int, value: tendermint.abci.Types.Snapshot) {
      _builder.setSnapshots(index, value)
    }/**
     * <code>repeated .tendermint.abci.Snapshot snapshots = 1;</code>
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearSnapshots")
    fun com.google.protobuf.kotlin.DslList<tendermint.abci.Types.Snapshot, SnapshotsProxy>.clear() {
      _builder.clearSnapshots()
    }}
}
@kotlin.jvm.JvmSynthetic
inline fun tendermint.abci.Types.ResponseListSnapshots.copy(block: tendermint.abci.ResponseListSnapshotsKt.Dsl.() -> Unit): tendermint.abci.Types.ResponseListSnapshots =
  tendermint.abci.ResponseListSnapshotsKt.Dsl._create(this.toBuilder()).apply { block() }._build()

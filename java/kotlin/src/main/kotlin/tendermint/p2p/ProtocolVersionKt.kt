//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: tendermint/p2p/types.proto

package tendermint.p2p;

@kotlin.jvm.JvmSynthetic
inline fun protocolVersion(block: tendermint.p2p.ProtocolVersionKt.Dsl.() -> Unit): tendermint.p2p.Types.ProtocolVersion =
  tendermint.p2p.ProtocolVersionKt.Dsl._create(tendermint.p2p.Types.ProtocolVersion.newBuilder()).apply { block() }._build()
object ProtocolVersionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: tendermint.p2p.Types.ProtocolVersion.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: tendermint.p2p.Types.ProtocolVersion.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): tendermint.p2p.Types.ProtocolVersion = _builder.build()

    /**
     * <code>uint64 p2p = 1 [(.gogoproto.customname) = "P2P"];</code>
     */
    var p2P: kotlin.Long
      @JvmName("getP2P")
      get() = _builder.getP2P()
      @JvmName("setP2P")
      set(value) {
        _builder.setP2P(value)
      }
    /**
     * <code>uint64 p2p = 1 [(.gogoproto.customname) = "P2P"];</code>
     */
    fun clearP2P() {
      _builder.clearP2P()
    }

    /**
     * <code>uint64 block = 2;</code>
     */
    var block: kotlin.Long
      @JvmName("getBlock")
      get() = _builder.getBlock()
      @JvmName("setBlock")
      set(value) {
        _builder.setBlock(value)
      }
    /**
     * <code>uint64 block = 2;</code>
     */
    fun clearBlock() {
      _builder.clearBlock()
    }

    /**
     * <code>uint64 app = 3;</code>
     */
    var app: kotlin.Long
      @JvmName("getApp")
      get() = _builder.getApp()
      @JvmName("setApp")
      set(value) {
        _builder.setApp(value)
      }
    /**
     * <code>uint64 app = 3;</code>
     */
    fun clearApp() {
      _builder.clearApp()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun tendermint.p2p.Types.ProtocolVersion.copy(block: tendermint.p2p.ProtocolVersionKt.Dsl.() -> Unit): tendermint.p2p.Types.ProtocolVersion =
  tendermint.p2p.ProtocolVersionKt.Dsl._create(this.toBuilder()).apply { block() }._build()

//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/base/tendermint/v1beta1/query.proto

package cosmos.base.tendermint.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun module(block: cosmos.base.tendermint.v1beta1.ModuleKt.Dsl.() -> Unit): cosmos.base.tendermint.v1beta1.Query.Module =
  cosmos.base.tendermint.v1beta1.ModuleKt.Dsl._create(cosmos.base.tendermint.v1beta1.Query.Module.newBuilder()).apply { block() }._build()
object ModuleKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.base.tendermint.v1beta1.Query.Module.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.base.tendermint.v1beta1.Query.Module.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.base.tendermint.v1beta1.Query.Module = _builder.build()

    /**
     * <pre>
     * module path
     * </pre>
     *
     * <code>string path = 1;</code>
     */
    var path: kotlin.String
      @JvmName("getPath")
      get() = _builder.getPath()
      @JvmName("setPath")
      set(value) {
        _builder.setPath(value)
      }
    /**
     * <pre>
     * module path
     * </pre>
     *
     * <code>string path = 1;</code>
     */
    fun clearPath() {
      _builder.clearPath()
    }

    /**
     * <pre>
     * module version
     * </pre>
     *
     * <code>string version = 2;</code>
     */
    var version: kotlin.String
      @JvmName("getVersion")
      get() = _builder.getVersion()
      @JvmName("setVersion")
      set(value) {
        _builder.setVersion(value)
      }
    /**
     * <pre>
     * module version
     * </pre>
     *
     * <code>string version = 2;</code>
     */
    fun clearVersion() {
      _builder.clearVersion()
    }

    /**
     * <pre>
     * checksum
     * </pre>
     *
     * <code>string sum = 3;</code>
     */
    var sum: kotlin.String
      @JvmName("getSum")
      get() = _builder.getSum()
      @JvmName("setSum")
      set(value) {
        _builder.setSum(value)
      }
    /**
     * <pre>
     * checksum
     * </pre>
     *
     * <code>string sum = 3;</code>
     */
    fun clearSum() {
      _builder.clearSum()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.base.tendermint.v1beta1.Query.Module.copy(block: cosmos.base.tendermint.v1beta1.ModuleKt.Dsl.() -> Unit): cosmos.base.tendermint.v1beta1.Query.Module =
  cosmos.base.tendermint.v1beta1.ModuleKt.Dsl._create(this.toBuilder()).apply { block() }._build()

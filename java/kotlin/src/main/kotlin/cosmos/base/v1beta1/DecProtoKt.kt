//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/base/v1beta1/coin.proto

package cosmos.base.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun decProto(block: cosmos.base.v1beta1.DecProtoKt.Dsl.() -> Unit): cosmos.base.v1beta1.CoinOuterClass.DecProto =
  cosmos.base.v1beta1.DecProtoKt.Dsl._create(cosmos.base.v1beta1.CoinOuterClass.DecProto.newBuilder()).apply { block() }._build()
object DecProtoKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.base.v1beta1.CoinOuterClass.DecProto.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.base.v1beta1.CoinOuterClass.DecProto.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.base.v1beta1.CoinOuterClass.DecProto = _builder.build()

    /**
     * <code>string dec = 1 [(.gogoproto.nullable) = false, (.gogoproto.customtype) = "Dec"];</code>
     */
    var dec: kotlin.String
      @JvmName("getDec")
      get() = _builder.getDec()
      @JvmName("setDec")
      set(value) {
        _builder.setDec(value)
      }
    /**
     * <code>string dec = 1 [(.gogoproto.nullable) = false, (.gogoproto.customtype) = "Dec"];</code>
     */
    fun clearDec() {
      _builder.clearDec()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.base.v1beta1.CoinOuterClass.DecProto.copy(block: cosmos.base.v1beta1.DecProtoKt.Dsl.() -> Unit): cosmos.base.v1beta1.CoinOuterClass.DecProto =
  cosmos.base.v1beta1.DecProtoKt.Dsl._create(this.toBuilder()).apply { block() }._build()

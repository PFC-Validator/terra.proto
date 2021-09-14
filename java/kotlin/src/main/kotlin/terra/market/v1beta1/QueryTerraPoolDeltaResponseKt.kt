//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: terra/market/v1beta1/query.proto

package terra.market.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun queryTerraPoolDeltaResponse(block: terra.market.v1beta1.QueryTerraPoolDeltaResponseKt.Dsl.() -> Unit): terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse =
  terra.market.v1beta1.QueryTerraPoolDeltaResponseKt.Dsl._create(terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse.newBuilder()).apply { block() }._build()
object QueryTerraPoolDeltaResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse = _builder.build()

    /**
     * <pre>
     * terra_pool_delta defines the gap between the TerraPool and the TerraBasePool
     * </pre>
     *
     * <code>bytes terra_pool_delta = 1 [(.gogoproto.nullable) = false, (.gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec"];</code>
     */
    var terraPoolDelta: com.google.protobuf.ByteString
      @JvmName("getTerraPoolDelta")
      get() = _builder.getTerraPoolDelta()
      @JvmName("setTerraPoolDelta")
      set(value) {
        _builder.setTerraPoolDelta(value)
      }
    /**
     * <pre>
     * terra_pool_delta defines the gap between the TerraPool and the TerraBasePool
     * </pre>
     *
     * <code>bytes terra_pool_delta = 1 [(.gogoproto.nullable) = false, (.gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec"];</code>
     */
    fun clearTerraPoolDelta() {
      _builder.clearTerraPoolDelta()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse.copy(block: terra.market.v1beta1.QueryTerraPoolDeltaResponseKt.Dsl.() -> Unit): terra.market.v1beta1.QueryOuterClass.QueryTerraPoolDeltaResponse =
  terra.market.v1beta1.QueryTerraPoolDeltaResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

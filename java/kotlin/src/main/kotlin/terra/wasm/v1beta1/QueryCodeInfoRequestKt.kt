//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: terra/wasm/v1beta1/query.proto

package terra.wasm.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun queryCodeInfoRequest(block: terra.wasm.v1beta1.QueryCodeInfoRequestKt.Dsl.() -> Unit): terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest =
  terra.wasm.v1beta1.QueryCodeInfoRequestKt.Dsl._create(terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest.newBuilder()).apply { block() }._build()
object QueryCodeInfoRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest = _builder.build()

    /**
     * <pre>
     * grpc-gateway_out does not support Go style CodID
     * </pre>
     *
     * <code>uint64 code_id = 1;</code>
     */
    var codeId: kotlin.Long
      @JvmName("getCodeId")
      get() = _builder.getCodeId()
      @JvmName("setCodeId")
      set(value) {
        _builder.setCodeId(value)
      }
    /**
     * <pre>
     * grpc-gateway_out does not support Go style CodID
     * </pre>
     *
     * <code>uint64 code_id = 1;</code>
     */
    fun clearCodeId() {
      _builder.clearCodeId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest.copy(block: terra.wasm.v1beta1.QueryCodeInfoRequestKt.Dsl.() -> Unit): terra.wasm.v1beta1.QueryOuterClass.QueryCodeInfoRequest =
  terra.wasm.v1beta1.QueryCodeInfoRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

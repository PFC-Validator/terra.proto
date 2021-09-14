//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/distribution/v1beta1/query.proto

package cosmos.distribution.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun queryValidatorSlashesRequest(block: cosmos.distribution.v1beta1.QueryValidatorSlashesRequestKt.Dsl.() -> Unit): cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest =
  cosmos.distribution.v1beta1.QueryValidatorSlashesRequestKt.Dsl._create(cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest.newBuilder()).apply { block() }._build()
object QueryValidatorSlashesRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest = _builder.build()

    /**
     * <pre>
     * validator_address defines the validator address to query for.
     * </pre>
     *
     * <code>string validator_address = 1;</code>
     */
    var validatorAddress: kotlin.String
      @JvmName("getValidatorAddress")
      get() = _builder.getValidatorAddress()
      @JvmName("setValidatorAddress")
      set(value) {
        _builder.setValidatorAddress(value)
      }
    /**
     * <pre>
     * validator_address defines the validator address to query for.
     * </pre>
     *
     * <code>string validator_address = 1;</code>
     */
    fun clearValidatorAddress() {
      _builder.clearValidatorAddress()
    }

    /**
     * <pre>
     * starting_height defines the optional starting height to query the slashes.
     * </pre>
     *
     * <code>uint64 starting_height = 2;</code>
     */
    var startingHeight: kotlin.Long
      @JvmName("getStartingHeight")
      get() = _builder.getStartingHeight()
      @JvmName("setStartingHeight")
      set(value) {
        _builder.setStartingHeight(value)
      }
    /**
     * <pre>
     * starting_height defines the optional starting height to query the slashes.
     * </pre>
     *
     * <code>uint64 starting_height = 2;</code>
     */
    fun clearStartingHeight() {
      _builder.clearStartingHeight()
    }

    /**
     * <pre>
     * starting_height defines the optional ending height to query the slashes.
     * </pre>
     *
     * <code>uint64 ending_height = 3;</code>
     */
    var endingHeight: kotlin.Long
      @JvmName("getEndingHeight")
      get() = _builder.getEndingHeight()
      @JvmName("setEndingHeight")
      set(value) {
        _builder.setEndingHeight(value)
      }
    /**
     * <pre>
     * starting_height defines the optional ending height to query the slashes.
     * </pre>
     *
     * <code>uint64 ending_height = 3;</code>
     */
    fun clearEndingHeight() {
      _builder.clearEndingHeight()
    }

    /**
     * <pre>
     * pagination defines an optional pagination for the request.
     * </pre>
     *
     * <code>.cosmos.base.query.v1beta1.PageRequest pagination = 4;</code>
     */
    var pagination: cosmos.base.query.v1beta1.Pagination.PageRequest
      @JvmName("getPagination")
      get() = _builder.getPagination()
      @JvmName("setPagination")
      set(value) {
        _builder.setPagination(value)
      }
    /**
     * <pre>
     * pagination defines an optional pagination for the request.
     * </pre>
     *
     * <code>.cosmos.base.query.v1beta1.PageRequest pagination = 4;</code>
     */
    fun clearPagination() {
      _builder.clearPagination()
    }
    /**
     * <pre>
     * pagination defines an optional pagination for the request.
     * </pre>
     *
     * <code>.cosmos.base.query.v1beta1.PageRequest pagination = 4;</code>
     * @return Whether the pagination field is set.
     */
    fun hasPagination(): kotlin.Boolean {
      return _builder.hasPagination()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest.copy(block: cosmos.distribution.v1beta1.QueryValidatorSlashesRequestKt.Dsl.() -> Unit): cosmos.distribution.v1beta1.QueryOuterClass.QueryValidatorSlashesRequest =
  cosmos.distribution.v1beta1.QueryValidatorSlashesRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: ibc/core/client/v1/query.proto

package ibc.core.client.v1;

@kotlin.jvm.JvmSynthetic
inline fun queryConsensusStateRequest(block: ibc.core.client.v1.QueryConsensusStateRequestKt.Dsl.() -> Unit): ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest =
  ibc.core.client.v1.QueryConsensusStateRequestKt.Dsl._create(ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest.newBuilder()).apply { block() }._build()
object QueryConsensusStateRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest = _builder.build()

    /**
     * <pre>
     * client identifier
     * </pre>
     *
     * <code>string client_id = 1;</code>
     */
    var clientId: kotlin.String
      @JvmName("getClientId")
      get() = _builder.getClientId()
      @JvmName("setClientId")
      set(value) {
        _builder.setClientId(value)
      }
    /**
     * <pre>
     * client identifier
     * </pre>
     *
     * <code>string client_id = 1;</code>
     */
    fun clearClientId() {
      _builder.clearClientId()
    }

    /**
     * <pre>
     * consensus state revision number
     * </pre>
     *
     * <code>uint64 revision_number = 2;</code>
     */
    var revisionNumber: kotlin.Long
      @JvmName("getRevisionNumber")
      get() = _builder.getRevisionNumber()
      @JvmName("setRevisionNumber")
      set(value) {
        _builder.setRevisionNumber(value)
      }
    /**
     * <pre>
     * consensus state revision number
     * </pre>
     *
     * <code>uint64 revision_number = 2;</code>
     */
    fun clearRevisionNumber() {
      _builder.clearRevisionNumber()
    }

    /**
     * <pre>
     * consensus state revision height
     * </pre>
     *
     * <code>uint64 revision_height = 3;</code>
     */
    var revisionHeight: kotlin.Long
      @JvmName("getRevisionHeight")
      get() = _builder.getRevisionHeight()
      @JvmName("setRevisionHeight")
      set(value) {
        _builder.setRevisionHeight(value)
      }
    /**
     * <pre>
     * consensus state revision height
     * </pre>
     *
     * <code>uint64 revision_height = 3;</code>
     */
    fun clearRevisionHeight() {
      _builder.clearRevisionHeight()
    }

    /**
     * <pre>
     * latest_height overrrides the height field and queries the latest stored
     * ConsensusState
     * </pre>
     *
     * <code>bool latest_height = 4;</code>
     */
    var latestHeight: kotlin.Boolean
      @JvmName("getLatestHeight")
      get() = _builder.getLatestHeight()
      @JvmName("setLatestHeight")
      set(value) {
        _builder.setLatestHeight(value)
      }
    /**
     * <pre>
     * latest_height overrrides the height field and queries the latest stored
     * ConsensusState
     * </pre>
     *
     * <code>bool latest_height = 4;</code>
     */
    fun clearLatestHeight() {
      _builder.clearLatestHeight()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest.copy(block: ibc.core.client.v1.QueryConsensusStateRequestKt.Dsl.() -> Unit): ibc.core.client.v1.QueryOuterClass.QueryConsensusStateRequest =
  ibc.core.client.v1.QueryConsensusStateRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

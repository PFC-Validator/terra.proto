//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: tendermint/abci/types.proto

package tendermint.abci;

@kotlin.jvm.JvmSynthetic
inline fun requestCommit(block: tendermint.abci.RequestCommitKt.Dsl.() -> Unit): tendermint.abci.Types.RequestCommit =
  tendermint.abci.RequestCommitKt.Dsl._create(tendermint.abci.Types.RequestCommit.newBuilder()).apply { block() }._build()
object RequestCommitKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: tendermint.abci.Types.RequestCommit.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: tendermint.abci.Types.RequestCommit.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): tendermint.abci.Types.RequestCommit = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
inline fun tendermint.abci.Types.RequestCommit.copy(block: tendermint.abci.RequestCommitKt.Dsl.() -> Unit): tendermint.abci.Types.RequestCommit =
  tendermint.abci.RequestCommitKt.Dsl._create(this.toBuilder()).apply { block() }._build()
//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/crypto/ed25519/keys.proto

package cosmos.crypto.ed25519;

@kotlin.jvm.JvmSynthetic
inline fun privKey(block: cosmos.crypto.ed25519.PrivKeyKt.Dsl.() -> Unit): cosmos.crypto.ed25519.Keys.PrivKey =
  cosmos.crypto.ed25519.PrivKeyKt.Dsl._create(cosmos.crypto.ed25519.Keys.PrivKey.newBuilder()).apply { block() }._build()
object PrivKeyKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.crypto.ed25519.Keys.PrivKey.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.crypto.ed25519.Keys.PrivKey.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.crypto.ed25519.Keys.PrivKey = _builder.build()

    /**
     * <code>bytes key = 1 [(.gogoproto.casttype) = "crypto/ed25519.PrivateKey"];</code>
     */
    var key: com.google.protobuf.ByteString
      @JvmName("getKey")
      get() = _builder.getKey()
      @JvmName("setKey")
      set(value) {
        _builder.setKey(value)
      }
    /**
     * <code>bytes key = 1 [(.gogoproto.casttype) = "crypto/ed25519.PrivateKey"];</code>
     */
    fun clearKey() {
      _builder.clearKey()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.crypto.ed25519.Keys.PrivKey.copy(block: cosmos.crypto.ed25519.PrivKeyKt.Dsl.() -> Unit): cosmos.crypto.ed25519.Keys.PrivKey =
  cosmos.crypto.ed25519.PrivKeyKt.Dsl._create(this.toBuilder()).apply { block() }._build()

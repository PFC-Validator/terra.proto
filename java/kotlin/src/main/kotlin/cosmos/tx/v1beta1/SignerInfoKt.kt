//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/tx/v1beta1/tx.proto

package cosmos.tx.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun signerInfo(block: cosmos.tx.v1beta1.SignerInfoKt.Dsl.() -> Unit): cosmos.tx.v1beta1.TxOuterClass.SignerInfo =
  cosmos.tx.v1beta1.SignerInfoKt.Dsl._create(cosmos.tx.v1beta1.TxOuterClass.SignerInfo.newBuilder()).apply { block() }._build()
object SignerInfoKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.tx.v1beta1.TxOuterClass.SignerInfo.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.tx.v1beta1.TxOuterClass.SignerInfo.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.tx.v1beta1.TxOuterClass.SignerInfo = _builder.build()

    /**
     * <pre>
     * public_key is the public key of the signer. It is optional for accounts
     * that already exist in state. If unset, the verifier can use the required &#92;
     * signer address for this position and lookup the public key.
     * </pre>
     *
     * <code>.google.protobuf.Any public_key = 1;</code>
     */
    var publicKey: com.google.protobuf.Any
      @JvmName("getPublicKey")
      get() = _builder.getPublicKey()
      @JvmName("setPublicKey")
      set(value) {
        _builder.setPublicKey(value)
      }
    /**
     * <pre>
     * public_key is the public key of the signer. It is optional for accounts
     * that already exist in state. If unset, the verifier can use the required &#92;
     * signer address for this position and lookup the public key.
     * </pre>
     *
     * <code>.google.protobuf.Any public_key = 1;</code>
     */
    fun clearPublicKey() {
      _builder.clearPublicKey()
    }
    /**
     * <pre>
     * public_key is the public key of the signer. It is optional for accounts
     * that already exist in state. If unset, the verifier can use the required &#92;
     * signer address for this position and lookup the public key.
     * </pre>
     *
     * <code>.google.protobuf.Any public_key = 1;</code>
     * @return Whether the publicKey field is set.
     */
    fun hasPublicKey(): kotlin.Boolean {
      return _builder.hasPublicKey()
    }

    /**
     * <pre>
     * mode_info describes the signing mode of the signer and is a nested
     * structure to support nested multisig pubkey's
     * </pre>
     *
     * <code>.cosmos.tx.v1beta1.ModeInfo mode_info = 2;</code>
     */
    var modeInfo: cosmos.tx.v1beta1.TxOuterClass.ModeInfo
      @JvmName("getModeInfo")
      get() = _builder.getModeInfo()
      @JvmName("setModeInfo")
      set(value) {
        _builder.setModeInfo(value)
      }
    /**
     * <pre>
     * mode_info describes the signing mode of the signer and is a nested
     * structure to support nested multisig pubkey's
     * </pre>
     *
     * <code>.cosmos.tx.v1beta1.ModeInfo mode_info = 2;</code>
     */
    fun clearModeInfo() {
      _builder.clearModeInfo()
    }
    /**
     * <pre>
     * mode_info describes the signing mode of the signer and is a nested
     * structure to support nested multisig pubkey's
     * </pre>
     *
     * <code>.cosmos.tx.v1beta1.ModeInfo mode_info = 2;</code>
     * @return Whether the modeInfo field is set.
     */
    fun hasModeInfo(): kotlin.Boolean {
      return _builder.hasModeInfo()
    }

    /**
     * <pre>
     * sequence is the sequence of the account, which describes the
     * number of committed transactions signed by a given address. It is used to
     * prevent replay attacks.
     * </pre>
     *
     * <code>uint64 sequence = 3;</code>
     */
    var sequence: kotlin.Long
      @JvmName("getSequence")
      get() = _builder.getSequence()
      @JvmName("setSequence")
      set(value) {
        _builder.setSequence(value)
      }
    /**
     * <pre>
     * sequence is the sequence of the account, which describes the
     * number of committed transactions signed by a given address. It is used to
     * prevent replay attacks.
     * </pre>
     *
     * <code>uint64 sequence = 3;</code>
     */
    fun clearSequence() {
      _builder.clearSequence()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.tx.v1beta1.TxOuterClass.SignerInfo.copy(block: cosmos.tx.v1beta1.SignerInfoKt.Dsl.() -> Unit): cosmos.tx.v1beta1.TxOuterClass.SignerInfo =
  cosmos.tx.v1beta1.SignerInfoKt.Dsl._create(this.toBuilder()).apply { block() }._build()

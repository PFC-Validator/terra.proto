//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/staking/v1beta1/staking.proto

package cosmos.staking.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun description(block: cosmos.staking.v1beta1.DescriptionKt.Dsl.() -> Unit): cosmos.staking.v1beta1.Staking.Description =
  cosmos.staking.v1beta1.DescriptionKt.Dsl._create(cosmos.staking.v1beta1.Staking.Description.newBuilder()).apply { block() }._build()
object DescriptionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.staking.v1beta1.Staking.Description.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.staking.v1beta1.Staking.Description.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.staking.v1beta1.Staking.Description = _builder.build()

    /**
     * <pre>
     * moniker defines a human-readable name for the validator.
     * </pre>
     *
     * <code>string moniker = 1;</code>
     */
    var moniker: kotlin.String
      @JvmName("getMoniker")
      get() = _builder.getMoniker()
      @JvmName("setMoniker")
      set(value) {
        _builder.setMoniker(value)
      }
    /**
     * <pre>
     * moniker defines a human-readable name for the validator.
     * </pre>
     *
     * <code>string moniker = 1;</code>
     */
    fun clearMoniker() {
      _builder.clearMoniker()
    }

    /**
     * <pre>
     * identity defines an optional identity signature (ex. UPort or Keybase).
     * </pre>
     *
     * <code>string identity = 2;</code>
     */
    var identity: kotlin.String
      @JvmName("getIdentity")
      get() = _builder.getIdentity()
      @JvmName("setIdentity")
      set(value) {
        _builder.setIdentity(value)
      }
    /**
     * <pre>
     * identity defines an optional identity signature (ex. UPort or Keybase).
     * </pre>
     *
     * <code>string identity = 2;</code>
     */
    fun clearIdentity() {
      _builder.clearIdentity()
    }

    /**
     * <pre>
     * website defines an optional website link.
     * </pre>
     *
     * <code>string website = 3;</code>
     */
    var website: kotlin.String
      @JvmName("getWebsite")
      get() = _builder.getWebsite()
      @JvmName("setWebsite")
      set(value) {
        _builder.setWebsite(value)
      }
    /**
     * <pre>
     * website defines an optional website link.
     * </pre>
     *
     * <code>string website = 3;</code>
     */
    fun clearWebsite() {
      _builder.clearWebsite()
    }

    /**
     * <pre>
     * security_contact defines an optional email for security contact.
     * </pre>
     *
     * <code>string security_contact = 4 [(.gogoproto.moretags) = "yaml:&#92;"security_contact&#92;""];</code>
     */
    var securityContact: kotlin.String
      @JvmName("getSecurityContact")
      get() = _builder.getSecurityContact()
      @JvmName("setSecurityContact")
      set(value) {
        _builder.setSecurityContact(value)
      }
    /**
     * <pre>
     * security_contact defines an optional email for security contact.
     * </pre>
     *
     * <code>string security_contact = 4 [(.gogoproto.moretags) = "yaml:&#92;"security_contact&#92;""];</code>
     */
    fun clearSecurityContact() {
      _builder.clearSecurityContact()
    }

    /**
     * <pre>
     * details define other optional details.
     * </pre>
     *
     * <code>string details = 5;</code>
     */
    var details: kotlin.String
      @JvmName("getDetails")
      get() = _builder.getDetails()
      @JvmName("setDetails")
      set(value) {
        _builder.setDetails(value)
      }
    /**
     * <pre>
     * details define other optional details.
     * </pre>
     *
     * <code>string details = 5;</code>
     */
    fun clearDetails() {
      _builder.clearDetails()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.staking.v1beta1.Staking.Description.copy(block: cosmos.staking.v1beta1.DescriptionKt.Dsl.() -> Unit): cosmos.staking.v1beta1.Staking.Description =
  cosmos.staking.v1beta1.DescriptionKt.Dsl._create(this.toBuilder()).apply { block() }._build()

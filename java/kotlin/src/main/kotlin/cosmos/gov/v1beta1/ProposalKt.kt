//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: cosmos/gov/v1beta1/gov.proto

package cosmos.gov.v1beta1;

@kotlin.jvm.JvmSynthetic
inline fun proposal(block: cosmos.gov.v1beta1.ProposalKt.Dsl.() -> Unit): cosmos.gov.v1beta1.Gov.Proposal =
  cosmos.gov.v1beta1.ProposalKt.Dsl._create(cosmos.gov.v1beta1.Gov.Proposal.newBuilder()).apply { block() }._build()
object ProposalKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: cosmos.gov.v1beta1.Gov.Proposal.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: cosmos.gov.v1beta1.Gov.Proposal.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): cosmos.gov.v1beta1.Gov.Proposal = _builder.build()

    /**
     * <code>uint64 proposal_id = 1 [(.gogoproto.jsontag) = "id", (.gogoproto.moretags) = "yaml:&#92;"id&#92;""];</code>
     */
    var proposalId: kotlin.Long
      @JvmName("getProposalId")
      get() = _builder.getProposalId()
      @JvmName("setProposalId")
      set(value) {
        _builder.setProposalId(value)
      }
    /**
     * <code>uint64 proposal_id = 1 [(.gogoproto.jsontag) = "id", (.gogoproto.moretags) = "yaml:&#92;"id&#92;""];</code>
     */
    fun clearProposalId() {
      _builder.clearProposalId()
    }

    /**
     * <code>.google.protobuf.Any content = 2 [(.cosmos_proto.accepts_interface) = "Content"];</code>
     */
    var content: com.google.protobuf.Any
      @JvmName("getContent")
      get() = _builder.getContent()
      @JvmName("setContent")
      set(value) {
        _builder.setContent(value)
      }
    /**
     * <code>.google.protobuf.Any content = 2 [(.cosmos_proto.accepts_interface) = "Content"];</code>
     */
    fun clearContent() {
      _builder.clearContent()
    }
    /**
     * <code>.google.protobuf.Any content = 2 [(.cosmos_proto.accepts_interface) = "Content"];</code>
     * @return Whether the content field is set.
     */
    fun hasContent(): kotlin.Boolean {
      return _builder.hasContent()
    }

    /**
     * <code>.cosmos.gov.v1beta1.ProposalStatus status = 3 [(.gogoproto.moretags) = "yaml:&#92;"proposal_status&#92;""];</code>
     */
    var status: cosmos.gov.v1beta1.Gov.ProposalStatus
      @JvmName("getStatus")
      get() = _builder.getStatus()
      @JvmName("setStatus")
      set(value) {
        _builder.setStatus(value)
      }
    /**
     * <code>.cosmos.gov.v1beta1.ProposalStatus status = 3 [(.gogoproto.moretags) = "yaml:&#92;"proposal_status&#92;""];</code>
     */
    fun clearStatus() {
      _builder.clearStatus()
    }

    /**
     * <code>.cosmos.gov.v1beta1.TallyResult final_tally_result = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"final_tally_result&#92;""];</code>
     */
    var finalTallyResult: cosmos.gov.v1beta1.Gov.TallyResult
      @JvmName("getFinalTallyResult")
      get() = _builder.getFinalTallyResult()
      @JvmName("setFinalTallyResult")
      set(value) {
        _builder.setFinalTallyResult(value)
      }
    /**
     * <code>.cosmos.gov.v1beta1.TallyResult final_tally_result = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"final_tally_result&#92;""];</code>
     */
    fun clearFinalTallyResult() {
      _builder.clearFinalTallyResult()
    }
    /**
     * <code>.cosmos.gov.v1beta1.TallyResult final_tally_result = 4 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"final_tally_result&#92;""];</code>
     * @return Whether the finalTallyResult field is set.
     */
    fun hasFinalTallyResult(): kotlin.Boolean {
      return _builder.hasFinalTallyResult()
    }

    /**
     * <code>.google.protobuf.Timestamp submit_time = 5 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"submit_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    var submitTime: com.google.protobuf.Timestamp
      @JvmName("getSubmitTime")
      get() = _builder.getSubmitTime()
      @JvmName("setSubmitTime")
      set(value) {
        _builder.setSubmitTime(value)
      }
    /**
     * <code>.google.protobuf.Timestamp submit_time = 5 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"submit_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    fun clearSubmitTime() {
      _builder.clearSubmitTime()
    }
    /**
     * <code>.google.protobuf.Timestamp submit_time = 5 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"submit_time&#92;"", (.gogoproto.stdtime) = true];</code>
     * @return Whether the submitTime field is set.
     */
    fun hasSubmitTime(): kotlin.Boolean {
      return _builder.hasSubmitTime()
    }

    /**
     * <code>.google.protobuf.Timestamp deposit_end_time = 6 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"deposit_end_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    var depositEndTime: com.google.protobuf.Timestamp
      @JvmName("getDepositEndTime")
      get() = _builder.getDepositEndTime()
      @JvmName("setDepositEndTime")
      set(value) {
        _builder.setDepositEndTime(value)
      }
    /**
     * <code>.google.protobuf.Timestamp deposit_end_time = 6 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"deposit_end_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    fun clearDepositEndTime() {
      _builder.clearDepositEndTime()
    }
    /**
     * <code>.google.protobuf.Timestamp deposit_end_time = 6 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"deposit_end_time&#92;"", (.gogoproto.stdtime) = true];</code>
     * @return Whether the depositEndTime field is set.
     */
    fun hasDepositEndTime(): kotlin.Boolean {
      return _builder.hasDepositEndTime()
    }

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    class TotalDepositProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     */
     val totalDeposit: com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getTotalDepositList()
      )
    /**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param value The totalDeposit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addTotalDeposit")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>.add(value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      _builder.addTotalDeposit(value)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param value The totalDeposit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignTotalDeposit")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>.plusAssign(value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      add(value)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param values The totalDeposit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllTotalDeposit")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>.addAll(values: kotlin.collections.Iterable<cosmos.base.v1beta1.CoinOuterClass.Coin>) {
      _builder.addAllTotalDeposit(values)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param values The totalDeposit to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllTotalDeposit")
    inline operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>.plusAssign(values: kotlin.collections.Iterable<cosmos.base.v1beta1.CoinOuterClass.Coin>) {
      addAll(values)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     * @param index The index to set the value at.
     * @param value The totalDeposit to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setTotalDeposit")
    operator fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>.set(index: kotlin.Int, value: cosmos.base.v1beta1.CoinOuterClass.Coin) {
      _builder.setTotalDeposit(index, value)
    }/**
     * <code>repeated .cosmos.base.v1beta1.Coin total_deposit = 7 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"total_deposit&#92;"", (.gogoproto.castrepeated) = "github.com/cosmos/cosmos-sdk/types.Coins"];</code>
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearTotalDeposit")
    fun com.google.protobuf.kotlin.DslList<cosmos.base.v1beta1.CoinOuterClass.Coin, TotalDepositProxy>.clear() {
      _builder.clearTotalDeposit()
    }
    /**
     * <code>.google.protobuf.Timestamp voting_start_time = 8 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"voting_start_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    var votingStartTime: com.google.protobuf.Timestamp
      @JvmName("getVotingStartTime")
      get() = _builder.getVotingStartTime()
      @JvmName("setVotingStartTime")
      set(value) {
        _builder.setVotingStartTime(value)
      }
    /**
     * <code>.google.protobuf.Timestamp voting_start_time = 8 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"voting_start_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    fun clearVotingStartTime() {
      _builder.clearVotingStartTime()
    }
    /**
     * <code>.google.protobuf.Timestamp voting_start_time = 8 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"voting_start_time&#92;"", (.gogoproto.stdtime) = true];</code>
     * @return Whether the votingStartTime field is set.
     */
    fun hasVotingStartTime(): kotlin.Boolean {
      return _builder.hasVotingStartTime()
    }

    /**
     * <code>.google.protobuf.Timestamp voting_end_time = 9 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"voting_end_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    var votingEndTime: com.google.protobuf.Timestamp
      @JvmName("getVotingEndTime")
      get() = _builder.getVotingEndTime()
      @JvmName("setVotingEndTime")
      set(value) {
        _builder.setVotingEndTime(value)
      }
    /**
     * <code>.google.protobuf.Timestamp voting_end_time = 9 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"voting_end_time&#92;"", (.gogoproto.stdtime) = true];</code>
     */
    fun clearVotingEndTime() {
      _builder.clearVotingEndTime()
    }
    /**
     * <code>.google.protobuf.Timestamp voting_end_time = 9 [(.gogoproto.nullable) = false, (.gogoproto.moretags) = "yaml:&#92;"voting_end_time&#92;"", (.gogoproto.stdtime) = true];</code>
     * @return Whether the votingEndTime field is set.
     */
    fun hasVotingEndTime(): kotlin.Boolean {
      return _builder.hasVotingEndTime()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun cosmos.gov.v1beta1.Gov.Proposal.copy(block: cosmos.gov.v1beta1.ProposalKt.Dsl.() -> Unit): cosmos.gov.v1beta1.Gov.Proposal =
  cosmos.gov.v1beta1.ProposalKt.Dsl._create(this.toBuilder()).apply { block() }._build()

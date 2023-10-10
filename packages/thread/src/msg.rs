use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Uint64};

use crate::{
    config::Config,
    thread::{Thread, ThreadMsg},
    user::User,
};

// TODO: P0: add a proxy contract that can charge custom fee so people can build tailored frontend

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    // Membership contract address, must be provided
    pub membership_contract_addr: String,
    // Default to sender
    pub admin_addr: Option<String>,
    // Default to sender
    pub protocol_fee_collector_addr: Option<String>,

    // Default to 100
    pub max_thread_title_length: Option<Uint64>,
    // Default to 500
    pub max_thread_description_length: Option<Uint64>,
    // Max length of a single thread label
    pub max_thread_label_length: Option<Uint64>,
    // Max number of thread labels
    pub max_number_of_thread_labels: Option<Uint64>,
    // Default to 500
    pub max_thread_msg_length: Option<Uint64>,

    // Protocol fee for starting a new thread
    pub protocol_fee_start_new_thread_fixed_cost: Option<Uint128>,
    // Protocol fee percentage for asking in a thread
    pub protocol_fee_ask_in_thread_fee_percentage: Option<Uint64>,
    // Protocol fee percentage for replying in a thread
    pub protocol_fee_reply_in_thread_fee_percentage: Option<Uint64>,

    // Default ask me fee in my 1 membership price percentage
    pub default_ask_fee_percentage_of_membership: Option<Uint64>,
    // How much to pay thread creator when someone ask in thread
    pub default_ask_fee_to_thread_creator_percentage_of_membership: Option<Uint64>,
    // Default reply to me in my thread or my msg fee in my 1 membership price percentage
    pub default_reply_fee_percentage_of_membership: Option<Uint64>,

    // Default thread fee to membership issuer fee percentage
    pub default_share_to_issuer_percentage: Option<Uint64>,
    // Default thread fee to membership holder fee percentage
    pub default_share_to_all_members_percentage: Option<Uint64>,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    Enable(EnableMsg),
    Disable(DisableMsg),

    UpdateMembershipContractAddr(UpdateMembershipContractAddrMsg),

    UpdateConfig(UpdateConfigMsg),

    // Only membership issuer can update its ask fee percentage
    UpdateAskFeePercentageOfMembership(UpdateAskFeePercentageOfMembershipMsg),

    // Only membership issuer can update its ask fee to creator percentage
    UpdateAskFeeToThreadCreatorPercentageOfMembership(
        UpdateAskFeeToThreadCreatorPercentageOfMembershipMsg,
    ),

    // Only membership issuer can update its reply fee percentage
    UpdateReplyFeePercentageOfMembership(UpdateReplyFeePercentageOfMembershipMsg),

    // Only membership issuer can update its thread fee config
    UpdateThreadFeeShareConfig(UpdateThreadFeeShareConfigMsg),

    // Anyone can start a new thread
    StartNewThread(StartNewThreadMsg),

    // Membership holder can ask question to membership issuer in an existing thread or a new thread
    AskInThread(AskInThreadMsg),

    // Membership issuer can answer question to membership holder in an existing thread
    AnswerInThread(AnswerInThreadMsg),

    // You can reply as long as you hold the membership of the thread creator
    // And the membership of the msg creator (if replying to a msg)
    ReplyInThread(ReplyInThreadMsg),
}

#[cw_serde]
pub struct EnableMsg {}

#[cw_serde]
pub struct DisableMsg {}

#[cw_serde]
pub struct UpdateMembershipContractAddrMsg {
    pub membership_contract_addr: String,
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub admin_addr: Option<String>,
    pub registration_admin_addr: Option<String>,
    pub protocol_fee_collector_addr: Option<String>,

    pub max_thread_title_length: Option<Uint64>,
    pub max_thread_description_length: Option<Uint64>,
    pub max_thread_label_length: Option<Uint64>,
    pub max_number_of_thread_labels: Option<Uint64>,
    pub max_thread_msg_length: Option<Uint64>,

    pub protocol_fee_start_new_thread_fixed_cost: Option<Uint128>,
    pub protocol_fee_ask_in_thread_fee_percentage: Option<Uint64>,
    pub protocol_fee_reply_in_thread_fee_percentage: Option<Uint64>,

    pub default_ask_fee_percentage_of_membership: Option<Uint64>,
    pub default_ask_fee_to_thread_creator_percentage_of_membership: Option<Uint64>,
    pub default_reply_fee_percentage_of_membership: Option<Uint64>,

    pub default_share_to_issuer_percentage: Option<Uint64>,
    pub default_share_to_all_members_percentage: Option<Uint64>,
}

#[cw_serde]
pub struct UpdateAskFeePercentageOfMembershipMsg {
    pub membership_issuer_addr: String,
    pub ask_fee_percentage_of_membership: Uint64,
}

#[cw_serde]
pub struct UpdateAskFeeToThreadCreatorPercentageOfMembershipMsg {
    pub membership_issuer_addr: String,
    pub ask_fee_to_thread_creator_percentage_of_membership: Uint64,
}

#[cw_serde]
pub struct UpdateReplyFeePercentageOfMembershipMsg {
    pub membership_issuer_addr: String,
    pub reply_fee_percentage_of_membership: Uint64,
}

#[cw_serde]
pub struct UpdateThreadFeeShareConfigMsg {
    pub membership_issuer_addr: String,
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Uint64,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Uint64,
}

#[cw_serde]
pub struct StartNewThreadMsg {
    // Thread title
    pub title: String,
    // Thread description
    pub description: String,
    // List of labels
    pub labels: Vec<String>,
}

#[cw_serde]
pub struct AskInThreadMsg {
    // New to start a new thread, default to false
    pub start_new_thread: Option<bool>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_title: Option<String>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_description: Option<String>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_labels: Option<Vec<String>>,
    // Thread ID to ask question in
    // If start_new_thread is false, this field must be filled
    // Else start_new_thread is true, this field will be ignored
    pub thread_id: Option<Uint64>,
    // The address of the membership issuer that the user wants to ask question to
    pub ask_to_addr: String,
    // Question content
    pub content: String,
}

#[cw_serde]
pub struct AnswerInThreadMsg {
    // Thread ID to answer question in
    pub thread_id: Uint64,
    // Answer must be replying to a specific question in a thread
    pub question_id: Uint64,
    // Answer content
    pub content: String,
}

#[cw_serde]
pub struct ReplyInThreadMsg {
    // Thread ID to reply in
    pub thread_id: Uint64,
    // Reply can reply to a specific msg in a thread or the thread itself
    pub reply_to_thread_msg_id: Option<Uint64>,
    // Reply content
    pub content: String,
}

// ========== query ==========

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig(QueryConfigMsg),

    #[returns(UserResponse)]
    QueryUser(QueryUserMsg),

    // QueryCostToStartNewThread calculates the fee needed to ask a question
    #[returns(CostToStartNewThreadResponse)]
    QueryCostToStartNewThread(QueryCostToStartNewThreadMsg),

    // QueryCostToAsk calculates the fee needed to ask a question
    #[returns(CostToAskInThreadResponse)]
    QueryCostToAskInThread(QueryCostToAskInThreadMsg),

    // NOTE: answer has no cost
    #[returns(CostToReplyInThreadResponse)]
    QueryCostToReplyInThread(QueryCostToReplyInThreadMsg),

    #[returns(IDsOfAllThreadsUserBelongToResponse)]
    QueryIDsOfAllThreadsUserBelongTo(QueryIDsOfAllThreadsUserBelongToMsg),

    #[returns(IDsOfAllThreadsUserCreatedResponse)]
    QueryIDsOfAllThreadsUserCreated(QueryIDsOfAllThreadsUserCreatedMsg),

    #[returns(IDsOfAllThreadMsgsInThreadResponse)]
    QueryIDsOfAllThreadMsgsInThread(QueryIDsOfAllThreadMsgsInThreadMsg),

    #[returns(ThreadsResponse)]
    QueryThreadsByIDs(QueryThreadsByIDsMsg),

    #[returns(ThreadMsgsResponse)]
    QueryThreadMsgsByIDs(QueryThreadMsgsByIDsMsg),
}

#[cw_serde]
pub struct QueryConfigMsg {}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct QueryUserMsg {
    pub user_addr: String,
}

#[cw_serde]
pub struct UserResponse {
    pub user: User,
}

#[cw_serde]
pub struct QueryCostToStartNewThreadMsg {
    pub description_len: Uint64,
}

#[cw_serde]
pub struct CostToStartNewThreadResponse {
    pub protocol_fee: Uint128,
}

#[cw_serde]
pub struct QueryCostToAskInThreadMsg {
    // The address of user asking question
    pub asker_addr: String,
    // The address of the membership issuer that the user wants to ask question to
    pub ask_to_addr: String,
    // The address of the thread creator
    pub thread_creator_addr: String,
    // Number of characters in question content
    pub content_len: Uint64,
}

#[cw_serde]
pub struct CostToAskInThreadResponse {
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to answerer membership issuer
    pub ask_to_membership_issuer_fee: Uint128,
    // Fee paid to answerer membership holders
    pub ask_to_membership_holder_fee: Uint128,
    // Fee paid to thread creator membership issuer, 0 if asker is the thread creator
    pub thread_creator_membership_issuer_fee: Uint128,
    // Fee paid to thread creator membership holders, 0 if asker is the thread creator
    pub thread_creator_membership_holder_fee: Uint128,
    // Protocol fee + answer membership issuer fee + answer membership holder fee
    // + thread creator membership issuer fee + thread creator membership holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryCostToReplyInThreadMsg {
    // The address of user replying
    pub replier_addr: String,
    // The address of the membership issuer that the user wants to reply to
    // Either a msg (reply or question or answer) owner or a thread owner
    pub reply_to_addr: String,
    // The address of the thread creator
    pub thread_creator_addr: String,
    // Number of characters in question content
    pub content_len: Uint64,
}

#[cw_serde]
pub struct CostToReplyInThreadResponse {
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to membership issuer
    pub reply_to_membership_issuer_fee: Uint128,
    // Fee paid to all membership holders
    pub reply_to_membership_holder_fee: Uint128,
    // NOTE: reply doesn't pay thread creator now
    // // Fee paid to thread creator membership issuer, 0 if replier is the thread creator
    // pub thread_creator_membership_issuer_fee: Uint128,
    // // Fee paid to thread creator membership holders, 0 if replier is the thread creator
    // pub thread_creator_membership_holder_fee: Uint128,
    // Protocol fee + reply to membership issuer fee + reply to membership holder fee
    // + thread creator membership issuer fee + thread creator membership holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryIDsOfAllThreadsUserBelongToMsg {
    pub user_addr: String,
    pub start_after_thread_id: Option<Uint64>,
    pub limit: Option<u32>,
    pub include_start_after: Option<bool>,
}

#[cw_serde]
pub struct IDsOfAllThreadsUserBelongToResponse {
    pub thread_ids: Vec<Uint64>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryIDsOfAllThreadsUserCreatedMsg {
    pub user_addr: String,
    pub start_after_thread_id: Option<Uint64>,
    pub limit: Option<u32>,
    pub include_start_after: Option<bool>,
}

#[cw_serde]
pub struct IDsOfAllThreadsUserCreatedResponse {
    pub thread_ids: Vec<Uint64>,
    pub count: usize,
    pub total_count: usize,
}

// This means QueryIDsOfAllThreadMsgsInThread Msg, because query msg always ends with Msg
#[cw_serde]
pub struct QueryIDsOfAllThreadMsgsInThreadMsg {
    pub thread_id: Uint64,
    pub start_after_thread_msg_id: Option<Uint64>,
    pub limit: Option<u32>,
    pub include_start_after: Option<bool>,
}

#[cw_serde]
pub struct IDsOfAllThreadMsgsInThreadResponse {
    pub thread_msg_ids: Vec<Uint64>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryThreadsByIDsMsg {
    pub thread_ids: Vec<Uint64>,
}

#[cw_serde]
pub struct ThreadsResponse {
    pub threads: Vec<Thread>,
}

// You need both thread ID and thread msg ID to identity a thread msg
// Because thread msg ID is unique only within a thread
#[cw_serde]
pub struct QueryThreadMsgsByIDsMsg {
    pub thread_and_thread_msg_ids: Vec<(Uint64, Uint64)>,
}

#[cw_serde]
pub struct ThreadMsgsResponse {
    pub thread_msgs: Vec<ThreadMsg>,
}

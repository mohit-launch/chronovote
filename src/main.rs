mod decay;
mod threshold;
mod threshold_prog;
mod voter;
mod weight;
mod window;
mod blockchain;

use std::thread::sleep;

use crate::decay::*;
use crate::threshold::*;
use crate::threshold_prog::*;
use crate::voter::*;
use crate::weight::*;
use crate::window::*;
use crate::blockchain::*;

use chrono::Utc;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use rust_decimal_macros::dec;
use serde_json::{json, Value};

fn main() {
    println!("🪙 Blockchain-based Time-Decay Threshold Voting Simulation\n");

    // === Initialize Blockchain ===
    let mut blockchain = Blockchain::new();

    // === Proposal setup ===
    let now = Utc::now();
    let proposal_id = "proposal_1".to_string();
    let proposer_id = "admin".to_string();
    let voting_window = VotingWindow::Medium;

    let mut proposal_manager = ProposalManager::new(60);
    proposal_manager.add_proposal(proposal_id.clone(), proposer_id.clone(), voting_window);

    let vote_start = now;

    // === Simulate voters ===
    let mut csprng = OsRng;
    let voters = vec!["Alice", "Bob", "Charlie", "Dave", "Eve"];
    let validators = vec!["Val1", "Val2", "Val3", "Val4", "Val5"];

    let decay_model = DecayModel::Exponential(0.001);
    let mut weight_engine = WeightEngine::new();

    // Reputation bonuses
    weight_engine.set_reputation(&"Alice".to_string(), dec!(0.1));
    weight_engine.set_reputation(&"Bob".to_string(), dec!(0.05));
    weight_engine.set_reputation(&"Eve".to_string(), dec!(0.2));

    let mut signed_votes = vec![];

    println!("📥 Collecting votes...\n");

    for (i, voter_name) in voters.iter().enumerate() {
        let signing_key = SigningKey::generate(&mut csprng);

        let vote_time = now;

        let vote = Vote {
            voter_id: voter_name.to_string(),
            validator_id: validators[i].to_string(),
            vote_time,
            vote_weight: 1.0,
        };

        let signed_vote = vote.sign(&signing_key);

        if signed_vote.verify() {
            println!(
                "✅ {}'s vote verified at {}",
                voter_name, vote.vote_time
            );
            signed_votes.push(signed_vote);
        } else {
            println!("❌ {}'s vote INVALID.", voter_name);
        }
    }

    // === Compute weights ===
    println!("\n📊 Computing effective weights...\n");

    for signed_vote in &signed_votes {
        let rep_bonus = weight_engine
            .reputation
            .get(&signed_vote.vote.voter_id)
            .cloned()
            .unwrap_or(dec!(0.0));

        let weighted_vote = WeightedVote {
            voter_id: signed_vote.vote.voter_id.clone(),
            vote_time: signed_vote.vote.vote_time,
            orig_weight: dec!(1.0),
            decay_model: decay_model.clone(),
            reputation_bonus: rep_bonus,
        };

        let eff_weight =
            weight_engine.calculate_and_cache(&weighted_vote, &vote_start, now);

        println!(
            "📊 {} effective weight: {:.3} (at {})",
            weighted_vote.voter_id, eff_weight, weighted_vote.vote_time
        );
    }

    // === Threshold check ===
    println!("\n🔍 Checking threshold requirement...\n");

    let total_votes = signed_votes.len();
    let yes_votes = signed_votes.len(); // assume all YES
    let req = requirement_for_type(Proposaltype::Normal);

    println!(
        "📝 Threshold: min_percentage {:.2}, min_yes_votes {}",
        req.min_percentage, req.max_abs
    );

    let result = if req.is_met(yes_votes, total_votes) {
        println!("🎉 Proposal PASSED.");
        "PASSED"
    } else {
        println!("🚫 Proposal FAILED.");
        "FAILED"
    };

    // === Save proposal to blockchain ===
    println!("\n⛓️ Adding proposal to blockchain...\n");

    let vote_jsons: Vec<String> = signed_votes
        .iter()
        .map(|v| serde_json::to_string(&v.vote).unwrap())
        .collect();

    let data = json!({
        "proposal_id": proposal_id,
        "votes": vote_jsons,
        "result": result
    })
    .to_string();

    blockchain.add_blocks(data);

    // === Print blockchain ===
    println!("📜 Blockchain Ledger:\n");
    for blk in &blockchain.blocks {
        println!(
            "🧱 Block {} | Time: {} | Hash: {} | Prev: {}",
            blk.index, blk.timestamp, blk.hash, blk.prev_hash
        );

        if blk.index > 0 {
            if let Ok(json_val) = serde_json::from_str::<Value>(&blk.data) {
                println!("{}", serde_json::to_string_pretty(&json_val).unwrap());
            }
        }
        println!("---");
    }

    if blockchain.is_valid() {
        println!("✅ Blockchain integrity: VALID");
    } else {
        println!("🚨 Blockchain integrity: INVALID");
    }
}

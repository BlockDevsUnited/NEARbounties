use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Contains balance and allowances information for one account.
#[near_bindgen]
pub struct Bounties{
    pub U128 numBounties,
    pub Map<Vec<U128>, Bounty> bounties,
    pub admin:Vec<u8>,
}

enum State{
    intialized,
    approved,
    completed,
}

impl Bounties{
    pub fn new(_admin:Vec<u8>)->Self{
       Self{numBounties:0,admin:_admin}
   }
   pub fn postOpenBounty(_name:string,_description:string,_deadline:u48,_creatorIndex:AccountId,_available:u32,&self){
    let Temp:Bounty=Bounty::new(_available,0,_creatorIndex:,_deadline,_name,_description).unwrap();
    self.bounties.insert(self.numSubmissions+1,Temp);
   }
   pub fn postPersonalBounty(_deadline:u48,_name:string,_description:string,_creatorIndex:AccountId,_bountyHunter:u32,&self){
    let Temp:Bounty=Bounty::new(1,0,_bountyHunter,_creatorIndex:,_deadline,_name,_description).unwrap();
    self.bounties.insert(self.numSubmissions+1,Temp);
   }

   pub fn getReward(_owner_id:AccountId,_bountyID:U128,_submissionIndex:U128,&self){
    assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
    let TempBounty=self.bounties.get(_bountyID).unwrap();
    let TempSub=TempBounty.submissions.get(_submissionIndex).unwrap();
    assert!(TempSub.submitterIndex==owner_id,"sender must the ");
    assert!(Temp.status==State::approved,"status must be in an approved state");
    transferBounty( TempBounty.value,TempSub.submitterIndex);
    TempSub.complete();
   }

   pub fn submit(_bountyID:U128,_submissionString:string,_submitterIndex:u32,&self){
    let TempBounty=self.bounties.get(_bountyID).unwrap();
    assert!();
    if(TempBounty.)
    TempBounty.submissions.insert(TempBounty.numSubmissions+1,submission::new(_submitterIndex,_submissionString));
   }
   fn transferBounty(_value:U128,_receipient:AccountId){

   }
   pub fn getBountyData(_bountyID:U128,&self):Bounty{
     self.bounties.get(_bountyID).unwrap()
   }
}


pub struct Bounty {
    pub u8 available,       
    pub u8 numSubmissions,
    pub u32 hunterIndex,
    pub AccountId creatorIndex,
    pub U128 value;
    pub u48 deadline,
    pub string name,
    pub string description,   
    pub submissions: Map<Vec<U128>, submission>,
    pub string SEA_id,
    pub string resolver_address
}

pub struct submission{
    pub u32 submitterIndex,
    pub string submissionString,
    pub State status,
    pub Map<Vec<U128>, revision> revisions,
    pub u8 numRevisions,
}

pub struct revision{
    pub u8 revisionIndex,
    pub string revisionString,
}

impl Bounty{
    pub fn new(_available:u8,_hunterIndex:u32,_creatorIndex:AccountId,_deadline:u48,_name:string,_description:string) -> Self{
        Self{available:_available,numSubmissions:0,hunterIndex:_hunterIndex,creatorIndex:_creatorIndex,deadline:_deadline,name:_name,description:_description}
    }
    pub fn approveSubmission(_submissionIndex:u8,&self){
        let sub=self.submissions.get(_submissionIndex).unwrap();
        sub.approve();
        self.submissions.insert(_submissionIndex,sub);
    }
    pub fn completeSubmission(_submissionIndex:u8,&self){
        let sub=self.submissions.get(_submissionIndex).unwrap();
        sub.complete();
        self.submissions.insert(_submissionIndex,sub);
    }
    pub fn createSubmission(_index:u32_submissionString:string,&self){
        let subTotal =self.numSubmissions;
        let tempSubmssion=submission::new(_index,_submissionString);
        self.submissions.insert(subTotal+1,tempSubmssion);
    }
}

impl submission{
     fn new(_submitterIndex:u32,_submissionString:string) -> Self{
        Self{submitterIndex:_submitterIndex,submissionString:_submissionString,status:State::intialized,numRevision:0}
    }
    fn createRevision(_revision:String,&self){
        let total =&self.numRevisions;
        self.revisions.insert(revision::new(total+1,_revision));

    }
    fn approve(&self){
        self.status=State::approved;
    }
    fn complete(&self){
        self.status=State::completed;
    }
}

impl revision{
     fn new(_revisionIndex:u8,_revisionString:string) -> Self{
        Self{revisionIndex:_revisionIndex,revisionString:_revisionString}
    }
}
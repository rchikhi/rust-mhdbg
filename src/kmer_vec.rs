use super::revcomp_aware;
use std::hash::{Hash, Hasher};
use std::vec::Vec;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct KmerVec 
{
    data: Vec<u32>
}

impl KmerVec
{

    pub fn suffix(&self) -> KmerVec
    {
        let mut res = KmerVec { data: self.data.clone() };
        res.data.remove(0);
        res
    }
    
    pub fn prefix(&self) -> KmerVec
    {
        let mut res = KmerVec { data: self.data.clone() };
        res.data.pop();
        //println!("{:?} after {:?}",self.data,res.data);
        res
    }
    
    pub fn reverse(&self) -> KmerVec
    {
        let mut res = KmerVec { data: self.data.clone() };
        res.data.reverse();
        res
    }

    pub fn normalize(&self) -> (KmerVec,bool)
    {
        if !revcomp_aware { return (self.clone(), false)}
        let rev = self.reverse();
        if self.clone() < rev {
            (self.clone(), false)
        }
        else {
            (rev, true)
        }
    }

    pub fn make_from(ar: &[u32]) -> KmerVec
    {
        KmerVec {
        data: Vec::from(ar)
        }
    }

    pub fn print_as_string(&self) -> String
    {
        format!("{:?}",&self.data)
    }

    pub fn minimizers(&self) -> &Vec<u32>
    {
        &self.data
    }

}

impl PartialEq for KmerVec
{
    fn eq(&self, other: &KmerVec) -> bool {
        self.data == other.data
    }
}

impl Eq for KmerVec
{
}

impl Hash for KmerVec
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Default for KmerVec {
        fn default() -> Self { KmerVec { data: vec![] } }
}

impl Ord for KmerVec {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}


impl PartialOrd for KmerVec{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

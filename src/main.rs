use ink::primitives::Hash;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Provide the name of the event, e.g. Test::Created");
        return;
    }
    println!("{}", event_hash(&args[1]))
}

fn event_hash(name: &String) -> String {
    hex(encoded_into_hash(&PrefixedValue {
        value: &name.as_bytes(),
        prefix: b"",
    }))
}

fn hex(hash: Hash) -> String {
    hash.as_ref()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}

fn encoded_into_hash<T>(entity: &T) -> Hash
where
    T: scale::Encode,
{
    use ink::{
        env::hash::{Blake2x256, CryptoHash, HashOutput},
        primitives::Clear,
    };

    let mut result = Hash::CLEAR_HASH;
    let len_result = result.as_ref().len();
    let encoded = entity.encode();
    let len_encoded = encoded.len();
    if len_encoded <= len_result {
        result.as_mut()[..len_encoded].copy_from_slice(&encoded);
        return result;
    }
    let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
    <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
    let copy_len = core::cmp::min(hash_output.len(), len_result);
    result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
    result
}

/// For calculating the event topic hash.
struct PrefixedValue<'a, 'b> {
    pub prefix: &'a [u8],
    pub value: &'b [u8],
}

impl scale::Encode for PrefixedValue<'_, '_> {
    #[inline]
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        self.prefix.encode_to(dest);
        dest.write(self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events() {
        assert_eq!(
            "0045726332303a3a5472616e7366657200000000000000000000000000000000",
            event_hash(&"Erc20::Transfer".to_string())
        );
        assert_eq!(
            "00546573743a3a43726561746564000000000000000000000000000000000000",
            event_hash(&"Test::Created".to_string())
        );
    }
}

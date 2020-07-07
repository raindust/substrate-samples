// Tests to be written here

use crate::{*, mock::*};
use frame_support::{assert_ok};

#[test]
fn test_json_deserialize() {
    let info = GithubInfo {
        login: b"raindust".to_vec(),
        blog: b"www.raindust.xyz".to_vec(),
        public_repos: 43,
    };
    println!("print demo github info: {:?}", info);

    let json = "{\"login\":\"raindust\",\"blog\":\"www.raindust.xyz\",\"public_repos\":43}";
    let deserialized = serde_json::from_str::<GithubInfo>(&json).unwrap();

    assert_eq!(info.public_repos, deserialized.public_repos);
    assert_eq!(info.blog, deserialized.blog);
    assert_eq!(info.login, deserialized.login);
}


use crate::{prelude::*, protocols::ethernet2, test};
use float_duration::FloatDuration;
use serde_yaml;
use std::time::{Duration, Instant};

#[test]
fn immediate_reply() {
    // tests to ensure that an are request results in a reply.
    let now = Instant::now();
    let mut alice = test::new_alice(now);
    let mut bob = test::new_bob(now);
    let mut carrie = test::new_carrie(now);

    // this test is written based on certain assumptions.
    let options = alice.options();
    assert_eq!(
        options.arp.request_timeout.unwrap(),
        FloatDuration::seconds(1.0)
    );

    let fut = alice.arp_query(*test::carrie_ipv4_addr());
    let now = now + Duration::from_millis(1);
    match fut.poll(now) {
        Err(Fail::TryAgain {}) => (),
        x => panic!("expected Fail::TryAgain {{}}, got `{:?}`", x),
    }

    let mut request = {
        let effect = alice.poll(now).expect("expected an effect");
        match effect {
            Effect::Transmit(packet) => packet.to_vec(),
        }
    };

    assert!(request.len() >= ethernet2::MIN_PAYLOAD_SIZE);

    // bob hasn't heard of alice before, so he will ignore the request.
    match bob.receive(&mut request) {
        Err(Fail::Ignored {}) => (),
        x => panic!("expected Fail::Ignored {{}}, got `{:?}`", x),
    }
    let cache = bob.export_arp_cache();
    assert!(cache.get(test::alice_ipv4_addr()).is_none());

    carrie.receive(&mut request).unwrap();
    let cache = carrie.export_arp_cache();
    assert_eq!(
        cache.get(test::alice_ipv4_addr()),
        Some(test::alice_link_addr())
    );
    let mut reply = {
        let effect = carrie.poll(now).expect("expected an effect");
        match effect {
            Effect::Transmit(packet) => packet.to_vec(),
        }
    };

    alice.receive(&mut reply).unwrap();
    debug!(
        "ARP cache contains: \n{}",
        serde_yaml::to_string(&alice.export_arp_cache()).unwrap()
    );
    let now = now + Duration::from_millis(1);
    match fut.poll(now) {
        Ok(link_addr) => assert_eq!(*test::carrie_link_addr(), link_addr),
        x => panic!("expected future completion, got `{:?}`", x),
    }
}

#[test]
fn slow_reply() {
    // tests to ensure that an are request results in a reply.
    let now = Instant::now();
    let mut alice = test::new_alice(now);
    let mut bob = test::new_bob(now);
    let mut carrie = test::new_carrie(now);

    // this test is written based on certain assumptions.
    let options = alice.options();
    assert!(options.arp.retry_count.unwrap() > 0);
    assert_eq!(
        options.arp.request_timeout.unwrap(),
        FloatDuration::seconds(1.0)
    );

    let fut = alice.arp_query(*test::carrie_ipv4_addr());
    // move time forward enough to trigger a timeout.
    let now = now + Duration::from_secs(1);
    match fut.poll(now) {
        Err(Fail::TryAgain {}) => (),
        x => panic!("expected Fail::TryAgain {{}}, got `{:?}`", x),
    }

    let mut request = {
        let effect = alice.poll(now).expect("expected an effect");
        match effect {
            Effect::Transmit(packet) => packet.to_vec(),
        }
    };

    assert!(request.len() >= ethernet2::MIN_PAYLOAD_SIZE);

    // bob hasn't heard of alice before, so he will ignore the request.
    match bob.receive(&mut request) {
        Err(Fail::Ignored {}) => (),
        x => panic!("expected Fail::Ignored {{}}, got `{:?}`", x),
    }
    let cache = bob.export_arp_cache();
    assert!(cache.get(test::alice_ipv4_addr()).is_none());

    carrie.receive(&mut request).unwrap();
    let cache = carrie.export_arp_cache();
    assert_eq!(
        cache.get(test::alice_ipv4_addr()),
        Some(test::alice_link_addr())
    );
    let mut reply = {
        let effect = carrie.poll(now).expect("expected an effect");
        match effect {
            Effect::Transmit(packet) => packet.to_vec(),
        }
    };

    alice.receive(&mut reply).unwrap();
    debug!(
        "ARP cache contains: \n{}",
        serde_yaml::to_string(&alice.export_arp_cache()).unwrap()
    );
    let now = now + Duration::from_millis(1);
    match fut.poll(now) {
        Ok(link_addr) => assert_eq!(*test::carrie_link_addr(), link_addr),
        x => panic!("expected future completion, got `{:?}`", x),
    }
}

#[test]
fn no_reply() {
    // tests to ensure that an are request results in a reply.
    let now = Instant::now();
    let alice = test::new_alice(now);

    // this test is written based on certain assumptions.
    let options = alice.options();
    assert_eq!(options.arp.retry_count.unwrap(), 2);
    assert_eq!(
        options.arp.request_timeout.unwrap(),
        FloatDuration::seconds(1.0)
    );

    let fut = alice.arp_query(*test::carrie_ipv4_addr());

    // move time forward enough to trigger a timeout.
    let now = now + Duration::from_secs(1);
    match fut.poll(now) {
        Err(Fail::TryAgain {}) => (),
        x => panic!("expected Fail::TryAgain {{}}, got `{:?}`", x),
    }

    // retry #1
    let now = now + Duration::from_secs(1);
    match fut.poll(now) {
        Err(Fail::TryAgain {}) => (),
        x => panic!("expected Fail::TryAgain {{}}, got `{:?}`", x),
    }

    // retry #2
    let now = now + Duration::from_secs(1);
    match fut.poll(now) {
        Err(Fail::Timeout {}) => (),
        x => panic!("expected Fail::Timeout {{}}, got `{:?}`", x),
    }
}

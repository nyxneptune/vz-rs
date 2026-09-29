use super::codec::parse;
use super::expansion::expand;
use super::{Matrix, copy_n_times};

#[test]
fn test_prss() {
    // Asserting equality to vec![] is messy because the compiler can't infer types there...
    for i in 0..5 {
        assert!(expand(&vec![], i).is_empty());
        assert!(expand(&parse("0"), i).is_empty());
        assert_eq!(expand(&parse("0 0"), i), parse("0"));
        assert_eq!(expand(&parse("0 1"), i), copy_n_times(&parse("0"), i));
        assert_eq!(expand(&parse("0 1 0"), i), parse("0 1"));
        assert_eq!(
            expand(&parse("0 1 0 1"), i),
            [parse("0 1"), copy_n_times(&parse("0"), i)].concat()
        );
        assert_eq!(expand(&parse("0 1 1"), i), copy_n_times(&parse("0 1"), i));
        assert_eq!(
            expand(&parse("0 1 2"), i),
            [parse("0"), copy_n_times(&parse("1"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 2 1"), i),
            copy_n_times(&parse("0 1 2"), i)
        );
        assert_eq!(
            expand(&parse("0 1 2 2"), i),
            [parse("0"), copy_n_times(&parse("1 2"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 2 3"), i),
            [parse("0 1"), copy_n_times(&parse("2"), i)].concat()
        );
    }
}

#[test]
fn test_later_prss_like() {
    for i in 0..5 {
        assert_eq!(expand(&parse("0 1 21 0"), i), parse("0 1 21"));
        assert_eq!(
            expand(&parse("0 1 21 1"), i),
            copy_n_times(&parse("0 1 21"), i)
        );
        assert_eq!(
            expand(&parse("0 1 21 2"), i),
            [parse("0"), copy_n_times(&parse("1 21"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 2 3"), i),
            [parse("0 1 21"), copy_n_times(&parse("2"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 2 31 3"), i),
            [parse("0 1 21"), copy_n_times(&parse("2 31"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3"), i),
            [parse("0 1"), copy_n_times(&parse("21"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 1 21 3"), i),
            [parse("0 1 21 3 1"), copy_n_times(&parse("21"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 2 1 21 3"), i),
            [parse("0 1 21 3 2 1"), copy_n_times(&parse("21"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 21 3"), i),
            [parse("0 1 21 3"), copy_n_times(&parse("21"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 3"), i),
            [parse("0 1"), copy_n_times(&parse("21 3"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 4"), i),
            [parse("0 1 21"), copy_n_times(&parse("3"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 41 3"), i),
            [parse("0 1"), copy_n_times(&parse("21 3 41"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 41 4"), i),
            [parse("0 1 21"), copy_n_times(&parse("3 41"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 41 4 52 51 6"), i),
            [parse("0 1 21 3 41 4 52"), copy_n_times(&parse("51"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 41 41 4 52 51 5"), i),
            [parse("0 1 21 3 41 41"), copy_n_times(&parse("4 52 51"), i)].concat()
        );
        assert_eq!(
            expand(&parse("0 1 21 3 41 5"), i),
            [parse("0 1 21 3"), copy_n_times(&parse("41"), i)].concat()
        );
    }
}

#[test]
fn test_limit() {
    assert!(expand(&parse("0 1 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21"), 2), parse("0 1"));
    assert_eq!(expand(&parse("0 1 21"), 3), parse("0 1 2"));
    assert_eq!(expand(&parse("0 1 21"), 4), parse("0 1 2 3"));

    assert_eq!(expand(&parse("0 1 21 321"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 321"), 1), parse("0 1"));
    assert_eq!(expand(&parse("0 1 21 321"), 2), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 321"), 3), parse("0 1 21 32"));
    assert_eq!(expand(&parse("0 1 21 321"), 4), parse("0 1 21 32 43"));

    assert_eq!(expand(&parse("0 1 21 321 4321"), 0), parse("0 1"));
    assert_eq!(expand(&parse("0 1 21 321 4321"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 321 4321"), 2), parse("0 1 21 321"));
    assert_eq!(
        expand(&parse("0 1 21 321 4321"), 3),
        parse("0 1 21 321 432")
    );
    assert_eq!(
        expand(&parse("0 1 21 321 4321"), 4),
        parse("0 1 21 321 432 543")
    );

    assert_eq!(expand(&parse("0 1 21 321 4321 54321"), 0), parse("0 1 21"));
    assert_eq!(
        expand(&parse("0 1 21 321 4321 54321"), 1),
        parse("0 1 21 321")
    );
    assert_eq!(
        expand(&parse("0 1 21 321 4321 54321"), 2),
        parse("0 1 21 321 4321")
    );
    assert_eq!(
        expand(&parse("0 1 21 321 4321 54321"), 3),
        parse("0 1 21 321 4321 5432")
    );
    assert_eq!(
        expand(&parse("0 1 21 321 4321 54321"), 4),
        parse("0 1 21 321 4321 5432 6543")
    );
}

#[test]
fn test_eps0() {
    assert_eq!(expand(&parse("0 1 21 1 2 31"), 0), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 1 2 31"), 1), parse("0 1 21 1"));
    assert_eq!(expand(&parse("0 1 21 1 2 31"), 2), parse("0 1 21 1 2"));
    assert_eq!(expand(&parse("0 1 21 1 2 31"), 3), parse("0 1 21 1 2 3"));
    assert_eq!(expand(&parse("0 1 21 1 2 31"), 4), parse("0 1 21 1 2 3 4"));

    assert_eq!(expand(&parse("0 1 21 2 3 41"), 0), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 2 3 41"), 1), parse("0 1 21 2"));
    assert_eq!(expand(&parse("0 1 21 2 3 41"), 2), parse("0 1 21 2 3"));
    assert_eq!(expand(&parse("0 1 21 2 3 41"), 3), parse("0 1 21 2 3 4"));
    assert_eq!(expand(&parse("0 1 21 2 3 41"), 4), parse("0 1 21 2 3 4 5"));

    assert_eq!(expand(&parse("0 1 21 3 4 51"), 0), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 3 4 51"), 1), parse("0 1 21 3"));
    assert_eq!(expand(&parse("0 1 21 3 4 51"), 2), parse("0 1 21 3 4"));
    assert_eq!(expand(&parse("0 1 21 3 4 51"), 3), parse("0 1 21 3 4 5"));
    assert_eq!(expand(&parse("0 1 21 3 4 51"), 4), parse("0 1 21 3 4 5 6"));

    assert_eq!(
        expand(&parse("0 1 21 3 41 3 4 51"), 0),
        parse("0 1 21 3 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 4 51"), 1),
        parse("0 1 21 3 41 3")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 4 51"), 2),
        parse("0 1 21 3 41 3 4")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 4 51"), 3),
        parse("0 1 21 3 41 3 4 5")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 4 51"), 4),
        parse("0 1 21 3 41 3 4 5 6")
    );
}

#[test]
fn test_dbmslike() {
    assert!(expand(&parse("0 1 21 321 321"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 321 321"), 1), parse("0 1"));
    assert_eq!(expand(&parse("0 1 21 321 321"), 2), parse("0 1 21 321"));
    assert_eq!(
        expand(&parse("0 1 21 321 321"), 3),
        parse("0 1 21 321 32 432")
    );
    assert_eq!(
        expand(&parse("0 1 21 321 321"), 4),
        parse("0 1 21 321 32 432 43 543")
    );
}

#[test]
fn test_pss() {
    assert!(expand(&parse("0 1 21 1 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 1 21"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 1 21"), 2), parse("0 1 21 1 2 31"));
    assert_eq!(
        expand(&parse("0 1 21 1 21"), 3),
        parse("0 1 21 1 2 31 2 3 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 1 21"), 4),
        parse("0 1 21 1 2 31 2 3 41 3 4 51")
    );

    assert!(expand(&parse("0 1 21 2 31"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 2 31"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 2 31"), 2), parse("0 1 21 2 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 2 31"), 3),
        parse("0 1 21 2 3 41 4 5 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 2 31"), 4),
        parse("0 1 21 2 3 41 4 5 61 6 7 81")
    );

    assert!(expand(&parse("0 1 21 2 31 3 41"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 2 31 3 41"), 1), parse("0 1 21 2 31"));
    assert_eq!(
        expand(&parse("0 1 21 2 31 3 41"), 2),
        parse("0 1 21 2 31 3 4 51 5 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 2 31 3 41"), 3),
        parse("0 1 21 2 31 3 4 51 5 61 6 7 81 8 91")
    );
    assert_eq!(
        expand(&parse("0 1 21 2 31 3 41"), 4),
        parse("0 1 21 2 31 3 4 51 5 61 6 7 81 8 91 9 A B1 B C1")
    );

    assert_eq!(expand(&parse("0 1 21 2 32"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 2 32"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 2 32"), 2), parse("0 1 21 2 31"));
    assert_eq!(expand(&parse("0 1 21 2 32"), 3), parse("0 1 21 2 31 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 2 32"), 4),
        parse("0 1 21 2 31 3 41 4 51")
    );

    assert_eq!(expand(&parse("0 1 21 2 32 3 43"), 0), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 2 32 3 43"), 1), parse("0 1 21 2 32"));
    assert_eq!(
        expand(&parse("0 1 21 2 32 3 43"), 2),
        parse("0 1 21 2 32 3 42")
    );
    assert_eq!(
        expand(&parse("0 1 21 2 32 3 43"), 3),
        parse("0 1 21 2 32 3 42 4 52")
    );
    assert_eq!(
        expand(&parse("0 1 21 2 32 3 43"), 4),
        parse("0 1 21 2 32 3 42 4 52 5 62")
    );
}

#[test]
fn test_tss() {
    assert!(expand(&parse("0 1 21 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 21"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 21"), 2), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 21"), 3), parse("0 1 21 2 32"));
    assert_eq!(expand(&parse("0 1 21 21"), 4), parse("0 1 21 2 32 3 43"));

    assert!(expand(&parse("0 1 21 21 2 1 21 21"), 0).is_empty());
    assert_eq!(
        expand(&parse("0 1 21 21 2 1 21 21"), 1),
        parse("0 1 21 21 2")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 1 21 21"), 2),
        parse("0 1 21 21 2 1 21 2 32 31 3")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 1 21 21"), 3),
        parse("0 1 21 21 2 1 21 2 32 31 3 2 32 3 43 41 4")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 1 21 21"), 4),
        parse("0 1 21 21 2 1 21 2 32 31 3 2 32 3 43 41 4 3 43 4 54 51 5")
    );

    assert_eq!(
        expand(&parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 31"), 0),
        parse("0 1 21 21 2 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 31"), 1),
        parse("0 1 21 21 2 31 1 21 2 32 31 3 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 31"), 2),
        parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 3 43 41 4 51")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 31"), 3),
        parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 3 43 41 4 51 3 43 4 54 51 5 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 31"), 4),
        parse(
            "0 1 21 21 2 31 1 21 2 32 31 3 41 2 32 3 43 41 4 51 3 43 4 54 51 5 61 4 54 5 65 61 6 \
             71"
        )
    );

    assert!(expand(&parse("0 1 21 21 2 31 31"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 21 2 31 31"), 1), parse("0 1 21 21"));
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 31"), 2),
        parse("0 1 21 21 2 31 3 42 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 31"), 3),
        parse("0 1 21 21 2 31 3 42 41 4 52 5 63 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 31 31"), 4),
        parse("0 1 21 21 2 31 3 42 41 4 52 5 63 61 6 73 7 84 81")
    );

    assert_eq!(expand(&parse("0 1 21 21 2 32"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 21 2 32"), 1), parse("0 1 21 21"));
    assert_eq!(
        expand(&parse("0 1 21 21 2 32"), 2),
        parse("0 1 21 21 2 31 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 32"), 3),
        parse("0 1 21 21 2 31 31 3 41 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 32"), 4),
        parse("0 1 21 21 2 31 31 3 41 41 4 51 51")
    );

    assert!(expand(&parse("0 1 21 21 2 32 31"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 21 2 32 31"), 1), parse("0 1 21 21"));
    assert_eq!(
        expand(&parse("0 1 21 21 2 32 31"), 2),
        parse("0 1 21 21 2 32 3 43 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 32 31"), 3),
        parse("0 1 21 21 2 32 3 43 41 4 54 5 65 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 32 31"), 4),
        parse("0 1 21 21 2 32 3 43 41 4 54 5 65 61 6 76 7 87 81")
    );

    assert_eq!(expand(&parse("0 1 21 21 2 32 32"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 21 2 32 32"), 1), parse("0 1 21 21"));
    assert_eq!(
        expand(&parse("0 1 21 21 2 32 32"), 2),
        parse("0 1 21 21 2 32 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 32 32"), 3),
        parse("0 1 21 21 2 32 31 3 43 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 2 32 32"), 4),
        parse("0 1 21 21 2 32 31 3 43 41 4 54 51")
    );
}

#[test]
fn test_qss() {
    assert!(expand(&parse("0 1 21 21 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 21 21"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 21 21"), 2), parse("0 1 21 21"));
    assert_eq!(
        expand(&parse("0 1 21 21 21"), 3),
        parse("0 1 21 21 2 32 32")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 21"), 4),
        parse("0 1 21 21 2 32 32 3 43 43")
    );

    assert_eq!(expand(&parse("0 1 21 21 21 2 32 32 32"), 0), parse("0"));
    assert_eq!(
        expand(&parse("0 1 21 21 21 2 32 32 32"), 1),
        parse("0 1 21 21 21")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 21 2 32 32 32"), 2),
        parse("0 1 21 21 21 2 32 32 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 21 2 32 32 32"), 3),
        parse("0 1 21 21 21 2 32 32 31 3 43 43 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 21 21 2 32 32 32"), 4),
        parse("0 1 21 21 21 2 32 32 31 3 43 43 41 4 54 54 51")
    );
}

// It's easy to go wrong in the range between 0 1 21 3 and 0 1 21 3 21. A naive algorithm that gets
// all of the above right would probably get many of the below wrong, especially e.g. 0 1 21 3 2 32
// 31 and 0 1 21 3 2 32 32.
#[test]
fn test_later_bms_like() {
    assert!(expand(&parse("0 1 21 3 2 31 1 21 2 32 31 4 3 41"), 0).is_empty());
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 1 21 2 32 31 4 3 41"), 1),
        parse("0 1 21 3 2 31 1 21 2 32 31 4")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 1 21 2 32 31 4 3 41"), 2),
        parse("0 1 21 3 2 31 1 21 2 32 31 4 3 4 51 6 5 61 4 51 5 62 61 7")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 1 21 2 32 31 4 3 41"), 3),
        parse(
            "0 1 21 3 2 31 1 21 2 32 31 4 3 4 51 6 5 61 4 51 5 62 61 7 6 7 81 9 8 91 7 81 8 92 91 \
             A"
        )
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 1 21 2 32 31 4 3 41"), 4),
        parse(
            "0 1 21 3 2 31 1 21 2 32 31 4 3 4 51 6 5 61 4 51 5 62 61 7 6 7 81 9 8 91 7 81 8 92 91 \
             A 9 A B1 C B C1 A B1 B C2 C1 D"
        )
    );

    assert!(expand(&parse("0 1 21 3 2 31 31"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 2 31 31"), 1), parse("0 1 21 3"));
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 31"), 2),
        parse("0 1 21 3 2 31 3 42 41 5")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 31"), 3),
        parse("0 1 21 3 2 31 3 42 41 5 4 52 5 63 61 7")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 31 31"), 4),
        parse("0 1 21 3 2 31 3 42 41 5 4 52 5 63 61 7 6 73 7 84 81 9")
    );

    assert_eq!(expand(&parse("0 1 21 3 2 32"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 2 32"), 1), parse("0 1 21 3"));
    assert_eq!(expand(&parse("0 1 21 3 2 32"), 2), parse("0 1 21 3 2 31 4"));
    assert_eq!(
        expand(&parse("0 1 21 3 2 32"), 3),
        parse("0 1 21 3 2 31 4 3 41 5")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32"), 4),
        parse("0 1 21 3 2 31 4 3 41 5 4 51 6")
    );

    assert!(expand(&parse("0 1 21 3 2 32 31"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 2 32 31"), 1), parse("0 1 21 3"));
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 31"), 2),
        parse("0 1 21 3 2 32 3 43 41 5")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 31"), 3),
        parse("0 1 21 3 2 32 3 43 41 5 4 54 5 65 61 7")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 31"), 4),
        parse("0 1 21 3 2 32 3 43 41 5 4 54 5 65 61 7 6 76 7 87 81 9")
    );

    assert_eq!(expand(&parse("0 1 21 3 2 32 32"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 2 32 32"), 1), parse("0 1 21 3"));
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 32"), 2),
        parse("0 1 21 3 2 32 31 4")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 32"), 3),
        parse("0 1 21 3 2 32 31 4 3 43 41 5")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 32"), 4),
        parse("0 1 21 3 2 32 31 4 3 43 41 5 4 54 51 6")
    );

    assert_eq!(expand(&parse("0 1 21 3 2 32 4 3 42"), 0), parse("0"));
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 4 3 42"), 1),
        parse("0 1 21 3 2 32 4")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 4 3 42"), 2),
        parse("0 1 21 3 2 32 4 3 41 5 4 52 6")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 4 3 42"), 3),
        parse("0 1 21 3 2 32 4 3 41 5 4 52 6 5 61 7 6 72 8")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 2 32 4 3 42"), 4),
        parse("0 1 21 3 2 32 4 3 41 5 4 52 6 5 61 7 6 72 8 7 81 9 8 92 A")
    );

    assert!(expand(&parse("0 1 21 3 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 21"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 21"), 2), parse("0 1 21 3"));
    assert_eq!(expand(&parse("0 1 21 3 21"), 3), parse("0 1 21 3 2 32 4"));
    assert_eq!(
        expand(&parse("0 1 21 3 21"), 4),
        parse("0 1 21 3 2 32 4 3 43 5")
    );

    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 42"), 0),
        parse("0 1 21 3 21 2 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 42"), 1),
        parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 42"), 2),
        parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 41 5 4 52 6 51")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 42"), 3),
        parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 41 5 4 52 6 51 5 61 7 6 72 8 71")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 42"), 4),
        parse(
            "0 1 21 3 21 2 31 1 21 3 2 32 4 31 3 41 5 4 52 6 51 5 61 7 6 72 8 71 7 81 9 8 92 A 91"
        )
    );

    assert!(expand(&parse("0 1 21 3 21 2 31 4 31"), 0).is_empty());
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 4 31"), 1),
        parse("0 1 21 3 21")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 4 31"), 2),
        parse("0 1 21 3 21 2 31 4 3 42 5 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 4 31"), 3),
        parse("0 1 21 3 21 2 31 4 3 42 5 41 4 52 6 5 63 7 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 31 4 31"), 4),
        parse("0 1 21 3 21 2 31 4 3 42 5 41 4 52 6 5 63 7 61 6 73 8 7 84 9 81")
    );

    assert_eq!(expand(&parse("0 1 21 3 21 2 32"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 21 2 32"), 1), parse("0 1 21 3 21"));
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32"), 2),
        parse("0 1 21 3 21 2 31 4 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32"), 3),
        parse("0 1 21 3 21 2 31 4 31 3 41 5 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32"), 4),
        parse("0 1 21 3 21 2 31 4 31 3 41 5 41 4 51 6 51")
    );

    assert!(expand(&parse("0 1 21 3 21 2 32 4 31"), 0).is_empty());
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 31"), 1),
        parse("0 1 21 3 21")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 31"), 2),
        parse("0 1 21 3 21 2 32 4 3 43 5 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 31"), 3),
        parse("0 1 21 3 21 2 32 4 3 43 5 41 4 54 6 5 65 7 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 31"), 4),
        parse("0 1 21 3 21 2 32 4 3 43 5 41 4 54 6 5 65 7 61 6 76 8 7 87 9 81")
    );

    assert_eq!(expand(&parse("0 1 21 3 21 2 32 4 32"), 0), parse("0"));
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 32"), 1),
        parse("0 1 21 3 21")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 32"), 2),
        parse("0 1 21 3 21 2 32 4 31")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 32"), 3),
        parse("0 1 21 3 21 2 32 4 31 3 43 5 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 2 32 4 32"), 4),
        parse("0 1 21 3 21 2 32 4 31 3 43 5 41 4 54 6 51")
    );

    assert!(expand(&parse("0 1 21 3 21 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 21 21"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 21 21"), 2), parse("0 1 21 3 21"));
    assert_eq!(
        expand(&parse("0 1 21 3 21 21"), 3),
        parse("0 1 21 3 21 2 32 4 32")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 21 21"), 4),
        parse("0 1 21 3 21 2 32 4 32 3 43 5 43")
    );

    assert_eq!(expand(&parse("0 1 21 3 41 2 32 32"), 0), parse("0"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 32"), 1),
        parse("0 1 21 3 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 32"), 2),
        parse("0 1 21 3 41 2 32 31 4 51")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 32"), 3),
        parse("0 1 21 3 41 2 32 31 4 51 3 43 41 5 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 32"), 4),
        parse("0 1 21 3 41 2 32 31 4 51 3 43 41 5 61 4 54 51 6 71")
    );

    assert!(expand(&parse("0 1 21 3 41 21"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 41 21"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 41 21"), 2), parse("0 1 21 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 21"), 3),
        parse("0 1 21 3 41 2 32 4 52")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 21"), 4),
        parse("0 1 21 3 41 2 32 4 52 3 43 5 63")
    );
}

#[test]
fn test_uncountable_tbms() {
    assert!(expand(&parse("0 1 21 3 41"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 41"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 3 41"), 2), parse("0 1 21 3 4 51"));
    assert_eq!(
        expand(&parse("0 1 21 3 41"), 3),
        parse("0 1 21 3 4 51 6 7 81")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41"), 4),
        parse("0 1 21 3 4 51 6 7 81 9 A B1")
    );

    assert_eq!(expand(&parse("0 1 21 3 41 2 32 4 52"), 0), parse("0"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 4 52"), 1),
        parse("0 1 21 3 41 2 32")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 4 52"), 2),
        parse("0 1 21 3 41 2 32 4 51 6 71 5 62")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 4 52"), 3),
        parse("0 1 21 3 41 2 32 4 51 6 71 5 62 7 81 9 A1 8 92")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 2 32 4 52"), 4),
        parse("0 1 21 3 41 2 32 4 51 6 71 5 62 7 81 9 A1 8 92 A B1 C D1 B C2")
    );

    assert!(expand(&parse("0 1 21 3 41 21 3 41"), 0).is_empty());
    assert_eq!(
        expand(&parse("0 1 21 3 41 21 3 41"), 1),
        parse("0 1 21 3 41 21")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 21 3 41"), 2),
        parse("0 1 21 3 41 21 3 4 51 6 71 51")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 21 3 41"), 3),
        parse("0 1 21 3 41 21 3 4 51 6 71 51 6 7 81 9 A1 81")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 21 3 41"), 4),
        parse("0 1 21 3 41 21 3 4 51 6 71 51 6 7 81 9 A1 81 9 A B1 C D1 B1")
    );

    assert!(expand(&parse("0 1 21 3 41 3 41"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 41 3 41"), 1), parse("0 1 21 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 41"), 2),
        parse("0 1 21 3 41 3 4 51 6 71")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 41"), 3),
        parse("0 1 21 3 41 3 4 51 6 71 6 7 81 9 A1")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 3 41"), 4),
        parse("0 1 21 3 41 3 4 51 6 71 6 7 81 9 A1 9 A B1 C D1")
    );

    assert!(expand(&parse("0 1 21 3 41 4 51"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 41 4 51"), 1), parse("0 1 21 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 51"), 2),
        parse("0 1 21 3 41 4 5 61 7 81")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 51"), 3),
        parse("0 1 21 3 41 4 5 61 7 81 8 9 A1 B C1")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 51"), 4),
        parse("0 1 21 3 41 4 5 61 7 81 8 9 A1 B C1 C D E1 F G1")
    );

    assert_eq!(expand(&parse("0 1 21 3 41 4 52"), 0), parse("0 1 21 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52"), 1),
        parse("0 1 21 3 41 4 51")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52"), 2),
        parse("0 1 21 3 41 4 51 5 61")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52"), 3),
        parse("0 1 21 3 41 4 51 5 61 6 71")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52"), 4),
        parse("0 1 21 3 41 4 51 5 61 6 71 7 81")
    );

    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51"), 0),
        parse("0 1 21 3 41")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51"), 1),
        parse("0 1 21 3 41 4 52")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51"), 2),
        parse("0 1 21 3 41 4 52 5 63")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51"), 3),
        parse("0 1 21 3 41 4 52 5 63 6 74")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51"), 4),
        parse("0 1 21 3 41 4 52 5 63 6 74 7 85")
    );

    assert!(expand(&parse("0 1 21 3 41 4 52 51 6 71"), 0).is_empty());
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51 6 71"), 1),
        parse("0 1 21 3 41 4 52 51")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51 6 71"), 2),
        parse("0 1 21 3 41 4 52 51 6 7 81 9 A1 A B2 B1")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51 6 71"), 3),
        parse("0 1 21 3 41 4 52 51 6 7 81 9 A1 A B2 B1 C D E1 F G1 G H2 H1")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 4 52 51 6 71"), 4),
        parse("0 1 21 3 41 4 52 51 6 7 81 9 A1 A B2 B1 C D E1 F G1 G H2 H1 I J K1 L M1 M N2 N1")
    );

    assert!(expand(&parse("0 1 21 3 41 41"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 41 41"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 41 41"), 2), parse("0 1 21 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 41"), 3),
        parse("0 1 21 3 41 4 52 6 72")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 41"), 4),
        parse("0 1 21 3 41 4 52 6 72 7 83 9 A3")
    );

    assert!(expand(&parse("0 1 21 3 41 5 61"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 3 41 5 61"), 1), parse("0 1 21 3 41"));
    assert_eq!(
        expand(&parse("0 1 21 3 41 5 61"), 2),
        parse("0 1 21 3 41 5 6 71 8 91")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 5 61"), 3),
        parse("0 1 21 3 41 5 6 71 8 91 A B C1 D E1")
    );
    assert_eq!(
        expand(&parse("0 1 21 3 41 5 61"), 4),
        parse("0 1 21 3 41 5 6 71 8 91 A B C1 D E1 F G H1 I J1")
    );

    assert_eq!(expand(&parse("0 1 21 3 42"), 0), parse("0"));
    assert_eq!(expand(&parse("0 1 21 3 42"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 3 42"), 2), parse("0 1 21 3 41"));
    assert_eq!(expand(&parse("0 1 21 3 42"), 3), parse("0 1 21 3 41 5 61"));
    assert_eq!(
        expand(&parse("0 1 21 3 42"), 4),
        parse("0 1 21 3 41 5 61 7 81")
    );

    assert!(expand(&parse("0 1 21 31"), 0).is_empty());
    assert_eq!(expand(&parse("0 1 21 31"), 1), parse("0"));
    assert_eq!(expand(&parse("0 1 21 31"), 2), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 31"), 3), parse("0 1 21 3 42"));
    assert_eq!(expand(&parse("0 1 21 31"), 4), parse("0 1 21 3 42 5 63"));

    assert_eq!(expand(&parse("0 1 21 32"), 0), parse("0 1"));
    assert_eq!(expand(&parse("0 1 21 32"), 1), parse("0 1 21"));
    assert_eq!(expand(&parse("0 1 21 32"), 2), parse("0 1 21 31"));
    assert_eq!(expand(&parse("0 1 21 32"), 3), parse("0 1 21 31 41"));
    assert_eq!(expand(&parse("0 1 21 32"), 4), parse("0 1 21 31 41 51"));
}

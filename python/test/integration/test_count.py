from opendp.mod import enable_features

enable_features('contrib')


def test_count():
    from opendp.trans import make_count, make_split_dataframe, make_select_column
    from opendp.meas import make_base_geometric
    from opendp.mod import binary_search_chain
    preprocess = (
        make_split_dataframe(",", ['A', 'B']) >>
        make_select_column("A", TOA=str) >>
        make_count(TIA=str)
    )

    noisy_count_from_dataframe = binary_search_chain(
        lambda s: preprocess >> make_base_geometric(s),
        d_in=1, d_out=1.)

    assert noisy_count_from_dataframe.check(1, 1.)

    k = 40
    data = "\n".join(map(str, range(k)))

    print(noisy_count_from_dataframe(data))


def test_count_distinct():
    from opendp.trans import make_count_distinct, make_split_dataframe, make_select_column
    from opendp.meas import make_base_geometric
    from opendp.mod import binary_search_chain
    preprocess = (
        make_split_dataframe(",", ['A', 'B']) >>
        make_select_column("A", TOA=str) >>
        make_count_distinct(TIA=str)
    )

    noisy_count_from_dataframe = binary_search_chain(
        lambda s: preprocess >> make_base_geometric(s),
        d_in=1, d_out=1.)

    assert noisy_count_from_dataframe.check(1, 1.)

    k = 40
    data = "\n".join(map(str, range(k)))

    print(noisy_count_from_dataframe(data))


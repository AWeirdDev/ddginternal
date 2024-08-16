import pytest
import ddginternal


def test_sum_as_string():
    assert ddginternal.sum_as_string(1, 1) == "2"

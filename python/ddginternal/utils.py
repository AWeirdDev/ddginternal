from .exceptions import RateLimitError
from .primp import Response


def raise_for_status(res: Response):
    if res.status_code in [202, 301, 403]:
        raise RateLimitError(
            "ddginternal: Oops, rate limited (or forbidden)! Response text:\n"
            + res.text
        )

    assert res.status_code == 200, res.text

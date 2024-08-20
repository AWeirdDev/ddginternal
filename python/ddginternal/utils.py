from .primp import Response


def raise_for_status(res: Response):
    assert res.status_code == 200, res.text

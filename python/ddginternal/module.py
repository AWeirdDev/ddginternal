from typing import NoReturn, Optional
from .ddginternal import PlacesModule


class Module:
    """Represents a module collector (for result)."""

    def __init__(self, *, places: Optional[PlacesModule] = None):
        self._places = places

    @staticmethod
    def inexistent(name: str) -> NoReturn:
        """Raises an error if the module does not exist."""
        raise ReferenceError(f"ddginternal: module {name!r} does not exist")

    @property
    def places(self) -> PlacesModule:
        """Gets the 'places' module.

        Returns:
            PlacesModule: The 'places' module.
        """
        if not self._places:
            self.inexistent("places")

        return self._places

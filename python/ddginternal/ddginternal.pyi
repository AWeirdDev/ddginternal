"""Duckduckgo internal APIs from Rust."""

from typing import Any, Dict, List, Literal, Optional

def get_djs(html: str) -> str:
    """Gets `d.js` from the initial HTML.

    Args:
        html (str): The HTML.

    Raises:
        RegexError: If the regex fails to compile.
    """

def get_result_binding(html: str, djs: str):
    """Gets the result binding from HTML and `d.js`.

    See :py:func:`ddginternal.organic_search`

    Args:
        html (str): The HTML.
        djs (str): The `d.js` content.
    """

def get_embedded_abstract(html: str) -> str:
    """Gets the embedded abstract from `d.js`.

    Args:
        html (str): The `d.js` content.

    Raises:
        RegexError: If the regex fails to compile.
    """

def get_nrj_instances(djs: str) -> List[tuple[str, str]]: ...
def assign_nrj_instances(instances: List[tuple[str, str]]) -> List["Assignee"]: ...

class Assignee:
    def who(self) -> str: ...
    def places(self) -> Any: ...
    def recipes(self) -> Any: ...

class Result:
    web: List["_Web"]
    images: List["_Image"]
    news: List["_NewsArticle"]
    abstract: Optional[object]

class _Web:
    raw_description: str
    description: str
    domain: str
    shortened_url: str
    url: str
    title: str

class _Image:
    url: str
    title: str
    description: str
    thumbnail: str
    width: int
    height: int

class _NewsArticle:
    date: int
    excerpt: str
    title: str
    url: str
    source: str
    relative_time: str
    image: Optional[str]

class RegexError(Exception):
    """Represents a regex error."""

class PlacesModule:
    type: Literal["places"]
    geoip_lat: float
    geoip_lon: float
    obfus_lat: float
    obfus_lon: float
    more_at: str
    results: List["Place"]

class Place:
    address: str
    address_lines: List[str]
    city: str
    closed: Optional[int]
    lat: float
    lon: float
    country_code: Optional[str]
    category: str
    display_phone: str
    distance: float
    distance_to_user_meters: float
    facebook_id: str
    hotel_id: Optional[str]
    hours: "Hours"
    image: Optional[str]
    name: str
    photo: str
    rating: int
    reviews: List["Review"]

class Hours:
    table: Dict[str, str]
    closes_soon: Optional[int]
    opens_soon: Optional[int]
    state_switch_time: Optional[str]

class Review:
    excerpt: str
    rating: int
    time_created: int
    user: Dict[str, str]

class RecipesModule:
    type: Literal["recipes"]
    results: List["Recipe"]

class Recipe:
    vegetarian: bool
    vegan: bool
    gluten_free: bool
    dairy_free: bool
    very_healthy: bool
    cheap: bool
    very_popular: bool
    sustainable: bool
    low_fodmap: bool
    weight_watcher_smart_points: int
    gaps: str
    preparation_minutes: int
    cooking_minutes: int
    ready_in_minutes: int
    aggregate_likes: int
    health_score: int
    credits_text: str
    source_name: str
    price_per_serving: float
    extended_ingredients: List["ExtendedIngredient"]
    title: str
    servings: int
    source_url: str
    image: str
    raw_summary: str
    summary: str
    cuisines: List[str]
    dish_types: List[str]
    diets: List[str]
    occasions: List[str]
    spoonacular_score: float

class ExtendedIngredient:
    aisle: str
    consistency: str
    name: str
    name_clean: str
    original: str
    original_name: str
    amount: float
    unit: str
    meta: List[str]
    measures: Dict[str, "Measure"]

class Measure:
    amount: float
    unit_short: str
    unit_long: str

# coding: utf-8

"""
    lsproxy

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field
from typing import Any, ClassVar, Dict, List, Optional
from lsproxy.models.code_context import CodeContext
from lsproxy.models.file_position import FilePosition
from typing import Optional, Set
from typing_extensions import Self

class DefinitionResponse(BaseModel):
    """
    Response to a definition request.  The definition(s) of the symbol. Points to the start position of the symbol's identifier.  e.g. for the definition of `User` on line 5 of `src/main.py` with the code: ``` 0: class User: _________^ 1:     def __init__(self, name, age): 2:         self.name = name 3:         self.age = age 4: 5: user = User(\"John\", 30) __________^ ``` The definition(s) will be `[{\"path\": \"src/main.py\", \"line\": 0, \"character\": 6}]`.
    """ # noqa: E501
    definitions: List[FilePosition]
    raw_response: Optional[Any] = Field(default=None, description="The raw response from the langserver.  https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_definition")
    source_code_context: Optional[List[CodeContext]] = Field(default=None, description="The source code of symbol definitions.")
    __properties: ClassVar[List[str]] = ["definitions", "raw_response", "source_code_context"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of DefinitionResponse from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of each item in definitions (list)
        _items = []
        if self.definitions:
            for _item_definitions in self.definitions:
                if _item_definitions:
                    _items.append(_item_definitions.to_dict())
            _dict['definitions'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in source_code_context (list)
        _items = []
        if self.source_code_context:
            for _item_source_code_context in self.source_code_context:
                if _item_source_code_context:
                    _items.append(_item_source_code_context.to_dict())
            _dict['source_code_context'] = _items
        # set to None if raw_response (nullable) is None
        # and model_fields_set contains the field
        if self.raw_response is None and "raw_response" in self.model_fields_set:
            _dict['raw_response'] = None

        # set to None if source_code_context (nullable) is None
        # and model_fields_set contains the field
        if self.source_code_context is None and "source_code_context" in self.model_fields_set:
            _dict['source_code_context'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of DefinitionResponse from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "definitions": [FilePosition.from_dict(_item) for _item in obj["definitions"]] if obj.get("definitions") is not None else None,
            "raw_response": obj.get("raw_response"),
            "source_code_context": [CodeContext.from_dict(_item) for _item in obj["source_code_context"]] if obj.get("source_code_context") is not None else None
        })
        return _obj



# coding: utf-8

"""
    lsproxy

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from lsproxy_sdk.api.symbol_api import SymbolApi


class TestSymbolApi(unittest.TestCase):
    """SymbolApi unit test stubs"""

    def setUp(self) -> None:
        self.api = SymbolApi()

    def tearDown(self) -> None:
        pass

    def test_definition(self) -> None:
        """Test case for definition

        Get the definition of a symbol at a specific position in a file
        """
        pass

    def test_file_symbols(self) -> None:
        """Test case for file_symbols

        Get symbols in a specific file
        """
        pass

    def test_references(self) -> None:
        """Test case for references

        Find all references to a symbol
        """
        pass


if __name__ == '__main__':
    unittest.main()

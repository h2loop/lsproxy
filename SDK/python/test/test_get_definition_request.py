# coding: utf-8

"""
    lsproxy

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from lsproxy_sdk.models.get_definition_request import GetDefinitionRequest

class TestGetDefinitionRequest(unittest.TestCase):
    """GetDefinitionRequest unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> GetDefinitionRequest:
        """Test GetDefinitionRequest
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `GetDefinitionRequest`
        """
        model = GetDefinitionRequest()
        if include_optional:
            return GetDefinitionRequest(
                include_raw_response = False,
                include_source_code = False,
                position = lsproxy_sdk.models.file_position.FilePosition(
                    path = 'src/main.py', 
                    position = lsproxy_sdk.models.position.Position(
                        character = 5, 
                        line = 10, ), )
            )
        else:
            return GetDefinitionRequest(
                position = lsproxy_sdk.models.file_position.FilePosition(
                    path = 'src/main.py', 
                    position = lsproxy_sdk.models.position.Position(
                        character = 5, 
                        line = 10, ), ),
        )
        """

    def testGetDefinitionRequest(self):
        """Test GetDefinitionRequest"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()

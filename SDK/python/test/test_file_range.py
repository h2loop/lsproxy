# coding: utf-8

"""
    lsproxy

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from lsproxy_sdk.models.file_range import FileRange

class TestFileRange(unittest.TestCase):
    """FileRange unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> FileRange:
        """Test FileRange
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `FileRange`
        """
        model = FileRange()
        if include_optional:
            return FileRange(
                end = lsproxy_sdk.models.position.Position(
                    character = 5, 
                    line = 10, ),
                path = 'src/main.py',
                start = lsproxy_sdk.models.position.Position(
                    character = 5, 
                    line = 10, )
            )
        else:
            return FileRange(
                end = lsproxy_sdk.models.position.Position(
                    character = 5, 
                    line = 10, ),
                path = 'src/main.py',
                start = lsproxy_sdk.models.position.Position(
                    character = 5, 
                    line = 10, ),
        )
        """

    def testFileRange(self):
        """Test FileRange"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()

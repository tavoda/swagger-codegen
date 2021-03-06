﻿using NUnit.Framework;
using System;
using System.Collections.Generic;
using IO.Swagger.Client;
using IO.Swagger.Api;
using IO.Swagger.Model;

namespace IO.Swagger.Test
{
	public class ConfigurationTests
	{
		public ConfigurationTests ()
		{
		}

		[SetUp()]
		public void BeforeEach()
		{
			var config = new GlobalConfiguration();
			Configuration.Default = config;

			// Reset to default, just in case
			Configuration.Default.DateTimeFormat = "o";
		}

		[Test ()]
		public void TestAuthentication ()
		{
			Configuration c = new Configuration ();
			c.Username = "test_username";
			c.Password = "test_password";

			c.ApiKey ["api_key_identifier"] = "1233456778889900";
			c.ApiKeyPrefix ["api_key_identifier"] = "PREFIX";
			Assert.AreEqual (c.GetApiKeyWithPrefix("api_key_identifier"), "PREFIX 1233456778889900");

		}

		[Test ()]
		public void TestBasePath ()
		{
			PetApi p = new PetApi ("http://new-basepath.com");
			Assert.AreEqual (p.Configuration.ApiClient.RestClient.BaseUrl, "http://new-basepath.com");
			// Given that PetApi is initailized with a base path, a new configuration (with a new ApiClient)
			// is created by default
			Assert.AreNotSame (p.Configuration, Configuration.Default);
		}

		[Test ()]
		public void TestDateTimeFormat_Default ()
		{
			// Should default to the Round-trip Format Specifier - "o"
			// https://msdn.microsoft.com/en-us/library/az4se3k1(v=vs.110).aspx#Anchor_8
			Assert.AreEqual("o", Configuration.Default.DateTimeFormat);
		}

		[Test ()]
		public void TestDateTimeFormat_UType()
		{
			Configuration.Default.DateTimeFormat = "u";

			Assert.AreEqual("u", Configuration.Default.DateTimeFormat);
		}

		[Test()]
		public void TestDateTimeFormat_UType_NonGlobal()
		{
			Configuration configuration = new Configuration();
			configuration.DateTimeFormat = "u";

			Assert.AreEqual("u", configuration.DateTimeFormat);
			Assert.AreNotEqual("u", Configuration.Default.DateTimeFormat);
		}

		[Test ()]
		public void TestConstruction()
		{
			Configuration c = new Configuration {  Username = "test username", Password = "test password" };
			Assert.AreEqual (c.Username, "test username");
			Assert.AreEqual (c.Password, "test password");

		}

		[Test ()]
		public void TestDefautlConfiguration ()
		{
			PetApi p1 = new PetApi ();
			PetApi p2 = new PetApi ();
			Assert.AreSame (p1.Configuration, p2.Configuration);
			// same as the default
			Assert.AreSame (p1.Configuration, Configuration.Default);

			Configuration c = new Configuration ();
			Assert.AreNotSame (c, p1.Configuration);

			PetApi p3 = new PetApi (c);
			// same as c
			Assert.AreSame (p3.Configuration, c);
			// not same as default
			Assert.AreNotSame (p3.Configuration, p1.Configuration);

		}

		[Test ()]
		public void TestUsage ()
		{
			// basic use case using default base URL
			PetApi p1 = new PetApi ();
			Assert.AreSame (p1.Configuration, Configuration.Default, "PetApi should use default configuration");

			// using a different base URL
			PetApi p2 = new PetApi ("http://new-base-url.com/");
			Assert.AreEqual (p2.Configuration.ApiClient.RestClient.BaseUrl.ToString(), "http://new-base-url.com/");

			// using a different configuration
			Configuration c1 = new Configuration ();
			PetApi p3 = new PetApi (c1);
			Assert.AreSame (p3.Configuration, c1);

			// using a different base URL via a new Configuration
			String newApiClientUrl = "http://new-api-client.com";
			Configuration c2 = new Configuration { BasePath = newApiClientUrl };
			PetApi p4 = new PetApi (c2);
			Assert.AreEqual(p4.Configuration.ApiClient.RestClient.BaseUrl, new Uri(newApiClientUrl));
		}

		[Test ()]
		public void TestTimeout ()
		{
			Configuration c1 = new Configuration ();
			Assert.AreEqual (100000, c1.Timeout); // default vaue

			c1.Timeout = 50000;
			Assert.AreEqual (50000, c1.Timeout);

			Configuration c2 = new Configuration { Timeout = 20000 };
			Assert.AreEqual (20000, c2.Timeout);
		}

		[Test()]
		public void TestAddingInstanceHeadersDoesNotModifyGlobal()
		{
			// Arrange
			Configuration.Default.DefaultHeader.Add("Content-Type", "application/json");
			Configuration.Default.ApiKey.Add("api_key_identifier", "1233456778889900");
			Configuration.Default.ApiKeyPrefix.Add("api_key_identifier", "PREFIX");

			Configuration c = new Configuration(
				Configuration.Default.DefaultHeader,
				Configuration.Default.ApiKey,
				Configuration.Default.ApiKeyPrefix
			);

			// sanity
			CollectionAssert.AreEquivalent(c.DefaultHeader, Configuration.Default.DefaultHeader);
			CollectionAssert.AreEquivalent(c.ApiKey, Configuration.Default.ApiKey);
			CollectionAssert.AreEquivalent(c.ApiKeyPrefix, Configuration.Default.ApiKeyPrefix);

			// Act
			Configuration.Default.DefaultHeader["Content-Type"] = "application/xml";
			Configuration.Default.ApiKey["api_key_identifier"] = "00000000000001234";
			Configuration.Default.ApiKeyPrefix["api_key_identifier"] = "MODIFIED";

			// Assert
			CollectionAssert.AreNotEquivalent(c.DefaultHeader, Configuration.Default.DefaultHeader);
			CollectionAssert.AreNotEquivalent(c.ApiKey, Configuration.Default.ApiKey);
			CollectionAssert.AreNotEquivalent(c.ApiKeyPrefix, Configuration.Default.ApiKeyPrefix);
		}
	}
}


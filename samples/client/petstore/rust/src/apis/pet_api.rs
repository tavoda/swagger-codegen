/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct PetApiImpl<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PetApiImpl<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PetApiImpl<C> {
        PetApiImpl {
            configuration: configuration,
        }
    }
}

pub trait PetApi {
    fn AddPet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>>;
    fn DeletePet(&self, pet_id: i64, api_key: &str) -> Box<Future<Item = (), Error = Error>>;
    fn FindPetsByStatus(&self, status: Vec<String>) -> Box<Future<Item = Vec<::models::pet::Pet>, Error = Error>>;
    fn FindPetsByTags(&self, tags: Vec<String>) -> Box<Future<Item = Vec<::models::pet::Pet>, Error = Error>>;
    fn GetPetById(&self, pet_id: i64) -> Box<Future<Item = ::models::pet::Pet, Error = Error>>;
    fn UpdatePet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>>;
    fn UpdatePetWithForm(&self, pet_id: i64, name: &str, status: &str) -> Box<Future<Item = (), Error = Error>>;
    fn UploadFile(&self, pet_id: i64, additional_metadata: &str, file: File) -> Box<Future<Item = ::models::api_response::ApiResponse, Error = Error>>;
}


impl<C: hyper::client::Connect>PetApi for PetApiImpl<C> {
    fn AddPet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri = format!("{}/pet", configuration.base_path);

        let mut req = hyper::Request::new(method, uri);


        let serialized = serde_json::to_string(body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn DeletePet(&self, pet_id: i64, api_key: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let uri = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);

        let mut req = hyper::Request::new(method, uri);

        let mut headers = req.headers_mut();
        headers.set_raw("api_key", api_key);


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn FindPetsByStatus(&self, status: Vec<String>) -> Box<Future<Item = Vec<::models::pet::Pet>, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("status", status)
            .finish();
        let uri = format!("{}/pet/findByStatus{}", configuration.base_path, query);

        let mut req = hyper::Request::new(method, uri);



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<Vec<::models::pet::Pet>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn FindPetsByTags(&self, tags: Vec<String>) -> Box<Future<Item = Vec<::models::pet::Pet>, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("tags", tags)
            .finish();
        let uri = format!("{}/pet/findByTags{}", configuration.base_path, query);

        let mut req = hyper::Request::new(method, uri);



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<Vec<::models::pet::Pet>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn GetPetById(&self, pet_id: i64) -> Box<Future<Item = ::models::pet::Pet, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);

        let mut req = hyper::Request::new(method, uri);



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::pet::Pet, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn UpdatePet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let uri = format!("{}/pet", configuration.base_path);

        let mut req = hyper::Request::new(method, uri);


        let serialized = serde_json::to_string(body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn UpdatePetWithForm(&self, pet_id: i64, name: &str, status: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);

        let mut req = hyper::Request::new(method, uri);



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn UploadFile(&self, pet_id: i64, additional_metadata: &str, file: File) -> Box<Future<Item = ::models::api_response::ApiResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri = format!("{}/pet/{petId}/uploadImage", configuration.base_path, petId=pet_id);

        let mut req = hyper::Request::new(method, uri);



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::api_response::ApiResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}

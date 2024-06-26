use serde::{Serialize, Deserialize};
use figment::{Figment, providers::Serialized};
use pretty_assertions::assert_eq;

use rocket::{Config, uri};
use rocket::http::uri::{Absolute, Asterisk, Authority, Origin, Reference};
use rocket::http::{Method, Status};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct UriContainer<'a> {
    asterisk: Asterisk,
    origin: Origin<'a>,
    authority: Authority<'a>,
    absolute: Absolute<'a>,
    reference: Reference<'a>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct UriContainerOwned {
    asterisk: Asterisk,
    origin: Origin<'static>,
    authority: Authority<'static>,
    absolute: Absolute<'static>,
    reference: Reference<'static>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct MethodContainer {
    mget: Method,
    mput: Method,
    mpost: Method,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct StatusContainer {
    a: Status,
    b: Status,
    c: Status,
}

#[test]
fn uri_serde() {
    figment::Jail::expect_with(|jail| {
        jail.create_file("Rocket.toml", r#"
            [default]
            asterisk = "*"
            origin = "/foo/bar?baz"
            authority = "user:pass@rocket.rs:80"
            absolute = "https://rocket.rs/foo/bar"
            reference = "https://rocket.rs:8000/index.html"
        "#)?;

        let uris: UriContainer<'_> = Config::figment().extract()?;
        assert_eq!(uris, UriContainer {
            asterisk: Asterisk,
            origin: uri!("/foo/bar?baz"),
            authority: uri!("user:pass@rocket.rs:80"),
            absolute: uri!("https://rocket.rs/foo/bar"),
            reference: uri!("https://rocket.rs:8000/index.html").into(),
        });

        let uris: UriContainerOwned = Config::figment().extract()?;
        assert_eq!(uris, UriContainerOwned {
            asterisk: Asterisk,
            origin: uri!("/foo/bar?baz"),
            authority: uri!("user:pass@rocket.rs:80"),
            absolute: uri!("https://rocket.rs/foo/bar"),
            reference: uri!("https://rocket.rs:8000/index.html").into(),
        });

        Ok(())
    });
}

#[test]
fn uri_serde_round_trip() {
    let tmp = Figment::from(Serialized::defaults(UriContainer {
        asterisk: Asterisk,
        origin: uri!("/foo/bar?baz"),
        authority: uri!("user:pass@rocket.rs:80"),
        absolute: uri!("https://rocket.rs/foo/bar"),
        reference: uri!("https://rocket.rs:8000/index.html").into(),
    }));

    let uris: UriContainer<'_> = tmp.extract().unwrap();
    assert_eq!(uris, UriContainer {
        asterisk: Asterisk,
        origin: uri!("/foo/bar?baz"),
        authority: uri!("user:pass@rocket.rs:80"),
        absolute: uri!("https://rocket.rs/foo/bar"),
        reference: uri!("https://rocket.rs:8000/index.html").into(),
    });

    let uris: UriContainerOwned = tmp.extract().unwrap();
    assert_eq!(uris, UriContainerOwned {
        asterisk: Asterisk,
        origin: uri!("/foo/bar?baz"),
        authority: uri!("user:pass@rocket.rs:80"),
        absolute: uri!("https://rocket.rs/foo/bar"),
        reference: uri!("https://rocket.rs:8000/index.html").into(),
    });

    let tmp = Figment::from(Serialized::defaults(UriContainerOwned {
        asterisk: Asterisk,
        origin: uri!("/foo/bar?baz"),
        authority: uri!("user:pass@rocket.rs:80"),
        absolute: uri!("https://rocket.rs/foo/bar"),
        reference: uri!("https://rocket.rs:8000/index.html").into(),
    }));

    let uris: UriContainer<'_> = tmp.extract().unwrap();
    assert_eq!(uris, UriContainer {
        asterisk: Asterisk,
        origin: uri!("/foo/bar?baz"),
        authority: uri!("user:pass@rocket.rs:80"),
        absolute: uri!("https://rocket.rs/foo/bar"),
        reference: uri!("https://rocket.rs:8000/index.html").into(),
    });

    let uris: UriContainerOwned = tmp.extract().unwrap();
    assert_eq!(uris, UriContainerOwned {
        asterisk: Asterisk,
        origin: uri!("/foo/bar?baz"),
        authority: uri!("user:pass@rocket.rs:80"),
        absolute: uri!("https://rocket.rs/foo/bar"),
        reference: uri!("https://rocket.rs:8000/index.html").into(),
    });
}

#[test]
fn method_serde() {
    figment::Jail::expect_with(|jail| {
        jail.create_file("Rocket.toml", r#"
            [default]
            mget = "GET"
            mput = "PUT"
            mpost = "post"
        "#)?;

        let methods: MethodContainer = Config::figment().extract()?;
        assert_eq!(methods, MethodContainer {
            mget: Method::Get,
            mput: Method::Put,
            mpost: Method::Post
        });

        let tmp = Figment::from(Serialized::defaults(methods));
        let methods: MethodContainer = tmp.extract()?;
        assert_eq!(methods, MethodContainer {
            mget: Method::Get,
            mput: Method::Put,
            mpost: Method::Post
        });

        Ok(())
    });
}

#[test]
fn status_serde() {
    figment::Jail::expect_with(|jail| {
        jail.create_file("Rocket.toml", r#"
            [default]
            a = 500
            b = 100
            c = 404
        "#)?;

        let statuses: StatusContainer = Config::figment().extract()?;
        assert_eq!(statuses, StatusContainer {
            a: Status::InternalServerError,
            b: Status::Continue,
            c: Status::NotFound
        });

        let tmp = Figment::from(Serialized::defaults(statuses));
        let statuses: StatusContainer = tmp.extract()?;
        assert_eq!(statuses, StatusContainer {
            a: Status::InternalServerError,
            b: Status::Continue,
            c: Status::NotFound
        });

        jail.create_file("Rocket.toml", r#"
            [default]
            a = 99
            b = 100
            c = 404
        "#)?;

        let statuses: Result<StatusContainer, _> = Config::figment().extract();
        assert!(statuses.is_err());

        jail.create_file("Rocket.toml", r#"
            [default]
            a = 500
            b = 100
            c = 600
        "#)?;

        let statuses: Result<StatusContainer, _> = Config::figment().extract();
        assert!(statuses.is_err());

        Ok(())
    });
}

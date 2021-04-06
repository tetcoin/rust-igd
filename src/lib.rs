//! This library allows you to communicate with an IG enabled device.
//! Use one of the `search_gateway` functions to obtain a `Gateway` object.
//! You can then communicate with the device via this object.

#![deny(missing_docs)]

extern crate hyper;
extern crate regex;
extern crate xml;
extern crate xmltree;
extern crate rand;

// data structures
pub use self::gateway::Gateway;
pub use self::gateway::PortMappingProtocol;
pub use self::gateway::RequestError;
pub use self::gateway::GetExternalIpError;
pub use self::gateway::RemovePortError;
pub use self::gateway::AddPortError;
pub use self::gateway::AddAnyPortError;

// search of gateway
pub use self::search::search_gateway;
pub use self::search::search_gateway_timeout;
pub use self::search::search_gateway_from;
pub use self::search::search_gateway_from_timeout;
pub use self::search::SearchError;

// re-export error types
pub use hyper::Error as HttpError;
pub use xml::common::Error as XmlError;

mod gateway;
mod search;
mod soap;

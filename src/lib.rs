use std::sync::Arc;
use crate::errors::Error;
use crate::keys::*;
use crate::network::{AuthToken, NetworkClient};

mod errors;
pub mod models;
mod network;
pub(crate) mod coersion;
pub mod keys;


pub struct Supernova {
    service: Arc<NetworkClient>,
}

impl Supernova {
    pub fn new() -> Supernova {
        Supernova {
            service: Arc::new(NetworkClient::default())
        }
    }

    pub async fn connect(self: &Arc<Supernova>, username: String, password: String) -> Result<AuthToken, Error> {
        Ok(String::new())
    }

    pub async fn reconnect(self: &Arc<Supernova>, token: String) -> Result<(), Error> {
        Ok(())
    }

    pub fn get_building(&self, id: BuildingKey) -> Result<models::Building, Error> {
        self.service.fetch_building(id, self.service.clone())
    }

    pub fn get_room(&self, id: RoomKey) -> Result<models::Room, Error> {
        self.service.fetch_room(id, self.service.clone())
    }

    pub fn get_course(&self, id: CourseKey) -> Result<models::Course, Error> {
        self.service.fetch_course(id, self.service.clone())
    }

    pub fn get_class(&self, id: ClassKey) -> Result<models::Class, Error> {
        self.service.fetch_class(id, self.service.clone())
    }

    pub fn get_class_instance(&self, id: ClassInstanceKey) -> Result<models::ClassInstance, Error> {
        self.service.fetch_class_instance(id, self.service.clone())
    }

    pub fn get_enrollment(&self, id: EnrollmentKey) -> Result<models::Enrollment, Error> {
        self.service.fetch_enrollment(id, self.service.clone())
    }

    pub fn get_shift(&self, id: ShiftKey) -> Result<models::ClassShift, Error> {
        self.service.fetch_shift(id, self.service.clone())
    }

    pub fn warmup(&self, id: ShiftKey) -> Result<(), Error> {
        self.service.fetch_buildings(self.service.clone())?;
        self.service.fetch_courses(self.service.clone())?;
        self.service.fetch_classes(self.service.clone())?;
        self.service.fetch_departments(self.service.clone())?;
        self.service.fetch_rooms(self.service.clone())?;
        Ok(())
    }
}

// impl Building {
//     fn fetch_all(
//         client: &Client,
//         cache: &mut LocalCache,
//     ) -> Result<Vec<BuildingKey>, Error> {
//         let resp = client.get(&Endpoint::Buildings.to_string()).await?;
//         if let Data::Json(string)  = resp{
//
//         } else {
//             Err(Error::NamelessError)
//         }
//     }
//
//     fn fetch_additional(id: u32, client: &Client, cache: &mut LocalCache) -> Result<Room, Error> {
//         client.get(&Endpoint::Room(id).to_string())
//     }
// }


// pub(crate) struct ObjectLogic {
//     client: Arc<RwLock<NetworkClient>>,
//
// }
//
// trait CoersibleEntity<I> {
//     fn coerce(id: &I, client: Arc<NetworkClient>) -> Result<Self, ()> where Self: Sized;
// }
//
// pub(crate) struct ObjRef<T: CoersibleEntity<I>, I> {
//     identifier: I,
//     _type: PhantomData<T>,
//     // obj: Option<T>,// OnceCell
//     client: Arc<NetworkClient>,
// }
//
// impl<T: CoersibleEntity<I>, I> ObjRef<T, I> {
//     pub(crate) fn new(identifier: I, client: Arc<NetworkClient>) -> ObjRef<T, I> {
//         ObjRef {
//             identifier,
//             _type: Default::default(),
//             client,
//         }
//     }
// }
//
// impl<T: CoersibleEntity<I>, I> ObjRef<T, I> {
//     fn coerce(&self) -> Result<T, ()> {
//         T::coerce(&self.identifier, self.client.clone())
//     }
// }
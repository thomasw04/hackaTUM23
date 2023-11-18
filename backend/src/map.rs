use std::collections::HashMap;

use rstar::{RTree, RTreeObject, Point, Envelope, PointDistance, AABB};

use crate::data::{ServiceProvider, Postcode};

#[derive(Clone)]
pub struct InServiceProvider {
    id: u32,
    pos: (f64, f64),
    min: (f64, f64),
    max: (f64, f64),
    max_driving_distance: u64,
}

impl Into<InServiceProvider> for ServiceProvider {
    fn into(self) -> InServiceProvider {
        let angular_radius = self.max_driving_distance as f64 / 6371000.0;
        let delta_lon = (angular_radius.sin() / self.lat.cos()).asin();

        InServiceProvider {
            id: self.id,
            pos: (self.lon, self.lat),
            min: (self.lon - delta_lon, self.lat - angular_radius),
            max: (self.lon + delta_lon, self.lat + angular_radius),
            max_driving_distance: self.max_driving_distance,
        }
    }
}

impl Into<(f64, f64)> for Postcode {
    fn into(self) -> (f64, f64) {
        (self.lon, self.lat)
    }
}

impl RTreeObject for InServiceProvider {
    type Envelope = rstar::AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.min.0, self.min.1], [self.max.0, self.max.1])
    }
}

impl PointDistance for InServiceProvider {
    fn contains_point(&self, point: &<Self::Envelope as Envelope>::Point) -> bool {
        self.distance_2(point) <= self.max_driving_distance as f64
    }

    fn distance_2(&self, point: &<Self::Envelope as Envelope>::Point) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
        let other_lon = point.get(0).unwrap();
        let other_lat = point.get(1).unwrap();

        let sin_prod = self.pos.1.sin() * other_lat.sin();
        let cos_prod = self.pos.1.cos() * other_lat.cos() * (self.pos.0 - other_lon).cos();
        (sin_prod + cos_prod).acos() * 6371000.0
    }

    fn distance_2_if_less_or_equal(&self, point: &<Self::Envelope as Envelope>::Point, max_distance_2: <<Self::Envelope as Envelope>::Point as Point>::Scalar) -> Option<<<Self::Envelope as Envelope>::Point as Point>::Scalar> {
        let dist = self.distance_2(point);

        if dist <= max_distance_2 {
            Some(dist)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Map {
    postcodes: HashMap<u32, Postcode>,
    service_providers: HashMap<u32, ServiceProvider>,
    a_tree: RTree<InServiceProvider>,
    b_tree: RTree<InServiceProvider>,
    c_tree: RTree<InServiceProvider>,
}

impl Map {
    pub fn new(
        postcodes: HashMap<u32, Postcode>,
        service_providers: HashMap<u32, ServiceProvider>
    ) -> Self {
        let converted_providers: Vec<ServiceProvider> = service_providers.clone().into_values().collect();

        let a_tree: RTree<InServiceProvider> = RTree::bulk_load(converted_providers.iter().map(|x| (*x).clone().into()).collect());

        let b_tree: RTree<InServiceProvider> = RTree::bulk_load(converted_providers.iter().map(|x| {
            let mut service_provider: InServiceProvider = (*x).clone().into();
            service_provider.max_driving_distance += 2000;
            return service_provider;
        }).collect());

        let c_tree: RTree<InServiceProvider> = RTree::bulk_load(converted_providers.iter().map(|x| {
            let mut service_provider: InServiceProvider = (*x).clone().into();
            service_provider.max_driving_distance += 5000;
            return service_provider;
        }).collect());

        return Map {postcodes, service_providers, a_tree, b_tree, c_tree};
    }

    pub fn add_service_provider(&mut self, service_provider: &ServiceProvider) {
        self.a_tree.insert(service_provider.clone().into());
        let mut tmp = service_provider.clone();
        tmp.max_driving_distance += 2000;
        self.b_tree.insert(tmp.clone().into());
        tmp.max_driving_distance += 3000;
        self.c_tree.insert(tmp.into());
    }

    pub fn get_service_providers(&self, postcode: u32) -> Vec<ServiceProvider> {
        if let Some(code) = self.postcodes.get(&postcode) {
            match code.postcode_extension_distance_group {
                0 => self.a_tree.locate_all_at_point(&[code.lon, code.lat]).map(|x| self.service_providers.get(&x.id).unwrap().to_owned()).collect(),
                1 => self.b_tree.locate_all_at_point(&[code.lon, code.lat]).map(|x| self.service_providers.get(&x.id).unwrap().to_owned()).collect(),
                2 => self.c_tree.locate_all_at_point(&[code.lon, code.lat]).map(|x| self.service_providers.get(&x.id).unwrap().to_owned()).collect(),
                _ => panic!("Wrong data lol.")
            }
        } else {
            Vec::new()
        }
    }
}



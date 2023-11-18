use std::collections::HashMap;

use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use serde::Serialize;

use crate::data::{Postcode, PostcodeGroup, QualityFactor, ServiceProvider, ServiceProviderView};

#[derive(Clone, Serialize)]
pub struct InServiceProvider {
    id: u32,
    name: String,
    pos: (f64, f64),
    min: (f64, f64),
    max: (f64, f64),
    max_driving_distance: u64,
    rank: Option<f64>,
}

impl Into<(f64, f64)> for Postcode {
    fn into(self) -> (f64, f64) {
        (self.lon, self.lat)
    }
}

impl Into<InServiceProvider> for ServiceProvider {
    fn into(self) -> InServiceProvider {
        let angular_radius = self.max_driving_distance as f64 / 6371000.0;
        let delta_lon = (angular_radius.sin() / self.lat.cos()).asin();

        InServiceProvider {
            id: self.id,
            name: self.first_name + self.last_name.as_str(),
            pos: (self.lon, self.lat),
            min: (self.lon - delta_lon, self.lat - angular_radius),
            max: (self.lon + delta_lon, self.lat + angular_radius),
            max_driving_distance: self.max_driving_distance,
            rank: None,
        }
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

    fn distance_2(
        &self,
        point: &<Self::Envelope as Envelope>::Point,
    ) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
        let other_lon = point.get(0).unwrap();
        let other_lat = point.get(1).unwrap();

        let sin_prod = self.pos.1.sin() * other_lat.sin();
        let cos_prod = self.pos.1.cos() * other_lat.cos() * (self.pos.0 - other_lon).cos();
        (sin_prod + cos_prod).acos() * 6371000.0
    }

    fn distance_2_if_less_or_equal(
        &self,
        point: &<Self::Envelope as Envelope>::Point,
        max_distance_2: <<Self::Envelope as Envelope>::Point as Point>::Scalar,
    ) -> Option<<<Self::Envelope as Envelope>::Point as Point>::Scalar> {
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
    quality_factor: HashMap<u32, QualityFactor>,
    service_providers: HashMap<u32, ServiceProvider>,
    a_tree: RTree<InServiceProvider>,
    b_tree: RTree<InServiceProvider>,
    c_tree: RTree<InServiceProvider>,
}

impl Map {
    pub fn new(
        postcodes: HashMap<u32, Postcode>,
        quality_factor: HashMap<u32, QualityFactor>,
        service_providers: HashMap<u32, ServiceProvider>,
    ) -> Self {
        let converted_providers: Vec<ServiceProvider> =
            service_providers.clone().into_values().collect();

        let a_tree: RTree<InServiceProvider> = RTree::bulk_load(
            converted_providers
                .iter()
                .map(|x| (*x).clone().into())
                .collect(),
        );

        let b_tree: RTree<InServiceProvider> = RTree::bulk_load(
            converted_providers
                .iter()
                .map(|x| {
                    let mut service_provider: InServiceProvider = (*x).clone().into();
                    service_provider.max_driving_distance += 2000;
                    return service_provider;
                })
                .collect(),
        );

        let c_tree: RTree<InServiceProvider> = RTree::bulk_load(
            converted_providers
                .iter()
                .map(|x| {
                    let mut service_provider: InServiceProvider = (*x).clone().into();
                    service_provider.max_driving_distance += 5000;
                    return service_provider;
                })
                .collect(),
        );

        return Map {
            postcodes,
            quality_factor,
            service_providers,
            a_tree,
            b_tree,
            c_tree,
        };
    }

    fn calculate_distance(point_a: (f64, f64), point_b: (f64, f64)) -> f64 {
        let sin_prod = point_b.1.sin() * point_a.1.sin();
        let cos_prod = point_b.1.cos() * point_a.1.cos() * (point_b.0 - point_a.0).cos();
        (sin_prod + cos_prod).acos() * 6371000.0
    }

    fn calculate_rank(&self, point: (f64, f64), service_provider: &InServiceProvider) -> f64 {
        let quality = self.quality_factor.get(&service_provider.id).unwrap();
        let quality_factor =
            0.4 * quality.profile_description_score + 0.6 * quality.profile_picture_score;

        let distance = Map::calculate_distance(point, service_provider.pos);

        let default_distance = 80000.0;
        let distance_score = 1.0 - (distance / default_distance);
        let distance_weight = if distance > default_distance {
            0.01
        } else {
            0.15
        };

        distance_weight * distance_score + (1.0 - distance_weight) * quality_factor
    }

    pub fn add_service_provider(&mut self, service_provider: &ServiceProvider) {
        self.a_tree.insert(service_provider.clone().into());
        let mut tmp = service_provider.clone();
        tmp.max_driving_distance += 2000;
        self.b_tree.insert(tmp.clone().into());
        tmp.max_driving_distance += 3000;
        self.c_tree.insert(tmp.into());
    }

    pub fn service_provider_by_id(&self, id: u32) -> Option<ServiceProvider> {
        self.service_providers.get(&id).cloned()
    }

    fn get_service_providers(&self, postcode: u32) -> Option<Vec<InServiceProvider>> {
        if let Some(code) = self.postcodes.get(&postcode) {
            let tree = match code.postcode_extension_distance_group {
                PostcodeGroup::GroupA => &self.a_tree,
                PostcodeGroup::GroupB => &self.b_tree,
                PostcodeGroup::GroupC => &self.c_tree,
            };

            let in_range: Vec<InServiceProvider> = tree
                .locate_all_at_point(&[code.lon, code.lat])
                .cloned()
                .collect();
            return Some(in_range);
        } else {
            None
        }
    }

    pub fn ranked_by_score(&self, postcode: u32) -> Option<Vec<ServiceProviderView>> {
        if let Some(code) = self.postcodes.get(&postcode) {
            if let Some(in_range) = self.get_service_providers(postcode) {
                let mut ranked: Vec<ServiceProviderView> = in_range
                    .into_iter()
                    .map(|x| ServiceProviderView {
                        id: x.id,
                        rankingScore: self.calculate_rank((code.lon, code.lat), &x),
                        name: x.name,
                    })
                    .collect();

                ranked.sort_by(|a, b| b.rankingScore.total_cmp(&a.rankingScore));
                return Some(ranked);
            }
        }

        return None;
    }

    pub fn ranked_by_distance(&self, postcode: u32) -> Option<Vec<ServiceProviderView>> {
        if let Some(code) = self.postcodes.get(&postcode) {
            if let Some(in_range) = self.get_service_providers(postcode) {
                let mut ranked: Vec<ServiceProviderView> = in_range
                    .into_iter()
                    .map(|x| ServiceProviderView {
                        id: x.id,
                        rankingScore: Map::calculate_distance(x.pos, (code.lon, code.lat)),
                        name: x.name,
                    })
                    .collect();

                ranked.sort_by(|a, b| a.rankingScore.total_cmp(&b.rankingScore));
                return Some(ranked);
            }
        }

        return None;
    }

    pub fn ranked_by_profile(&self, postcode: u32) -> Option<Vec<ServiceProviderView>> {
        if let Some(code) = self.postcodes.get(&postcode) {
            if let Some(in_range) = self.get_service_providers(postcode) {
                let mut ranked: Vec<ServiceProviderView> = in_range
                    .into_iter()
                    .map(|x| {
                        let quality = self.quality_factor.get(&x.id).unwrap();
                        ServiceProviderView {
                            id: x.id,
                            rankingScore: 0.4 * quality.profile_description_score
                                + 0.6 * quality.profile_picture_score,
                            name: x.name,
                        }
                    })
                    .collect();

                ranked.sort_by(|a, b| b.rankingScore.total_cmp(&a.rankingScore));
                return Some(ranked);
            }
        }

        return None;
    }
}

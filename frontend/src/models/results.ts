export interface ServiceProvider {
  id: number;
  first_name: string;
  last_name: string;
  city: string;
  street: string;
  house_number: string;
  lon: number;
  lat: number;
  max_driving_distance: number;
}

export function getNormalCoords(sp: ServiceProvider): [number, number] {
  return [(sp.lat * 180) / Math.PI, (sp.lon * 180) / Math.PI];
}

// Source: https://github.com/arnobroekhof/rd2wgs84
// Constants
const X0: f64 = 155000.0;
const Y0: f64 = 463000.0;
const LAT0: f64 = 52.15517;
const LON0: f64 = 5.387206;


pub struct RDCoordinate {
    pub x: f64,
    pub y: f64,
}


pub struct WGS84Coordinate {
    pub lat: f64,
    pub lon: f64,
}

impl WGS84Coordinate {
    // Nieuw wgs84 coordinate
    pub fn new(lat: f64, lon: f64) -> WGS84Coordinate {
        WGS84Coordinate{lat, lon}
    }

    // Conversie naar Rijksdriekhoekstelsel
    pub fn to_rdnew(&self) -> RDCoordinate {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        let rp: &[f64; 9] = &[0_f64, 1.0, 2.0, 0.0, 1.0, 3.0, 1.0, 0.0, 2.0];
        let rq: &[f64; 9] = &[1_f64, 1.0, 1.0, 3.0, 0.0, 1.0, 3.0, 2.0, 3.0];
        let rpq: &[f64; 9] = &[190094.945, -11832.228, -114.221, -32.391, -0.705, -2.340, -0.608, -0.008, 0.148];

        let sp: &[f64; 10] = &[1.0, 0.0, 2.0, 1.0, 3.0, 0.0, 2.0, 1.0, 0.0, 1.0];
        let sq: &[f64; 10] = &[0.0, 2.0, 0.0, 2.0, 0.0, 1.0, 2.0, 1.0, 4.0, 4.0];
        let spq: &[f64; 10] = &[309056.544, 3638.893, 73.077, -157.984, 59.788, 0.433, -6.439, -0.032, 0.092, -0.054];

        let lat = 0.36 * (self.lat - LAT0);
        let lon = 0.36 * (self.lon - LON0);

        for (i, _v) in rpq.iter().enumerate() {
            x = x + (rpq[i] * f64::powf(lat, rp[i]) * f64::powf(lon, rq[i])  )
        }

        for (i, _v) in spq.iter().enumerate() {
            y = y + (spq[i] * f64::powf(lat, sp[i]) * f64::powf(lon, sq[i])  )
        }

        return RDCoordinate{
            x: f64::round(X0 + x),
            y: f64::round(Y0 + y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgs84_to_rd() {
        let wgs84 = WGS84Coordinate{lat: 52.37214383811702, lon: 4.905597604352241};
        let rd = wgs84.to_rdnew();

        let wgs84_twee = WGS84Coordinate::new(53.310, 6.560);
        let rd_twee = wgs84_twee.to_rdnew();
        assert_eq!(122202_f64, rd.x);
        assert_eq!(487250_f64, rd.y);
        assert_eq!(233171_f64, rd_twee.x);
        assert_eq!(592141_f64, rd_twee.y)
    }
}
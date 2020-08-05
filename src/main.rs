use std::cmp::{max, min};
//pub use geo_types::{Coordinate};
use  std::f64::consts::PI;

const MAX_ZOOM:u8 = 31;
const MAX_LONGITUDE:f64 = 180.0;
const MAX_LATITUDE:f64 = 85.05112877980659;

const MIN_LONGITUDE:f64 = -MAX_LONGITUDE;
const MIN_LATITUDE:f64 = -MAX_LATITUDE;

const WEBMERCATOR_R:f64 = 6378137.0;

const XY_SCALE:f64 = 2147483648.0;
const INV_XY_SCALE:f64 = 1.0/XY_SCALE;
const WM_RANGE:f64 = 2.0*PI*WEBMERCATOR_R;
const INV_WM_RANGE:f64 = 1.0/WM_RANGE;
const WM_MAX:f64 = PI*WEBMERCATOR_R;

pub fn xy2quadint(mut x: i64,mut y: i64) -> i64 {

    const B:(i64, i64, i64, i64, i64) = (0x5555555555555555, 0x3333333333333333, 0x0F0F0F0F0F0F0F0F, 0x00FF00FF00FF00FF, 0x0000FFFF0000FFFF);
    const S:(i64, i64, i64, i64, i64) = (1, 2, 4, 8, 16);

    x = (x | (x << S.4)) & B.4;
    y = (y | (y << S.4)) & B.4;

    x = (x | (x << S.3)) & B.3;
    y = (y | (y << S.3)) & B.3;

    x = (x | (x << S.2)) & B.2;
    y = (y | (y << S.2)) & B.2;

    x = (x | (x << S.1)) & B.1;
    y = (y | (y << S.1)) & B.1;

    x = (x | (x << S.0)) & B.0;
    y = (y | (y << S.0)) & B.0;

    return x | (y << 1);

}

pub fn intquadxy(quadint: i64) ->  (i64, i64) {
    const B:(i64, i64, i64, i64, i64, i64) = (
        0x5555555555555555, 0x3333333333333333, 0x0F0F0F0F0F0F0F0F, 0x00FF00FF00FF00FF, 0x0000FFFF0000FFFF,
        0x00000000FFFFFFFF
    );
    const S:(i64, i64, i64, i64, i64, i64) =(0, 1, 2, 4, 8, 16 );

    let mut x = quadint;
    let mut y = quadint >> 1;

    x = (x | (x >> S.0)) & B.0;
    y = (y | (y >> S.0)) & B.0;

    x = (x | (x >> S.1)) & B.1;
    y = (y | (y >> S.1)) & B.1;

    x = (x | (x >> S.2)) & B.2;
    y = (y | (y >> S.2)) & B.2;

    x = (x | (x >> S.3)) & B.3;
    y = (y | (y >> S.3)) & B.3;

    x = (x | (x >> S.4)) & B.4;
    y = (y | (y >> S.4)) & B.4;

    x = (x | (x >> S.5)) & B.5;
    y = (y | (y >> S.5)) & B.5;

    return (x, y)
}

pub fn lonlat2xy(lon:f64, lat:f64) -> (i64, i64) {
  let _lon = MAX_LONGITUDE.min(MIN_LONGITUDE.max(lon));
  let _lat = MAX_LATITUDE.min(MIN_LATITUDE.max(lat));
  println!(" {:?}", _lon);
  println!("{:?}", _lat);

  let fx = (_lon+180.0)/360.0;
  let sinlat = (_lat * PI/180.0).sin();
  let fy = 0.5 - ((1.0+sinlat)/(1.0-sinlat)).log10() / (4.0*PI);
  println!("{:?}", fx);
  println!("{:?}", sinlat);
  println!("{:?}", fy);
  let mapsize = (1 << MAX_ZOOM) as f64;
  println!("{:?}", mapsize);
  let _x = (fx*mapsize).floor() as i64;
  println!("{:?}", _x);
  let _y = (fy*mapsize).floor() as i64;
  println!("{:?}", _y);
  let x = min(mapsize as i64 - 1, min(0, _x));
  let y = max(mapsize as i64 - 1, max(0, _y));
  return (x, y)

}

pub fn lonlat2quadint(lon:f64, lat:f64) -> i64 {
	let (x,y) = lonlat2xy(lon, lat);
	let quadint = xy2quadint(x,y);
	return quadint
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xy2quandint_test() {
        assert_eq!(57, xy2quadint(5,6));
    }
    #[test]
    fn intquadxy_test() {
    	assert_eq!((5,6), intquadxy(57));
    }
    #[test]
    fn lonlat2xy_test() {
    	let xy = lonlat2xy(-73.969558715820312, 40.757678985595703);
    	assert_eq!((632496219, 807059307), xy);
    }

    #[test]
    fn longlat2quadint_test() {
    	let quadint = lonlat2quadint(-73.969558715820312, 40.757678985595703);
    	assert_eq!(1013670064444553679, quadint);
    }
}
extern crate geo;

use nom::{be_u32, le_f64, le_u32};
use self::geo::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub file_code: u32,
    pub file_length: u32,
    pub version: u32,
    pub shape_type: u32,
    pub bbox_x_min: f64,
    pub bbox_y_min: f64,
    pub bbox_x_max: f64,
    pub bbox_y_max: f64,
    pub bbox_z_min: f64,
    pub bbox_z_max: f64,
    pub bbox_m_min: f64,
    pub bbox_m_max: f64,
}

named!(pub header<Header>,
    do_parse!(
        file_code:   be_u32    >>
                     take!(20) >>
        file_length: be_u32    >>
        version:     le_u32    >>
        shape_type:  le_u32    >>
        bbox_x_min:  le_f64    >>
        bbox_y_min:  le_f64    >>
        bbox_x_max:  le_f64    >>
        bbox_y_max:  le_f64    >>
        bbox_z_min:  le_f64    >>
        bbox_z_max:  le_f64    >>
        bbox_m_min:  le_f64    >>
        bbox_m_max:  le_f64    >>
        (Header {
            file_code:   file_code,
            file_length: file_length,
            version:     version,
            shape_type:  shape_type,
            bbox_x_min:  bbox_x_min,
            bbox_y_min:  bbox_y_min,
            bbox_x_max:  bbox_x_max,
            bbox_y_max:  bbox_y_max,
            bbox_z_min:  bbox_z_min,
            bbox_z_max:  bbox_z_max,
            bbox_m_min:  bbox_m_min,
            bbox_m_max:  bbox_m_max
        })
    )
);

#[derive(Clone, Debug, PartialEq)]
pub struct RecordHeader {
    pub record_number: u32,
    pub content_length: u32,
}

named!(pub record_header<RecordHeader>,
    do_parse!(
        record_number:  be_u32    >>
        content_length: be_u32    >>
        (RecordHeader {
            record_number: record_number,
            content_length: content_length
        })
    )
);

#[derive(Clone, Debug, PartialEq)]
pub struct NullShape {
    shape_type: u32,
}

named!(pub record_content_null<NullShape>,
    do_parse!(
        shape_type: le_u32 >>
        (NullShape {
            shape_type: shape_type
        })
    )
);

named!(pub record_content_point<Point<f64>>,
    do_parse!(
        shape_type: le_u32 >>
        x: le_f64 >>
        y: le_f64 >>
        (Point::new(x, y))
    )
);

#[allow(non_upper_case_globals)]
#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    const countries: &'static [u8] = include_bytes!("../assets/ne_110m_admin_0_countries.shp");
    const places: &'static [u8] = include_bytes!("../assets/ne_110m_populated_places.shp");

    #[test]
    fn headers() {
        assert_eq!(
            header(&countries[..100]),
            IResult::Done(
                &b""[..],
                Header {
                    file_code: 9994,
                    file_length: 90462,
                    version: 1000,
                    shape_type: 5,
                    bbox_x_min: -179.9999999999999716,
                    bbox_y_min: -90.0000000000000284,
                    bbox_x_max: 180.0000000000000284,
                    bbox_y_max: 83.6451300000000089,
                    bbox_z_min: 0.0,
                    bbox_z_max: 0.0,
                    bbox_m_min: 0.0,
                    bbox_m_max: 0.0,
                }
            )
        );
        assert_eq!(
            header(&places[..100]),
            IResult::Done(
                &b""[..],
                Header {
                    file_code: 9994,
                    file_length: 3452,
                    version: 1000,
                    shape_type: 1,
                    bbox_x_min: -175.22056447761656,
                    bbox_y_min: -41.29998785369173,
                    bbox_x_max: 179.21664709402887,
                    bbox_y_max: 64.15002361973922,
                    bbox_z_min: 0.0,
                    bbox_z_max: 0.0,
                    bbox_m_min: 0.0,
                    bbox_m_max: 0.0,
                }
            )
        );
    }

    #[test]
    fn record_headers() {
        assert_eq!(
            record_header(&countries[100..108]),
            IResult::Done(
                &b""[..],
                RecordHeader {
                    record_number: 1,
                    content_length: 576,
                }
            )
        );
        assert_eq!(
            record_header(&places[100..108]),
            IResult::Done(
                &b""[..],
                RecordHeader {
                    record_number: 1,
                    content_length: 10,
                }
            )
        );
    }

    #[test]
    fn record_content_points() {
        assert_eq!(
            record_content_point(&places[108..128]),
            IResult::Done(&b""[..], Point::new(12.453386544971766, 41.903282179960115))
        );
    }
}

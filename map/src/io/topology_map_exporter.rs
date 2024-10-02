use image::{ImageBuffer, Rgb, RgbImage};

use crate::{
    graph::graph::Graph,
    map::topology::{topology_edge::TopologyEdge, topology_node::TopologyNode},
};

type TopologyMap = Graph<TopologyNode, TopologyEdge>;

pub struct TopologyMapExporter;

impl TopologyMapExporter {
    pub fn export_topology_as_image(
        file_name: &str,
        topology_map: &TopologyMap,
        pixel_size: f64,
        margin_px: usize,
        draw_waypoints: bool,
    ) {
        let roi = TopologyMapExporter::get_roi(topology_map);
        let ((x1, y1), (x2, y2)) = roi;
        let image_width = ((x2 - x1) / pixel_size).ceil() as usize + 2 * margin_px;
        let image_height = ((y1 - y2) / pixel_size).ceil() as usize + 2 * margin_px;
        let mut img = RgbImage::new(image_width as u32, image_height as u32);

        for (edge_id, edge) in topology_map.get_edges().iter() {
            let node1 = topology_map.get_node_by_id(&edge.node1());
            let node2 = topology_map.get_node_by_id(&edge.node2());

            if draw_waypoints {
                TopologyMapExporter::draw_edge_waypoints(
                    &mut img,
                    &roi,
                    &node1.unwrap().node_info().position,
                    &node2.unwrap().node_info().position,
                    edge.edge_info().get_waypoints(),
                    margin_px,
                    pixel_size,
                )
            } else {
                TopologyMapExporter::draw_edge_straight_line(
                    &mut img,
                    &roi,
                    &node1.unwrap().node_info().position,
                    &node2.unwrap().node_info().position,
                    margin_px,
                    pixel_size,
                );
            }
        }

        img.save(file_name);
    }

    /// Get the RoI (Region of Interest).
    /// Return value is (x, y) pairs of top-left and down-right.
    fn get_roi(topology_map: &TopologyMap) -> ((f64, f64), (f64, f64)) {
        let mut top: f64 = std::f64::NEG_INFINITY;
        let mut bottom: f64 = std::f64::INFINITY;
        let mut left: f64 = std::f64::INFINITY;
        let mut right: f64 = std::f64::NEG_INFINITY;

        for (_, node) in topology_map.get_nodes().iter() {
            let pos = node.node_info().position;
            top = f64::max(top, pos.y);
            bottom = f64::min(bottom, pos.y);
            left = f64::min(left, pos.x);
            right = f64::max(right, pos.x);
        }

        for (_, edge) in topology_map.get_edges() {
            for waypoint in edge.edge_info().get_waypoints() {
                top = f64::max(top, waypoint.y);
                bottom = f64::min(bottom, waypoint.y);
                left = f64::min(left, waypoint.x);
                right = f64::max(right, waypoint.x);
            }
        }

        return ((left, top), (right, bottom));
    }

    fn draw_edge_straight_line(
        img: &mut RgbImage,
        roi: &((f64, f64), (f64, f64)),
        src_pos: &Vector2D,
        dst_pos: &Vector2D,
        margin_px: usize,
        pixel_size: f64,
    ) {
        let margin_vec = Vector2I::from_xy(margin_px as i64, margin_px as i64);
        let bottom_left = Vector2D::from_xy(roi.0 .0.clone(), roi.1 .1.clone());

        // Convert positions into pixel coordinate.
        let _src_pos = (src_pos - bottom_left) / pixel_size;
        let _dst_pos = (dst_pos - bottom_left) / pixel_size;
        let src_pixel = Vector2I::from_xy(_src_pos.x as i64, _src_pos.y as i64) + margin_vec;
        let dst_pixel = Vector2I::from_xy(_dst_pos.x as i64, _dst_pos.y as i64) + margin_vec;

        // Draw line segment of edge.
        imageproc::drawing::draw_line_segment_mut(
            img,
            (src_pixel.x as f32, src_pixel.y as f32),
            (dst_pixel.x as f32, dst_pixel.y as f32),
            Rgb([255, 255, 255]),
        );

        // Draw node on src pixel.
        imageproc::drawing::draw_filled_circle_mut(
            img,
            (src_pixel.x as i32, src_pixel.y as i32),
            0,
            Rgb([0, 255, 0]),
        );

        // Draw node on dst pixel.
        imageproc::drawing::draw_filled_circle_mut(
            img,
            (dst_pixel.x as i32, dst_pixel.y as i32),
            0,
            Rgb([0, 255, 0]),
        );
    }

    fn draw_edge_waypoints(
        img: &mut RgbImage,
        roi: &((f64, f64), (f64, f64)),
        pos1: &Vector2D,
        pos2: &Vector2D,
        waypoints: &Vec<Vector2D>,
        margin_px: usize,
        pixel_size: f64,
    ) {
        let margin_vec = Vector2I::from_xy(margin_px as i64, margin_px as i64);
        let bottom_left = Vector2D::from_xy(roi.0 .0.clone(), roi.1 .1.clone());

        for point in waypoints.iter() {
            let _point = (point - bottom_left) / pixel_size;
            let pixel: Vector2I = Vector2I::from_xy(_point.x as i64, _point.y as i64) + margin_vec;
            match img.get_pixel_mut_checked(pixel.x as u32, pixel.y as u32) {
                Some(px) => *px = Rgb([255, 255, 255]),
                None => {}
            };
        }

        let _pos1 = (pos1 - bottom_left) / pixel_size;
        let _pos2 = (pos2 - bottom_left) / pixel_size;
        let pixel1: Vector2I = Vector2I::from_xy(_pos1.x as i64, _pos1.y as i64) + margin_vec;
        let pixel2: Vector2I = Vector2I::from_xy(_pos2.x as i64, _pos2.y as i64) + margin_vec;

        imageproc::drawing::draw_filled_circle_mut(
            img,
            (pixel1.x as i32, pixel1.y as i32),
            0,
            Rgb([0, 255, 0]),
        );

        imageproc::drawing::draw_filled_circle_mut(
            img,
            (pixel2.x as i32, pixel2.y as i32),
            0,
            Rgb([0, 255, 0]),
        );
    }
}

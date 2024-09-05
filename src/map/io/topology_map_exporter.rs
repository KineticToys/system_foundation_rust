use image::{ImageBuffer, Rgb, RgbImage};

use crate::{
    graph::graph::Graph,
    map::topology::{topology_edge::TopologyEdge, topology_node::TopologyNode},
    math::numerics::{vector2d::Vector2D, vector2i::Vector2I},
};

pub struct TopologyMapExporter {
    xy_to_pixel_converter: Option<Box<dyn Fn(&Vector2D) -> Vector2I>>,
}

type TopologyMap = Graph<TopologyNode, TopologyEdge>;

impl TopologyMapExporter {
    pub fn new(xy_to_pixel_converter: Option<Box<dyn Fn(&Vector2D) -> Vector2I>>) -> Self {
        return Self {
            xy_to_pixel_converter: xy_to_pixel_converter,
        };
    }

    pub fn export_topology_as_image(
        &self,
        file_name: &str,
        topology_map: &TopologyMap,
        pixel_size: f64,
        margin_px: usize,
        draw_waypoints: bool,
    ) {
        let ((x1, y1), (x2, y2)) = TopologyMapExporter::get_roi(topology_map);
        let horizontal_pixels = ((x2 - x1) / pixel_size).ceil() as usize;
        let vertical_pixels = ((y1 - y2) / pixel_size).ceil() as usize;
        let image_width = horizontal_pixels + 2 * margin_px;
        let image_height = horizontal_pixels + 2 * margin_px;
        let mut img = RgbImage::new(image_width as u32, image_height as u32);

        for (edge_id, edge) in topology_map.get_edges().iter() {
            let node1 = topology_map.get_node_by_id(&edge.node1());
            let node2 = topology_map.get_node_by_id(&edge.node2());

            if draw_waypoints {
                self.draw_edge_waypoints(
                    &mut img,
                    &node1.unwrap().node_info().position,
                    &node2.unwrap().node_info().position,
                    edge.edge_info().get_waypoints(),
                )
            } else {
                self.draw_edge_straight_line(
                    &mut img,
                    &node1.unwrap().node_info().position,
                    &node2.unwrap().node_info().position,
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

        for (node_id, node) in topology_map.get_nodes().iter() {
            let pos = node.node_info().position;
            top = f64::max(top, pos.y);
            bottom = f64::min(bottom, pos.y);
            left = f64::min(left, pos.x);
            right = f64::max(right, pos.x);
        }

        return ((left, top), (right, bottom));
    }

    fn draw_edge_straight_line(&self, img: &mut RgbImage, src_pos: &Vector2D, dst_pos: &Vector2D) {
        // Convert positions into pixel coordinate.
        let src_pixel = match self.xy_to_pixel_converter.as_ref() {
            Some(cvt) => (cvt)(src_pos),
            None => src_pos.clone().into(),
        };
        let dst_pixel = match self.xy_to_pixel_converter.as_ref() {
            Some(cvt) => (cvt)(dst_pos),
            None => dst_pos.clone().into(),
        };
        // Draw node on src pixel.
        imageproc::drawing::draw_filled_circle(
            img,
            (src_pixel.x as i32, dst_pixel.y as i32),
            3,
            Rgb([0, 255, 0]),
        );

        // Draw node on dst pixel.
        imageproc::drawing::draw_filled_circle(
            img,
            (dst_pixel.x as i32, dst_pixel.y as i32),
            3,
            Rgb([0, 255, 0]),
        );

        // Draw line segment of edge.
        imageproc::drawing::draw_line_segment(
            img,
            (src_pixel.x as f32, src_pixel.y as f32),
            (dst_pixel.x as f32, dst_pixel.y as f32),
            Rgb([255, 0, 0]),
        );
    }

    fn draw_edge_waypoints(
        &self,
        img: &mut RgbImage,
        pos1: &Vector2D,
        pos2: &Vector2D,
        waypoints: &Vec<Vector2D>,
    ) {
    }
}

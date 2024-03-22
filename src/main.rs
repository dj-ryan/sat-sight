struct Triangel((f32, f32), (f32, f32), (f32, f32));

impl Triangel {
    fn area(&self) -> f32 {
        let (x1, y1) = self.0;
        let (x2, y2) = self.1;
        let (x3, y3) = self.2;
        ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs()) / 2.0
    }

}


fn main() {
    let triangel = Triangel((0.0, 0.0), (0.0, 1.0), (1.0, 0.0));
    println!("Area of triangel is {}", triangel.area()); 
}

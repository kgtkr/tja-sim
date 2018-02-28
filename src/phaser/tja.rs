//譜面
pub struct TJA {}

//音符
pub enum Note {
  Rest,
  Don,
  Ka,
  BigDon,
  BigKa,
  Renda(i32),
  BigRenda(i32),
  GekiRenda { time: i32, count: i32 },
}

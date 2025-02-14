import gleam/io
import gleam/list
import gleam/string

pub type Builder {
  Builder(fold: fn(List(String)) -> String)
}

const build_a = Builder(fold: string.concat)
const build_b = Builder(fold: list.fold(_, string.new(), string.append))

pub fn main() {

  io.println("Hello from issue_2177!")
}

use frunk::HCons;
use teloc::{Container, Dependency, Get, HList, Teloc};

#[derive(Clone)]
struct NumberServiceOptions(i32);

trait NumberService {
    fn get_num(&self) -> i32;
}

struct ConstService {
    number: i32,
}
impl Dependency<HList![NumberServiceOptions]> for ConstService {
    fn init(data: HList![NumberServiceOptions]) -> Self {
        let HCons { head: options, .. } = data;
        ConstService { number: options.0 }
    }
}
impl NumberService for ConstService {
    fn get_num(&self) -> i32 {
        self.number
    }
}

#[derive(Teloc)]
struct Controller<N: NumberService> {
    number_service: N,
}

#[test]
fn test() {
    let options = NumberServiceOptions(10);
    let mut container = Container::new()
        .add_instance(options)
        .add_transient::<ConstService>()
        .add_transient::<Controller<ConstService>>();
    let controller: Controller<_> = container.get();

    assert_eq!(controller.number_service.get_num(), 10);
}
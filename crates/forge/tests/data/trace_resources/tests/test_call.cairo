use snforge_std::{declare, ContractClassTrait};

use trace_resources::{
    trace_info_checker::{ITraceInfoCheckerDispatcherTrait, ITraceInfoCheckerDispatcher},
    trace_info_proxy::{ITraceInfoProxyDispatcherTrait, ITraceInfoProxyDispatcher}
};

#[test]
fn test_call() {
    let empty_hash = declare("Empty").class_hash;
    let proxy = declare("TraceInfoProxy");
    let checker = declare("TraceInfoChecker");
    let dummy = declare("TraceDummy");

    trace_resources::use_builtins_and_syscalls(empty_hash, 7);

    let checker_address = checker.deploy(@array![]).unwrap();
    let proxy_address = proxy
        .deploy(@array![checker_address.into(), empty_hash.into(), 5])
        .unwrap();
    let dummy_address = dummy.deploy(@array![]).unwrap();

    let proxy_dispatcher = ITraceInfoProxyDispatcher { contract_address: proxy_address };
    proxy_dispatcher.regular_call(checker_address, empty_hash, 1);
    proxy_dispatcher.with_libcall(checker.class_hash, empty_hash, 2);
    proxy_dispatcher.call_two(checker_address, dummy_address, empty_hash, 3);

    let chcecker_dispatcher = ITraceInfoCheckerDispatcher { contract_address: checker_address };
    chcecker_dispatcher.from_proxy(4, empty_hash, 4);
}
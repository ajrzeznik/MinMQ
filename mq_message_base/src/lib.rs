pub mod MQMessage_generated;
pub mod NodeAddress_generated;
pub mod PubSub_generated;
pub use NodeAddress_generated::com::ajrzeznik::*;
pub use MQMessage_generated::com::ajrzeznik::*;
pub use PubSub_generated::com::ajrzeznik::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

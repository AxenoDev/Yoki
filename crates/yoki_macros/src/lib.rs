mod attrs;
mod expand;
mod find_min_max_variants;
mod packet_in;
mod packet_out;
mod packet_report;
mod parsed_variant;
mod pvn_attribute;

use proc_macro::TokenStream;

#[proc_macro_derive(PacketIn, attributes(protocol, present_if))]
pub fn derive_packet_in(input: TokenStream) -> TokenStream {
    packet_in::expand(input)
}

#[proc_macro_derive(PacketOut, attributes(protocol, present_if))]
pub fn derive_packet_out(input: TokenStream) -> TokenStream {
    packet_out::expand(input)
}

#[proc_macro_derive(PacketReport, attributes(protocol_id))]
pub fn derive_packet_report(input: TokenStream) -> TokenStream {
    packet_report::expand(input)
}

#[proc_macro_derive(Pvn, attributes(pvn, default))]
pub fn protocol_version_derive(input: TokenStream) -> TokenStream {
    expand::expand_protocol_version_derive(input)
}

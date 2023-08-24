# Canonical ABI

From the perspective of Core WebAssembly running inside a component, the Component Model is an Embedding. 
As such, the Component Model defines the Core WebAssembly imports passed to module_instantiate 
and how Core WebAssembly exports are called via func_invoke. This allows the Component Model to specify 
how core modules are linked together (as shown above) but it also allows the Component Model to arbitrarily synthesize 
Core WebAssembly functions (via func_alloc) that are imported by Core WebAssembly. These synthetic core functions are 
created via one of several canonical definitions defined below.

To implement or call a component-level function, we need to cross a shared-nothing boundary. 
Traditionally, this problem is solved by defining a serialization format. The Component Model MVP uses roughly 
this same approach, defining a linear-memory-based ABI called the "Canonical ABI" which specifies, 
for any functype, a corresponding core:functype and rules for copying values into and out of linear memory. 
The Component Model differs from traditional approaches, though, in that the ABI is configurable, 
allowing multiple different memory representations of the same abstract value. 
In the MVP, this configurability is limited to the small set of canonopt shown below. However, 
Post-MVP, adapter functions could be added to allow far more programmatic control.

The Canonical ABI is explicitly applied to "wrap" existing functions in one of two directions:

lift wraps a core function (of type core:functype) to produce a component function (of type functype) that can be passed to other components.
lower wraps a component function (of type functype) to produce a core function (of type core:functype) 
that can be imported and called from Core WebAssembly code inside the current component.

The Canonical ABI specifies, for each component function signature, a corresponding core function signature 
and the process for reading component-level values into and out of linear memory.

An ABI is an application binary interface - an agreement on how to pass data around in a binary format. 
ABIs are specifically concerned with data layout at the bits-and-bytes level. 
For example, an ABI might define how integers are represented (big-endian or little-endian?), 
how strings are represented (pointer to null-terminated character sequence or length-prefixd? UTF-8 or UTF-16 encoded?), 
and how composite types are represented (the offsets of each field from the start of the structure).

The component model defines a canonical ABI - an ABI to which all components adhere. 
This guarantees that components can talk to each other without confusion, even if they are built in 
different languages. Internally, a C component might represent strings in a quite different way from 
a Rust component, but the canonical ABI provides a format for them to pass strings across the boundary between them.
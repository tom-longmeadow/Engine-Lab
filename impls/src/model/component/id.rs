 

use base::component_id_macro;
use base::prelude::ComponentId;

// Making a component ID of type name = IDu, primitive type = usize
component_id_macro!(IDu, usize);

// Making a component ID of type name = ID64, primitive type = u64
component_id_macro!(ID64, u64);


pub(crate) mod rover {
    use serde::{Deserialize, Serialize};

    use crate::{GridCoordinate, StructureType};

    /// Instructs the rover to move toward the given grid coordinates
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct SetDestination {
        pub target: GridCoordinate,
    }

    impl SetDestination {
        pub const TYPE: &str = "set_destination";
    }

    /// Instructs the rover to stop moving
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct Stop;

    impl Stop {
        pub const TYPE: &str = "stop";
    }

    /// Instructs the rover to start moving. If a destination has been set, the rover will move toward it.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct Start;

    impl Start {
        pub const TYPE: &str = "start";
    }

    /// Instructs the rover to begin the construction process at its given location. The rover's location
    /// must be buildable or this command will be rejected.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct BuildStructure {
        pub structure_id: String,
        pub structure_type: StructureType,
    }

    impl BuildStructure {
        pub const TYPE: &str = "build_structure";
    }

    /// Provisions/initializes a new Rover at the given location. If the Rover has an autopilot installed,
    /// the identifier for that should be supplied.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct ProvisionRover {
        /// The identifier of the mothership making this initialization request
        pub mothership_id: String,
        pub colony_id: String,
        pub autopilot_id: Option<String>,
        pub location: GridCoordinate,
    }

    impl ProvisionRover {
        pub const TYPE: &str = "provision_rover";
    }

    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct CancelConstruction {
        pub rover_id: String,
        pub reason: Option<String>,
    }

    impl CancelConstruction {
        pub const TYPE: &str = "cancel_construction";
    }
}

pub(crate) mod colony {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct EstablishColony {
        pub colony_id: String,
        pub moon_id: String,
        pub name: String,
        pub grid_height: u32,
        pub grid_width: u32,
    }
}

pub(crate) mod mothership {}

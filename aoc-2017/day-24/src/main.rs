struct Component {
    id: usize,
    ports: [u32; 2],
}

impl Component {
    fn is_compatible(&self, port: u32) -> bool {
        self.ports[0] == port || self.ports[1] == port
    }

    fn strength(&self) -> u32 {
        self.ports[0] + self.ports[1]
    }
}

#[derive(Clone)]
struct Bridge {
    /// List of used components, identified by their component id.
    used: Vec<bool>,

    /// Value of the exposed port of the bridge.
    port: u32,
}

impl Bridge {
    fn new(size: usize) -> Self {
        let used = vec![false; size];
        let port = 0;

        Self { used, port }
    }

    /// Computes the total strength of the bridge.
    ///
    /// The strength of a bridge is the sum of the strength
    /// of its components.
    fn strength(&self, components: &[Component]) -> u32 {
        (0..self.used.len())
            .filter_map(|idx| match self.used[idx] {
                true => Some(components[idx].strength()),
                false => None,
            })
            .sum()
    }

    /// Computes the length of the bridge.
    ///
    /// The length is the number of components used in the bridge.
    fn len(&self) -> usize {
        self.used.iter().filter(|&used| *used).count()
    }

    /// Checks whether a component can be added to the end of the bridge.
    ///
    /// This is the case if the component has at least one port with
    /// value matching the value of the bridge's exposed port, and if the
    /// component is available (i.e. not yet in use in the bridge).
    fn is_compatible(&self, component: &Component) -> bool {
        component.is_compatible(self.port) && !self.used[component.id]
    }

    /// Extends the bridge by adding a component.
    ///
    /// Returns a new bridge with the component added.
    fn extend(&self, component: &Component) -> Self {
        // TODO remove? we check before extending
        assert!(self.is_compatible(component));

        let mut bridge = self.clone();

        bridge.used[component.id] = true;

        bridge.port = if self.port == component.ports[0] {
            component.ports[1]
        } else {
            component.ports[0]
        };

        bridge
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Create the list of components.
    // The component ids are equal to the order in which the component appears in the input list.
    let components = input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut vec, (id, line)| {
            let ports: Vec<_> = line.split('/').map(|s| s.parse().unwrap()).collect();
            let ports = ports.try_into().unwrap();

            vec.push(Component { id, ports });

            vec
        });

    // store the properties of the completed bridges
    let mut strength_values = Vec::new();
    let mut length_values = Vec::new();

    // Build all possible bridges using depth-first search.
    // Start with a single with an empty bridge with exposed port 0.
    let mut stack: Vec<Bridge> = Vec::from([Bridge::new(components.len())]);

    while let Some(bridge) = stack.pop() {
        // get a list of available compatible components to extend the bridge
        let compat: Vec<_> = components
            .iter()
            .filter(|comp| bridge.is_compatible(comp))
            .collect();

        // if no component is available, the bridge is complete
        if compat.is_empty() {
            strength_values.push(bridge.strength(&components));
            length_values.push(bridge.len());
        }

        // add every possible extension of the bridge to the stack
        for comp in compat {
            let new_bridge = bridge.extend(comp);
            stack.push(new_bridge);
        }
    }

    // --- Part One --- //

    let part_one = *strength_values.iter().max().unwrap();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let max_length = length_values.iter().max().unwrap();

    let part_two = *strength_values
        .iter()
        .enumerate()
        .filter_map(|(idx, strength)| match length_values[idx] == *max_length {
            true => Some(strength),
            false => None,
        })
        .max()
        .unwrap();

    println!("Part Two: {}", part_two);
}

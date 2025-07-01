# Suggestions for Future Improvements

> [!NOTE]  
These are some rough outlines of things to be fixed or improved, which
don't have a natural place in code as either `todo!()` macros or `//
TODO` comments

## Known Bugs
+ Sliding window around local blocks
    + Currently only hovers around the node, even after moving outward
      (in loops), or expands out to the entire function (in CONDTL) 
+ First element insertions
    + In for loops, a node inserted at the top of the loop is not
      rendered in its correct spot (appears above the entire loop itself)
    + In conditionals, a node rendered right below the "yes" or "no"
      labels is not rendered in its correct spot (appears to the left of
      the label).

## UX Improvements
This is a list of improvements to the overall user experience:
+ Layout persistence
    + If the user drags nodes around, zooms in/out or changes the layout
      in any way from the default, those settings are lost immediately
      when performing any action to change the state of the `Canvas`.
    + This might require restructuring the way the `Canvas` is rendered:
        + Currently set to re-render on any Digraph change.
        + **Potential fix**: store persistent node positions and
          statically set them on future renders.
+ Run panel animations
    + When the run panel flips between panel mode and FAB mode, there
      should be an animation to indicate this change.
    + Future support for a full-blown panel (with the maximize button),
      should also support these animations.
+ Code mini-map
    + Display a mini-map of the digraph at the top right corner of the
      screen, similar to VSCode maps for more efficient navigation. 
> [!TIP]
> This also prevents excessive zoom outs when a new node is added.

![Excessively zoomed out UI][./public/ZoomedOutUI.png]

## Feature Ideas
This is a list of currently unplanned, unscheduled features:
+ Multi-language parsing
    + Part of the mission to make AuralStudio a singular platform for
      code sharing and exchanges, that is non-language dependent. 
    + Could provide support for slim languages.
        + E.g., Python, JavaScript, Go, Swift, etc.
        + Not languages like Rust or C++. 
+ Interactive Code Walk-through
    + Would allow you to zoom in on a single node or a set of nodes,
      and view their state update in real time as the program executes.  
        + Similar to a `gdb` or `lldb` powered visual debugger.
    + Requires support for code highlighting as well.
+ Clipboard Buffers 
    + Support for highlighting nodes, which would allow for:
        + Yanking
        + Cutting
        + Deleting
        + Pasting
    + Requires a new data `struct` for serializing buffers.
+ Step-based AI Agents
    + AI agents that can write code while keeping a log of the steps
      they follow to execute a task.
        + Those steps can then be rendered in a side panel with names
          corresponding to `Rattle` function names.
        + For instance, for a program to encrypt and decrypt Hill
          Ciphers, the agent might generate the following step
          hierarchy:
        ```
        Process Hill Cipher
        ├── Encrypt Hill -> encrpyt_hill()
        │   ├── Multiply Matrices -> matmul()
        │   └── Stream Text -> stream_text()
        └── Decrypt Hill -> decrypt_hill()
            ├── Multiply Matrices -> matmul()
            └── Invert Matrix -> inv_matrix()
                ├── Compute Determinant -> compute_det()
                └── Compute Adjoint -> compute_adj()
        ```
+ Sub-Function Symlinks
    + In the step hierarchy above, the `matmul()` function appears twice.
    + The two functions should behave identically, so it would be
      redundant for the user to define them twice.
    + However, the two definitions make the step hierarchy clearer.
        + With symlink support, the second instance of `matmul` should
          simply point back to the original function, without being a
          redeclaration.
        + This preserves the hierarchy and eliminates redundancy


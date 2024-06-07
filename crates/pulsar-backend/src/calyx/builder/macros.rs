// calyx `build_assignments!` but for `CalyxCell`s.
#[macro_export(local_inner_macros)]
macro_rules! build_assignments_2_aux {
    // Unguarded assignment.
    (@base $builder:expr;
     $dst_node:ident[$dst_port:expr] = ? $src_node:ident[$src_port:expr]) => {
        $builder.build_assignment(
            $dst_node.value.borrow().get($dst_port),
            $src_node.value.borrow().get($src_port),
            calyx_ir::Guard::True)
    };

    // Guarded assignment.
    (@base $builder:expr;
     $dst_node:ident[$dst_port:expr] =
        $guard:ident ?
        $src_node:ident[$src_port:expr]) => {
        $builder.build_assignment(
            $dst_node.value.borrow().get($dst_port),
            $src_node.value.borrow().get($src_port),
            $guard.clone())
    };

    ($builder:expr;
     $($dst_node:ident[$dst_port:expr] =
         $($guard:ident)? ?
         $src_node:ident[$src_port:expr];)*)  => {
        [$(
            build_assignments_2_aux!(@base $builder;
                $dst_node[$dst_port] = $($guard)? ? $src_node[$src_port])
        ),*]

    };
}

/// Behaves like [`calyx_ir::build_assignments!`] but takes in a
/// [`CalyxComponent`] instead of a [`calyx_ir::Builder`] and uses
/// [`CalyxCell`]s instead of `RRC<calyx_ir::Cell>`s.
#[macro_export(local_inner_macros)]
macro_rules! build_assignments_2 {
    ($component:expr; $($args:tt)*) => {
        $component.with_calyx_builder(|builder| {
            build_assignments_2_aux!(builder; $($args)*)
        })
    }
}

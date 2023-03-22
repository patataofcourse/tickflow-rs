#[macro_export]
macro_rules! tf_op_args {
    ($cmdname:literal $(<$arg0:literal>)?, [$(($argnum:literal$(, $special:literal)?)),* $(,)?] $(, $scene:literal)? $(,)?) => {
        {
        #[allow(unused_mut, unused_assignments)]
        let mut arg0 = None;
        $(arg0 = Some($arg0);)?

        #[allow(unused_mut, unused_assignments)]
        let mut scene = -1;
        $(scene = $scene;)?

        #[allow(unused_mut)]
        let mut args = vec![];
        $(
            #[allow(unused_mut)]
            let mut val = ($argnum, false);
            $(val.1 = $special;)?
            args.push(val);
        )*

        $crate::data::ArgsTickflowOp {
            op: $cmdname,
            arg0,
            args,
            scene,
            }
        }
    };
    ($cmdname:literal $(<$arg0:literal>)?$ (, $scene:literal)? $(,)?) => {
        {
            #[allow(unused_mut, unused_assignments)]
            let mut arg0 = None;
            $(arg0 = Some($arg0);)?

            #[allow(unused_mut, unused_assignments)]
            let mut scene = -1;
            $(scene = $scene;)?

            $crate::data::ArgsTickflowOp {
                op: $cmdname,
                arg0,
                args: vec![],
                scene,
            }
        }
    };
}
